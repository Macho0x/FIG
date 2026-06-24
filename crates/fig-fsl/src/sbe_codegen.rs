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
    let desc = schema.description.as_deref().unwrap_or(&schema.name);

    out.push_str(&format!(
        "// Auto-generated SBE encode/decode by fig-fsl from schema '{}' v{}\n",
        schema.name, schema.version
    ));
    out.push_str(&format!("// {}\n\n", desc));

    // Collect all enum types used across messages and type defs
    let mut enum_types: Vec<(String, Vec<String>)> = Vec::new();
    let mut seen_enums = std::collections::HashSet::new();
    let mut imported_message_enums = std::collections::BTreeSet::new();

    for msg in &schema.messages {
        for field in &msg.fields {
            if let FieldType::Enum(enum_def) = &field.field_type {
                let enum_name = format!("{}{}", msg.name, pascal_case(&field.name));
                let canonical = canonical_enum_name(&field.name, &msg.name);
                if enum_name == canonical {
                    imported_message_enums.insert(canonical);
                    continue;
                }
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
                    let canonical = canonical_enum_name(&field.name, &td.name);
                    if is_shared_messages_enum(&canonical) {
                        imported_message_enums.insert(canonical);
                        continue;
                    }
                    let enum_name = format!("{}{}", td.name, pascal_case(&field.name));
                    if seen_enums.insert(enum_name.clone()) {
                        enum_types.push((enum_name, enum_def.variants.clone()));
                    }
                }
            }
        }
    }

    let generated_enum_names: std::collections::HashSet<String> =
        enum_types.iter().map(|(n, _)| n.clone()).collect();
    out.push_str(&generate_sbe_imports(
        schema,
        &generated_enum_names,
        &imported_message_enums,
    ));

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

    // FSL `type Foo { ... }` struct definitions (CandleBar, PriceLevel, …)
    let struct_type_defs = collect_struct_type_defs(schema);
    let struct_names: std::collections::HashSet<String> =
        struct_type_defs.iter().map(|(n, _)| n.clone()).collect();
    let message_names: std::collections::HashSet<String> =
        schema.messages.iter().map(|m| m.name.clone()).collect();

    for (name, fields) in &struct_type_defs {
        out.push_str(&generate_sbe_struct_encoder(
            name,
            fields,
            &struct_names,
            &message_names,
        ));
        out.push('\n');
        out.push_str(&generate_sbe_struct_decoder(
            name,
            fields,
            &struct_names,
            &message_names,
        ));
        out.push('\n');
    }

    // Body codecs for messages nested as field types (e.g. MarginSummary in MarginUpdate)
    for msg in &schema.messages {
        out.push_str(&generate_sbe_message_body_encoder(
            msg,
            &struct_names,
            &message_names,
        ));
        out.push('\n');
        out.push_str(&generate_sbe_message_body_decoder(
            msg,
            &struct_names,
            &message_names,
        ));
        out.push('\n');
    }

    // Generate encoder/decoder for each message
    for (template_id, msg) in schema.messages.iter().enumerate() {
        let tid = (template_id + 1) as u16;
        out.push_str(&generate_sbe_message_encoder(
            msg,
            tid,
            &struct_names,
            &message_names,
        ));
        out.push('\n');
        out.push_str(&generate_sbe_message_decoder(
            msg,
            tid,
            &struct_names,
            &message_names,
        ));
        out.push('\n');
    }

    out
}

fn collect_struct_type_defs(schema: &Schema) -> Vec<(String, Vec<Field>)> {
    schema
        .type_defs
        .iter()
        .filter_map(|td| {
            td.fields
                .as_ref()
                .filter(|f| !f.is_empty())
                .map(|fields| (td.name.clone(), fields.clone()))
        })
        .collect()
}

fn generate_sbe_imports(
    schema: &Schema,
    generated_enum_names: &std::collections::HashSet<String>,
    imported_message_enums: &std::collections::BTreeSet<String>,
) -> String {
    let mut types = std::collections::BTreeSet::new();
    for msg in &schema.messages {
        types.insert(msg.name.clone());
    }
    for td in &schema.type_defs {
        types.insert(td.name.clone());
    }
    for msg in &schema.messages {
        for field in &msg.fields {
            collect_import_types(&field.field_type, &mut types);
        }
    }
    for (_, fields) in collect_struct_type_defs(schema) {
        for field in &fields {
            collect_import_types(&field.field_type, &mut types);
        }
    }
    let list: Vec<String> = types.into_iter().collect();
    let shared = [
        "Side",
        "OrderType",
        "TimeInForce",
        "SecurityIdSource",
        "ExecType",
        "OrdStatus",
        "CancelRejectReason",
        "MarketDataAction",
        "CapabilityPathPattern",
        "OrderListStatusStatus",
        "BalanceUpdateReason",
    ];
    let mut imports: Vec<String> = list;
    for name in imported_message_enums {
        if !imports.iter().any(|t| t == name) {
            imports.push(name.clone());
        }
    }
    for name in shared {
        if generated_enum_names.contains(name) {
            continue;
        }
        if !imports.iter().any(|t| t == name) {
            imports.push(name.to_string());
        }
    }
    imports.sort();
    format!("use crate::messages::{{{}}};\n\n", imports.join(", "))
}

fn collect_import_types(ft: &FieldType, out: &mut std::collections::BTreeSet<String>) {
    match ft {
        FieldType::Named(name) => {
            out.insert(name.clone());
        }
        FieldType::List(inner) => collect_import_types(inner, out),
        FieldType::InlineStruct(fields) => {
            for f in fields {
                collect_import_types(&f.field_type, out);
            }
        }
        _ => {}
    }
}

/// Generate SBE enum type with explicit values
pub(crate) fn generate_sbe_enum(name: &str, variants: &[String]) -> String {
    let mut out = String::new();
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]\n");
    out.push_str(&format!("pub enum {} {{\n", name));
    for (i, v) in variants.iter().enumerate() {
        let pascal = pascal_case(v);
        out.push_str(&format!("    {} = {},\n", pascal, i + 1));
    }
    out.push_str("}\n\n");

    out.push_str(&format!("impl {} {{\n", name));
    out.push_str("    pub fn from_value(v: u8) -> Option<Self> {\n");
    out.push_str("        match v {\n");
    for (i, v) in variants.iter().enumerate() {
        let pascal = pascal_case(v);
        out.push_str(&format!(
            "            {} => Some({}::{}),\n",
            i + 1,
            name,
            pascal
        ));
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
pub(crate) fn generate_sbe_inline_struct(name: &str, fields: &[Field]) -> String {
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

/// Body-only SBE encoder for an FSL struct type (no message header).
pub(crate) fn generate_sbe_struct_encoder(
    name: &str,
    fields: &[Field],
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
) -> String {
    let encoder_name = format!("{name}Encoder");
    let mut out = String::new();
    out.push_str(&format!("/// SBE body encoder for {name}\n"));
    out.push_str(&format!("pub struct {encoder_name};\n\n"));
    out.push_str(&format!("impl {encoder_name} {{\n"));
    out.push_str(&format!(
        "    pub fn encode(value: &{name}, buf: &mut Vec<u8>) {{\n"
    ));
    for field in fields {
        out.push_str(&generate_encode_field(
            field,
            name,
            struct_names,
            message_names,
            SbeEnumStyle::MessagesShared,
            "buf",
            "value.",
            true,
        ));
    }
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Body-only SBE decoder for an FSL struct type (no message header).
pub(crate) fn generate_sbe_struct_decoder(
    name: &str,
    fields: &[Field],
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
) -> String {
    let decoder_name = format!("{name}Decoder");
    let mut out = String::new();
    out.push_str(&format!("/// SBE body decoder for {name}\n"));
    out.push_str(&format!("pub struct {decoder_name};\n\n"));
    out.push_str(&format!("impl {decoder_name} {{\n"));
    out.push_str(&format!(
        "    pub fn decode(buf: &[u8]) -> Result<({name}, usize), String> {{\n"
    ));
    out.push_str("        let mut pos: usize = 0;\n\n");
    for field in fields {
        out.push_str(&generate_decode_field(
            field,
            name,
            struct_names,
            message_names,
            SbeEnumStyle::MessagesShared,
        ));
    }
    out.push_str(&format!("\n        Ok(({name} {{\n"));
    for field in fields {
        out.push_str(&format!("            {},\n", field.name));
    }
    out.push_str("        }, pos))\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Body-only encoder for a message type used as a nested field (no SBE header).
pub(crate) fn generate_sbe_message_body_encoder(
    msg: &Message,
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
) -> String {
    let encoder_name = format!("{}BodyEncoder", msg.name);
    let mut out = String::new();
    out.push_str(&format!("/// SBE body encoder for nested {}\n", msg.name));
    out.push_str(&format!("pub struct {encoder_name};\n\n"));
    out.push_str(&format!("impl {encoder_name} {{\n"));
    out.push_str(&format!(
        "    pub fn encode(value: &{}, buf: &mut Vec<u8>) {{\n",
        msg.name
    ));
    for field in &msg.fields {
        out.push_str(&generate_encode_field(
            field,
            &msg.name,
            struct_names,
            message_names,
            SbeEnumStyle::MessagesShared,
            "buf",
            "value.",
            true,
        ));
    }
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Body-only decoder for a message type used as a nested field (no SBE header).
pub(crate) fn generate_sbe_message_body_decoder(
    msg: &Message,
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
) -> String {
    let decoder_name = format!("{}BodyDecoder", msg.name);
    let mut out = String::new();
    out.push_str(&format!("/// SBE body decoder for nested {}\n", msg.name));
    out.push_str(&format!("pub struct {decoder_name};\n\n"));
    out.push_str(&format!("impl {decoder_name} {{\n"));
    out.push_str(&format!(
        "    pub fn decode(buf: &[u8]) -> Result<({}, usize), String> {{\n",
        msg.name
    ));
    out.push_str("        let mut pos: usize = 0;\n\n");
    for field in &msg.fields {
        out.push_str(&generate_decode_field(
            field,
            &msg.name,
            struct_names,
            message_names,
            SbeEnumStyle::MessagesShared,
        ));
    }
    out.push_str(&format!("\n        Ok(({} {{\n", msg.name));
    for field in &msg.fields {
        out.push_str(&format!(
            "            {}\n",
            generate_message_field_init(field, &msg.name)
        ));
    }
    out.push_str("        }, pos))\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

fn generate_message_field_init(field: &Field, _msg_name: &str) -> String {
    format!("{},", field.name)
}

#[derive(Clone, Copy)]
pub(crate) enum SbeEnumStyle {
    WirePerContext,
    MessagesShared,
}

fn is_struct_type(name: &str, struct_names: &std::collections::HashSet<String>) -> bool {
    struct_names.contains(name)
}

fn is_message_type(name: &str, message_names: &std::collections::HashSet<String>) -> bool {
    message_names.contains(name)
}

fn canonical_enum_name(field_name: &str, parent: &str) -> String {
    match field_name {
        "side" => "Side".to_string(),
        "order_type" => "OrderType".to_string(),
        "time_in_force" => "TimeInForce".to_string(),
        "id_source" => "SecurityIdSource".to_string(),
        "exec_type" => "ExecType".to_string(),
        "ord_status" => "OrdStatus".to_string(),
        "reject_reason" => "CancelRejectReason".to_string(),
        "action" if parent.contains("MarketData") => "MarketDataAction".to_string(),
        "pattern" => "CapabilityPathPattern".to_string(),
        other => format!("{parent}{}", pascal_case(other)),
    }
}

fn is_shared_messages_enum(name: &str) -> bool {
    matches!(
        name,
        "Side"
            | "OrderType"
            | "TimeInForce"
            | "SecurityIdSource"
            | "ExecType"
            | "OrdStatus"
            | "CancelRejectReason"
            | "MarketDataAction"
            | "CapabilityPathPattern"
            | "OrderListStatusStatus"
            | "BalanceUpdateReason"
    )
}

fn wrap_f64_value(field_name: &str, ft: &FieldType, raw: &str) -> String {
    match ft {
        FieldType::Named(n) if n == "Price" => format!("let {field_name} = Price({raw});"),
        FieldType::Named(n) if n == "Quantity" => format!("let {field_name} = Quantity({raw});"),
        _ => format!("let {field_name} = {raw};"),
    }
}

fn f64_wire_expr(ref_name: &str, ft: &FieldType, optional: bool) -> String {
    let access = match ft {
        FieldType::Named(n) if n == "Price" || n == "Quantity" => {
            if optional {
                return format!("{ref_name}.as_ref().map(|v| v.0).unwrap_or(0.0)");
            }
            format!("{ref_name}.0")
        }
        _ => {
            if optional {
                return format!("{ref_name}.unwrap_or(0.0)");
            }
            ref_name.to_string()
        }
    };
    access
}

pub(crate) fn generate_sbe_message_encoder(
    msg: &Message,
    template_id: u16,
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
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
        out.push_str(&generate_encode_field(
            field,
            &msg.name,
            struct_names,
            message_names,
            SbeEnumStyle::WirePerContext,
            "buf",
            "",
            false,
        ));
    }

    out.push_str("\n        buf\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Generate the encoder function parameter list
pub(crate) fn encoder_params(msg: &Message) -> String {
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
pub(crate) fn generate_sbe_message_decoder(
    msg: &Message,
    template_id: u16,
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
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
    out.push_str("        // version = u16::from_be_bytes([buf[4], buf[5]]);\n");
    out.push_str("        // block_length = u16::from_be_bytes([buf[6], buf[7]]);\n\n");

    out.push_str(&format!("        if schema_id != {} {{\n", SCHEMA_ID));
    out.push_str("            return Err(format!(\"invalid schema_id: {}\", schema_id));\n");
    out.push_str("        }\n");
    out.push_str(&format!("        if tmpl_id != {} {{\n", template_id));
    out.push_str("            return Err(format!(\"invalid template_id: {}\", tmpl_id));\n");
    out.push_str("        }\n\n");

    out.push_str("        let mut pos: usize = 8;\n\n");

    // Decode each field
    for field in &msg.fields {
        out.push_str(&generate_decode_field(
            field,
            &msg.name,
            struct_names,
            message_names,
            SbeEnumStyle::WirePerContext,
        ));
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
pub(crate) fn generate_decode_field(
    field: &Field,
    context: &str,
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
    enum_style: SbeEnumStyle,
) -> String {
    let mut out = String::new();
    let field_name = &field.name;
    let optional = field.optional;

    match &field.field_type {
        FieldType::Enum(_) => {
            let canonical = canonical_enum_name(field_name, context);
            let wire_enum = format!("{context}{}", pascal_case(field_name));
            let use_shared = matches!(enum_style, SbeEnumStyle::MessagesShared)
                && is_shared_messages_enum(&canonical);
            let use_messages_enum = is_shared_messages_enum(&canonical) && wire_enum == canonical;
            let type_name = if use_shared || use_messages_enum {
                canonical
            } else {
                wire_enum
            };
            let varname = field_name.clone();
            out.push_str(&format!("        let {varname}_raw = buf[pos];\n"));
            out.push_str("        pos += 1;\n");
            if optional {
                out.push_str(&format!("        let {varname} = if buf[pos] == 1 {{\n"));
                out.push_str("            pos += 1;\n");
                out.push_str(&format!(
                    "            Some({type_name}::from_value({varname}_raw).ok_or_else(|| format!(\"invalid {type_name} value: {{}}\", {varname}_raw))?)\n"
                ));
                out.push_str("        } else {\n");
                out.push_str("            pos += 1;\n");
                out.push_str("            None\n");
                out.push_str("        };\n");
            } else {
                out.push_str(&format!(
                    "        let {varname} = {type_name}::from_value({varname}_raw)\n"
                ));
                out.push_str(&format!(
                    "            .ok_or_else(|| format!(\"invalid {type_name} value: {{}}\", {varname}_raw))?;\n"
                ));
            }
        }
        FieldType::Named(n) if is_struct_type(n, struct_names) => {
            out.push_str(&format!(
                "        let ({field_name}, {field_name}_n) = {n}Decoder::decode(&buf[pos..])?;\n"
            ));
            out.push_str(&format!("        pos += {field_name}_n;\n"));
        }
        FieldType::Named(n) if is_message_type(n, message_names) => {
            out.push_str(&format!(
                "        let ({field_name}, {field_name}_n) = {n}BodyDecoder::decode(&buf[pos..])?;\n"
            ));
            out.push_str(&format!("        pos += {field_name}_n;\n"));
        }
        ft @ (FieldType::Named(_) | FieldType::InlineBase(_, _)) => {
            let bt = if let FieldType::Named(n) = ft {
                sbe_named_type_to_wire_type(n)
            } else if let FieldType::InlineBase(bt, _) = ft {
                sbe_base_type_to_wire_type(bt)
            } else {
                "u8".to_string()
            };
            let _rust_type =
                sbe_field_type_name_for_message(&field.field_type, context, field_name);

            match bt.as_str() {
                "str" => {
                    if !optional {
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
                }
                "f64" => {
                    out.push_str(&format!(
                        "        let {}_raw = f64::from_be_bytes([\n",
                        field_name
                    ));
                    for i in 0..8 {
                        out.push_str(&format!("            buf[pos+{}],\n", i));
                    }
                    out.push_str("        ]);\n");
                    out.push_str("        pos += 8;\n");
                    if optional {
                        out.push_str(&format!("        let {field_name} = if buf[pos] == 1 {{\n"));
                        out.push_str("            pos += 1;\n");
                        match &field.field_type {
                            FieldType::Named(n) if n == "Price" => {
                                out.push_str(&format!(
                                    "            Some(Price({field_name}_raw))\n"
                                ));
                            }
                            FieldType::Named(n) if n == "Quantity" => {
                                out.push_str(&format!(
                                    "            Some(Quantity({field_name}_raw))\n"
                                ));
                            }
                            _ => {
                                out.push_str(&format!("            Some({field_name}_raw)\n"));
                            }
                        }
                        out.push_str("        } else {\n");
                        out.push_str("            pos += 1;\n");
                        out.push_str("            None\n");
                        out.push_str("        };\n");
                    } else {
                        out.push_str("        ");
                        out.push_str(&wrap_f64_value(
                            field_name,
                            &field.field_type,
                            &format!("{field_name}_raw"),
                        ));
                        out.push('\n');
                    }
                }
                "i64" => {
                    out.push_str(&format!(
                        "        let {}_raw = i64::from_be_bytes([\n",
                        field_name
                    ));
                    for i in 0..8 {
                        out.push_str(&format!("            buf[pos+{}],\n", i));
                    }
                    out.push_str("        ]);\n");
                    out.push_str("        pos += 8;\n");
                    if optional {
                        out.push_str(&format!(
                            "        let {field_name} = if buf[pos] == 1 {{\n            pos += 1;\n            Some({field_name}_raw)\n        }} else {{\n            pos += 1;\n            None\n        }};\n"
                        ));
                    } else {
                        out.push_str(&format!("        let {field_name} = {field_name}_raw;\n"));
                    }
                }
                "u32" => {
                    out.push_str(&format!(
                        "        let {}_raw = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]);\n",
                        field_name
                    ));
                    out.push_str("        pos += 4;\n");
                    if optional {
                        out.push_str(&format!(
                            "        let {field_name} = if buf[pos] == 1 {{\n            pos += 1;\n            Some({field_name}_raw)\n        }} else {{\n            pos += 1;\n            None\n        }};\n"
                        ));
                    } else {
                        out.push_str(&format!("        let {field_name} = {field_name}_raw;\n"));
                    }
                }
                "u64" => {
                    out.push_str(&format!(
                        "        let {}_raw = u64::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3], buf[pos+4], buf[pos+5], buf[pos+6], buf[pos+7]]);\n",
                        field_name
                    ));
                    out.push_str("        pos += 8;\n");
                    if optional {
                        out.push_str(&format!(
                            "        let {field_name} = if buf[pos] == 1 {{\n            pos += 1;\n            Some({field_name}_raw)\n        }} else {{\n            pos += 1;\n            None\n        }};\n"
                        ));
                    } else {
                        out.push_str(&format!("        let {field_name} = {field_name}_raw;\n"));
                    }
                }
                "u8" => {
                    if optional {
                        out.push_str(&format!(
                            "        let {field_name}_val = buf[pos];\n        pos += 1;\n"
                        ));
                        out.push_str(&format!(
                            "        let {field_name} = if buf[pos] == 1 {{\n            pos += 1;\n            Some({field_name}_val != 0)\n        }} else {{\n            pos += 1;\n            None\n        }};\n"
                        ));
                    } else if matches!(&field.field_type, FieldType::InlineBase(BaseType::Bool, _))
                    {
                        out.push_str(&format!(
                            "        let {field_name} = buf[pos] != 0;\n        pos += 1;\n"
                        ));
                    } else {
                        out.push_str(&format!("        let {field_name} = buf[pos];\n"));
                        out.push_str("        pos += 1;\n");
                    }
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

            if let FieldType::Named(n) = inner.as_ref() {
                if is_struct_type(n, struct_names) {
                    out.push_str(&format!("        for _ in 0..{field_name}_count {{\n"));
                    out.push_str("            let item_len = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize;\n");
                    out.push_str("            pos += 4;\n");
                    out.push_str(&format!(
                        "            let (item, n) = {n}Decoder::decode(&buf[pos..pos+item_len])?;\n"
                    ));
                    out.push_str("            if n != item_len {\n");
                    out.push_str(&format!(
                        "                return Err(format!(\"{n} length mismatch: expected {{item_len}}, got {{n}}\"));\n"
                    ));
                    out.push_str("            }\n");
                    out.push_str("            pos += item_len;\n");
                    out.push_str(&format!("            {field_name}.push(item);\n"));
                    out.push_str("        }\n");
                } else if is_message_type(n, message_names) {
                    out.push_str(&format!("        for _ in 0..{field_name}_count {{\n"));
                    out.push_str("            let item_len = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize;\n");
                    out.push_str("            pos += 4;\n");
                    out.push_str(&format!(
                        "            let (item, n) = {n}BodyDecoder::decode(&buf[pos..pos+item_len])?;\n"
                    ));
                    out.push_str("            if n != item_len {\n");
                    out.push_str(&format!(
                        "                return Err(format!(\"{n} length mismatch: expected {{item_len}}, got {{n}}\"));\n"
                    ));
                    out.push_str("            }\n");
                    out.push_str("            pos += item_len;\n");
                    out.push_str(&format!("            {field_name}.push(item);\n"));
                    out.push_str("        }\n");
                } else {
                    out.push_str(&format!("        // TODO: decode list<{}>\n", inner_type));
                }
            } else if let FieldType::InlineStruct(_) = inner.as_ref() {
                let inline_name = format!("{context}{}", pascal_case(field_name));
                out.push_str(&format!("        for _ in 0..{field_name}_count {{\n"));
                out.push_str(&format!(
                    "            let (item, item_n) = {inline_name}Decoder::decode(&buf[pos..])?;\n"
                ));
                out.push_str("            pos += item_n;\n");
                out.push_str(&format!("            {field_name}.push(item);\n"));
                out.push_str("        }\n");
            } else {
                out.push_str(&format!("        // TODO: decode list<{}>\n", inner_type));
            }
        }
        FieldType::InlineStruct(_) => {
            let struct_name = format!("{context}{}", pascal_case(field_name));
            out.push_str(&format!(
                "        let ({field_name}, {field_name}_n) = {struct_name}Decoder::decode(&buf[pos..])?;\n"
            ));
            out.push_str(&format!("        pos += {field_name}_n;\n"));
        }
    }

    // Handle optional presence flags for string fields
    if optional {
        if let ft @ (FieldType::Named(_) | FieldType::InlineBase(_, _)) = &field.field_type {
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
    }

    out
}

/// Generate encode code for a single field
#[allow(clippy::too_many_arguments)]
pub(crate) fn generate_encode_field(
    field: &Field,
    context: &str,
    struct_names: &std::collections::HashSet<String>,
    message_names: &std::collections::HashSet<String>,
    enum_style: SbeEnumStyle,
    buf_var: &str,
    acc: &str,
    buf_is_mut_ref: bool,
) -> String {
    let mut_buf = if buf_is_mut_ref {
        buf_var.to_string()
    } else {
        format!("&mut {buf_var}")
    };
    let mut out = String::new();
    let field_name = &field.name;
    let ref_name = format!("{acc}{field_name}");
    let optional = field.optional;

    match &field.field_type {
        FieldType::Enum(_) => {
            let canonical = canonical_enum_name(field_name, context);
            let wire_enum = format!("{context}{}", pascal_case(field_name));
            let use_shared = matches!(enum_style, SbeEnumStyle::MessagesShared)
                && is_shared_messages_enum(&canonical);
            let use_messages_enum = wire_enum == canonical;
            if use_shared || use_messages_enum {
                if optional {
                    out.push_str(&format!(
                        "        {buf_var}.push({ref_name}.map(|v| v.to_value()).unwrap_or(0));\n"
                    ));
                    out.push_str(&format!(
                        "        {buf_var}.push(u8::from({ref_name}.is_some()));\n"
                    ));
                } else {
                    out.push_str(&format!("        {buf_var}.push({ref_name}.to_value());\n"));
                }
            } else if optional {
                out.push_str(&format!(
                    "        {buf_var}.push({ref_name}.map(|v| v as u8).unwrap_or(0));\n"
                ));
                out.push_str(&format!(
                    "        {buf_var}.push(u8::from({ref_name}.is_some()));\n"
                ));
            } else {
                out.push_str(&format!("        {buf_var}.push({ref_name} as u8);\n"));
            }
        }
        FieldType::Named(n) if is_struct_type(n, struct_names) => {
            out.push_str(&format!(
                "        {n}Encoder::encode(&{ref_name}, {mut_buf});\n"
            ));
        }
        FieldType::Named(n) if is_message_type(n, message_names) => {
            out.push_str(&format!(
                "        {n}BodyEncoder::encode(&{ref_name}, {mut_buf});\n"
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

            match bt.as_str() {
                "str" => {
                    if optional {
                        out.push_str(&format!(
                            "        {buf_var}.push(if {ref_name}.is_some() {{ 1 }} else {{ 0 }});\n"
                        ));
                        out.push_str(&format!("        if let Some(ref s) = {ref_name} {{\n"));
                        out.push_str("            let b = s.as_bytes();\n");
                        out.push_str(&format!(
                            "            {buf_var}.extend_from_slice(&(b.len() as u16).to_be_bytes());\n"
                        ));
                        out.push_str(&format!("            {buf_var}.extend_from_slice(b);\n"));
                        out.push_str("        }\n");
                    } else {
                        out.push_str(&format!(
                            "        let {field_name}_bytes = {ref_name}.as_bytes();\n"
                        ));
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice(&({field_name}_bytes.len() as u16).to_be_bytes());\n"
                        ));
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice({field_name}_bytes);\n"
                        ));
                    }
                }
                "f64" => {
                    let expr = f64_wire_expr(&ref_name, &field.field_type, optional);
                    if optional {
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice(&{expr}.to_be_bytes());\n"
                        ));
                        out.push_str(&format!(
                            "        {buf_var}.push(if {ref_name}.is_some() {{ 1 }} else {{ 0 }});\n"
                        ));
                    } else {
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice(&{expr}.to_be_bytes());\n"
                        ));
                    }
                }
                "i64" => {
                    if optional {
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice(&{ref_name}.unwrap_or(0).to_be_bytes());\n"
                        ));
                        out.push_str(&format!(
                            "        {buf_var}.push(u8::from({ref_name}.is_some()));\n"
                        ));
                    } else {
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice(&{ref_name}.to_be_bytes());\n"
                        ));
                    }
                }
                "u64" => {
                    if optional {
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice(&{ref_name}.unwrap_or(0).to_be_bytes());\n"
                        ));
                        out.push_str(&format!(
                            "        {buf_var}.push(u8::from({ref_name}.is_some()));\n"
                        ));
                    } else {
                        out.push_str(&format!(
                            "        {buf_var}.extend_from_slice(&{ref_name}.to_be_bytes());\n"
                        ));
                    }
                }
                "u8" => {
                    if optional {
                        out.push_str(&format!(
                            "        {buf_var}.push({ref_name}.unwrap_or(false) as u8);\n"
                        ));
                        out.push_str(&format!(
                            "        {buf_var}.push(u8::from({ref_name}.is_some()));\n"
                        ));
                    } else if matches!(&field.field_type, FieldType::InlineBase(BaseType::Bool, _))
                    {
                        out.push_str(&format!("        {buf_var}.push({ref_name} as u8);\n"));
                    } else {
                        out.push_str(&format!("        {buf_var}.push({ref_name});\n"));
                    }
                }
                "u32" => {
                    let expr = if optional {
                        format!("{ref_name}.unwrap_or(0)")
                    } else {
                        ref_name.clone()
                    };
                    out.push_str(&format!(
                        "        {buf_var}.extend_from_slice(&{expr}.to_be_bytes());\n"
                    ));
                    if optional {
                        out.push_str(&format!(
                            "        {buf_var}.push(u8::from({ref_name}.is_some()));\n"
                        ));
                    }
                }
                _ => {
                    out.push_str(&format!("        // TODO: encode {field_name} as {bt}\n"));
                }
            }
        }
        FieldType::List(inner) => {
            let inner_type = sbe_field_type_name(inner);
            out.push_str(&format!(
                "        {buf_var}.extend_from_slice(&({ref_name}.len() as u32).to_be_bytes());\n"
            ));
            out.push_str(&format!("        for item in &{ref_name} {{\n"));
            if let FieldType::Named(n) = inner.as_ref() {
                if is_struct_type(n, struct_names) {
                    out.push_str("            let mut item_buf = Vec::new();\n");
                    out.push_str(&format!(
                        "            {n}Encoder::encode(&item, &mut item_buf);\n"
                    ));
                    out.push_str(&format!(
                        "            {buf_var}.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());\n"
                    ));
                    out.push_str(&format!(
                        "            {buf_var}.extend_from_slice(&item_buf);\n"
                    ));
                } else if is_message_type(n, message_names) {
                    out.push_str("            let mut item_buf = Vec::new();\n");
                    out.push_str(&format!(
                        "            {n}BodyEncoder::encode(item, &mut item_buf);\n"
                    ));
                    out.push_str(&format!(
                        "            {buf_var}.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());\n"
                    ));
                    out.push_str(&format!(
                        "            {buf_var}.extend_from_slice(&item_buf);\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "            // TODO: encode list<{}>\n",
                        inner_type
                    ));
                }
            } else {
                out.push_str(&format!(
                    "            // TODO: encode list<{}>\n",
                    inner_type
                ));
            }
            out.push_str("        }\n");
        }
        FieldType::InlineStruct(_) => {
            let struct_name = format!("{context}{}", pascal_case(field_name));
            out.push_str(&format!(
                "        {struct_name}Encoder::encode(&{ref_name}, {mut_buf});\n"
            ));
        }
    }

    out
}

// ── SBE type mapping helpers ──────────────────────────────────

/// Resolve the SBE wire type for a field (str, f64, i64, u8, …).
pub(crate) fn sbe_field_wire_type(field: &Field) -> String {
    match &field.field_type {
        FieldType::Named(n) => sbe_named_type_to_wire_type(n),
        FieldType::InlineBase(bt, _) => sbe_base_type_to_wire_type(bt),
        FieldType::Enum(_) => "u8".to_string(),
        FieldType::List(_) => "list".to_string(),
        FieldType::InlineStruct(_) => "struct".to_string(),
    }
}

/// Get the Rust type name for a field in SBE context
pub(crate) fn sbe_field_type_name(ft: &FieldType) -> String {
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
pub(crate) fn sbe_field_type_name_for_message(
    ft: &FieldType,
    msg_name: &str,
    field_name: &str,
) -> String {
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
pub(crate) fn sbe_named_type_to_wire_type(name: &str) -> String {
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
pub(crate) fn sbe_base_type_to_wire_type(bt: &BaseType) -> String {
    match bt {
        BaseType::String => "str".to_string(),
        BaseType::Int8 | BaseType::UInt8 | BaseType::Bool => "u8".to_string(),
        BaseType::Int16 | BaseType::UInt16 => "u16".to_string(),
        BaseType::Int32 | BaseType::UInt32 | BaseType::Float32 => "u32".to_string(),
        BaseType::Int64 => "i64".to_string(),
        BaseType::UInt64 => "u64".to_string(),
        BaseType::Float64 | BaseType::Decimal64 => "f64".to_string(),
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
pub(crate) fn compute_block_length(fields: &[Field]) -> u16 {
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
        FieldType::InlineStruct(fields) => fields
            .iter()
            .map(|f| sbe_field_encoded_size(&f.field_type))
            .sum(),
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
pub(crate) fn pascal_case(s: &str) -> String {
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

        assert!(code.contains("GreetingEncoder"));
        assert!(code.contains("GreetingDecoder"));
        assert!(code.contains("GreetingLanguage"));
        assert!(!code.contains("pub enum GreetingLanguage"));
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
