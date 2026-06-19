//! SBE (Simple Binary Encoding) code generator from FSL AST.
//!
//! Generates Rust encoder/decoder structs for SBE wire format
//! from a parsed FSL schema.

use crate::ast::*;

/// Schema ID for well-known trading schemas
const SCHEMA_ID: u16 = 0x01;

/// Generate SBE Rust code from an FSL schema
pub fn generate_sbe(schema: &Schema) -> String {
    let mut out = String::new();
    let desc = schema
        .description
        .as_deref()
        .unwrap_or(&schema.name);

    out.push_str(&format!(
        "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' v{}\n",
        schema.name, schema.version
    ));
    out.push_str(&format!("// {}\n\n", desc));
    out.push_str("use std::convert::TryFrom;\n\n");

    // Collect all enum types used across messages and type defs
    let mut enum_types: Vec<(String, Vec<String>)> = Vec::new();
    let mut seen_enums = std::collections::HashSet::new();

    for msg in &schema.messages {
        for field in &msg.fields {
            if let FieldType::Enum(enum_def) = &field.field_type {
                let enum_name = format!("{}{}", msg.name, pascal_case(&field.name));
                if seen_enums.insert(enum_name.clone()) {
                    enum_types.push((enum_name, enum_def.variants.clone()));
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
            }
        }
    }

    // Collect inline struct types
    let mut inline_struct_types: Vec<(String, Vec<Field>)> = Vec::new();
    let mut seen_structs = std::collections::HashSet::new();

    for msg in &schema.messages {
        for field in &msg.fields {
            if let FieldType::InlineStruct(fields) = &field.field_type {
                let struct_name = format!("{}{}", msg.name, pascal_case(&field.name));
                if seen_structs.insert(struct_name.clone()) {
                    inline_struct_types.push((struct_name, fields.clone()));
                }
            }
        }
    }
    for td in &schema.type_defs {
        if let Some(td_fields) = &td.fields {
            for field in td_fields {
                if let FieldType::InlineStruct(fields) = &field.field_type {
                    let struct_name = format!("{}{}", td.name, pascal_case(&field.name));
                    if seen_structs.insert(struct_name.clone()) {
                        inline_struct_types.push((struct_name, fields.clone()));
                    }
                }
            }
        }
    }

    // Generate all enum types first
    for (name, variants) in &enum_types {
        out.push_str(&generate_sbe_enum(name, variants));
        out.push('\n');
    }

    // Generate all inline struct types
    for (name, fields) in &inline_struct_types {
        out.push_str(&generate_sbe_inline_struct(name, fields));
        out.push('\n');
    }

    // Generate encoder/decoder for each message
    for (template_id, msg) in schema.messages.iter().enumerate() {
        let tid = (template_id + 1) as u16;
        out.push_str(&generate_sbe_message_encoder(msg, tid, &enum_types));
        out.push('\n');
        out.push_str(&generate_sbe_message_decoder(msg, tid, &enum_types));
        out.push('\n');
    }

    out
}

/// Generate SBE enum type with explicit values
fn generate_sbe_enum(name: &str, variants: &[String]) -> String {
    let mut out = String::new();
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]\n");
    out.push_str(&format!("pub enum {} {{\n", name));
    for (i, v) in variants.iter().enumerate() {
        let pascal = pascal_case(v);
        out.push_str(&format!("    {} = {},\n", pascal, i + 1));
    }
    out.push_str("}\n\n");

    out.push_str(&format!("impl {} {{\n", name));
    out.push_str(&format!("    pub fn from_value(v: u8) -> Option<Self> {{\n"));
    out.push_str("        match v {\n");
    for (i, v) in variants.iter().enumerate() {
        let pascal = pascal_case(v);
        out.push_str(&format!("            {} => Some({}::{}),\n", i + 1, name, pascal));
    }
    out.push_str("            _ => None,\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    pub fn to_value(self) -> u8 {\n");
    out.push_str("        self as u8\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Generate SBE inline struct type
fn generate_sbe_inline_struct(name: &str, fields: &[Field]) -> String {
    let mut out = String::new();
    out.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    out.push_str(&format!("pub struct {} {{\n", name));
    for field in fields {
        let rust_type = sbe_field_type_name(&field.field_type);
        if field.optional {
            out.push_str(&format!("    pub {}: Option<{}>,\n", field.name, rust_type));
        } else {
            out.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
        }
    }
    out.push_str("}\n\n");

    let encoded_len: usize = fields
        .iter()
        .map(|f| sbe_field_encoded_size(&f.field_type))
        .sum();
    let sizes: Vec<String> = fields
        .iter()
        .map(|f| sbe_field_encoded_size(&f.field_type).to_string())
        .collect();

    out.push_str(&format!("impl {} {{\n", name));
    out.push_str(&format!(
        "    pub const ENCODED_LEN: usize = {}; // {}\n",
        encoded_len,
        sizes.join(" + ")
    ));
    out.push_str("}\n");
    out
}

/// Generate encoder struct for a message
fn generate_sbe_message_encoder(
    msg: &Message,
    template_id: u16,
    _enum_types: &[(String, Vec<String>)],
) -> String {
    let encoder_name = format!("{}Encoder", msg.name);
    let version: u16 = 0;

    let block_length = compute_block_length(&msg.fields);

    let mut out = String::new();
    out.push_str(&format!("/// SBE encoder for {}\n", msg.name));
    out.push_str(&format!("pub struct {};\n\n", encoder_name));
    out.push_str(&format!("impl {} {{\n", encoder_name));
    out.push_str("    /// Encode this message into a byte buffer.\n");
    out.push_str("    /// Returns the filled buffer.\n");
    out.push_str(&format!(
        "    pub fn encode({}) -> Vec<u8> {{\n",
        encoder_params(msg)
    ));
    out.push_str("        let mut buf = Vec::new();\n\n");
    out.push_str("        // SBE Message Header (8 bytes)\n");
    out.push_str(&format!(
        "        buf.extend_from_slice(&{}u16.to_be_bytes()); // schema_id\n",
        SCHEMA_ID
    ));
    out.push_str(&format!(
        "        buf.extend_from_slice(&{}u16.to_be_bytes()); // template_id\n",
        template_id
    ));
    out.push_str(&format!(
        "        buf.extend_from_slice(&{}u16.to_be_bytes()); // version\n",
        version
    ));
    out.push_str(&format!(
        "        buf.extend_from_slice(&{}u16.to_be_bytes()); // block_length\n\n",
        block_length
    ));

    // Write fixed fields and variable fields
    out.push_str("        // Fixed fields\n");
    for field in &msg.fields {
        out.push_str(&generate_encode_field(field, msg, "buf"));
    }

    out.push_str("\n        buf\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Generate the encoder function parameter list
fn encoder_params(msg: &Message) -> String {
    let mut params = Vec::new();
    for field in &msg.fields {
        let rust_type = sbe_field_type_name_for_message(&field.field_type, &msg.name, &field.name);
        let param_type = if field.optional {
            format!("{}: Option<{}>", &field.name, rust_type)
        } else {
            format!("{}: {}", &field.name, rust_type)
        };
        params.push(param_type);
    }
    params.join(", ")
}

/// Generate the decoder struct for a message
fn generate_sbe_message_decoder(
    msg: &Message,
    template_id: u16,
    _enum_types: &[(String, Vec<String>)],
) -> String {
    let decoder_name = format!("{}Decoder", msg.name);
    let mut out = String::new();

    // Decoder struct with all fields
    out.push_str(&format!("/// SBE decoder for {}\n", msg.name));
    out.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    out.push_str(&format!("pub struct {} {{\n", decoder_name));
    for field in &msg.fields {
        let rust_type = sbe_field_type_name_for_message(&field.field_type, &msg.name, &field.name);
        let field_type = if field.optional {
            format!("Option<{}>", rust_type)
        } else {
            rust_type
        };
        out.push_str(&format!("    pub {}: {},\n", field.name, field_type));
    }
    out.push_str("}\n\n");

    // Decode impl
    out.push_str(&format!("impl {} {{\n", decoder_name));
    out.push_str("    /// Decode from a byte buffer.\n");
    out.push_str("    pub fn decode(buf: &[u8]) -> Result<Self, String> {\n");
    out.push_str("        if buf.len() < 8 {\n");
    out.push_str("            return Err(\"buffer too short for SBE header\".to_string());\n");
    out.push_str("        }\n\n");

    // Read header
    out.push_str("        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);\n");
    out.push_str("        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);\n");
    out.push_str(&format!(
        "        // version = u16::from_be_bytes([buf[4], buf[5]]);\n"
    ));
    out.push_str(&format!(
        "        // block_length = u16::from_be_bytes([buf[6], buf[7]]);\n\n"
    ));

    out.push_str(&format!("        if schema_id != {} {{\n", SCHEMA_ID));
    out.push_str("            return Err(format!(\"invalid schema_id: {}\", schema_id));\n");
    out.push_str("        }\n");
    out.push_str(&format!(
        "        if tmpl_id != {} {{\n",
        template_id
    ));
    out.push_str("            return Err(format!(\"invalid template_id: {}\", tmpl_id));\n");
    out.push_str("        }\n\n");

    out.push_str("        let mut pos: usize = 8;\n\n");

    // Decode each field
    for field in &msg.fields {
        out.push_str(&generate_decode_field(field, msg));
    }

    // Build return value
    out.push_str("\n        Ok(Self {\n");
    for field in &msg.fields {
        out.push_str(&format!("            {},\n", &field.name));
    }
    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Generate decode code for a single field
fn generate_decode_field(field: &Field, msg: &Message) -> String {
    let mut out = String::new();
    let field_name = &field.name;
    let optional = field.optional;

    match &field.field_type {
        FieldType::Enum(_enum_def) => {
            let enum_name = format!("{}{}", msg.name, pascal_case(field_name));
            let varname = field_name.clone();
            out.push_str(&format!(
                "        let {}_raw = buf[pos];\n",
                varname
            ));
            out.push_str("        pos += 1;\n");
            out.push_str(&format!(
                "        let {} = {}::from_value({}_raw)\n",
                varname, enum_name, varname
            ));
            out.push_str(&format!(
                "            .ok_or_else(|| format!(\"invalid {} value: {{}}\", {}_raw))?;\n",
                enum_name, varname
            ));
        }
        ft @ (FieldType::Named(_) | FieldType::InlineBase(_, _)) => {
            let bt = if let FieldType::Named(n) = ft {
                sbe_named_type_to_wire_type(n)
            } else if let FieldType::InlineBase(bt, _) = ft {
                sbe_base_type_to_wire_type(bt)
            } else {
                "u8".to_string()
            };
            let _rust_type = sbe_field_type_name_for_message(&field.field_type, &msg.name, field_name);

            match bt.as_str() {
                "str" => {
                    // String: length-prefixed (u16 len + bytes)
                    out.push_str(&format!(
                        "        let {}_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;\n",
                        field_name
                    ));
                    out.push_str("        pos += 2;\n");
                    out.push_str(&format!(
                        "        let {}_bytes = &buf[pos..pos+{}_len];\n",
                        field_name, field_name
                    ));
                    out.push_str(&format!("        pos += {}_len;\n", field_name));
                    out.push_str(&format!(
                        "        let {} = std::str::from_utf8({}_bytes)\n",
                        field_name, field_name
                    ));
                    out.push_str("            .map_err(|e| format!(\"invalid UTF-8: {}\", e))?.to_string();\n");
                }
                "f64" => {
                    out.push_str(&format!(
                        "        let {}_raw = f64::from_be_bytes([\n",
                        field_name
                    ));
                    for i in 0..8 {
                        out.push_str(&format!(
                            "            buf[pos+{}],\n",
                            i
                        ));
                    }
                    out.push_str("        ]);\n");
                    out.push_str("        pos += 8;\n");
                    if optional {
                        out.push_str(&format!("        let {} = if buf[pos] == 1 {{ Some({}_raw) }} else {{ None }};\n", field_name, field_name));
                        out.push_str("        pos += 1;\n");
                    } else {
                        out.push_str(&format!("        let {} = {}_raw;\n", field_name, field_name));
                    }
                }
                "i64" => {
                    out.push_str(&format!(
                        "        let {} = i64::from_be_bytes([\n",
                        field_name
                    ));
                    for i in 0..8 {
                        out.push_str(&format!(
                            "            buf[pos+{}],\n",
                            i
                        ));
                    }
                    out.push_str("        ]);\n");
                    out.push_str("        pos += 8;\n");
                }
                "u8" => {
                    out.push_str(&format!(
                        "        let {} = buf[pos];\n",
                        field_name
                    ));
                    out.push_str("        pos += 1;\n");
                }
                _ => {
                    out.push_str(&format!(
                        "        let {} = buf[pos]; // TODO: decode {}\n",
                        field_name, bt
                    ));
                    out.push_str("        pos += 1;\n");
                }
            }
        }
        FieldType::List(inner) => {
            // Repeating group: read count, then N items
            let inner_type = sbe_field_type_name(inner);
            out.push_str(&format!(
                "        let {}_count = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize;\n",
                field_name
            ));
            out.push_str("        pos += 4;\n");
            out.push_str(&format!(
                "        let mut {} = Vec::with_capacity({}_count);\n",
                field_name, field_name
            ));

            if let FieldType::Named(_) = inner.as_ref() {
                // Named struct type
                let inner_decoder = format!("{}Decoder", inner_type);
                out.push_str(&format!(
                    "        for _ in 0..{}_count {{\n",
                    field_name
                ));
                out.push_str(&format!(
                    "            let item = {}::decode(&buf[pos..])?;\n",
                    inner_decoder
                ));
                out.push_str(&format!(
                    "            let item_len = item.encoded_len();\n",
                ));
                out.push_str("            pos += item_len;\n");
                out.push_str(&format!(
                    "            {}.push(item);\n",
                    field_name
                ));
                out.push_str("        }\n");
            } else {
                // Inline struct
                let inner_wire = sbe_field_type_name(inner);
                out.push_str(&format!(
                    "        for _ in 0..{}_count {{\n",
                    field_name
                ));
                out.push_str(&format!(
                    "            let item = {}Decoder::decode(&buf[pos..])?;\n",
                    inner_wire
                ));
                out.push_str("            pos += item.encoded_len();\n");
                out.push_str(&format!(
                    "            {}.push(item);\n",
                    field_name
                ));
                out.push_str("        }\n");
            }
        }
        FieldType::InlineStruct(_) => {
            let struct_name = format!("{}{}", msg.name, pascal_case(field_name));
            out.push_str(&format!(
                "        let {}_decoder = {}Decoder;\n",
                field_name, struct_name
            ));
            out.push_str(&format!(
                "        let {} = {}Decoder::decode(&buf[pos..])?;\n",
                field_name, struct_name
            ));
            out.push_str(&format!(
                "        pos += {}.encoded_len();\n",
                field_name
            ));
        }
    }

    // Handle optional presence flags for string fields
    if optional {
        match &field.field_type {
            ft @ (FieldType::Named(_) | FieldType::InlineBase(_, _)) => {
                let bt = if let FieldType::Named(n) = ft {
                    sbe_named_type_to_wire_type(n)
                } else if let FieldType::InlineBase(bt, _) = ft {
                    sbe_base_type_to_wire_type(bt)
                } else {
                    "".to_string()
                };
                if bt == "str" {
                    // Read presence flag and optional string
                    out.push_str(&format!(
                        "        let {} = if buf[pos] == 1 {{\n",
                        field_name
                    ));
                    out.push_str("            pos += 1;\n");
                    out.push_str(&format!(
                        "            let {}_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;\n",
                        field_name
                    ));
                    out.push_str("            pos += 2;\n");
                    out.push_str(&format!(
                        "            let {}_bytes = &buf[pos..pos+{}_len];\n",
                        field_name, field_name
                    ));
                    out.push_str(&format!("            pos += {}_len;\n", field_name));
                    out.push_str(&format!(
                        "            Some(std::str::from_utf8({}_bytes)\n",
                        field_name
                    ));
                    out.push_str("                .map_err(|e| format!(\"invalid UTF-8: {}\", e))?.to_string())\n");
                    out.push_str("        } else {\n");
                    out.push_str("            pos += 1;\n");
                    out.push_str("            None\n");
                    out.push_str("        };\n");
                }
            }
            _ => {}
        }
    }

    out
}

/// Generate encode code for a single field
fn generate_encode_field(field: &Field, msg: &Message, buf_var: &str) -> String {
    let mut out = String::new();
    let field_name = &field.name;
    let optional = field.optional;

    match &field.field_type {
        FieldType::Enum(_) => {
            if optional {
                out.push_str(&format!(
                    "        {}.push({}.map(|v| v as u8).unwrap_or(0));\n",
                    buf_var, field_name
                ));
            } else {
                out.push_str(&format!(
                    "        {}.push({} as u8);\n",
                    buf_var, field_name
                ));
            }
        }
        ft @ (FieldType::Named(_) | FieldType::InlineBase(_, _)) => {
            let bt = if let FieldType::Named(n) = ft {
                sbe_named_type_to_wire_type(n)
            } else if let FieldType::InlineBase(bt, _) = ft {
                sbe_base_type_to_wire_type(bt)
            } else {
                "u8".to_string()
            };

            match bt.as_str() {
                "str" => {
                    // String: u16 length prefix + UTF-8 bytes
                    if optional {
                        out.push_str(&format!(
                            "        {}.push(if {}.is_some() {{ 1 }} else {{ 0 }});\n",
                            buf_var, field_name
                        ));
                        out.push_str(&format!(
                            "        if let Some(ref s) = {} {{\n",
                            field_name
                        ));
                        out.push_str(&format!(
                            "            let b = s.as_bytes();\n",
                        ));
                        out.push_str(&format!(
                            "            {}.extend_from_slice(&(b.len() as u16).to_be_bytes());\n",
                            buf_var
                        ));
                        out.push_str(&format!(
                            "            {}.extend_from_slice(b);\n",
                            buf_var
                        ));
                        out.push_str("        }\n");
                    } else {
                        out.push_str(&format!(
                            "        let {}_bytes = {}.as_bytes();\n",
                            field_name, field_name
                        ));
                        out.push_str(&format!(
                            "        {}.extend_from_slice(&({}_bytes.len() as u16).to_be_bytes());\n",
                            buf_var, field_name
                        ));
                        out.push_str(&format!(
                            "        {}.extend_from_slice({}_bytes);\n",
                            buf_var, field_name
                        ));
                    }
                }
                "f64" => {
                    if optional {
                        out.push_str(&format!(
                            "        {}.extend_from_slice(&{}.unwrap_or(0.0).to_be_bytes());\n",
                            buf_var, field_name
                        ));
                        out.push_str(&format!(
                            "        {}.push(if {}.is_some() {{ 1 }} else {{ 0 }});\n",
                            buf_var, field_name
                        ));
                    } else {
                        out.push_str(&format!(
                            "        {}.extend_from_slice(&{}.to_be_bytes());\n",
                            buf_var, field_name
                        ));
                    }
                }
                "i64" => {
                    if optional {
                        out.push_str(&format!(
                            "        {}.push(if {}.is_some() {{ 1 }} else {{ 0 }});\n",
                            buf_var, field_name
                        ));
                        out.push_str(&format!(
                            "        if let Some(v) = {} {{\n",
                            field_name
                        ));
                        out.push_str(&format!(
                            "            {}.extend_from_slice(&v.to_be_bytes());\n",
                            buf_var
                        ));
                        out.push_str("        }\n");
                    } else {
                        out.push_str(&format!(
                            "        {}.extend_from_slice(&{}.to_be_bytes());\n",
                            buf_var, field_name
                        ));
                    }
                }
                "u8" => {
                    if optional {
                        out.push_str(&format!(
                            "        {}.push({}.unwrap_or(0));\n",
                            buf_var, field_name
                        ));
                        out.push_str(&format!(
                            "        {}.push(if {}.is_some() {{ 1 }} else {{ 0 }});\n",
                            buf_var, field_name
                        ));
                    } else {
                        out.push_str(&format!(
                            "        {}.push({});\n",
                            buf_var, field_name
                        ));
                    }
                }
                _ => {
                    out.push_str(&format!(
                        "        // TODO: encode {} as {}\n",
                        field_name, bt
                    ));
                }
            }
        }
        FieldType::List(inner) => {
            // Repeating group: u32 count + items
            let inner_type = sbe_field_type_name(inner);
            out.push_str(&format!(
                "        {}.extend_from_slice(&({}.len() as u32).to_be_bytes());\n",
                buf_var, field_name
            ));
            out.push_str(&format!(
                "        for item in &{} {{\n",
                field_name
            ));
            if let FieldType::Named(_) = inner.as_ref() {
                out.push_str(&format!(
                    "            {}Encoder::encode(item, {});\n",
                    inner_type, buf_var
                ));
            } else {
                out.push_str(&format!(
                    "            {}Encoder::encode(item.clone(), {});  // TODO\n",
                    inner_type, buf_var
                ));
            }
            out.push_str("        }\n");
        }
        FieldType::InlineStruct(_) => {
            let struct_name = format!("{}{}", msg.name, pascal_case(field_name));
            out.push_str(&format!(
                "        {}Encoder::encode({}, {});\n",
                struct_name, field_name, buf_var
            ));
        }
    }

    out
}

// ── SBE type mapping helpers ──────────────────────────────────

/// Get the Rust type name for a field in SBE context
fn sbe_field_type_name(ft: &FieldType) -> String {
    match ft {
        FieldType::Named(name) => name.clone(),
        FieldType::Enum(_) => "u8".to_string(),
        FieldType::List(inner) => {
            format!("Vec<{}>", sbe_field_type_name(inner))
        }
        FieldType::InlineStruct(_) => "()".to_string(), // resolved in context
        FieldType::InlineBase(bt, _) => sbe_base_type_to_rust(bt),
    }
}

/// Get the Rust type name for a field in a specific message context
fn sbe_field_type_name_for_message(ft: &FieldType, msg_name: &str, field_name: &str) -> String {
    match ft {
        FieldType::Named(name) => name.clone(),
        FieldType::Enum(_) => format!("{}{}", msg_name, pascal_case(field_name)),
        FieldType::List(inner) => {
            let inner_name = sbe_field_type_name_for_message_inner(inner, "");
            format!("Vec<{}>", inner_name)
        }
        FieldType::InlineStruct(_) => format!("{}{}", msg_name, pascal_case(field_name)),
        FieldType::InlineBase(bt, _) => sbe_base_type_to_rust(bt),
    }
}

fn sbe_field_type_name_for_message_inner(ft: &FieldType, _msg_name: &str) -> String {
    match ft {
        FieldType::Named(name) => name.clone(),
        FieldType::InlineStruct(_) => "()".to_string(),
        _ => sbe_field_type_name(ft),
    }
}

/// Map a named FSL type to its SBE wire type
fn sbe_named_type_to_wire_type(name: &str) -> String {
    match name {
        "ClientOrderId" | "Symbol" | "String" => "str".to_string(),
        "Price" | "Quantity" => "f64".to_string(),
        "TradeTimestamp" => "i64".to_string(),
        "i64" => "i64".to_string(),
        "u64" => "u64".to_string(),
        "i32" => "i32".to_string(),
        "u32" => "u32".to_string(),
        "i16" => "i16".to_string(),
        "u16" => "u16".to_string(),
        "f64" => "f64".to_string(),
        "f32" => "f32".to_string(),
        "bool" => "u8".to_string(),
        _ => "str".to_string(), // default to string for unknown named types
    }
}

/// Map a base type to its SBE wire type
fn sbe_base_type_to_wire_type(bt: &BaseType) -> String {
    match bt {
        BaseType::String => "str".to_string(),
        BaseType::Int8 | BaseType::UInt8 | BaseType::Bool => "u8".to_string(),
        BaseType::Int16 | BaseType::UInt16 => "u16".to_string(),
        BaseType::Int32 | BaseType::UInt32 | BaseType::Float32 => "u32".to_string(),
        BaseType::Int64 | BaseType::UInt64 | BaseType::Float64 | BaseType::Decimal64 => "i64".to_string(),
        BaseType::Bytes => "bytes".to_string(),
        BaseType::List(_) => "list".to_string(),
    }
}

fn sbe_base_type_to_rust(bt: &BaseType) -> String {
    match bt {
        BaseType::String => "String".to_string(),
        BaseType::Int8 => "i8".to_string(),
        BaseType::Int16 => "i16".to_string(),
        BaseType::Int32 => "i32".to_string(),
        BaseType::Int64 => "i64".to_string(),
        BaseType::UInt8 => "u8".to_string(),
        BaseType::UInt16 => "u16".to_string(),
        BaseType::UInt32 => "u32".to_string(),
        BaseType::UInt64 => "u64".to_string(),
        BaseType::Float32 => "f32".to_string(),
        BaseType::Float64 => "f64".to_string(),
        BaseType::Decimal64 => "f64".to_string(),
        BaseType::Bool => "bool".to_string(),
        BaseType::Bytes => "Vec<u8>".to_string(),
        BaseType::List(inner) => format!("Vec<{}>", sbe_base_type_to_rust(inner)),
    }
}

/// Compute the fixed block length for a message
fn compute_block_length(fields: &[Field]) -> u16 {
    let mut total: usize = 0;
    for field in fields {
        total += sbe_field_encoded_size(&field.field_type);
    }
    total as u16
}

/// Return the SBE encoded size of a field type
fn sbe_field_encoded_size(ft: &FieldType) -> usize {
    match ft {
        FieldType::Named(name) => match name.as_str() {
            "String" | "Vec<u8>" | "ClientOrderId" | "Symbol" => 0, // variable-length
            "Price" | "Quantity" => 8,
            "TradeTimestamp" | "i64" | "u64" => 8,
            "i32" | "u32" | "f32" => 4,
            "i16" | "u16" => 2,
            "i8" | "u8" | "bool" => 1,
            "f64" => 8,
            _ => 0,
        },
        FieldType::Enum(_) => 1,
        FieldType::List(_) => 0,
        FieldType::InlineStruct(fields) => {
            fields.iter().map(|f| sbe_field_encoded_size(&f.field_type)).sum()
        }
        FieldType::InlineBase(bt, _) => match bt {
            BaseType::String | BaseType::Bytes => 0,
            BaseType::Int8 | BaseType::UInt8 | BaseType::Bool => 1,
            BaseType::Int16 | BaseType::UInt16 => 2,
            BaseType::Int32 | BaseType::UInt32 | BaseType::Float32 => 4,
            BaseType::Int64 | BaseType::UInt64 | BaseType::Float64 | BaseType::Decimal64 => 8,
            BaseType::List(_) => 0,
        },
    }
}

/// Convert a snake_case identifier to PascalCase
fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_generate_sbe_simple() {
        let input = r#"
            schema test v1.0 {
                description: "Test schema"
                well_known_id: 0x01
                type Name: string(max_len: 32)
                message Greeting {
                    channel_type: request_response
                    name: Name @1
                    language: enum { en, fr, de } @2
                }
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        let code = generate_sbe(&schema);

        assert!(code.contains("pub enum GreetingLanguage"));
        assert!(code.contains("GreetingEncoder"));
        assert!(code.contains("GreetingDecoder"));
        assert!(code.contains("from_value(v: u8)"));
        assert!(code.contains("to_value(self) -> u8"));
    }

    #[test]
    fn test_generate_sbe_with_optional() {
        let input = r#"
            schema x v1 {
                message M {
                    channel_type: stream_item
                    a: string(max_len: 10) @1
                    b: string(max_len: 10) @2 optional
                }
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        let code = generate_sbe(&schema);

        assert!(code.contains("MEncoder"));
        assert!(code.contains("MDecoder"));
        assert!(code.contains(".is_some()"));
    }

    #[test]
    fn test_block_length_computation() {
        let fields = vec![
            Field {
                name: "x".to_string(),
                field_type: FieldType::Named("u32".to_string()),
                number: Some(1),
                optional: false,
            },
            Field {
                name: "y".to_string(),
                field_type: FieldType::Enum(EnumDef {
                    variants: vec!["a".to_string(), "b".to_string()],
                }),
                number: Some(2),
                optional: false,
            },
        ];
        // u32 = 4 bytes + enum = 1 byte = 5 total
        assert_eq!(compute_block_length(&fields), 5);
    }
}
