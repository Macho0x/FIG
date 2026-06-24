//! Multi-language SBE encode/decode code generator from FSL AST.
//!
//! Mirrors the wire layout produced by [`crate::sbe_codegen::generate_sbe`].

use crate::ast::*;
use crate::sbe_codegen::{
    compute_block_length, encoder_params, generate_sbe_enum, generate_sbe_inline_struct,
    generate_sbe_message_decoder, generate_sbe_message_encoder, pascal_case,
    sbe_field_type_name_for_message, sbe_field_wire_type, sbe_named_type_to_wire_type,
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

    let (enum_types, inline_struct_types) = collect_schema_types(schema);
    for (name, variants) in &enum_types {
        out.push_str(&generate_target_enum(lang, name, variants));
        out.push('\n');
    }
    for (name, fields) in &inline_struct_types {
        out.push_str(&generate_target_inline_struct(lang, name, fields));
        out.push('\n');
    }
    for (template_id, msg) in schema.messages.iter().enumerate() {
        let tid = (template_id + 1) as u16;
        out.push_str(&generate_target_message_encoder(lang, msg, tid));
        out.push('\n');
        out.push_str(&generate_target_message_decoder(lang, msg, tid));
        out.push('\n');
    }
    out.push_str(&file_footer(lang));
    out
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
            "//go:build !conformance\n\n// Auto-generated SBE encode/decode by fig-fsl from schema '{}' v{}\n// {}\n\npackage figsbe\n\nimport (\n\t\"fmt\"\n\t\"math\"\n)\n\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::Cpp => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' v{}\n// {}\n\n#pragma once\n\n#include <cstdint>\n#include <cstring>\n#include <optional>\n#include <stdexcept>\n#include <string>\n#include <vector>\n\nnamespace fig::sbe {{\n\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::Csharp => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' v{}\n// {}\n\nusing System;\nusing System.Collections.Generic;\nusing System.Text;\n\nnamespace Fig.Sbe\n{{\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::TypeScript => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' v{}\n// {}\n\n",
            schema.name, schema.version, desc
        ),
        SbeTargetLang::Zig => format!(
            "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' v{}\n// {}\n\nconst std = @import(\"std\");\n\n",
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
            "func writeU16BE(buf *[]byte, v uint16) {\n\t*buf = append(*buf, byte(v>>8), byte(v))\n}\n\nfunc writeU32BE(buf *[]byte, v uint32) {\n\t*buf = append(*buf, byte(v>>24), byte(v>>16), byte(v>>8), byte(v))\n}\n\nfunc writeU64BE(buf *[]byte, v uint64) {\n\t*buf = append(*buf, byte(v>>56), byte(v>>48), byte(v>>40), byte(v>>32), byte(v>>24), byte(v>>16), byte(v>>8), byte(v))\n}\n\nfunc writeF64BE(buf *[]byte, v float64) { writeU64BE(buf, math.Float64bits(v)) }\nfunc writeI64BE(buf *[]byte, v int64) { writeU64BE(buf, uint64(v)) }\n\nfunc writeString(buf *[]byte, s string) {\n\twriteU16BE(buf, uint16(len(s)))\n\t*buf = append(*buf, []byte(s)...)\n}\n\nfunc readU16BE(buf []byte, pos *int) uint16 {\n\tv := uint16(buf[*pos])<<8 | uint16(buf[*pos+1])\n\t*pos += 2\n\treturn v\n}\n\nfunc readU32BE(buf []byte, pos *int) uint32 {\n\tv := uint32(buf[*pos])<<24 | uint32(buf[*pos+1])<<16 | uint32(buf[*pos+2])<<8 | uint32(buf[*pos+3])\n\t*pos += 4\n\treturn v\n}\n\nfunc readU64BE(buf []byte, pos *int) uint64 {\n\tv := uint64(buf[*pos])<<56 | uint64(buf[*pos+1])<<48 | uint64(buf[*pos+2])<<40 | uint64(buf[*pos+3])<<32 |\n\t\tuint64(buf[*pos+4])<<24 | uint64(buf[*pos+5])<<16 | uint64(buf[*pos+6])<<8 | uint64(buf[*pos+7])\n\t*pos += 8\n\treturn v\n}\n\nfunc readF64BE(buf []byte, pos *int) float64 { return math.Float64frombits(readU64BE(buf, pos)) }\nfunc readI64BE(buf []byte, pos *int) int64 { return int64(readU64BE(buf, pos)) }\n\nfunc readString(buf []byte, pos *int) string {\n\tn := int(readU16BE(buf, pos))\n\ts := string(buf[*pos : *pos+n])\n\t*pos += n\n\treturn s\n}\n".to_string()
        }
        SbeTargetLang::Cpp => r#"inline void write_u16_be(std::vector<uint8_t>& buf, uint16_t v) {
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
        SbeTargetLang::Csharp => r#"    internal static class Wire
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
        SbeTargetLang::TypeScript => r#"function writeU16BE(buf: number[], v: number): void { buf.push((v >> 8) & 0xff, v & 0xff); }
function writeU32BE(buf: number[], v: number): void { buf.push((v >> 24) & 0xff, (v >> 16) & 0xff, (v >> 8) & 0xff, v & 0xff); }
function writeU64BE(buf: number[], v: bigint): void { for (let i = 7; i >= 0; i--) buf.push(Number((v >> BigInt(i * 8)) & 0xffn)); }
function writeF64BE(buf: number[], v: number): void { const dv = new DataView(new ArrayBuffer(8)); dv.setFloat64(0, v, false); for (let i = 0; i < 8; i++) buf.push(dv.getUint8(i)); }
function writeI64BE(buf: number[], v: bigint): void { writeU64BE(buf, BigInt.asUintN(64, v)); }
function writeString(buf: number[], s: string): void { const bytes = new TextEncoder().encode(s); writeU16BE(buf, bytes.length); for (const b of bytes) buf.push(b); }
function readU16BE(buf: Uint8Array, pos: { value: number }): number { const v = (buf[pos.value] << 8) | buf[pos.value + 1]; pos.value += 2; return v; }
function readU32BE(buf: Uint8Array, pos: { value: number }): number { const v = (buf[pos.value] << 24) | (buf[pos.value + 1] << 16) | (buf[pos.value + 2] << 8) | buf[pos.value + 3]; pos.value += 4; return v >>> 0; }
function readU64BE(buf: Uint8Array, pos: { value: number }): bigint { let v = 0n; for (let i = 0; i < 8; i++) v = (v << 8n) | BigInt(buf[pos.value + i]); pos.value += 8; return v; }
function readF64BE(buf: Uint8Array, pos: { value: number }): number { const dv = new DataView(buf.buffer, buf.byteOffset + pos.value, 8); const v = dv.getFloat64(0, false); pos.value += 8; return v; }
function readI64BE(buf: Uint8Array, pos: { value: number }): bigint { return readU64BE(buf, pos); }
function readString(buf: Uint8Array, pos: { value: number }): string { const len = readU16BE(buf, pos); const s = new TextDecoder().decode(buf.subarray(pos.value, pos.value + len)); pos.value += len; return s; }
"#.to_string(),
        SbeTargetLang::Zig => r#"fn writeU16BE(buf: *std.ArrayList(u8), v: u16) !void { try buf.append(@intCast(v >> 8)); try buf.append(@intCast(v & 0xFF)); }
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

fn target_field_type(
    lang: SbeTargetLang,
    ft: &FieldType,
    msg_name: &str,
    field_name: &str,
) -> String {
    match ft {
        FieldType::Enum(_) | FieldType::InlineStruct(_) => {
            sbe_field_type_name_for_message(ft, msg_name, field_name)
        }
        FieldType::List(inner) => {
            let inner_name = target_field_type(lang, inner, msg_name, field_name);
            match lang {
                SbeTargetLang::Go => format!("[]{inner_name}"),
                SbeTargetLang::Cpp => format!("std::vector<{inner_name}>"),
                SbeTargetLang::Csharp => format!("List<{inner_name}>"),
                SbeTargetLang::TypeScript => format!("{inner_name}[]"),
                SbeTargetLang::Zig => format!("[]{inner_name}"),
            }
        }
        _ => map_named_type(
            lang,
            &sbe_field_type_name_for_message(ft, msg_name, field_name),
        ),
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
        "i64" | "u64" => match lang {
            SbeTargetLang::Go => "int64".into(),
            SbeTargetLang::Cpp => "int64_t".into(),
            SbeTargetLang::Csharp => "long".into(),
            SbeTargetLang::TypeScript => "bigint".into(),
            SbeTargetLang::Zig => "i64".into(),
        },
        "u8" | "bool" => match lang {
            SbeTargetLang::Go => "uint8".into(),
            SbeTargetLang::Cpp => "uint8_t".into(),
            SbeTargetLang::Csharp => "byte".into(),
            SbeTargetLang::TypeScript => "number".into(),
            SbeTargetLang::Zig => "u8".into(),
        },
        _ => name.to_string(),
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

fn target_encoder_params(lang: SbeTargetLang, msg: &Message) -> String {
    msg.fields
        .iter()
        .map(|field| {
            let base = target_field_type(lang, &field.field_type, &msg.name, &field.name);
            let ty = if field.optional {
                target_optional_type(lang, &base)
            } else {
                base
            };
            format!("{} {}", target_param_name(lang, &field.name), ty)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_target_enum(lang: SbeTargetLang, name: &str, variants: &[String]) -> String {
    let _ = generate_sbe_enum(name, variants);
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

fn generate_target_inline_struct(lang: SbeTargetLang, name: &str, fields: &[Field]) -> String {
    let _ = generate_sbe_inline_struct(name, fields);
    let encoded_len = compute_block_length(fields);
    match lang {
        SbeTargetLang::Go => {
            let mut out = format!("type {name} struct {{\n");
            for field in fields {
                let base = target_field_type(lang, &field.field_type, name, &field.name);
                let ty = if field.optional {
                    target_optional_type(lang, &base)
                } else {
                    base
                };
                out.push_str(&format!(
                    "\t{} {}\n",
                    target_param_name(lang, &field.name),
                    ty
                ));
            }
            out.push_str(&format!("}}\n\nconst {name}EncodedLen = {encoded_len}\n"));
            out
        }
        SbeTargetLang::Cpp => {
            let mut out = format!("struct {name} {{\n");
            for field in fields {
                let base = target_field_type(lang, &field.field_type, name, &field.name);
                let ty = if field.optional {
                    target_optional_type(lang, &base)
                } else {
                    base
                };
                out.push_str(&format!("    {ty} {};\n", field.name));
            }
            out.push_str(&format!(
                "}};\n\ninline constexpr size_t {name}EncodedLen = {encoded_len};\n"
            ));
            out
        }
        SbeTargetLang::Csharp => {
            let mut out = format!("    public class {name}\n    {{\n");
            for field in fields {
                let base = target_field_type(lang, &field.field_type, name, &field.name);
                let ty = if field.optional {
                    target_optional_type(lang, &base)
                } else {
                    base
                };
                out.push_str(&format!(
                    "        public {ty} {} {{ get; set; }}\n",
                    target_param_name(lang, &field.name)
                ));
            }
            out.push_str(&format!(
                "    }}\n\n    public const int {name}EncodedLen = {encoded_len};\n"
            ));
            out
        }
        SbeTargetLang::TypeScript => {
            let mut out = format!("export interface {name} {{\n");
            for field in fields {
                let base = target_field_type(lang, &field.field_type, name, &field.name);
                let ty = if field.optional {
                    target_optional_type(lang, &base)
                } else {
                    base
                };
                out.push_str(&format!("  {}: {ty};\n", field.name));
            }
            out.push_str(&format!(
                "}}\n\nexport const {name}EncodedLen = {encoded_len};\n"
            ));
            out
        }
        SbeTargetLang::Zig => {
            let mut out = format!("pub const {name} = struct {{\n");
            for field in fields {
                let base = target_field_type(lang, &field.field_type, name, &field.name);
                let ty = if field.optional {
                    target_optional_type(lang, &base)
                } else {
                    base
                };
                out.push_str(&format!("    {}: {ty},\n", field.name));
            }
            out.push_str(&format!(
                "}};\n\npub const {name}EncodedLen: usize = {encoded_len};\n"
            ));
            out
        }
    }
}

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

fn decode_header(lang: SbeTargetLang, decoder_name: &str, template_id: u16) -> String {
    match lang {
        SbeTargetLang::Go => format!(
            "\tif len(buf) < 8 {{\n\t\treturn {decoder_name}{{}}, fmt.Errorf(\"buffer too short for SBE header\")\n\t}}\n\tpos := 0\n\tschemaID := readU16BE(buf, &pos)\n\ttmplID := readU16BE(buf, &pos)\n\t_ = readU16BE(buf, &pos)\n\t_ = readU16BE(buf, &pos)\n\tif schemaID != SCHEMA_ID {{\n\t\treturn {decoder_name}{{}}, fmt.Errorf(\"invalid schema_id: %d\", schemaID)\n\t}}\n\tif tmplID != {template_id} {{\n\t\treturn {decoder_name}{{}}, fmt.Errorf(\"invalid template_id: %d\", tmplID)\n\t}}\n"
        ),
        SbeTargetLang::Cpp => format!(
            "        if (buf.size() < 8) throw std::runtime_error(\"buffer too short for SBE header\");\n        size_t pos = 0;\n        uint16_t schema_id = read_u16_be(buf, pos);\n        uint16_t tmpl_id = read_u16_be(buf, pos);\n        read_u16_be(buf, pos); read_u16_be(buf, pos);\n        if (schema_id != SCHEMA_ID) throw std::runtime_error(\"invalid schema_id\");\n        if (tmpl_id != {template_id}) throw std::runtime_error(\"invalid template_id\");\n"
        ),
        SbeTargetLang::Csharp => format!(
            "            if (buf.Length < 8) throw new InvalidOperationException(\"buffer too short for SBE header\");\n            int pos = 0;\n            ushort schemaId = Wire.ReadU16BE(buf, ref pos);\n            ushort tmplId = Wire.ReadU16BE(buf, ref pos);\n            Wire.ReadU16BE(buf, ref pos); Wire.ReadU16BE(buf, ref pos);\n            if (schemaId != SCHEMA_ID) throw new InvalidOperationException(\"invalid schema_id\");\n            if (tmplId != {template_id}) throw new InvalidOperationException(\"invalid template_id\");\n"
        ),
        SbeTargetLang::TypeScript => format!(
            "  if (buf.length < 8) throw new Error('buffer too short for SBE header');\n  const pos = {{ value: 0 }};\n  const schemaId = readU16BE(buf, pos);\n  const tmplId = readU16BE(buf, pos);\n  readU16BE(buf, pos); readU16BE(buf, pos);\n  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');\n  if (tmplId !== {template_id}) throw new Error('invalid template_id');\n"
        ),
        SbeTargetLang::Zig => format!(
            "        if (buf.len < 8) return error.BufferTooShort;\n        var pos: usize = 0;\n        const schema_id = readU16BE(buf, &pos);\n        const tmpl_id = readU16BE(buf, &pos);\n        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);\n        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;\n        if (tmpl_id != {template_id}) return error.InvalidTemplateId;\n"
        ),
    }
}

fn target_encode_field(lang: SbeTargetLang, field: &Field, msg: &Message, buf_var: &str) -> String {
    let field_name = &field.name;
    let pname = target_param_name(lang, field_name);
    let optional = field.optional;
    let mut out = String::new();

    match &field.field_type {
        FieldType::Enum(_) => {
            let enum_name = format!("{}{}", msg.name, pascal_case(field_name));
            match lang {
                SbeTargetLang::Go if optional => out.push_str(&format!("\tif {pname} != nil {{ {buf_var} = append({buf_var}, byte({pname}.ToValue())) }} else {{ {buf_var} = append({buf_var}, 0) }}\n")),
                SbeTargetLang::Go => out.push_str(&format!("\t{buf_var} = append({buf_var}, byte({pname}.ToValue()))\n")),
                SbeTargetLang::Cpp if optional => out.push_str(&format!("        buf.push_back({pname}.has_value() ? {enum_name}ToValue(*{pname}) : 0);\n")),
                SbeTargetLang::Cpp => out.push_str(&format!("        buf.push_back({enum_name}ToValue({pname}));\n")),
                SbeTargetLang::Csharp if optional => out.push_str(&format!("            buf.Add({pname}.HasValue ? {enum_name}ToValue({pname}.Value) : (byte)0);\n")),
                SbeTargetLang::Csharp => out.push_str(&format!("            buf.Add({enum_name}ToValue({pname}));\n")),
                SbeTargetLang::TypeScript if optional => out.push_str(&format!("  buf.push({pname} != null ? {enum_name}ToValue({pname}) : 0);\n")),
                SbeTargetLang::TypeScript => out.push_str(&format!("  buf.push({enum_name}ToValue({pname}));\n")),
                SbeTargetLang::Zig if optional => out.push_str(&format!("        try buf.append(if ({pname}) |v| {enum_name}ToValue(v) else 0);\n")),
                SbeTargetLang::Zig => out.push_str(&format!("        try buf.append({enum_name}ToValue({pname}));\n")),
            }
        }
        FieldType::List(inner) => {
            let inner_type = sbe_field_type_name_for_message(inner, &msg.name, field_name);
            match lang {
                SbeTargetLang::Go => {
                    out.push_str(&format!("\twriteU32BE(&{buf_var}, uint32(len({pname})))\n\tfor _, item := range {pname} {{\n\t\t{inner_type}Encoder{{}}.Encode(item, &{buf_var})\n\t}}\n"));
                }
                SbeTargetLang::Cpp => {
                    out.push_str(&format!("        write_u32_be({buf_var}, static_cast<uint32_t>({pname}.size()));\n        for (const auto& item : {pname}) {{\n            {inner_type}Encoder::encode(item, {buf_var});\n        }}\n"));
                }
                SbeTargetLang::Csharp => {
                    out.push_str(&format!("            Wire.WriteU32BE({buf_var}, (uint){pname}.Count);\n            foreach (var item in {pname}) {{\n                {inner_type}Encoder.Encode(item, {buf_var});\n            }}\n"));
                }
                SbeTargetLang::TypeScript => {
                    out.push_str(&format!("  writeU32BE({buf_var}, {pname}.length);\n  for (const item of {pname}) {{\n    {inner_type}Encode(item, {buf_var});\n  }}\n"));
                }
                SbeTargetLang::Zig => {
                    out.push_str(&format!("        try writeU32BE(&{buf_var}, @intCast({pname}.len));\n        for ({pname}) |item| {{\n            try {inner_type}.encode(allocator, item, &{buf_var});\n        }}\n"));
                }
            }
        }
        FieldType::InlineStruct(_) => {
            let struct_name = format!("{}{}", msg.name, pascal_case(field_name));
            match lang {
                SbeTargetLang::Go => out.push_str(&format!(
                    "\t{struct_name}Encoder{{}}.Encode({pname}, &{buf_var})\n"
                )),
                SbeTargetLang::Cpp => out.push_str(&format!(
                    "        {struct_name}Encoder::encode({pname}, {buf_var});\n"
                )),
                SbeTargetLang::Csharp => out.push_str(&format!(
                    "            {struct_name}Encoder.Encode({pname}, {buf_var});\n"
                )),
                SbeTargetLang::TypeScript => {
                    out.push_str(&format!("  {struct_name}Encode({pname}, {buf_var});\n"))
                }
                SbeTargetLang::Zig => out.push_str(&format!(
                    "        try {struct_name}.encode(allocator, {pname}, &{buf_var});\n"
                )),
            }
        }
        _ => {
            let bt = sbe_field_wire_type(field);
            match bt.as_str() {
                "str" => encode_string(lang, buf_var, &pname, optional, &mut out),
                "f64" => encode_f64(lang, buf_var, &pname, optional, &mut out),
                "i64" | "u64" => encode_i64(lang, buf_var, &pname, optional, &mut out),
                "u8" | "bool" => encode_u8(lang, buf_var, &pname, optional, &mut out),
                _ => out.push_str(&format!("        // TODO: encode {field_name} as {bt}\n")),
            }
        }
    }
    out
}

fn encode_string(lang: SbeTargetLang, buf: &str, field: &str, optional: bool, out: &mut String) {
    if optional {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\tif {field} != nil {{\n\t\t{buf} = append({buf}, 1)\n\t\twriteString(&{buf}, *{field})\n\t}} else {{\n\t\t{buf} = append({buf}, 0)\n\t}}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        if ({field}.has_value()) {{ buf.push_back(1); write_string({buf}, *{field}); }} else {{ buf.push_back(0); }}\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            if ({field}.HasValue) {{ buf.Add(1); Wire.WriteString({buf}, {field}.Value); }} else {{ buf.Add(0); }}\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  if ({field} != null) {{ buf.push(1); writeString({buf}, {field}); }} else {{ buf.push(0); }}\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        if ({field}) |s| {{ try buf.append(1); try writeString(&{buf}, s); }} else {{ try buf.append(0); }}\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\twriteString(&{buf}, {field})\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        write_string({buf}, {field});\n")),
            SbeTargetLang::Csharp => {
                out.push_str(&format!("            Wire.WriteString({buf}, {field});\n"))
            }
            SbeTargetLang::TypeScript => out.push_str(&format!("  writeString({buf}, {field});\n")),
            SbeTargetLang::Zig => {
                out.push_str(&format!("        try writeString(&{buf}, {field});\n"))
            }
        }
    }
}

fn encode_f64(lang: SbeTargetLang, buf: &str, field: &str, optional: bool, out: &mut String) {
    if optional {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\tv := 0.0\n\tif {field} != nil {{ v = *{field} }}\n\twriteF64BE(&{buf}, v)\n\tif {field} != nil {{ {buf} = append({buf}, 1) }} else {{ {buf} = append({buf}, 0) }}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        write_f64_be({buf}, {field}.value_or(0.0));\n        buf.push_back({field}.has_value() ? 1 : 0);\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            Wire.WriteF64BE({buf}, {field} ?? 0.0);\n            buf.Add((byte)({field}.HasValue ? 1 : 0));\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  writeF64BE({buf}, {field} ?? 0);\n  buf.push({field} != null ? 1 : 0);\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        try writeF64BE(&{buf}, {field} orelse 0);\n        try buf.append(if ({field} != null) 1 else 0);\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\twriteF64BE(&{buf}, {field})\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        write_f64_be({buf}, {field});\n")),
            SbeTargetLang::Csharp => {
                out.push_str(&format!("            Wire.WriteF64BE({buf}, {field});\n"))
            }
            SbeTargetLang::TypeScript => out.push_str(&format!("  writeF64BE({buf}, {field});\n")),
            SbeTargetLang::Zig => {
                out.push_str(&format!("        try writeF64BE(&{buf}, {field});\n"))
            }
        }
    }
}

fn encode_i64(lang: SbeTargetLang, buf: &str, field: &str, optional: bool, out: &mut String) {
    if optional {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\tif {field} != nil {{ {buf} = append({buf}, 1); writeI64BE(&{buf}, *{field}) }} else {{ {buf} = append({buf}, 0) }}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        if ({field}.has_value()) {{ buf.push_back(1); write_i64_be({buf}, *{field}); }} else {{ buf.push_back(0); }}\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            if ({field}.HasValue) {{ buf.Add(1); Wire.WriteI64BE({buf}, {field}.Value); }} else {{ buf.Add(0); }}\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  if ({field} != null) {{ buf.push(1); writeI64BE({buf}, {field}); }} else {{ buf.push(0); }}\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        if ({field}) |v| {{ try buf.append(1); try writeI64BE(&{buf}, v); }} else {{ try buf.append(0); }}\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\twriteI64BE(&{buf}, {field})\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        write_i64_be({buf}, {field});\n")),
            SbeTargetLang::Csharp => {
                out.push_str(&format!("            Wire.WriteI64BE({buf}, {field});\n"))
            }
            SbeTargetLang::TypeScript => out.push_str(&format!("  writeI64BE({buf}, {field});\n")),
            SbeTargetLang::Zig => {
                out.push_str(&format!("        try writeI64BE(&{buf}, {field});\n"))
            }
        }
    }
}

fn encode_u8(lang: SbeTargetLang, buf: &str, field: &str, optional: bool, out: &mut String) {
    if optional {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\tv := uint8(0)\n\tif {field} != nil {{ v = *{field} }}\n\t{buf} = append({buf}, v)\n\tif {field} != nil {{ {buf} = append({buf}, 1) }} else {{ {buf} = append({buf}, 0) }}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        buf.push_back({field}.value_or(0));\n        buf.push_back({field}.has_value() ? 1 : 0);\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            buf.Add({field} ?? 0);\n            buf.Add((byte)({field}.HasValue ? 1 : 0));\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  buf.push({field} ?? 0);\n  buf.push({field} != null ? 1 : 0);\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        try buf.append({field} orelse 0);\n        try buf.append(if ({field} != null) 1 else 0);\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\t{buf} = append({buf}, {field})\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        buf.push_back({field});\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            buf.Add({field});\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  buf.push({field});\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        try buf.append({field});\n")),
        }
    }
}

fn target_decode_field(
    lang: SbeTargetLang,
    field: &Field,
    msg: &Message,
    decoder_name: &str,
) -> String {
    let var = field.name.clone();
    let optional = field.optional;
    let mut out = String::new();

    match &field.field_type {
        FieldType::Enum(_) => {
            let enum_name = format!("{}{}", msg.name, pascal_case(&var));
            match lang {
                SbeTargetLang::Go => out.push_str(&format!("\t{var}Raw := buf[pos]\n\tpos++\n\t{var}, err := {enum_name}FromValue({var}Raw)\n\tif err != nil {{ return {decoder_name}{{}}, err }}\n")),
                SbeTargetLang::Cpp => out.push_str(&format!("        uint8_t {var}_raw = buf[pos++];\n        auto {var}_opt = {enum_name}FromValue({var}_raw);\n        if (!{var}_opt) throw std::runtime_error(\"invalid {enum_name}\");\n        {enum_name} {var} = *{var}_opt;\n")),
                SbeTargetLang::Csharp => out.push_str(&format!("            byte {var}Raw = buf[pos++];\n            var {var} = {enum_name}FromValue({var}Raw) ?? throw new InvalidOperationException(\"invalid {enum_name}\");\n")),
                SbeTargetLang::TypeScript => out.push_str(&format!("  const {var}Raw = buf[pos.value++];\n  const {var} = {enum_name}FromValue({var}Raw);\n  if ({var} == null) throw new Error('invalid {enum_name}');\n")),
                SbeTargetLang::Zig => out.push_str(&format!("        const {var}_raw = buf[pos];\n        pos += 1;\n        const {var} = try {enum_name}FromValue({var}_raw);\n")),
            }
        }
        FieldType::List(inner) => {
            let inner_type = sbe_field_type_name_for_message(inner, &msg.name, &var);
            match lang {
                SbeTargetLang::Go => out.push_str(&format!("\t{var}Count := int(readU32BE(buf, &pos))\n\t{var} := make([]{inner_type}, 0, {var}Count)\n\tfor i := 0; i < {var}Count; i++ {{\n\t\titem, err := {inner_type}Decoder{{}}.Decode(buf[pos:])\n\t\tif err != nil {{ return {decoder_name}{{}}, err }}\n\t\tpos += item.EncodedLen()\n\t\t{var} = append({var}, item)\n\t}}\n")),
                SbeTargetLang::Cpp => out.push_str(&format!("        uint32_t {var}_count = read_u32_be(buf, pos);\n        std::vector<{inner_type}> {var};\n        {var}.reserve({var}_count);\n        for (uint32_t i = 0; i < {var}_count; ++i) {{\n            auto item = {inner_type}Decoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));\n            pos += item.encoded_len();\n            {var}.push_back(item);\n        }}\n")),
                SbeTargetLang::Csharp => out.push_str(&format!("            int {var}Count = (int)Wire.ReadU32BE(buf, ref pos);\n            var {var} = new List<{inner_type}>({var}Count);\n            for (int i = 0; i < {var}Count; i++) {{\n                var item = {inner_type}Decoder.Decode(buf[pos..]);\n                pos += item.EncodedLen();\n                {var}.Add(item);\n            }}\n")),
                SbeTargetLang::TypeScript => out.push_str(&format!("  const {var}Count = readU32BE(buf, pos);\n  const {var}: {inner_type}[] = [];\n  for (let i = 0; i < {var}Count; i++) {{\n    const item = {inner_type}DecoderDecode(buf.subarray(pos.value));\n    pos.value += item.encodedLen();\n    {var}.push(item);\n  }}\n")),
                SbeTargetLang::Zig => out.push_str(&format!("        const {var}_count = readU32BE(buf, &pos);\n        var {var} = try allocator.alloc({inner_type}, {var}_count);\n        var idx: usize = 0;\n        while (idx < {var}_count) : (idx += 1) {{\n            const item = try {inner_type}Decoder.decode(allocator, buf[pos..]);\n            pos += item.encodedLen();\n            {var}[idx] = item;\n        }}\n")),
            }
        }
        FieldType::InlineStruct(_) => {
            let struct_name = format!("{}{}", msg.name, pascal_case(&var));
            match lang {
                SbeTargetLang::Go => out.push_str(&format!("\t{var}, err := {struct_name}Decoder{{}}.Decode(buf[pos:])\n\tif err != nil {{ return {decoder_name}{{}}, err }}\n\tpos += {var}.EncodedLen()\n")),
                SbeTargetLang::Cpp => out.push_str(&format!("        auto {var} = {struct_name}Decoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));\n        pos += {var}.encoded_len();\n")),
                SbeTargetLang::Csharp => out.push_str(&format!("            var {var} = {struct_name}Decoder.Decode(buf[pos..]);\n            pos += {var}.EncodedLen();\n")),
                SbeTargetLang::TypeScript => out.push_str(&format!("  const {var} = {struct_name}DecoderDecode(buf.subarray(pos.value));\n  pos.value += {var}.encodedLen();\n")),
                SbeTargetLang::Zig => out.push_str(&format!("        const {var} = try {struct_name}Decoder.decode(allocator, buf[pos..]);\n        pos += {var}.encodedLen();\n")),
            }
        }
        _ => {
            let bt = sbe_field_wire_type(field);
            match bt.as_str() {
                "str" if optional => decode_optional_string(lang, &var, &mut out),
                "str" => decode_string(lang, &var, &mut out),
                "f64" => decode_f64(lang, &var, optional, &mut out),
                "i64" | "u64" => decode_i64(lang, &var, optional, &mut out),
                "u8" | "bool" => decode_u8(lang, &var, optional, &mut out),
                _ => out.push_str(&format!("        // TODO: decode {var}\n")),
            }
        }
    }
    out
}

fn decode_string(lang: SbeTargetLang, var: &str, out: &mut String) {
    match lang {
        SbeTargetLang::Go => out.push_str(&format!("\t{var} := readString(buf, &pos)\n")),
        SbeTargetLang::Cpp => out.push_str(&format!(
            "        std::string {var} = read_string(buf, pos);\n"
        )),
        SbeTargetLang::Csharp => out.push_str(&format!(
            "            string {var} = Wire.ReadString(buf, ref pos);\n"
        )),
        SbeTargetLang::TypeScript => {
            out.push_str(&format!("  const {var} = readString(buf, pos);\n"))
        }
        SbeTargetLang::Zig => out.push_str(&format!(
            "        const {var} = try readString(buf, &pos, allocator);\n"
        )),
    }
}

fn decode_optional_string(lang: SbeTargetLang, var: &str, out: &mut String) {
    match lang {
        SbeTargetLang::Go => out.push_str(&format!("\tif buf[pos] == 1 {{\n\t\tpos++\n\t\ts := readString(buf, &pos)\n\t\t{var} = &s\n\t}} else {{\n\t\tpos++\n\t\t{var} = nil\n\t}}\n")),
        SbeTargetLang::Cpp => out.push_str(&format!("        std::optional<std::string> {var};\n        if (buf[pos++] == 1) {var} = read_string(buf, pos);\n")),
        SbeTargetLang::Csharp => out.push_str(&format!("            string? {var};\n            if (buf[pos++] == 1) {var} = Wire.ReadString(buf, ref pos);\n")),
        SbeTargetLang::TypeScript => out.push_str(&format!("  let {var}: string | null;\n  if (buf[pos.value++] === 1) {var} = readString(buf, pos); else {var} = null;\n")),
        SbeTargetLang::Zig => out.push_str(&format!("        var {var}: ?[]u8 = null;\n        if (buf[pos] == 1) {{ pos += 1; {var} = try readString(buf, &pos, allocator); }} else {{ pos += 1; }}\n")),
    }
}

fn decode_f64(lang: SbeTargetLang, var: &str, optional: bool, out: &mut String) {
    if optional {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\traw := readF64BE(buf, &pos)\n\tif buf[pos] == 1 {{ pos++; {var} = &raw }} else {{ pos++; {var} = nil }}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        double {var}_raw = read_f64_be(buf, pos);\n        std::optional<double> {var};\n        if (buf[pos++] == 1) {var} = {var}_raw;\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            double {var}Raw = Wire.ReadF64BE(buf, ref pos);\n            double? {var} = buf[pos++] == 1 ? {var}Raw : null;\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  const {var}Raw = readF64BE(buf, pos);\n  const {var} = buf[pos.value++] === 1 ? {var}Raw : null;\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        const {var}_raw = readF64BE(buf, &pos);\n        var {var}: ?f64 = null;\n        if (buf[pos] == 1) {{ pos += 1; {var} = {var}_raw; }} else {{ pos += 1; }}\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\t{var} := readF64BE(buf, &pos)\n")),
            SbeTargetLang::Cpp => {
                out.push_str(&format!("        double {var} = read_f64_be(buf, pos);\n"))
            }
            SbeTargetLang::Csharp => out.push_str(&format!(
                "            double {var} = Wire.ReadF64BE(buf, ref pos);\n"
            )),
            SbeTargetLang::TypeScript => {
                out.push_str(&format!("  const {var} = readF64BE(buf, pos);\n"))
            }
            SbeTargetLang::Zig => {
                out.push_str(&format!("        const {var} = readF64BE(buf, &pos);\n"))
            }
        }
    }
}

fn decode_i64(lang: SbeTargetLang, var: &str, optional: bool, out: &mut String) {
    if optional {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\tif buf[pos] == 1 {{ pos++; v := readI64BE(buf, &pos); {var} = &v }} else {{ pos++; {var} = nil }}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        std::optional<int64_t> {var};\n        if (buf[pos++] == 1) {var} = read_i64_be(buf, pos);\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            long? {var};\n            if (buf[pos++] == 1) {var} = Wire.ReadI64BE(buf, ref pos);\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  let {var}: bigint | null = null;\n  if (buf[pos.value++] === 1) {var} = readI64BE(buf, pos);\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        var {var}: ?i64 = null;\n        if (buf[pos] == 1) {{ pos += 1; {var} = readI64BE(buf, &pos); }} else {{ pos += 1; }}\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\t{var} := readI64BE(buf, &pos)\n")),
            SbeTargetLang::Cpp => {
                out.push_str(&format!("        int64_t {var} = read_i64_be(buf, pos);\n"))
            }
            SbeTargetLang::Csharp => out.push_str(&format!(
                "            long {var} = Wire.ReadI64BE(buf, ref pos);\n"
            )),
            SbeTargetLang::TypeScript => {
                out.push_str(&format!("  const {var} = readI64BE(buf, pos);\n"))
            }
            SbeTargetLang::Zig => {
                out.push_str(&format!("        const {var} = readI64BE(buf, &pos);\n"))
            }
        }
    }
}

fn decode_u8(lang: SbeTargetLang, var: &str, optional: bool, out: &mut String) {
    if optional {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\tv := buf[pos]\n\tpos++\n\tif buf[pos] == 1 {{ pos++; {var} = &v }} else {{ pos++; {var} = nil }}\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        uint8_t v = buf[pos++];\n        std::optional<uint8_t> {var};\n        if (buf[pos++] == 1) {var} = v;\n")),
            SbeTargetLang::Csharp => out.push_str(&format!("            byte v = buf[pos++];\n            byte? {var} = buf[pos++] == 1 ? v : null;\n")),
            SbeTargetLang::TypeScript => out.push_str(&format!("  const v = buf[pos.value++];\n  const {var} = buf[pos.value++] === 1 ? v : null;\n")),
            SbeTargetLang::Zig => out.push_str(&format!("        const v = buf[pos];\n        pos += 1;\n        var {var}: ?u8 = null;\n        if (buf[pos] == 1) {{ pos += 1; {var} = v; }} else {{ pos += 1; }}\n")),
        }
    } else {
        match lang {
            SbeTargetLang::Go => out.push_str(&format!("\t{var} := buf[pos]\n\tpos++\n")),
            SbeTargetLang::Cpp => out.push_str(&format!("        uint8_t {var} = buf[pos++];\n")),
            SbeTargetLang::Csharp => {
                out.push_str(&format!("            byte {var} = buf[pos++];\n"))
            }
            SbeTargetLang::TypeScript => {
                out.push_str(&format!("  const {var} = buf[pos.value++];\n"))
            }
            SbeTargetLang::Zig => out.push_str(&format!(
                "        const {var} = buf[pos];\n        pos += 1;\n"
            )),
        }
    }
}

fn generate_target_message_encoder(lang: SbeTargetLang, msg: &Message, template_id: u16) -> String {
    let empty_structs = std::collections::HashSet::new();
    let empty_messages = std::collections::HashSet::new();
    let _ = generate_sbe_message_encoder(msg, template_id, &empty_structs, &empty_messages);
    let block_length = compute_block_length(&msg.fields);
    let _ = encoder_params(msg);
    let params = target_encoder_params(lang, msg);
    let encoder_name = format!("{}Encoder", msg.name);

    let mut body = String::new();
    body.push_str("        // SBE Message Header (8 bytes)\n");
    body.push_str(&target_write_header(lang, template_id, block_length));
    body.push_str("\n        // Fixed fields\n");
    for field in &msg.fields {
        body.push_str(&target_encode_field(lang, field, msg, "buf"));
    }

    match lang {
        SbeTargetLang::Go => format!(
            "// SBE encoder for {}\ntype {encoder_name} struct{{}}\n\nfunc ({encoder_name}) Encode({params}) ([]byte, error) {{\n\tbuf := make([]byte, 0, 256)\n{}\n\treturn buf, nil\n}}\n",
            msg.name,
            body.replace("        ", "\t")
        ),
        SbeTargetLang::Cpp => format!(
            "// SBE encoder for {}\nstruct {encoder_name} {{\n    static std::vector<uint8_t> encode({params}) {{\n        std::vector<uint8_t> buf;\n{}        return buf;\n    }}\n}};\n",
            msg.name, body
        ),
        SbeTargetLang::Csharp => format!(
            "    /// <summary>SBE encoder for {}</summary>\n    public static class {encoder_name}\n    {{\n        public static byte[] Encode({params})\n        {{\n            var buf = new List<byte>();\n{}            return buf.ToArray();\n        }}\n    }}\n",
            msg.name,
            body.replace("        ", "            ")
        ),
        SbeTargetLang::TypeScript => format!(
            "/** SBE encoder for {} */\nexport function {encoder_name}Encode({params}): Uint8Array {{\n  const buf: number[] = [];\n{}\n  return new Uint8Array(buf);\n}}\n",
            msg.name,
            body.replace("        ", "  ")
        ),
        SbeTargetLang::Zig => format!(
            "/// SBE encoder for {}\npub const {encoder_name} = struct {{\n    pub fn encode(allocator: std.mem.Allocator, {params}) ![]u8 {{\n        var buf = std.ArrayList(u8).init(allocator);\n        defer buf.deinit();\n{}\n        return buf.toOwnedSlice();\n    }}\n}};\n",
            msg.name, body
        ),
    }
}

fn generate_target_message_decoder(lang: SbeTargetLang, msg: &Message, template_id: u16) -> String {
    let empty_structs = std::collections::HashSet::new();
    let empty_messages = std::collections::HashSet::new();
    let _ = generate_sbe_message_decoder(msg, template_id, &empty_structs, &empty_messages);
    let decoder_name = format!("{}Decoder", msg.name);

    let mut fields_decl = String::new();
    for field in &msg.fields {
        let base = target_field_type(lang, &field.field_type, &msg.name, &field.name);
        let ty = if field.optional {
            target_optional_type(lang, &base)
        } else {
            base
        };
        match lang {
            SbeTargetLang::Go => fields_decl.push_str(&format!(
                "\t{} {}\n",
                target_param_name(lang, &field.name),
                ty
            )),
            SbeTargetLang::Cpp => fields_decl.push_str(&format!("    {ty} {};\n", field.name)),
            SbeTargetLang::Csharp => fields_decl.push_str(&format!(
                "        public {ty} {} {{ get; set; }}\n",
                target_param_name(lang, &field.name)
            )),
            SbeTargetLang::TypeScript => {
                fields_decl.push_str(&format!("  {}: {ty};\n", field.name))
            }
            SbeTargetLang::Zig => fields_decl.push_str(&format!("    {}: {ty},\n", field.name)),
        }
    }

    let mut decode_body = decode_header(lang, &decoder_name, template_id);
    for field in &msg.fields {
        decode_body.push_str(&target_decode_field(lang, field, msg, &decoder_name));
    }

    match lang {
        SbeTargetLang::Go => {
            let assigns = msg
                .fields
                .iter()
                .map(|f| format!("\t\t{}: {},", target_param_name(lang, &f.name), f.name))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "// SBE decoder for {}\ntype {decoder_name} struct {{\n{fields_decl}}}\n\nfunc ({decoder_name}) EncodedLen() int {{ return 0 }}\n\nfunc {decoder_name}Decode(buf []byte) ({decoder_name}, error) {{\n{decode_body}\n\treturn {decoder_name}{{\n{assigns}\n\t}}, nil\n}}\n",
                msg.name
            )
        }
        SbeTargetLang::Cpp => {
            let assigns = msg
                .fields
                .iter()
                .map(|f| format!("        out.{} = {};", f.name, f.name))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "// SBE decoder for {}\nstruct {decoder_name} {{\n{fields_decl}    size_t encoded_len() const {{ return 0; }}\n\n    static {decoder_name} decode(const std::vector<uint8_t>& buf) {{\n{decode_body}        {decoder_name} out{{}};\n{assigns}\n        return out;\n    }}\n}};\n",
                msg.name
            )
        }
        SbeTargetLang::Csharp => {
            let assigns = msg
                .fields
                .iter()
                .map(|f| {
                    format!(
                        "                {} = {},",
                        target_param_name(lang, &f.name),
                        f.name
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "    /// <summary>SBE decoder for {}</summary>\n    public class {decoder_name}\n    {{\n{fields_decl}        public int EncodedLen() => 0;\n\n        public static {decoder_name} Decode(byte[] buf)\n        {{\n{decode_body}            return new {decoder_name}\n            {{\n{assigns}\n            }};\n        }}\n    }}\n",
                msg.name
            )
        }
        SbeTargetLang::TypeScript => {
            let assigns = msg
                .fields
                .iter()
                .map(|f| format!("    {}: {},", f.name, f.name))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "/** SBE decoder for {} */\nexport interface {decoder_name} {{\n{fields_decl}  encodedLen(): number;\n}}\n\nexport function {decoder_name}Decode(buf: Uint8Array): {decoder_name} {{\n{decode_body}  return {{\n{assigns}\n    encodedLen: () => 0,\n  }};\n}}\n",
                msg.name
            )
        }
        SbeTargetLang::Zig => {
            let assigns = msg
                .fields
                .iter()
                .map(|f| format!("            .{} = {},", f.name, f.name))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "/// SBE decoder for {}\npub const {decoder_name} = struct {{\n{fields_decl}    pub fn encodedLen(self: @This()) usize {{ return 0; }}\n\n    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {{\n{decode_body}        return .{{\n{assigns}\n        }};\n    }}\n}};\n",
                msg.name
            )
        }
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
