//! Multi-language SBE encode/decode code generator from FSL AST.
//!
//! Mirrors the wire layout produced by [`crate::sbe_codegen::generate_sbe`].

use crate::ast::*;
use crate::sbe_codegen::{
    compute_block_length, pascal_case, sbe_base_type_to_wire_type, sbe_field_type_name_for_message,
    sbe_named_type_to_wire_type,
};

const SCHEMA_ID: u16 = 0x01;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbeTargetLang {
    Go,
    Cpp,
    Csharp,
    TypeScript,
    Zig,
}

pub fn generate_sbe_target(schema: &Schema, lang: SbeTargetLang) -> String {
    let mut out = String::new();
    let desc = schema.description.as_deref().unwrap_or(&schema.name);
    out.push_str(&file_header(schema, lang, desc));
    out.push_str(&wire_helpers(lang));
    out.push('\n');

    let structs = collect_struct_names(schema);
    let messages = collect_message_names(schema);
    let sets: NameSets = (structs.clone(), messages.clone());

    let (enum_types, inline_struct_types) = collect_schema_types(schema);
    for (name, variants) in &enum_types {
        out.push_str(&generate_target_enum(lang, name, variants));
        out.push('\n');
    }

    // Named struct type-defs (`type X { ... }`), then inline struct fields.
    for td in &schema.type_defs {
        if let Some(fields) = &td.fields {
            out.push_str(&generate_target_struct(
                lang, &td.name, fields, &td.name, &structs, &messages,
            ));
            out.push('\n');
        }
    }
    for (name, fields) in &inline_struct_types {
        out.push_str(&generate_target_inline_struct(lang, name, fields));
        out.push('\n');
    }

    for (template_id, msg) in schema.messages.iter().enumerate() {
        let tid = (template_id + 1) as u16;
        // Value struct for the message.
        out.push_str(&target_message_struct_decl(lang, msg, &structs, &messages));
        out.push('\n');
        out.push_str(&generate_target_message_body_encoder(lang, msg, &sets));
        out.push('\n');
        out.push_str(&generate_target_message_body_decoder(lang, msg, &sets));
        out.push('\n');
        out.push_str(&generate_target_message_encoder(lang, msg, tid, &sets));
        out.push('\n');
        out.push_str(&generate_target_message_decoder(lang, msg, tid, &sets));
        out.push('\n');
    }
    out.push_str(&file_footer(lang));
    out
}

/// Declare the value struct for a message (fields only).
fn target_message_struct_decl(
    lang: SbeTargetLang,
    msg: &Message,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    let name = &msg.name;
    let decls = target_field_decls(lang, name, &msg.fields, structs, messages);
    match lang {
        SbeTargetLang::Go => format!("// SBE message {name}\ntype {name} struct {{\n{decls}}}\n"),
        SbeTargetLang::Cpp => format!("// SBE message {name}\nstruct {name} {{\n{decls}}};\n"),
        SbeTargetLang::Csharp => {
            format!("    /// <summary>SBE message {name}</summary>\n    public class {name}\n    {{\n{decls}    }}\n")
        }
        SbeTargetLang::TypeScript => {
            format!("/** SBE message {name} */\nexport interface {name} {{\n{decls}}}\n")
        }
        SbeTargetLang::Zig => {
            format!("/// SBE message {name}\npub const {name} = struct {{\n{decls}}};\n")
        }
    }
}

type SbeEnumTypes = Vec<(String, Vec<String>)>;
type SbeInlineStructTypes = Vec<(String, Vec<Field>)>;

fn collect_schema_types(schema: &Schema) -> (SbeEnumTypes, SbeInlineStructTypes) {
    let mut enum_types = Vec::new();
    let mut seen_enums = std::collections::HashSet::new();
    let mut inline_struct_types = Vec::new();
    let mut seen_structs = std::collections::HashSet::new();

    for msg in &schema.messages {
        for field in &msg.fields {
            if let FieldType::Enum(enum_def) = &field.field_type {
                let enum_name = format!("{}{}", msg.name, pascal_case(&field.name));
                if seen_enums.insert(enum_name.clone()) {
                    enum_types.push((enum_name, enum_def.variants.clone()));
                }
            }
            if let FieldType::InlineStruct(fields) = &field.field_type {
                let struct_name = format!("{}{}", msg.name, pascal_case(&field.name));
                if seen_structs.insert(struct_name.clone()) {
                    inline_struct_types.push((struct_name, fields.clone()));
                }
            }
        }
    }
    for td in &schema.type_defs {
        if let Some(fields) = &td.fields {
            for field in fields {
                if let FieldType::Enum(enum_def) = &field.field_type {
                    let enum_name = format!("{}{}", td.name, pascal_case(&field.name));
                    if seen_enums.insert(enum_name.clone()) {
                        enum_types.push((enum_name, enum_def.variants.clone()));
                    }
                }
                if let FieldType::InlineStruct(fields) = &field.field_type {
                    let struct_name = format!("{}{}", td.name, pascal_case(&field.name));
                    if seen_structs.insert(struct_name.clone()) {
                        inline_struct_types.push((struct_name, fields.clone()));
                    }
                }
            }
        }
    }
    (enum_types, inline_struct_types)
}

fn file_header(schema: &Schema, lang: SbeTargetLang, desc: &str) -> String {
    let mut out = match lang {
        SbeTargetLang::Go => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' {}\n// {}\n\npackage figsbe\n\nimport (\n\t\"fmt\"\n\t\"math\"\n)\n\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::Cpp => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' {}\n// {}\n\n#pragma once\n\n#include <cstdint>\n#include <cstring>\n#include <optional>\n#include <stdexcept>\n#include <string>\n#include <vector>\n\nnamespace fig::sbe {{\n\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::Csharp => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' {}\n// {}\n\nusing System;\nusing System.Collections.Generic;\nusing System.Text;\n\nnamespace Fig.Sbe\n{{\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::TypeScript => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' {}\n// {}\n\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::Zig => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' {}\n// {}\n\nconst std = @import(\"std\");\n\n",
            schema.name, schema.version, desc
        ),
    };
    out.push_str(&schema_id_const(lang));
    out
}

fn file_footer(lang: SbeTargetLang) -> String {
    match lang {
        SbeTargetLang::Cpp | SbeTargetLang::Csharp => "}\n".to_string(),
        _ => String::new(),
    }
}

fn schema_id_const(lang: SbeTargetLang) -> String {
    match lang {
        SbeTargetLang::Go => format!("const SCHEMA_ID uint16 = 0x{:02X}\n\n", SCHEMA_ID),
        SbeTargetLang::Cpp => format!("constexpr uint16_t SCHEMA_ID = 0x{:02X};\n\n", SCHEMA_ID),
        SbeTargetLang::Csharp => format!(
            "    public const ushort SCHEMA_ID = 0x{:02X};\n\n",
            SCHEMA_ID
        ),
        SbeTargetLang::TypeScript => {
            format!("export const SCHEMA_ID = 0x{:02X} as const;\n\n", SCHEMA_ID)
        }
        SbeTargetLang::Zig => format!("pub const SCHEMA_ID: u16 = 0x{:02X};\n\n", SCHEMA_ID),
    }
}

fn wire_helpers(lang: SbeTargetLang) -> String {
    match lang {
        SbeTargetLang::Go => {
            "func writeU16BE(buf *[]byte, v uint16) {\n\t*buf = append(*buf, byte(v>>8), byte(v))\n}\n\nfunc writeU32BE(buf *[]byte, v uint32) {\n\t*buf = append(*buf, byte(v>>24), byte(v>>16), byte(v>>8), byte(v))\n}\n\nfunc writeU64BE(buf *[]byte, v uint64) {\n\t*buf = append(*buf, byte(v>>56), byte(v>>48), byte(v>>40), byte(v>>32), byte(v>>24), byte(v>>16), byte(v>>8), byte(v))\n}\n\nfunc writeF64BE(buf *[]byte, v float64) { writeU64BE(buf, math.Float64bits(v)) }\nfunc writeI64BE(buf *[]byte, v int64) { writeU64BE(buf, uint64(v)) }\n\nfunc writeString(buf *[]byte, s string) {\n\twriteU16BE(buf, uint16(len(s)))\n\t*buf = append(*buf, []byte(s)...)\n}\n\nfunc readU8(buf []byte, pos *int) uint8 {\n\tv := buf[*pos]\n\t*pos = *pos + 1\n\treturn v\n}\n\nfunc readU16BE(buf []byte, pos *int) uint16 {\n\tv := uint16(buf[*pos])<<8 | uint16(buf[*pos+1])\n\t*pos += 2\n\treturn v\n}\n\nfunc readU32BE(buf []byte, pos *int) uint32 {\n\tv := uint32(buf[*pos])<<24 | uint32(buf[*pos+1])<<16 | uint32(buf[*pos+2])<<8 | uint32(buf[*pos+3])\n\t*pos += 4\n\treturn v\n}\n\nfunc readU64BE(buf []byte, pos *int) uint64 {\n\tv := uint64(buf[*pos])<<56 | uint64(buf[*pos+1])<<48 | uint64(buf[*pos+2])<<40 | uint64(buf[*pos+3])<<32 |\n\t\tuint64(buf[*pos+4])<<24 | uint64(buf[*pos+5])<<16 | uint64(buf[*pos+6])<<8 | uint64(buf[*pos+7])\n\t*pos += 8\n\treturn v\n}\n\nfunc readF64BE(buf []byte, pos *int) float64 { return math.Float64frombits(readU64BE(buf, pos)) }\nfunc readI64BE(buf []byte, pos *int) int64 { return int64(readU64BE(buf, pos)) }\n\nfunc readString(buf []byte, pos *int) string {\n\tn := int(readU16BE(buf, pos))\n\ts := string(buf[*pos : *pos+n])\n\t*pos += n\n\treturn s\n}\n".to_string()
        }
        SbeTargetLang::Cpp => r#"inline uint8_t take_u8(const std::vector<uint8_t>& buf, std::size_t& pos) { return buf[pos++]; }
inline float f32_from_bits(uint32_t bits) { float v; std::memcpy(&v, &bits, 4); return v; }
inline uint32_t f32_bits(float v) { uint32_t bits; std::memcpy(&bits, &v, 4); return bits; }

inline void write_u16_be(std::vector<uint8_t>& buf, uint16_t v) {
    buf.push_back(static_cast<uint8_t>(v >> 8));
    buf.push_back(static_cast<uint8_t>(v));
}
inline void write_u32_be(std::vector<uint8_t>& buf, uint32_t v) {
    buf.push_back(static_cast<uint8_t>(v >> 24));
    buf.push_back(static_cast<uint8_t>(v >> 16));
    buf.push_back(static_cast<uint8_t>(v >> 8));
    buf.push_back(static_cast<uint8_t>(v));
}
inline void write_u64_be(std::vector<uint8_t>& buf, uint64_t v) {
    for (int i = 7; i >= 0; --i) buf.push_back(static_cast<uint8_t>((v >> (i * 8)) & 0xFF));
}
inline void write_f64_be(std::vector<uint8_t>& buf, double v) {
    uint64_t bits; std::memcpy(&bits, &v, sizeof(bits)); write_u64_be(buf, bits);
}
inline void write_i64_be(std::vector<uint8_t>& buf, int64_t v) { write_u64_be(buf, static_cast<uint64_t>(v)); }
inline void write_string(std::vector<uint8_t>& buf, const std::string& s) {
    write_u16_be(buf, static_cast<uint16_t>(s.size()));
    buf.insert(buf.end(), s.begin(), s.end());
}
inline uint16_t read_u16_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint16_t v = (static_cast<uint16_t>(buf[pos]) << 8) | buf[pos + 1]; pos += 2; return v;
}
inline uint32_t read_u32_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint32_t v = (static_cast<uint32_t>(buf[pos]) << 24) | (static_cast<uint32_t>(buf[pos + 1]) << 16) |
                 (static_cast<uint32_t>(buf[pos + 2]) << 8) | buf[pos + 3]; pos += 4; return v;
}
inline uint64_t read_u64_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint64_t v = 0; for (int i = 0; i < 8; ++i) v = (v << 8) | buf[pos + i]; pos += 8; return v;
}
inline double read_f64_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint64_t bits = read_u64_be(buf, pos); double v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
inline int64_t read_i64_be(const std::vector<uint8_t>& buf, size_t& pos) { return static_cast<int64_t>(read_u64_be(buf, pos)); }
inline std::string read_string(const std::vector<uint8_t>& buf, size_t& pos) {
    uint16_t len = read_u16_be(buf, pos); std::string s(reinterpret_cast<const char*>(&buf[pos]), len); pos += len; return s;
}
"#.to_string(),
        SbeTargetLang::Csharp => r#"
        public static byte TakeU8(byte[] buf, ref int pos) => buf[pos++];
    internal static class Wire
    {
        public static void WriteU16BE(List<byte> buf, ushort v) { buf.Add((byte)(v >> 8)); buf.Add((byte)v); }
        public static void WriteU32BE(List<byte> buf, uint v) {
            buf.Add((byte)(v >> 24)); buf.Add((byte)(v >> 16)); buf.Add((byte)(v >> 8)); buf.Add((byte)v);
        }
        public static void WriteU64BE(List<byte> buf, ulong v) {
            for (int i = 7; i >= 0; i--) buf.Add((byte)((v >> (i * 8)) & 0xFF));
        }
        public static void WriteF64BE(List<byte> buf, double v) => WriteU64BE(buf, BitConverter.DoubleToUInt64Bits(v));
        public static void WriteI64BE(List<byte> buf, long v) => WriteU64BE(buf, (ulong)v);
        public static void WriteString(List<byte> buf, string s) {
            var bytes = Encoding.UTF8.GetBytes(s); WriteU16BE(buf, (ushort)bytes.Length); buf.AddRange(bytes);
        }
        public static byte TakeU8(byte[] buf, ref int pos) => buf[pos++];

        public static ushort ReadU16BE(byte[] buf, ref int pos) {
            ushort v = (ushort)((buf[pos] << 8) | buf[pos + 1]); pos += 2; return v;
        }
        public static uint ReadU32BE(byte[] buf, ref int pos) {
            uint v = ((uint)buf[pos] << 24) | ((uint)buf[pos + 1] << 16) | ((uint)buf[pos + 2] << 8) | buf[pos + 3];
            pos += 4; return v;
        }
        public static ulong ReadU64BE(byte[] buf, ref int pos) {
            ulong v = 0; for (int i = 0; i < 8; i++) v = (v << 8) | buf[pos + i]; pos += 8; return v;
        }
        public static double ReadF64BE(byte[] buf, ref int pos) => BitConverter.Int64BitsToDouble((long)ReadU64BE(buf, ref pos));
        public static long ReadI64BE(byte[] buf, ref int pos) => (long)ReadU64BE(buf, ref pos);
        public static string ReadString(byte[] buf, ref int pos) {
            ushort len = ReadU16BE(buf, ref pos); var s = Encoding.UTF8.GetString(buf, pos, len); pos += len; return s;
        }
    }
"#.to_string(),
        SbeTargetLang::TypeScript => r#"function readU8(buf: Uint8Array, pos: { value: number }): number { return buf[pos.value++]; }
function f32Bits(v: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setFloat32(0, v, false); return dv.getUint32(0, false); }
function f32FromBits(bits: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setUint32(0, bits >>> 0, false); return dv.getFloat32(0, false); }
function writeU16BE(buf: number[], v: number): void { buf.push((v >> 8) & 0xff, v & 0xff); }
function writeU32BE(buf: number[], v: number): void { buf.push((v >> 24) & 0xff, (v >> 16) & 0xff, (v >> 8) & 0xff, v & 0xff); }
function writeU64BE(buf: number[], v: bigint): void { for (let i = 7; i >= 0; i--) buf.push(Number((v >> BigInt(i * 8)) & 0xffn)); }
function writeF64BE(buf: number[], v: number): void { const dv = new DataView(new ArrayBuffer(8)); dv.setFloat64(0, v, false); for (let i = 0; i < 8; i++) buf.push(dv.getUint8(i)); }
function writeI64BE(buf: number[], v: bigint): void { writeU64BE(buf, BigInt.asUintN(64, v)); }
function writeString(buf: number[], s: string): void { const bytes = new TextEncoder().encode(s); writeU16BE(buf, bytes.length); for (const b of bytes) buf.push(b); }
function readU8(buf: Uint8Array, pos: { value: number }): number { return buf[pos.value++]; }
function f32Bits(v: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setFloat32(0, v, false); return dv.getUint32(0, false); }
function f32FromBits(bits: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setUint32(0, bits >>> 0, false); return dv.getFloat32(0, false); }

function readU16BE(buf: Uint8Array, pos: { value: number }): number { const v = (buf[pos.value] << 8) | buf[pos.value + 1]; pos.value += 2; return v; }
function readU32BE(buf: Uint8Array, pos: { value: number }): number { const v = (buf[pos.value] << 24) | (buf[pos.value + 1] << 16) | (buf[pos.value + 2] << 8) | buf[pos.value + 3]; pos.value += 4; return v >>> 0; }
function readU64BE(buf: Uint8Array, pos: { value: number }): bigint { let v = 0n; for (let i = 0; i < 8; i++) v = (v << 8n) | BigInt(buf[pos.value + i]); pos.value += 8; return v; }
function readF64BE(buf: Uint8Array, pos: { value: number }): number { const dv = new DataView(buf.buffer, buf.byteOffset + pos.value, 8); const v = dv.getFloat64(0, false); pos.value += 8; return v; }
function readI64BE(buf: Uint8Array, pos: { value: number }): bigint { return readU64BE(buf, pos); }
function readString(buf: Uint8Array, pos: { value: number }): string { const len = readU16BE(buf, pos); const s = new TextDecoder().decode(buf.subarray(pos.value, pos.value + len)); pos.value += len; return s; }
"#.to_string(),
        SbeTargetLang::Zig => r#"fn readU8(buf: []const u8, pos: *usize) u8 { const v = buf[pos.*]; pos.* += 1; return v; }
fn writeU16BE(buf: *std.ArrayList(u8), v: u16) !void { try buf.append(@intCast(v >> 8)); try buf.append(@intCast(v & 0xFF)); }
fn writeU32BE(buf: *std.ArrayList(u8), v: u32) !void { try buf.append(@intCast(v >> 24)); try buf.append(@intCast(v >> 16)); try buf.append(@intCast(v >> 8)); try buf.append(@intCast(v & 0xFF)); }
fn writeU64BE(buf: *std.ArrayList(u8), v: u64) !void { var i: i32 = 7; while (i >= 0) : (i -= 1) try buf.append(@intCast((v >> @intCast(i * 8)) & 0xFF)); }
fn writeF64BE(buf: *std.ArrayList(u8), v: f64) !void { try writeU64BE(buf, @bitCast(v)); }
fn writeI64BE(buf: *std.ArrayList(u8), v: i64) !void { try writeU64BE(buf, @bitCast(v)); }
fn writeString(buf: *std.ArrayList(u8), s: []const u8) !void { try writeU16BE(buf, @intCast(s.len)); try buf.appendSlice(s); }
fn readU16BE(buf: []const u8, pos: *usize) u16 { const v = (@as(u16, buf[pos.*]) << 8) | buf[pos.* + 1]; pos.* += 2; return v; }
fn readU32BE(buf: []const u8, pos: *usize) u32 { const v = (@as(u32, buf[pos.*]) << 24) | (@as(u32, buf[pos.* + 1]) << 16) | (@as(u32, buf[pos.* + 2]) << 8) | buf[pos.* + 3]; pos.* += 4; return v; }
fn readU64BE(buf: []const u8, pos: *usize) u64 { var v: u64 = 0; var i: usize = 0; while (i < 8) : (i += 1) v = (v << 8) | buf[pos.* + i]; pos.* += 8; return v; }
fn readF64BE(buf: []const u8, pos: *usize) f64 { return @bitCast(readU64BE(buf, pos)); }
fn readI64BE(buf: []const u8, pos: *usize) i64 { return @bitCast(readU64BE(buf, pos)); }
fn readString(buf: []const u8, pos: *usize, allocator: std.mem.Allocator) ![]u8 { const len = readU16BE(buf, pos); const s = try allocator.dupe(u8, buf[pos.* .. pos.* + len]); pos.* += len; return s; }
"#.to_string(),
    }
}

/// Field type with struct/message awareness.
fn target_field_type_ctx(
    lang: SbeTargetLang,
    ft: &FieldType,
    msg_name: &str,
    field_name: &str,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    match ft {
        FieldType::Enum(_) | FieldType::InlineStruct(_) => {
            sbe_field_type_name_for_message(ft, msg_name, field_name)
        }
        FieldType::Named(n) if is_struct_name(n, structs) || is_message_name(n, messages) => {
            n.clone()
        }
        FieldType::Named(n) => map_named_type(lang, n),
        FieldType::InlineBase(bt, _) => map_named_type(lang, &sbe_base_type_to_wire_type(bt)),
        FieldType::List(inner) => {
            let inner_name =
                target_field_type_ctx(lang, inner, msg_name, field_name, structs, messages);
            match lang {
                SbeTargetLang::Go => format!("[]{inner_name}"),
                SbeTargetLang::Cpp => format!("std::vector<{inner_name}>"),
                SbeTargetLang::Csharp => format!("List<{inner_name}>"),
                SbeTargetLang::TypeScript => format!("{inner_name}[]"),
                SbeTargetLang::Zig => format!("[]{inner_name}"),
            }
        }
    }
}

fn map_named_type(lang: SbeTargetLang, name: &str) -> String {
    match sbe_named_type_to_wire_type(name).as_str() {
        "str" => match lang {
            SbeTargetLang::Go => "string".into(),
            SbeTargetLang::Cpp => "std::string".into(),
            SbeTargetLang::Csharp => "string".into(),
            SbeTargetLang::TypeScript => "string".into(),
            SbeTargetLang::Zig => "[]const u8".into(),
        },
        "f64" => match lang {
            SbeTargetLang::Go => "float64".into(),
            SbeTargetLang::Cpp => "double".into(),
            SbeTargetLang::Csharp => "double".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "f64".into(),
        },
        "f32" => match lang {
            SbeTargetLang::Go => "float32".into(),
            SbeTargetLang::Cpp => "float".into(),
            SbeTargetLang::Csharp => "float".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "f32".into(),
        },
        "i64" => match lang {
            SbeTargetLang::Go => "int64".into(),
            SbeTargetLang::Cpp => "int64_t".into(),
            SbeTargetLang::Csharp => "long".into(),
            SbeTargetLang::TypeScript => "bigint".into(),
            SbeTargetLang::Zig => "i64".into(),
        },
        "u64" => match lang {
            SbeTargetLang::Go => "uint64".into(),
            SbeTargetLang::Cpp => "uint64_t".into(),
            SbeTargetLang::Csharp => "ulong".into(),
            SbeTargetLang::TypeScript => "bigint".into(),
            SbeTargetLang::Zig => "u64".into(),
        },
        "i32" => match lang {
            SbeTargetLang::Go => "int32".into(),
            SbeTargetLang::Cpp => "int32_t".into(),
            SbeTargetLang::Csharp => "int".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "i32".into(),
        },
        "u32" => match lang {
            SbeTargetLang::Go => "uint32".into(),
            SbeTargetLang::Cpp => "uint32_t".into(),
            SbeTargetLang::Csharp => "uint".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "u32".into(),
        },
        "i16" => match lang {
            SbeTargetLang::Go => "int16".into(),
            SbeTargetLang::Cpp => "int16_t".into(),
            SbeTargetLang::Csharp => "short".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "i16".into(),
        },
        "u16" => match lang {
            SbeTargetLang::Go => "uint16".into(),
            SbeTargetLang::Cpp => "uint16_t".into(),
            SbeTargetLang::Csharp => "ushort".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "u16".into(),
        },
        "u8" | "bool" => match lang {
            SbeTargetLang::Go => "uint8".into(),
            SbeTargetLang::Cpp => "uint8_t".into(),
            SbeTargetLang::Csharp => "byte".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "u8".into(),
        },
        other => panic!("sbe target codegen: no {lang:?} mapping for wire type {other:?}"),
    }
}

fn target_optional_type(lang: SbeTargetLang, base: &str) -> String {
    match lang {
        SbeTargetLang::Go => format!("*{base}"),
        SbeTargetLang::Cpp => format!("std::optional<{base}>"),
        SbeTargetLang::Csharp => format!("{base}?"),
        SbeTargetLang::TypeScript => format!("{base} | null"),
        SbeTargetLang::Zig => format!("?{base}"),
    }
}

fn target_param_name(lang: SbeTargetLang, name: &str) -> String {
    match lang {
        SbeTargetLang::Go | SbeTargetLang::Csharp => pascal_case(name),
        _ => name.to_string(),
    }
}

/// Declare one function parameter in the target language's syntax.
///
/// Go, C#, and C++ declare `Type name`; TypeScript and Zig declare `name: Type`.
fn target_param_decl(lang: SbeTargetLang, name: &str, ty: &str) -> String {
    match lang {
        SbeTargetLang::Zig | SbeTargetLang::TypeScript => format!("{name}: {ty}"),
        SbeTargetLang::Cpp => format!("{ty} {name}"),
        SbeTargetLang::Go => format!("{} {ty}", target_param_name(lang, name)),
        SbeTargetLang::Csharp => format!("{} {ty}", target_param_name(lang, name)),
    }
}

fn target_encoder_params(
    lang: SbeTargetLang,
    msg: &Message,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    msg.fields
        .iter()
        .map(|field| {
            let base = target_field_type_full(
                lang,
                &field.field_type,
                &msg.name,
                &field.name,
                structs,
                messages,
            );
            let ty = if field.optional {
                target_optional_type(lang, &base)
            } else {
                base
            };
            target_param_decl(lang, &field.name, &ty)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_target_enum(lang: SbeTargetLang, name: &str, variants: &[String]) -> String {
    match lang {
        SbeTargetLang::Go => {
            let mut out = format!("type {name} uint8\n\nconst (\n");
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!("\t{name}{} {name} = {}\n", pascal_case(v), i + 1));
            }
            out.push_str(")\n\n");
            out.push_str(&format!(
                "func {name}FromValue(v uint8) ({name}, error) {{\n\tswitch v {{\n"
            ));
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!(
                    "\tcase {}: return {name}{}, nil\n",
                    i + 1,
                    pascal_case(v)
                ));
            }
            out.push_str(&format!("\tdefault: return 0, fmt.Errorf(\"invalid {name} value: %d\", v)\n\t}}\n}}\n\nfunc (e {name}) ToValue() uint8 {{ return uint8(e) }}\n"));
            out
        }
        SbeTargetLang::Cpp => {
            let mut out = format!("enum class {name} : uint8_t {{\n");
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!("    {} = {},\n", pascal_case(v), i + 1));
            }
            out.push_str("};\n\n");
            out.push_str(&format!(
                "inline std::optional<{name}> {name}FromValue(uint8_t v) {{\n    switch (v) {{\n"
            ));
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!(
                    "    case {}: return {name}::{};\n",
                    i + 1,
                    pascal_case(v)
                ));
            }
            out.push_str(&format!("    default: return std::nullopt;\n    }}\n}}\n\ninline uint8_t {name}ToValue({name} e) {{ return static_cast<uint8_t>(e); }}\n"));
            out
        }
        SbeTargetLang::Csharp => {
            let mut out = format!("    public enum {name} : byte\n    {{\n");
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!("        {} = {},\n", pascal_case(v), i + 1));
            }
            out.push_str("    }\n\n");
            out.push_str(&format!(
                "    public static {name}? {name}FromValue(byte v) => v switch {{\n"
            ));
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!(
                    "        {} => {name}.{},\n",
                    i + 1,
                    pascal_case(v)
                ));
            }
            out.push_str("        _ => null,\n    };\n\n");
            out.push_str(&format!(
                "    public static byte {name}ToValue({name} e) => (byte)e;\n"
            ));
            out
        }
        SbeTargetLang::TypeScript => {
            let mut out = format!("export enum {name} {{\n");
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!("  {} = {},\n", pascal_case(v), i + 1));
            }
            out.push_str("}\n\n");
            out.push_str(&format!(
                "export function {name}FromValue(v: number): {name} | null {{\n  switch (v) {{\n"
            ));
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!(
                    "    case {}: return {name}.{};\n",
                    i + 1,
                    pascal_case(v)
                ));
            }
            out.push_str("    default: return null;\n  }\n}\n\n");
            out.push_str(&format!(
                "export function {name}ToValue(e: {name}): number {{ return e; }}\n"
            ));
            out
        }
        SbeTargetLang::Zig => {
            let mut out = format!("pub const {name} = enum(u8) {{\n");
            for v in variants {
                out.push_str(&format!("    {},\n", pascal_case(v)));
            }
            out.push_str("};\n\n");
            out.push_str(&format!(
                "pub fn {name}FromValue(v: u8) !{name} {{\n    return switch (v) {{\n"
            ));
            for (i, v) in variants.iter().enumerate() {
                out.push_str(&format!("        {} => .{},\n", i + 1, pascal_case(v)));
            }
            out.push_str("        else => error.InvalidEnumValue,\n    };\n}\n\n");
            out.push_str(&format!(
                "pub fn {name}ToValue(e: {name}) u8 {{ return @intFromEnum(e); }}\n"
            ));
            out
        }
    }
}

// ── Scalar kinds and per-language wire I/O ────────────────────
//
// The wire layout mirrors `crate::sbe_codegen::generate_sbe` exactly:
// fixed-width scalars are written big-endian, optional scalars write the value
// followed by a presence byte, optional strings write the presence byte first,
// and lists write a u32 count followed by (u32 length + body) per variable-length
// item or raw big-endian bytes per fixed-width item.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScalarKind {
    F64,
    F32,
    I64,
    U64,
    U32,
    I32,
    U16,
    I16,
    U8,
}

impl ScalarKind {
    fn from_wire_token(token: &str) -> Option<Self> {
        Some(match token {
            "f64" => ScalarKind::F64,
            "f32" => ScalarKind::F32,
            "i64" => ScalarKind::I64,
            "u64" => ScalarKind::U64,
            "u32" => ScalarKind::U32,
            "i32" => ScalarKind::I32,
            "u16" => ScalarKind::U16,
            "i16" => ScalarKind::I16,
            "u8" | "bool" => ScalarKind::U8,
            _ => return None,
        })
    }

    /// Language type used for this scalar.
    fn lang_type(self, lang: SbeTargetLang) -> String {
        match (self, lang) {
            (ScalarKind::F64, SbeTargetLang::Go) => "float64",
            (ScalarKind::F64, SbeTargetLang::Cpp) => "double",
            (ScalarKind::F64, SbeTargetLang::Csharp) => "double",
            (ScalarKind::F64, SbeTargetLang::TypeScript) => "number",
            (ScalarKind::F64, SbeTargetLang::Zig) => "f64",
            (ScalarKind::F32, SbeTargetLang::Go) => "float32",
            (ScalarKind::F32, SbeTargetLang::Cpp) => "float",
            (ScalarKind::F32, SbeTargetLang::Csharp) => "float",
            (ScalarKind::F32, SbeTargetLang::TypeScript) => "number",
            (ScalarKind::F32, SbeTargetLang::Zig) => "f32",
            (ScalarKind::I64, SbeTargetLang::Go) => "int64",
            (ScalarKind::I64, SbeTargetLang::Cpp) => "int64_t",
            (ScalarKind::I64, SbeTargetLang::Csharp) => "long",
            (ScalarKind::I64, SbeTargetLang::TypeScript) => "bigint",
            (ScalarKind::I64, SbeTargetLang::Zig) => "i64",
            (ScalarKind::U64, SbeTargetLang::Go) => "uint64",
            (ScalarKind::U64, SbeTargetLang::Cpp) => "uint64_t",
            (ScalarKind::U64, SbeTargetLang::Csharp) => "ulong",
            (ScalarKind::U64, SbeTargetLang::TypeScript) => "bigint",
            (ScalarKind::U64, SbeTargetLang::Zig) => "u64",
            (ScalarKind::U32, SbeTargetLang::Go) => "uint32",
            (ScalarKind::U32, SbeTargetLang::Cpp) => "uint32_t",
            (ScalarKind::U32, SbeTargetLang::Csharp) => "uint",
            (ScalarKind::U32, SbeTargetLang::TypeScript) => "number",
            (ScalarKind::U32, SbeTargetLang::Zig) => "u32",
            (ScalarKind::I32, SbeTargetLang::Go) => "int32",
            (ScalarKind::I32, SbeTargetLang::Cpp) => "int32_t",
            (ScalarKind::I32, SbeTargetLang::Csharp) => "int",
            (ScalarKind::I32, SbeTargetLang::TypeScript) => "number",
            (ScalarKind::I32, SbeTargetLang::Zig) => "i32",
            (ScalarKind::U16, SbeTargetLang::Go) => "uint16",
            (ScalarKind::U16, SbeTargetLang::Cpp) => "uint16_t",
            (ScalarKind::U16, SbeTargetLang::Csharp) => "ushort",
            (ScalarKind::U16, SbeTargetLang::TypeScript) => "number",
            (ScalarKind::U16, SbeTargetLang::Zig) => "u16",
            (ScalarKind::I16, SbeTargetLang::Go) => "int16",
            (ScalarKind::I16, SbeTargetLang::Cpp) => "int16_t",
            (ScalarKind::I16, SbeTargetLang::Csharp) => "short",
            (ScalarKind::I16, SbeTargetLang::TypeScript) => "number",
            (ScalarKind::I16, SbeTargetLang::Zig) => "i16",
            (ScalarKind::U8, SbeTargetLang::Go) => "uint8",
            (ScalarKind::U8, SbeTargetLang::Cpp) => "uint8_t",
            (ScalarKind::U8, SbeTargetLang::Csharp) => "byte",
            (ScalarKind::U8, SbeTargetLang::TypeScript) => "number",
            (ScalarKind::U8, SbeTargetLang::Zig) => "u8",
        }
        .to_string()
    }

    fn is_float_64(self) -> bool {
        matches!(self, ScalarKind::F64)
    }

    /// Unsigned wire helper backing this kind (`u16`/`u32`/`u64`).
    fn helper_width(self) -> &'static str {
        match self {
            ScalarKind::F64 | ScalarKind::I64 | ScalarKind::U64 => "U64",
            ScalarKind::F32 | ScalarKind::U32 | ScalarKind::I32 => "U32",
            ScalarKind::U16 | ScalarKind::I16 => "U16",
            ScalarKind::U8 => "U8",
        }
    }

    /// Expression converting a value of `lang_type` into the helper's argument type.
    fn to_wire(self, lang: SbeTargetLang, expr: &str) -> String {
        match (self, lang) {
            (ScalarKind::F64, _) => expr.to_string(),
            (ScalarKind::F32, SbeTargetLang::Go) => format!("math.Float32bits({expr})"),
            (ScalarKind::F32, SbeTargetLang::Zig) => format!("@bitCast({expr})"),
            (ScalarKind::F32, SbeTargetLang::Csharp) => {
                format!("BitConverter.SingleToUInt32Bits({expr})")
            }
            (ScalarKind::F32, SbeTargetLang::Cpp) => format!("f32_bits({expr})"),
            (ScalarKind::F32, SbeTargetLang::TypeScript) => format!("f32Bits({expr})"),
            (ScalarKind::I64 | ScalarKind::I32 | ScalarKind::I16, SbeTargetLang::Go) => {
                let w = format!("uint{}", self.bit_width());
                format!("{w}({expr})")
            }
            (ScalarKind::I64 | ScalarKind::I32 | ScalarKind::I16, SbeTargetLang::Csharp) => {
                let w = match self {
                    ScalarKind::I64 => "ulong",
                    ScalarKind::I32 => "uint",
                    _ => "ushort",
                };
                format!("({w})({expr})")
            }
            (ScalarKind::I64 | ScalarKind::I32 | ScalarKind::I16, SbeTargetLang::Cpp) => {
                let w = format!("uint{}_t", self.bit_width());
                format!("static_cast<{w}>({expr})")
            }
            (ScalarKind::I64, SbeTargetLang::TypeScript) => {
                format!("BigInt.asUintN(64, {expr})")
            }
            (ScalarKind::I64, SbeTargetLang::Zig) => format!("@as(i64, @bitCast({expr}))"),
            (ScalarKind::I32, SbeTargetLang::Zig) => format!("@as(i32, @bitCast({expr}))"),
            (ScalarKind::I16, SbeTargetLang::Zig) => format!("@as(i16, @bitCast({expr}))"),
            (ScalarKind::I32 | ScalarKind::I16, SbeTargetLang::TypeScript) => {
                format!("({expr}) >>> 0")
            }
            (_, SbeTargetLang::Go) => {
                let w = format!("uint{}", self.bit_width());
                format!("{w}({expr})")
            }
            (_, SbeTargetLang::Zig) => format!("@intCast({expr})"),
            (_, _) => expr.to_string(),
        }
    }

    /// Expression converting a value read by the unsigned helper back to `lang_type`.
    fn lang_from_wire(self, lang: SbeTargetLang, expr: &str) -> String {
        match (self, lang) {
            (ScalarKind::F64, _) => expr.to_string(),
            (ScalarKind::F32, SbeTargetLang::Go) => format!("math.Float32frombits({expr})"),
            (ScalarKind::F32, SbeTargetLang::Zig) => format!("@bitCast({expr})"),
            (ScalarKind::F32, SbeTargetLang::Csharp) => {
                format!("BitConverter.UInt32BitsToSingle({expr})")
            }
            (ScalarKind::F32, SbeTargetLang::Cpp) => format!("f32_from_bits({expr})"),
            (ScalarKind::F32, SbeTargetLang::TypeScript) => format!("f32FromBits({expr})"),
            (ScalarKind::I64, SbeTargetLang::Go) => format!("int64({expr})"),
            (ScalarKind::I32, SbeTargetLang::Go) => format!("int32({expr})"),
            (ScalarKind::I16, SbeTargetLang::Go) => format!("int16({expr})"),
            (ScalarKind::I64, SbeTargetLang::Csharp) => format!("(long){expr}"),
            (ScalarKind::I32, SbeTargetLang::Csharp) => format!("(int){expr}"),
            (ScalarKind::I16, SbeTargetLang::Csharp) => format!("(short){expr}"),
            (ScalarKind::I64, SbeTargetLang::Cpp) => {
                format!("static_cast<int64_t>({expr})")
            }
            (ScalarKind::I32, SbeTargetLang::Cpp) => {
                format!("static_cast<int32_t>({expr})")
            }
            (ScalarKind::I16, SbeTargetLang::Cpp) => {
                format!("static_cast<int16_t>({expr})")
            }
            (ScalarKind::I64, SbeTargetLang::TypeScript) => {
                format!("BigInt.asIntN(64, {expr})")
            }
            (ScalarKind::I32, SbeTargetLang::TypeScript) => format!("({expr} | 0)"),
            (ScalarKind::I16, SbeTargetLang::TypeScript) => format!("({expr} << 16 >> 16)"),
            (ScalarKind::I64, SbeTargetLang::Zig) => format!("@as(i64, @bitCast({expr}))"),
            (ScalarKind::I32, SbeTargetLang::Zig) => format!("@as(i32, @bitCast({expr}))"),
            (ScalarKind::I16, SbeTargetLang::Zig) => format!("@as(i16, @bitCast({expr}))"),
            (_, _) => expr.to_string(),
        }
    }

    fn bit_width(self) -> usize {
        match self {
            ScalarKind::F64 | ScalarKind::I64 | ScalarKind::U64 => 64,
            ScalarKind::F32 | ScalarKind::U32 | ScalarKind::I32 => 32,
            ScalarKind::U16 | ScalarKind::I16 => 16,
            ScalarKind::U8 => 8,
        }
    }
}

/// Emit one big-endian write of `value` for `kind`.
fn emit_write_scalar(lang: SbeTargetLang, buf: &str, value: &str, kind: ScalarKind) -> String {
    if kind.is_float_64() {
        return match lang {
            SbeTargetLang::Go => format!("\twriteF64BE(&{buf}, {value})\n"),
            SbeTargetLang::Cpp => format!("        write_f64_be(buf, {value});\n"),
            SbeTargetLang::Csharp => format!("            Wire.WriteF64BE(buf, {value});\n"),
            SbeTargetLang::TypeScript => format!("  writeF64BE(buf, {value});\n"),
            SbeTargetLang::Zig => format!("        try writeF64BE(&{buf}, {value});\n"),
        };
    }
    let w = kind.helper_width();
    let v = kind.to_wire(lang, value);
    if kind == ScalarKind::U8 {
        return match lang {
            SbeTargetLang::Go => format!("\t{buf} = append({buf}, {v})\n"),
            SbeTargetLang::Cpp => format!("        buf.push_back({v});\n"),
            SbeTargetLang::Csharp => format!("            buf.Add({v});\n"),
            SbeTargetLang::TypeScript => format!("  buf.push({v});\n"),
            SbeTargetLang::Zig => format!("        try buf.append({v});\n"),
        };
    }
    match lang {
        SbeTargetLang::Go => format!("\twrite{w}BE(&{buf}, {v})\n"),
        SbeTargetLang::Cpp => format!("        write_{}_be(buf, {v});\n", w.to_lowercase()),
        SbeTargetLang::Csharp => format!("            Wire.Write{w}BE(buf, {v});\n"),
        SbeTargetLang::TypeScript => format!("  write{w}BE(buf, {v});\n"),
        SbeTargetLang::Zig => format!("        try write{w}BE(&{buf}, {v});\n"),
    }
}

/// Expression reading one big-endian value of `kind` (before conversion).
fn emit_read_scalar(lang: SbeTargetLang, buf: &str, kind: ScalarKind) -> String {
    if kind.is_float_64() {
        return match lang {
            SbeTargetLang::Go => format!("readF64BE({buf}, &pos)"),
            SbeTargetLang::Cpp => format!("read_f64_be({buf}, pos)"),
            SbeTargetLang::Csharp => format!("Wire.ReadF64BE({buf}, ref pos)"),
            SbeTargetLang::TypeScript => format!("readF64BE({buf}, pos)"),
            SbeTargetLang::Zig => format!("readF64BE({buf}, &pos)"),
        };
    }
    let w = kind.helper_width();
    if kind == ScalarKind::U8 {
        return match lang {
            SbeTargetLang::Go => format!("readU8({buf}, &pos)"),
            SbeTargetLang::Cpp => format!("take_u8({buf}, pos)"),
            SbeTargetLang::Csharp => format!("TakeU8({buf}, ref pos)"),
            SbeTargetLang::TypeScript => format!("readU8({buf}, pos)"),
            SbeTargetLang::Zig => format!("readU8({buf}, &pos)"),
        };
    }
    match lang {
        SbeTargetLang::Go => format!("read{w}BE({buf}, &pos)"),
        SbeTargetLang::Cpp => format!("read_{}_be({buf}, pos)", w.to_lowercase()),
        SbeTargetLang::Csharp => format!("Wire.Read{w}BE({buf}, ref pos)"),
        SbeTargetLang::TypeScript => format!("read{w}BE({buf}, pos)"),
        SbeTargetLang::Zig => format!("read{w}BE({buf}, &pos)"),
    }
}

/// Emit an optional or required scalar field write (mirrors the Rust layout).
fn encode_scalar(
    lang: SbeTargetLang,
    buf: &str,
    field: &str,
    optional: bool,
    kind: ScalarKind,
    out: &mut String,
) {
    if optional {
        let ty = kind.lang_type(lang);
        let zero = zero_literal(lang, kind);
        let tmp = field.replace(['.', '-'], "_");
        // value first, then presence byte (SPEC/Rust layout)
        match lang {
            SbeTargetLang::Go => {
                out.push_str(&format!("\tvar {tmp}_v {ty}\n"));
                out.push_str(&format!(
                    "\tif {field} != nil {{\n\t\t{tmp}_v = *{field}\n\t}}\n"
                ));
                out.push_str(&emit_write_scalar(lang, buf, &format!("{tmp}_v"), kind));
                out.push_str(&format!("\tif {field} != nil {{\n\t\t{buf} = append({buf}, 1)\n\t}} else {{\n\t\t{buf} = append({buf}, 0)\n\t}}\n"));
            }
            SbeTargetLang::Cpp => {
                out.push_str(&emit_write_scalar(
                    lang,
                    buf,
                    &format!("{field}.value_or({zero})"),
                    kind,
                ));
                out.push_str(&format!(
                    "        buf.push_back({field}.has_value() ? 1 : 0);\n"
                ));
            }
            SbeTargetLang::Csharp => {
                out.push_str(&emit_write_scalar(
                    lang,
                    buf,
                    &format!("{field} ?? {zero}"),
                    kind,
                ));
                out.push_str(&format!(
                    "            buf.Add((byte)({field}.HasValue ? 1 : 0));\n"
                ));
            }
            SbeTargetLang::TypeScript => {
                out.push_str(&emit_write_scalar(
                    lang,
                    buf,
                    &format!("{field} ?? {zero}"),
                    kind,
                ));
                out.push_str(&format!("  buf.push({field} != null ? 1 : 0);\n"));
            }
            SbeTargetLang::Zig => {
                out.push_str(&emit_write_scalar(
                    lang,
                    buf,
                    &format!("{field} orelse {zero}"),
                    kind,
                ));
                out.push_str(&format!(
                    "        try buf.append(if ({field} != null) 1 else 0);\n"
                ));
            }
        }
    } else {
        out.push_str(&emit_write_scalar(lang, buf, field, kind));
    }
}

/// Zero literal for a scalar kind in the target language.
fn zero_literal(lang: SbeTargetLang, kind: ScalarKind) -> String {
    match (lang, kind) {
        (SbeTargetLang::Go, ScalarKind::F64) => "float64(0)".into(),
        (SbeTargetLang::Go, ScalarKind::F32) => "float32(0)".into(),
        (SbeTargetLang::Go, _) => format!("{}(0)", kind.lang_type(lang)),
        (SbeTargetLang::Csharp, ScalarKind::F64) => "0.0".into(),
        (SbeTargetLang::Csharp, ScalarKind::F32) => "0f".into(),
        (SbeTargetLang::Csharp, _) => format!("({})0", kind.lang_type(lang)),
        (SbeTargetLang::Cpp, ScalarKind::F64) => "0.0".into(),
        (SbeTargetLang::Cpp, ScalarKind::F32) => "0.0f".into(),
        (SbeTargetLang::TypeScript, ScalarKind::I64 | ScalarKind::U64) => "0n".into(),
        (SbeTargetLang::TypeScript, _) => "0".into(),
        (SbeTargetLang::Cpp, _) => "0".into(),
        (SbeTargetLang::Zig, ScalarKind::F64 | ScalarKind::F32) => "0".into(),
        (SbeTargetLang::Zig, _) => "0".into(),
    }
}

/// Emit an optional or required scalar field read.
fn decode_scalar(
    lang: SbeTargetLang,
    var: &str,
    optional: bool,
    kind: ScalarKind,
    out: &mut String,
) {
    let raw = emit_read_scalar(lang, "buf", kind);
    let value = kind.lang_from_wire(lang, &raw);
    let ty = kind.lang_type(lang);
    let var = &var.replace(['.', '-'], "_");
    if optional {
        // value is always present on the wire; the presence byte follows it.
        match lang {
            SbeTargetLang::Go => {
                out.push_str(&format!("\t{var}_v := {value}\n"));
                out.push_str(&format!("\tvar {var} *{ty}\n"));
                out.push_str(&format!(
                    "\tif buf[pos] == 1 {{\n\t\tpos++\n\t\t{var} = &{var}_v\n\t}} else {{\n\t\tpos++\n\t\t{var} = nil\n\t}}\n"
                ));
            }
            SbeTargetLang::Cpp => {
                out.push_str(&format!("        {ty} {var}_v = {value};\n"));
                out.push_str(&format!("        std::optional<{ty}> {var};\n"));
                out.push_str(&format!("        if (buf[pos++] == 1) {var} = {var}_v;\n"));
            }
            SbeTargetLang::Csharp => {
                out.push_str(&format!("            {ty} {var}_v = {value};\n"));
                out.push_str(&format!(
                    "            {ty}? {var} = buf[pos++] == 1 ? {var}_v : null;\n"
                ));
            }
            SbeTargetLang::TypeScript => {
                out.push_str(&format!("  const {var}_v = {value};\n"));
                out.push_str(&format!(
                    "  const {var} = buf[pos.value++] === 1 ? {var}_v : null;\n"
                ));
            }
            SbeTargetLang::Zig => {
                out.push_str(&format!("        const {var}_v = {value};\n"));
                out.push_str(&format!("        var {var}: ?{ty} = null;\n"));
                out.push_str(&format!(
                    "        if (buf[pos] == 1) {{ pos += 1; {var} = {var}_v; }} else {{ pos += 1; }}\n"
                ));
            }
        }
    } else {
        match lang {
            SbeTargetLang::Go => {
                out.push_str(&format!("\t{var} := {value}\n"));
            }
            SbeTargetLang::Cpp => {
                out.push_str(&format!("        {ty} {var} = {value};\n"));
            }
            SbeTargetLang::Csharp => {
                out.push_str(&format!("            {ty} {var} = {value};\n"));
            }
            SbeTargetLang::TypeScript => {
                out.push_str(&format!("  const {var} = {value};\n"));
            }
            SbeTargetLang::Zig => {
                out.push_str(&format!("        const {var} = {value};\n"));
            }
        }
    }
}

// ── Type resolution helpers ───────────────────────────────────

/// Names of type-defs that carry fields (struct-like).
fn collect_struct_names(schema: &Schema) -> std::collections::HashSet<String> {
    schema
        .type_defs
        .iter()
        .filter(|td| td.fields.is_some())
        .map(|td| td.name.clone())
        .collect()
}

fn collect_message_names(schema: &Schema) -> std::collections::HashSet<String> {
    schema.messages.iter().map(|m| m.name.clone()).collect()
}

fn is_struct_name(name: &str, structs: &std::collections::HashSet<String>) -> bool {
    structs.contains(name)
}

fn is_message_name(name: &str, messages: &std::collections::HashSet<String>) -> bool {
    messages.contains(name)
}

/// Scalar kind for a field, if it is a scalar (named alias or inline base).
fn field_scalar_kind(ft: &FieldType) -> Option<ScalarKind> {
    let token = match ft {
        FieldType::Named(n) => sbe_named_type_to_wire_type(n),
        FieldType::InlineBase(bt, _) => sbe_base_type_to_wire_type(bt),
        _ => return None,
    };
    ScalarKind::from_wire_token(&token)
}

/// Buffer type used by the generated encoders.
fn buf_type(lang: SbeTargetLang) -> &'static str {
    match lang {
        SbeTargetLang::Go => "*[]byte",
        SbeTargetLang::Cpp => "std::vector<uint8_t>&",
        SbeTargetLang::Csharp => "List<byte>",
        SbeTargetLang::TypeScript => "number[]",
        SbeTargetLang::Zig => "*std.ArrayList(u8)",
    }
}

/// Buffer parameter declaration, always named `buf`.
fn buf_param(lang: SbeTargetLang) -> &'static str {
    match lang {
        SbeTargetLang::Go => "buf *[]byte",
        SbeTargetLang::Cpp => "std::vector<uint8_t>& buf",
        SbeTargetLang::Csharp => "List<byte> buf",
        SbeTargetLang::TypeScript => "buf: number[]",
        SbeTargetLang::Zig => "buf: *std.ArrayList(u8)",
    }
}

/// Whether the wire layout for this field is variable-length (string, struct, message).
fn is_variable_length(
    ft: &FieldType,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> bool {
    match ft {
        FieldType::Named(n) if is_struct_name(n, structs) || is_message_name(n, messages) => true,
        _ => sbe_field_wire_type_of(ft) == "str",
    }
}

fn sbe_field_wire_type_of(ft: &FieldType) -> String {
    match ft {
        FieldType::Named(n) => sbe_named_type_to_wire_type(n),
        FieldType::InlineBase(bt, _) => sbe_base_type_to_wire_type(bt),
        FieldType::Enum(_) => "u8".to_string(),
        FieldType::List(inner) => sbe_field_wire_type_of(inner),
        FieldType::InlineStruct(_) => "struct".to_string(),
    }
}

// ── Inline structs and named struct type-defs ─────────────────

fn generate_target_inline_struct(lang: SbeTargetLang, name: &str, fields: &[Field]) -> String {
    let empty = std::collections::HashSet::new();
    generate_target_struct(lang, name, fields, "", &empty, &empty)
}

/// Emit `{name}Encoder` / `{name}Decoder` for a struct definition.
fn generate_target_struct(
    lang: SbeTargetLang,
    name: &str,
    fields: &[Field],
    parent: &str,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    let encoder = format!("{name}Encoder");
    let decoder = format!("{name}Decoder");
    let go_buf = if lang == SbeTargetLang::Go {
        "local"
    } else {
        "buf"
    };
    let enc_body = enc_fields(lang, fields, parent, "value.", go_buf, structs, messages);
    let (dec_decls, dec_body) = dec_fields(lang, fields, parent, structs, messages);

    let field_decls = target_field_decls(lang, name, fields, structs, messages);

    match lang {
        SbeTargetLang::Go => format!(
            "// SBE struct {name}\ntype {name} struct {{\n{field_decls}}}\n\n\
             // Encode writes {name} fields (no header) into buf.\ntype {encoder} struct{{}}\n\nfunc ({encoder}) Encode(value {name}, buf *[]byte) {{\n\tlocal := *buf\n{enc_body}\t*buf = local\n}}\n\n\
             // Decode{decoder} reads {name} fields starting at pos and returns the new offset.\nfunc {decoder}Decode(buf []byte, pos int) ({name}, int, error) {{\n{dec_decls}{dec_body}\treturn value, pos, nil\n}}\n"
        ),
        SbeTargetLang::Cpp => format!(
            "// SBE struct {name}\nstruct {name} {{\n{field_decls}}};\n\n\
             struct {encoder} {{\n    static void encode(const {name}& value, {buf});\n}};\n\n\
             struct {decoder} {{\n    static std::pair<{name}, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);\n}};\n\n\
             inline void {encoder}::encode(const {name}& value, {buf}) {{\n{enc_body}}}\n\n\
             inline std::pair<{name}, std::size_t> {decoder}::decode(const std::vector<uint8_t>& buf, std::size_t pos) {{\n    {name} value{{}};\n{dec_decls}{dec_body}    return {{value, pos}};\n}}\n",
            buf = buf_param(lang),
        ),
        SbeTargetLang::Csharp => format!(
            "    // SBE struct {name}\n    public struct {name}\n    {{\n{field_decls}    }}\n\n\
             public static class {encoder}\n    {{\n        public static void Encode({name} value, {buf})\n        {{\n{enc_body}        }}\n    }}\n\n\
             public static class {decoder}\n    {{\n        public static ({name} Value, int Pos) Decode(byte[] buf, int pos)\n        {{\n            var value = new {name}();\n{dec_decls}{dec_body}            return (value, pos);\n        }}\n    }}\n",
            buf = buf_param(lang),
        ),
        SbeTargetLang::TypeScript => format!(
            "/** SBE struct {name} */\nexport interface {name} {{\n{field_decls}}}\n\n\
             /** Encode {name} fields (no header). */\nexport function {encoder}Encode(value: {name}, buf: number[]): void {{\n{enc_body}}}\n\n\
             /** Decode {name} fields; advances `pos`. */\nexport function {decoder}Decode(buf: Uint8Array, pos: {{ value: number }}): {name} {{\n{dec_decls}{dec_body}  return value;\n}}\n"
        ),
        SbeTargetLang::Zig => format!(
            "/// SBE struct {name}\npub const {name} = struct {{\n{field_decls}\n    pub fn encode(allocator: std.mem.Allocator, value: {name}, buf: {buf}) !void {{\n{enc_discard}{enc_body}    }}\n\n    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !{name} {{\n{dec_discard}        {zig_value_kw} value: {name} = undefined;\n{dec_decls}{dec_body}        return value;\n    }}\n}};\n",
            enc_discard = zig_discards(&enc_body, &["allocator", "value", "buf"]),
            dec_discard = zig_discards(&dec_body, &["allocator", "buf", "pos"]),
            zig_value_kw = if dec_body.contains("value.") { "var" } else { "const" },
            buf = buf_type(lang),
        ),
    }
}

/// Struct field declarations (used by struct/decoder return types).
fn target_field_decls(
    lang: SbeTargetLang,
    parent: &str,
    fields: &[Field],
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    let mut out = String::new();
    for field in fields {
        let base = target_field_type_full(
            lang,
            &field.field_type,
            parent,
            &field.name,
            structs,
            messages,
        );
        let ty = if field.optional {
            target_optional_type(lang, &base)
        } else {
            base
        };
        match lang {
            SbeTargetLang::Go => out.push_str(&format!(
                "\t{} {ty}\n",
                target_param_name(lang, &field.name)
            )),
            SbeTargetLang::Cpp => out.push_str(&format!("    {ty} {};\n", field.name)),
            SbeTargetLang::Csharp => out.push_str(&format!(
                "        public {ty} {};\n",
                target_param_name(lang, &field.name)
            )),
            SbeTargetLang::TypeScript => out.push_str(&format!("  {}: {ty};\n", field.name)),
            SbeTargetLang::Zig => out.push_str(&format!("    {}: {ty},\n", field.name)),
        }
    }
    out
}

// ── Field encoding ────────────────────────────────────────────

/// Emit the writes for a run of fields into `buf`.
fn enc_fields(
    lang: SbeTargetLang,
    fields: &[Field],
    parent: &str,
    acc: &str,
    buf: &str,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    let mut out = String::new();
    for field in fields {
        out.push_str(&target_encode_field(
            lang, field, parent, acc, buf, structs, messages,
        ));
    }
    out
}

fn target_encode_field(
    lang: SbeTargetLang,
    field: &Field,
    parent: &str,
    acc: &str,
    buf: &str,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    let field_name = &field.name;
    let pname = format!("{acc}{}", target_param_name(lang, field_name));
    let optional = field.optional;
    let mut out = String::new();

    match &field.field_type {
        FieldType::Enum(_) => {
            let enum_name = format!("{parent}{}", pascal_case(field_name));
            // value, then presence byte (mirrors the Rust encoder)
            match (lang, optional) {
                (SbeTargetLang::Go, true) => {
                let etmp = pname.replace('.', "_");
                out.push_str(&format!("\t{etmp}_v := uint8(0)\n\tif {pname} != nil {{\n\t\t{etmp}_v = byte({pname}.ToValue())\n\t}}\n\t{buf} = append({buf}, {etmp}_v)\n\tif {pname} != nil {{\n\t\t{buf} = append({buf}, 1)\n\t}} else {{\n\t\t{buf} = append({buf}, 0)\n\t}}\n"));
            },
                (SbeTargetLang::Go, false) => out.push_str(&format!("\t{buf} = append({buf}, byte({pname}.ToValue()))\n")),
                (SbeTargetLang::Cpp, true) => out.push_str(&format!("        buf.push_back({pname}.has_value() ? {enum_name}ToValue(*{pname}) : 0);\n        buf.push_back({pname}.has_value() ? 1 : 0);\n")),
                (SbeTargetLang::Cpp, false) => out.push_str(&format!("        buf.push_back({enum_name}ToValue({pname}));\n")),
                (SbeTargetLang::Csharp, true) => out.push_str(&format!("            buf.Add({pname}.HasValue ? {enum_name}ToValue({pname}.Value) : (byte)0);\n            buf.Add((byte)({pname}.HasValue ? 1 : 0));\n")),
                (SbeTargetLang::Csharp, false) => out.push_str(&format!("            buf.Add({enum_name}ToValue({pname}));\n")),
                (SbeTargetLang::TypeScript, true) => out.push_str(&format!("  buf.push({pname} != null ? {enum_name}ToValue({pname}) : 0);\n  buf.push({pname} != null ? 1 : 0);\n")),
                (SbeTargetLang::TypeScript, false) => out.push_str(&format!("  buf.push({enum_name}ToValue({pname}));\n")),
                (SbeTargetLang::Zig, true) => out.push_str(&format!("        try buf.append(if ({pname}) |e| {enum_name}ToValue(e) else 0);\n        try buf.append(if ({pname} != null) 1 else 0);\n")),
                (SbeTargetLang::Zig, false) => out.push_str(&format!("        try buf.append({enum_name}ToValue({pname}));\n")),
            }
        }
        FieldType::List(inner) => {
            let composite = matches!(inner.as_ref(), FieldType::Named(n)
                if is_struct_name(n, structs) || is_message_name(n, messages));
            match lang {
                SbeTargetLang::Go => {
                    out.push_str(&format!("\twriteU32BE(&{buf}, uint32(len({pname})))\n"))
                }
                SbeTargetLang::Cpp => out.push_str(&format!(
                    "        write_u32_be(buf, static_cast<uint32_t>({pname}.size()));\n"
                )),
                SbeTargetLang::Csharp => out.push_str(&format!(
                    "            Wire.WriteU32BE(buf, (uint){pname}.Count);\n"
                )),
                SbeTargetLang::TypeScript => {
                    out.push_str(&format!("  writeU32BE(buf, {pname}.length);\n"))
                }
                SbeTargetLang::Zig => out.push_str(&format!(
                    "        try writeU32BE(&{buf}, @intCast({pname}.len));\n"
                )),
            }
            if !composite {
                // Scalar list items have no payload layout in the Rust generator yet
                // (sbe_codegen.rs writes a count plus a TODO); mirror it so both agree.
                return out;
            }
            match lang {
                SbeTargetLang::Go => out.push_str(&format!("\tfor _, item := range {pname} {{\n\t\tvar item_buf []byte\n")),
                SbeTargetLang::Cpp => out.push_str(&format!("        for (const auto& item : {pname}) {{\n            std::vector<uint8_t> item_buf;\n")),
                SbeTargetLang::Csharp => out.push_str(&format!("            foreach (var item in {pname})\n            {{\n                var itemBuf = new List<byte>();\n")),
                SbeTargetLang::TypeScript => out.push_str(&format!("  for (const item of {pname}) {{\n    const itemBuf: number[] = [];\n")),
                SbeTargetLang::Zig => out.push_str(&format!("        for ({pname}) |item| {{\n            var item_buf = std.ArrayList(u8).init(allocator);\n            defer item_buf.deinit();\n")),
            }
            let n = match inner.as_ref() {
                FieldType::Named(n) => n.as_str(),
                _ => unreachable!(),
            };
            let is_body = is_message_name(n, messages);
            out.push_str(&match (lang, is_body) {
                (SbeTargetLang::Go, true) => {
                    format!("\t\t{n}BodyEncoder{{}}.Encode(item, &item_buf)\n")
                }
                (SbeTargetLang::Go, false) => {
                    format!("\t\t{n}Encoder{{}}.Encode(item, &item_buf)\n")
                }
                (SbeTargetLang::Cpp, true) => {
                    format!("            {n}BodyEncoder::encode(item, item_buf);\n")
                }
                (SbeTargetLang::Cpp, false) => {
                    format!("            {n}Encoder::encode(item, item_buf);\n")
                }
                (SbeTargetLang::Csharp, true) => {
                    format!("                {n}BodyEncoder.Encode(item, itemBuf);\n")
                }
                (SbeTargetLang::Csharp, false) => {
                    format!("                {n}Encoder.Encode(item, itemBuf);\n")
                }
                (SbeTargetLang::TypeScript, true) => {
                    format!("    {n}BodyEncoderEncode(item, itemBuf);\n")
                }
                (SbeTargetLang::TypeScript, false) => {
                    format!("    {n}EncoderEncode(item, itemBuf);\n")
                }
                (SbeTargetLang::Zig, true) => {
                    format!("            try {n}BodyEncoder.encode(allocator, item, &item_buf);\n")
                }
                (SbeTargetLang::Zig, false) => {
                    format!("            try {n}.encode(allocator, item, &item_buf);\n")
                }
            });
            out.push_str(&match lang {
                SbeTargetLang::Go => format!("\t\twriteU32BE(&{buf}, uint32(len(item_buf)))\n\t\t{buf} = append({buf}, item_buf...)\n\t}}\n"),
                SbeTargetLang::Cpp => "            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));\n            buf.insert(buf.end(), item_buf.begin(), item_buf.end());\n        }\n".to_string(),
                SbeTargetLang::Csharp => "                Wire.WriteU32BE(buf, (uint)itemBuf.Count);\n                buf.AddRange(itemBuf);\n            }\n".to_string(),
                SbeTargetLang::TypeScript => "    writeU32BE(buf, itemBuf.length);\n    for (const b of itemBuf) buf.push(b);\n  }\n".to_string(),
                SbeTargetLang::Zig => format!("            try writeU32BE(&{buf}, @intCast(item_buf.items.len));\n            try {buf}.appendSlice(item_buf.items);\n        }}\n"),
            });
        }
        FieldType::InlineStruct(fields) => {
            let struct_name = format!("{parent}{}", pascal_case(field_name));
            let _ = fields;
            out.push_str(&encode_named_value(lang, &struct_name, &pname, buf, false));
        }
        FieldType::Named(n) if is_message_name(n, messages) => {
            out.push_str(&encode_body_value(lang, n, &pname, buf, false));
        }
        FieldType::Named(n) if is_struct_name(n, structs) => {
            out.push_str(&encode_named_value(lang, n, &pname, buf, false));
        }
        _ => {
            if sbe_field_wire_type_of(&field.field_type) == "str" {
                encode_string(lang, buf, &pname, optional, &mut out);
            } else if let Some(kind) = field_scalar_kind(&field.field_type) {
                encode_scalar(lang, buf, &pname, optional, kind, &mut out);
            } else {
                panic!(
                    "sbe target codegen: unsupported field type {:?}",
                    field.field_type
                );
            }
        }
    }
    out
}

/// Encode a nested message value through its Body encoder.
fn encode_body_value(
    lang: SbeTargetLang,
    name: &str,
    value: &str,
    buf: &str,
    into_item: bool,
) -> String {
    let enc = format!("{name}BodyEncoder");
    let b = if into_item { "item_buf" } else { buf };
    match lang {
        SbeTargetLang::Go => format!("\t{enc}{{}}.Encode({value}, &{b})\n"),
        SbeTargetLang::Cpp => format!("        {enc}::encode({value}, {b});\n"),
        SbeTargetLang::Csharp => format!("            {enc}.Encode({value}, {b});\n"),
        SbeTargetLang::TypeScript => format!("  {enc}Encode({value}, {b});\n"),
        SbeTargetLang::Zig => format!("        try {enc}.encode(allocator, {value}, &{b});\n"),
    }
}

/// Encode a struct/message value: `{Name}Encoder` for structs, `{Name}BodyEncoder` for messages.
///
/// `into_item` selects the buffer-name convention used inside list loops.
fn encode_named_value(
    lang: SbeTargetLang,
    name: &str,
    value: &str,
    buf: &str,
    into_item: bool,
) -> String {
    let enc = format!("{name}Encoder");
    let b = if into_item { "item_buf" } else { buf };
    match lang {
        SbeTargetLang::Go => format!("\t{enc}{{}}.Encode({value}, &{b})\n"),
        SbeTargetLang::Cpp => format!("        {enc}::encode({value}, {b});\n"),
        SbeTargetLang::Csharp => format!("            {enc}.Encode({value}, {b});\n"),
        SbeTargetLang::TypeScript => format!("  {enc}Encode({value}, {b});\n"),
        SbeTargetLang::Zig => format!("        try {name}.encode(allocator, {value}, &{b});\n"),
    }
}

fn encode_string(lang: SbeTargetLang, buf: &str, field: &str, optional: bool, out: &mut String) {
    if optional {
        // presence byte first, then u16 length + bytes (mirrors the Rust encoder)
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\tif {field} != nil {{\n\t\t{buf} = append({buf}, 1)\n\t\twriteString(&{buf}, *{field})\n\t}} else {{\n\t\t{buf} = append({buf}, 0)\n\t}}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        if ({field}.has_value()) {{ buf.push_back(1); write_string(buf, *{field}); }} else {{ buf.push_back(0); }}\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            if ({field}.HasValue) {{ buf.Add(1); Wire.WriteString(buf, {field}.Value); }} else {{ buf.Add(0); }}\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  if ({field} != null) {{ buf.push(1); writeString(buf, {field}); }} else {{ buf.push(0); }}\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        if ({field}) |s| {{ try buf.append(1); try writeString(&{buf}, s); }} else {{ try buf.append(0); }}\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\twriteString(&{buf}, {field})\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        write_string(buf, {field});\n")),
            SbeTargetLang::Csharp => {
                out.push_str(&format!("            Wire.WriteString(buf, {field});\n"))
            }
            SbeTargetLang::TypeScript => out.push_str(&format!("  writeString(buf, {field});\n")),
            SbeTargetLang::Zig => {
                out.push_str(&format!("        try writeString(&{buf}, {field});\n"))
            }
        }
    }
}

// ── Field decoding ────────────────────────────────────────────

/// Emit the reads for a run of fields: returns (declarations, statements).
fn dec_fields(
    lang: SbeTargetLang,
    fields: &[Field],
    parent: &str,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> (String, String) {
    let mut decls = String::new();
    let mut body = String::new();
    for field in fields {
        let (d, b) = target_decode_field(lang, field, parent, structs, messages);
        decls.push_str(&d);
        body.push_str(&b);
    }
    // Build the returned/assigned struct value.
    let mut assign = String::new();
    for field in fields {
        assign.push_str(&match lang {
            SbeTargetLang::Go => format!(
                "\t\t{}: {},\n",
                target_param_name(lang, &field.name),
                field.name.replace('-', "_")
            ),
            SbeTargetLang::Cpp => format!("    value.{} = {};\n", field.name, field.name),
            SbeTargetLang::Csharp => format!(
                "            value.{} = {};\n",
                target_param_name(lang, &field.name),
                field.name.replace('-', "_")
            ),
            SbeTargetLang::Zig => format!("        result.{} = {};\n", field.name, field.name),
            SbeTargetLang::TypeScript => String::new(),
        });
    }
    match lang {
        SbeTargetLang::Go => body.push_str(&format!("\tvalue := {parent}{{\n{assign}\t}}\n")),
        SbeTargetLang::Cpp => body.push_str(&assign),
        SbeTargetLang::Csharp => body.push_str(&assign),
        SbeTargetLang::Zig => body.push_str(&assign.replace("        result.", "        value.")),
        SbeTargetLang::TypeScript => {
            let mut ts = String::from("  const value: ");
            ts.push_str(parent);
            ts.push_str(" = {\n");
            for field in fields {
                ts.push_str(&format!("    {}: {},\n", field.name, field.name));
            }
            ts.push_str("  };\n");
            body.push_str(&ts);
        }
    }
    (decls, body)
}

fn target_decode_field(
    lang: SbeTargetLang,
    field: &Field,
    parent: &str,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> (String, String) {
    let var = field.name.replace('-', "_");
    let optional = field.optional;
    let mut decls = String::new();
    let mut out = String::new();

    match &field.field_type {
        FieldType::Enum(_) => {
            let enum_name = format!("{parent}{}", pascal_case(&field.name));
            match (lang, optional) {
                (SbeTargetLang::Go, true) => {
                    out.push_str(&format!("\t{var}_raw := readU8(buf, &pos)\n\tvar {var} *{enum_name}\n\tif buf[pos] == 1 {{\n\t\tpos++\n\t\te, {var}_err := {enum_name}FromValue({var}_raw)\n\t\tif {var}_err != nil {{\n\t\t\treturn {parent}{{}}, pos, {var}_err\n\t\t}}\n\t\t{var} = &e\n\t}} else {{\n\t\tpos++\n\t\t{var} = nil\n\t}}\n"));
                }
                (SbeTargetLang::Go, false) => {
                    out.push_str(&format!("\t{var}_raw := readU8(buf, &pos)\n\t{var}, {var}_err := {enum_name}FromValue({var}_raw)\n\tif {var}_err != nil {{\n\t\treturn {parent}{{}}, pos, {var}_err\n\t}}\n"));
                }
                (SbeTargetLang::Cpp, true) => {
                    out.push_str(&format!("        auto {var}_raw = take_u8(buf, pos);\n        std::optional<{enum_name}> {var};\n        if (buf[pos++] == 1) {{\n            auto v = {enum_name}FromValue({var}_raw);\n            if (!v) throw std::runtime_error(\"invalid {enum_name}\");\n            {var} = *v;\n        }}\n"));
                }
                (SbeTargetLang::Cpp, false) => {
                    out.push_str(&format!("        auto {var}_v = {enum_name}FromValue(take_u8(buf, pos));\n        if (!{var}_v) throw std::runtime_error(\"invalid {enum_name}\");\n        {enum_name} {var} = *{var}_v;\n"));
                }
                (SbeTargetLang::Csharp, true) => {
                    out.push_str(&format!("            var {var}Raw = TakeU8(buf, ref pos);\n            {enum_name}? {var} = null;\n            if (buf[pos++] == 1) {{\n                {var} = {enum_name}FromValue({var}Raw) ?? throw new InvalidOperationException(\"invalid {enum_name}\");\n            }}\n"));
                }
                (SbeTargetLang::Csharp, false) => {
                    out.push_str(&format!("            var {var} = {enum_name}FromValue(TakeU8(buf, ref pos)) ?? throw new InvalidOperationException(\"invalid {enum_name}\");\n"));
                }
                (SbeTargetLang::TypeScript, true) => {
                    out.push_str(&format!("  const {var}Raw = readU8(buf, pos);\n  let {var}: {enum_name} | null = null;\n  if (buf[pos.value++] === 1) {{\n    const v = {enum_name}FromValue({var}Raw);\n    if (v == null) throw new Error('invalid {enum_name}');\n    {var} = v;\n  }}\n"));
                }
                (SbeTargetLang::TypeScript, false) => {
                    out.push_str(&format!("  const {var}Raw = readU8(buf, pos);\n  const {var} = {enum_name}FromValue({var}Raw);\n  if ({var} == null) throw new Error('invalid {enum_name}');\n"));
                }
                (SbeTargetLang::Zig, true) => {
                    out.push_str(&format!("        const {var}_raw = readU8(buf, &pos);\n        var {var}: ?{enum_name} = null;\n        if (buf[pos] == 1) {{\n            pos += 1;\n            {var} = try {enum_name}FromValue({var}_raw);\n        }} else {{\n            pos += 1;\n        }}\n"));
                }
                (SbeTargetLang::Zig, false) => {
                    out.push_str(&format!(
                        "        const {var} = try {enum_name}FromValue(readU8(buf, &pos));\n"
                    ));
                }
            }
        }
        FieldType::List(inner) => {
            let inner_is_var = is_variable_length(inner, structs, messages);
            let inner_ty =
                target_field_type_full(lang, inner, parent, &field.name, structs, messages);
            let read_count = match lang {
                SbeTargetLang::Go => format!("\t{var}_count := readU32BE(buf, &pos)\n"),
                SbeTargetLang::Cpp => {
                    format!("        uint32_t {var}_count = read_u32_be(buf, pos);\n")
                }
                SbeTargetLang::Csharp => {
                    format!("            uint {var}Count = Wire.ReadU32BE(buf, ref pos);\n")
                }
                SbeTargetLang::TypeScript => format!("  const {var}Count = readU32BE(buf, pos);\n"),
                SbeTargetLang::Zig => {
                    format!("        const {var}_count = readU32BE(buf, &pos);\n")
                }
            };
            out.push_str(&read_count);
            if !matches!(inner.as_ref(), FieldType::Named(n)
                if is_struct_name(n, structs) || is_message_name(n, messages))
            {
                // Scalar list items: the Rust generator does not read them yet.
                match lang {
                    SbeTargetLang::Go => out.push_str(&format!("\t{var} := make([]{inner_ty}, 0)\n\t_ = {var}_count\n")),
                    SbeTargetLang::Cpp => out.push_str(&format!("        std::vector<{inner_ty}> {var};\n        (void){var}_count;\n")),
                    SbeTargetLang::Csharp => out.push_str(&format!("            var {var} = new List<{inner_ty}>();\n            _ = {var}Count;\n")),
                    SbeTargetLang::TypeScript => out.push_str(&format!("  const {var}: {inner_ty}[] = [];\n")),
                    SbeTargetLang::Zig => out.push_str(&format!("        const {var} = try allocator.alloc({inner_ty}, 0);\n        _ = {var}_count;\n")),
                }
                return (decls, out);
            }
            match lang {
                SbeTargetLang::Go => {
                    out.push_str(&format!("\t{var} := make([]{inner_ty}, 0, {var}_count)\n\tfor i := 0; i < int({var}_count); i++ {{\n"));
                }
                SbeTargetLang::Cpp => {
                    out.push_str(&format!("        std::vector<{inner_ty}> {var};\n        {var}.reserve({var}_count);\n        for (uint32_t i = 0; i < {var}_count; ++i) {{\n"));
                }
                SbeTargetLang::Csharp => {
                    out.push_str(&format!("            var {var} = new List<{inner_ty}>((int){var}Count);\n            for (int i = 0; i < (int){var}Count; i++)\n            {{\n"));
                }
                SbeTargetLang::TypeScript => {
                    out.push_str(&format!("  const {var}: {inner_ty}[] = [];\n  for (let i = 0; i < {var}Count; i++) {{\n"));
                }
                SbeTargetLang::Zig => {
                    out.push_str(&format!("        var {var} = try allocator.alloc({inner_ty}, {var}_count);\n        var {var}_idx: usize = 0;\n        while ({var}_idx < {var}_count) : ({var}_idx += 1) {{\n"));
                }
            }
            let inner_dec = match inner.as_ref() {
                FieldType::Named(n) if is_message_name(n, messages) => format!("{n}BodyDecoder"),
                FieldType::Named(n) => {
                    if lang == SbeTargetLang::Zig {
                        n.clone()
                    } else {
                        format!("{n}Decoder")
                    }
                }
                _ => String::new(),
            };
            if inner_is_var {
                // u32 item length, then the item body
                match lang {
                    SbeTargetLang::Go => out.push_str(&format!("\t\titem_len := int(readU32BE(buf, &pos))\n\t\titem_end := pos + item_len\n\t\titem, _, err := {inner_dec}Decode(buf, pos)\n\t\tif err != nil {{\n\t\t\treturn {parent}{{}}, pos, err\n\t\t}}\n\t\tpos = item_end\n\t\t{var} = append({var}, item)\n")),
                    SbeTargetLang::Cpp => out.push_str(&format!("            uint32_t item_len = read_u32_be(buf, pos);\n            std::size_t item_end = pos + item_len;\n            auto [item, consumed] = {inner_dec}::decode(buf, pos);\n            (void)consumed;\n            pos = item_end;\n            {var}.push_back(item);\n")),
                    SbeTargetLang::Csharp => out.push_str(&format!("                uint itemLen = Wire.ReadU32BE(buf, ref pos);\n                int itemEnd = pos + (int)itemLen;\n                var item = {inner_dec}.Decode(buf, pos).Value;\n                pos = itemEnd;\n                {var}.Add(item);\n")),
                    SbeTargetLang::TypeScript => out.push_str(&format!("    const itemLen = readU32BE(buf, pos);\n    const itemEnd = pos.value + itemLen;\n    const item = {inner_dec}Decode(buf, pos);\n    pos.value = itemEnd;\n    {var}.push(item);\n")),
                    SbeTargetLang::Zig => out.push_str(&format!("            const item_len = readU32BE(buf, &pos);\n            const item_end = pos + item_len;\n            const item = try {inner_dec}.decode(allocator, buf, &pos);\n            pos = item_end;\n            {var}[{var}_idx] = item;\n")),
                }
            } else {
                match lang {
                    SbeTargetLang::Go => out.push_str(&format!(
                        "\t\titem := {}\n\t\t{var} = append({var}, item)\n",
                        decode_scalar_expr(lang, inner, "buf")
                    )),
                    SbeTargetLang::Cpp => out.push_str(&format!(
                        "            {var}.push_back({});\n",
                        decode_scalar_expr(lang, inner, "buf")
                    )),
                    SbeTargetLang::Csharp => out.push_str(&format!(
                        "                {var}.Add({});\n",
                        decode_scalar_expr(lang, inner, "buf")
                    )),
                    SbeTargetLang::TypeScript => out.push_str(&format!(
                        "    {var}.push({});\n",
                        decode_scalar_expr(lang, inner, "buf")
                    )),
                    SbeTargetLang::Zig => out.push_str(&format!(
                        "            {var}[{var}_idx] = {};\n",
                        decode_scalar_expr(lang, inner, "buf")
                    )),
                }
            }
            out.push_str(match lang {
                SbeTargetLang::Go => "\t}\n",
                SbeTargetLang::Cpp => "        }\n",
                SbeTargetLang::Csharp => "            }\n",
                SbeTargetLang::TypeScript => "  }\n",
                SbeTargetLang::Zig => "        }\n",
            });
        }
        FieldType::Named(n) if is_message_name(n, messages) => {
            out.push_str(&decode_body_value(lang, n, &var, parent));
        }
        FieldType::Named(n) if is_struct_name(n, structs) => {
            out.push_str(&decode_named_value(lang, n, &var, parent));
        }
        _ => {
            if sbe_field_wire_type_of(&field.field_type) == "str" {
                if optional {
                    out.push_str(&decode_optional_string(lang, &var, &mut decls));
                } else {
                    out.push_str(&decode_string(lang, &var));
                }
            } else if let Some(kind) = field_scalar_kind(&field.field_type) {
                decode_scalar(lang, &var, optional, kind, &mut out);
            } else {
                panic!(
                    "sbe target codegen: unsupported field type {:?}",
                    field.field_type
                );
            }
        }
    }
    (decls, out)
}

/// Scalar read expression for list items (advanced immediately).
fn decode_scalar_expr(lang: SbeTargetLang, ft: &FieldType, buf: &str) -> String {
    let kind = field_scalar_kind(ft).expect("scalar list item");
    let raw = emit_read_scalar(lang, buf, kind);
    kind.lang_from_wire(lang, &raw)
}

/// Decode a nested message through its Body decoder.
fn decode_body_value(lang: SbeTargetLang, name: &str, var: &str, parent: &str) -> String {
    match lang {
        SbeTargetLang::Go => format!("\t{var}, {var}_pos, {var}_err := {name}BodyDecoderDecode(buf, pos)\n\tif {var}_err != nil {{\n\t\treturn {parent}{{}}, pos, {var}_err\n\t}}\n\tpos = {var}_pos\n"),
        SbeTargetLang::Cpp => format!("        auto [{var}, {var}_n] = {name}BodyDecoder::decode(buf, pos);\n        pos += {var}_n;\n"),
        SbeTargetLang::Csharp => format!("            var ({var}, {var}Pos) = {name}BodyDecoder.Decode(buf, pos);\n            pos = {var}Pos;\n"),
        SbeTargetLang::TypeScript => format!("  const {var} = {name}BodyDecoderDecode(buf, pos);\n"),
        SbeTargetLang::Zig => format!("        const {var} = try {name}BodyDecoder.decode(allocator, buf, &pos);\n"),
    }
}

fn decode_named_value(lang: SbeTargetLang, name: &str, var: &str, parent: &str) -> String {
    match lang {
        SbeTargetLang::Go => format!("\t{var}, {var}_pos, {var}_err := {name}DecoderDecode(buf, pos)\n\tif {var}_err != nil {{\n\t\treturn {parent}{{}}, pos, {var}_err\n\t}}\n\tpos = {var}_pos\n"),
        SbeTargetLang::Cpp => format!("        auto [{var}, {var}_n] = {name}Decoder::decode(buf, pos);\n        pos += {var}_n;\n"),
        SbeTargetLang::Csharp => format!("            var ({var}, {var}Pos) = {name}Decoder.Decode(buf, pos);\n            pos = {var}Pos;\n"),
        SbeTargetLang::TypeScript => format!("  const {var} = {name}DecoderDecode(buf, pos);\n"),
        SbeTargetLang::Zig => format!("        const {var} = try {name}.decode(allocator, buf, &pos);\n"),
    }
}

fn decode_string(lang: SbeTargetLang, var: &str) -> String {
    match lang {
        SbeTargetLang::Go => format!("\t{var} := readString(buf, &pos)\n"),
        SbeTargetLang::Cpp => format!("        std::string {var} = read_string(buf, pos);\n"),
        SbeTargetLang::Csharp => {
            format!("            string {var} = Wire.ReadString(buf, ref pos);\n")
        }
        SbeTargetLang::TypeScript => format!("  const {var} = readString(buf, pos);\n"),
        SbeTargetLang::Zig => {
            format!("        const {var} = try readString(buf, &pos, allocator);\n")
        }
    }
}

fn decode_optional_string(lang: SbeTargetLang, var: &str, decls: &mut String) -> String {
    match lang {
        SbeTargetLang::Go => {
            decls.push_str(&format!("\tvar {var} *string\n"));
            format!("\tif buf[pos] == 1 {{\n\t\tpos++\n\t\ts := readString(buf, &pos)\n\t\t{var} = &s\n\t}} else {{\n\t\tpos++\n\t}}\n")
        }
        SbeTargetLang::Cpp => {
            decls.push_str(&format!("        std::optional<std::string> {var};\n"));
            format!("        if (take_u8(buf, pos) == 1) {var} = read_string(buf, pos);\n")
        }
        SbeTargetLang::Csharp => {
            decls.push_str(&format!("            string? {var};\n"));
            format!("            if (TakeU8(buf, ref pos) == 1) {var} = Wire.ReadString(buf, ref pos);\n")
        }
        SbeTargetLang::TypeScript => {
            decls.push_str(&format!("  let {var}: string | null = null;\n"));
            format!("  if (readU8(buf, pos) === 1) {var} = readString(buf, pos);\n")
        }
        SbeTargetLang::Zig => {
            decls.push_str(&format!("        var {var}: ?[]u8 = null;\n"));
            format!("        if (buf[pos] == 1) {{\n            pos += 1;\n            {var} = try readString(buf, &pos, allocator);\n        }} else {{\n            pos += 1;\n        }}\n")
        }
    }
}

/// Field type including struct/message element names.
fn target_field_type_full(
    lang: SbeTargetLang,
    ft: &FieldType,
    parent: &str,
    field_name: &str,
    structs: &std::collections::HashSet<String>,
    messages: &std::collections::HashSet<String>,
) -> String {
    target_field_type_ctx(lang, ft, parent, field_name, structs, messages)
}

/// Zig discards for parameters the generated body does not reference.
fn zig_discards(body: &str, params: &[&str]) -> String {
    let mut out = String::new();
    for p in params {
        let used = body.contains(&format!("{p}."))
            || body.contains(&format!("{p} "))
            || body.contains(&format!("&{p}"))
            || body.contains(&format!("{p},"))
            || body.contains(&format!("({p}"))
            || body.contains(&format!("{p}="))
            || body.contains(&format!("{p})"));
        if !used {
            out.push_str(&format!("        _ = {p};\n"));
        }
    }
    out
}

// ── Message headers ───────────────────────────────────────────

fn target_write_header(lang: SbeTargetLang, template_id: u16, block_length: u16) -> String {
    match lang {
        SbeTargetLang::Go => format!(
            "\twriteU16BE(&buf, SCHEMA_ID)\n\twriteU16BE(&buf, {template_id})\n\twriteU16BE(&buf, 0)\n\twriteU16BE(&buf, {block_length})\n"
        ),
        SbeTargetLang::Cpp => format!(
            "        write_u16_be(buf, SCHEMA_ID);\n        write_u16_be(buf, {template_id});\n        write_u16_be(buf, 0);\n        write_u16_be(buf, {block_length});\n"
        ),
        SbeTargetLang::Csharp => format!(
            "            Wire.WriteU16BE(buf, SCHEMA_ID);\n            Wire.WriteU16BE(buf, {template_id});\n            Wire.WriteU16BE(buf, 0);\n            Wire.WriteU16BE(buf, {block_length});\n"
        ),
        SbeTargetLang::TypeScript => format!(
            "  writeU16BE(buf, SCHEMA_ID);\n  writeU16BE(buf, {template_id});\n  writeU16BE(buf, 0);\n  writeU16BE(buf, {block_length});\n"
        ),
        SbeTargetLang::Zig => format!(
            "        try writeU16BE(&buf, SCHEMA_ID);\n        try writeU16BE(&buf, {template_id});\n        try writeU16BE(&buf, 0);\n        try writeU16BE(&buf, {block_length});\n"
        ),
    }
}

/// Statements validating an SBE message header for `msg_name` / `template_id`.
fn target_check_header(lang: SbeTargetLang, msg_name: &str, template_id: u16) -> String {
    match lang {
        SbeTargetLang::Go => format!(
            "\tif len(buf) < 8 {{\n\t\treturn {msg_name}{{}}, 0, fmt.Errorf(\"buffer too short for SBE header\")\n\t}}\n\tpos := 0\n\tschemaID := readU16BE(buf, &pos)\n\ttmplID := readU16BE(buf, &pos)\n\t_ = readU16BE(buf, &pos)\n\t_ = readU16BE(buf, &pos)\n\tif schemaID != SCHEMA_ID {{\n\t\treturn {msg_name}{{}}, pos, fmt.Errorf(\"invalid schema_id: %d\", schemaID)\n\t}}\n\tif tmplID != {template_id} {{\n\t\treturn {msg_name}{{}}, pos, fmt.Errorf(\"invalid template_id: %d\", tmplID)\n\t}}\n"
        ),
        SbeTargetLang::Cpp => format!(
            "        if (buf.size() < 8) throw std::runtime_error(\"buffer too short for SBE header\");\n        std::size_t pos = 0;\n        uint16_t schema_id = read_u16_be(buf, pos);\n        uint16_t tmpl_id = read_u16_be(buf, pos);\n        (void)read_u16_be(buf, pos);\n        (void)read_u16_be(buf, pos);\n        if (schema_id != SCHEMA_ID) throw std::runtime_error(\"invalid schema_id\");\n        if (tmpl_id != {template_id}) throw std::runtime_error(\"invalid template_id\");\n"
        ),
        SbeTargetLang::Csharp => format!(
            "            if (buf.Length < 8) throw new InvalidOperationException(\"buffer too short for SBE header\");\n            int pos = 0;\n            ushort schemaId = Wire.ReadU16BE(buf, ref pos);\n            ushort tmplId = Wire.ReadU16BE(buf, ref pos);\n            _ = Wire.ReadU16BE(buf, ref pos);\n            _ = Wire.ReadU16BE(buf, ref pos);\n            if (schemaId != SCHEMA_ID) throw new InvalidOperationException(\"invalid schema_id\");\n            if (tmplId != {template_id}) throw new InvalidOperationException(\"invalid template_id\");\n"
        ),
        SbeTargetLang::TypeScript => format!(
            "  if (buf.length < 8) throw new Error(\"buffer too short for SBE header\");\n  const pos = {{ value: 0 }};\n  const schemaId = readU16BE(buf, pos);\n  const tmplId = readU16BE(buf, pos);\n  readU16BE(buf, pos);\n  readU16BE(buf, pos);\n  if (schemaId !== SCHEMA_ID) throw new Error(\"invalid schema_id\");\n  if (tmplId !== {template_id}) throw new Error(\"invalid template_id\");\n"
        ),
        SbeTargetLang::Zig => format!(
            "        if (buf.len < 8) return error.BufferTooShort;\n        var pos: usize = 0;\n        const schema_id = readU16BE(buf, &pos);\n        const tmpl_id = readU16BE(buf, &pos);\n        _ = readU16BE(buf, &pos);\n        _ = readU16BE(buf, &pos);\n        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;\n        if (tmpl_id != {template_id}) return error.InvalidTemplateId;\n"
        ),
    }
}

// ── Message emission ──────────────────────────────────────────

type NameSets = (
    std::collections::HashSet<String>,
    std::collections::HashSet<String>,
);

fn generate_target_message_encoder(
    lang: SbeTargetLang,
    msg: &Message,
    template_id: u16,
    sets: &NameSets,
) -> String {
    let (structs, messages) = sets;
    let block_length = compute_block_length(&msg.fields);
    let params = target_encoder_params(lang, msg, structs, messages);
    let enc = format!("{}Encoder", msg.name);
    let header = target_write_header(lang, template_id, block_length);
    let body = enc_fields(lang, &msg.fields, &msg.name, "", "buf", structs, messages);
    match lang {
        SbeTargetLang::Go => format!(
            "// SBE encoder for {name}\ntype {enc} struct{{}}\n\n// Encode builds a complete message (header + body).\nfunc ({enc}) Encode({params}) ([]byte, error) {{\n\tbuf := make([]byte, 0, 256)\n{header}{body}\treturn buf, nil\n}}\n",
            name = msg.name
        ),
        SbeTargetLang::Cpp => format!(
            "// SBE encoder for {name}\nstruct {enc} {{\n    static std::vector<uint8_t> encode({params});\n}};\n\ninline std::vector<uint8_t> {enc}::encode({params}) {{\n        std::vector<uint8_t> buf;\n{header}{body}        return buf;\n}}\n",
            name = msg.name
        ),
        SbeTargetLang::Csharp => format!(
            "    /// <summary>SBE encoder for {name}</summary>\n    public static class {enc}\n    {{\n        public static byte[] Encode({params})\n        {{\n            var buf = new List<byte>();\n{header}{body}            return buf.ToArray();\n        }}\n    }}\n",
            name = msg.name
        ),
        SbeTargetLang::TypeScript => format!(
            "/** SBE encoder for {name} — returns the complete message. */\nexport function {enc}Encode({params}): Uint8Array {{\n  const buf: number[] = [];\n{header}{body}  return new Uint8Array(buf);\n}}\n",
            name = msg.name
        ),
        SbeTargetLang::Zig => format!(
            "/// SBE encoder for {name}\npub const {enc} = struct {{\n    pub fn encode(allocator: std.mem.Allocator, {params}) ![]u8 {{\n        var buf = std.ArrayList(u8).init(allocator);\n        defer buf.deinit();\n{header}{body}        return buf.toOwnedSlice();\n    }}\n}};\n",
            name = msg.name
        ),
    }
}

/// Body encoder: writes a message's fields only (no header), used for nested messages.
fn generate_target_message_body_encoder(
    lang: SbeTargetLang,
    msg: &Message,
    sets: &NameSets,
) -> String {
    let (structs, messages) = sets;
    let name = &msg.name;
    let enc = format!("{name}BodyEncoder");
    let go_buf = if lang == SbeTargetLang::Go {
        "local"
    } else {
        "buf"
    };
    let body = enc_fields(lang, &msg.fields, name, "value.", go_buf, structs, messages);
    let zig_discard = zig_discards(&body, &["allocator", "value", "buf"]);
    match lang {
        SbeTargetLang::Go => format!(
            "// SBE body encoder for {name} (fields only, no header).\ntype {enc} struct{{}}\n\nfunc ({enc}) Encode(value {name}, buf *[]byte) {{\n\tlocal := *buf\n{body}\t*buf = local\n}}\n"
        ),
        SbeTargetLang::Cpp => format!(
            "// SBE body encoder for {name} (fields only, no header).\nstruct {enc} {{\n    static void encode(const {name}& value, {buf});\n}};\n\ninline void {enc}::encode(const {name}& value, {buf}) {{\n{body}}}\n",
            buf = buf_param(lang)
        ),
        SbeTargetLang::Csharp => format!(
            "    /// <summary>SBE body encoder for {name} (fields only, no header).</summary>\n    public static class {enc}\n    {{\n        public static void Encode({name} value, List<byte> buf)\n        {{\n{body}        }}\n    }}\n"
        ),
        SbeTargetLang::TypeScript => format!(
            "/** SBE body encoder for {name} (fields only, no header). */\nexport function {enc}Encode(value: {name}, buf: number[]): void {{\n{body}}}\n"
        ),
        SbeTargetLang::Zig => format!(
            "/// SBE body encoder for {name} (fields only, no header).\npub const {enc} = struct {{\n    pub fn encode(allocator: std.mem.Allocator, value: {name}, buf: *std.ArrayList(u8)) !void {{\n{discard}{body}    }}\n}};\n",
            discard = zig_discard
        ),
    }
}

/// Body decoder: fields only, starts at `pos`.
fn generate_target_message_body_decoder(
    lang: SbeTargetLang,
    msg: &Message,
    sets: &NameSets,
) -> String {
    let (structs, messages) = sets;
    let name = &msg.name;
    let dec = format!("{name}BodyDecoder");
    let (decls, body) = dec_fields(lang, &msg.fields, name, structs, messages);
    match lang {
        SbeTargetLang::Go => format!(
            "// SBE body decoder for {name}: reads fields at pos, returns the value and the new offset.\nfunc {dec}Decode(buf []byte, pos int) ({name}, int, error) {{\n{decls}{body}\treturn value, pos, nil\n}}\n"
        ),
        SbeTargetLang::Cpp => format!(
            "// SBE body decoder for {name}: reads fields at pos.\nstruct {dec} {{\n    static std::pair<{name}, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);\n}};\n\ninline std::pair<{name}, std::size_t> {dec}::decode(const std::vector<uint8_t>& buf, std::size_t pos) {{\n    {name} value{{}};\n{decls}{body}    return {{value, pos}};\n}}\n"
        ),
        SbeTargetLang::Csharp => format!(
            "    /// <summary>SBE body decoder for {name}: reads fields at pos.</summary>\n    public static class {dec}\n    {{\n        public static ({name} Value, int Pos) Decode(byte[] buf, int pos)\n        {{\n            var value = new {name}();\n{decls}{body}            return (value, pos);\n        }}\n    }}\n"
        ),
        SbeTargetLang::TypeScript => format!(
            "/** SBE body decoder for {name}: reads fields at `pos`. */\nexport function {dec}Decode(buf: Uint8Array, pos: {{ value: number }}): {name} {{\n{decls}{body}  return value;\n}}\n"
        ),
        SbeTargetLang::Zig => format!(
            "/// SBE body decoder for {name}: reads fields at `pos`.\npub const {dec} = struct {{\n    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !{name} {{\n{discard}        {vkw} value: {name} = undefined;\n{decls}{body}        return value;\n    }}\n}};\n",
            discard = zig_discards(&format!("{decls}{body}"), &["allocator", "buf", "pos"]),
            vkw = if body.contains("value.") { "var" } else { "const" }
        ),
    }
}

/// Full message decoder: validates the header, then reads the body.
fn generate_target_message_decoder(
    lang: SbeTargetLang,
    msg: &Message,
    template_id: u16,
    sets: &NameSets,
) -> String {
    let (structs, messages) = sets;
    let name = &msg.name;
    let dec = format!("{name}Decoder");
    let header = target_check_header(lang, name, template_id);
    let (decls, body) = dec_fields(lang, &msg.fields, name, structs, messages);
    match lang {
        SbeTargetLang::Go => format!(
            "// SBE decoder for {name}: validates the header and reads the body.\n// Returns the value, the offset just past the message, and an error.\nfunc {dec}Decode(buf []byte) ({name}, int, error) {{\n{header}{decls}{body}\treturn value, pos, nil\n}}\n"
        ),
        SbeTargetLang::Cpp => format!(
            "// SBE decoder for {name}: validates the header and reads the body.\nstruct {dec} {{\n    static {name} decode(const std::vector<uint8_t>& buf);\n}};\n\ninline {name} {dec}::decode(const std::vector<uint8_t>& buf) {{\n    {name} value{{}};\n{header}{decls}{body}    (void)value;\n    return value;\n}}\n"
        ),
        SbeTargetLang::Csharp => format!(
            "    /// <summary>SBE decoder for {name}: validates the header and reads the body.</summary>\n    public static class {dec}\n    {{\n        public static {name} Decode(byte[] buf)\n        {{\n            var value = new {name}();\n{header}{decls}{body}            return value;\n        }}\n    }}\n"
        ),
        SbeTargetLang::TypeScript => format!(
            "/** SBE decoder for {name}: validates the header and reads the body. */\nexport function {dec}Decode(buf: Uint8Array): {name} {{\n{header}{decls}{body}  return value;\n}}\n"
        ),
        SbeTargetLang::Zig => format!(
            "/// SBE decoder for {name}: validates the header and reads the body.\npub const {dec} = struct {{\n    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !{name} {{\n{header}{discard}        {vkw} value: {name} = undefined;\n{decls}{body}        return value;\n    }}\n}};\n",
            discard = zig_discards(&format!("{header}{decls}{body}"), &["allocator", "buf", "pos"]),
            vkw = if body.contains("value.") { "var" } else { "const" }
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_merged_trading_schema;

    #[test]
    fn test_generate_sbe_target_orders_go() {
        let schema = load_merged_trading_schema();
        let code = generate_sbe_target(&schema, SbeTargetLang::Go);
        assert!(code.contains("NewOrderSingleEncoder"));
        assert!(code.contains("SCHEMA_ID"));
        assert!(code.contains("package figsbe"));
    }

    #[test]
    fn test_generate_sbe_target_orders_cpp() {
        let schema = load_merged_trading_schema();
        let code = generate_sbe_target(&schema, SbeTargetLang::Cpp);
        assert!(code.contains("NewOrderSingleEncoder"));
        assert!(code.contains("SCHEMA_ID"));
        assert!(code.contains("namespace fig::sbe"));
    }

    #[test]
    fn test_generate_sbe_target_simple() {
        let input = r#"
            schema test v1.0 {
                message Greeting {
                    channel_type: request_response
                    name: string(max_len: 32) @1
                    language: enum { en, fr, de } @2
                }
            }
        "#;
        let schema = crate::Parser::parse(input).unwrap();
        for lang in [
            SbeTargetLang::Go,
            SbeTargetLang::Cpp,
            SbeTargetLang::Csharp,
            SbeTargetLang::TypeScript,
            SbeTargetLang::Zig,
        ] {
            let code = generate_sbe_target(&schema, lang);
            assert!(code.contains("GreetingEncoder"), "{lang:?}");
            assert!(code.contains("SCHEMA_ID"), "{lang:?}");
        }
    }
}
