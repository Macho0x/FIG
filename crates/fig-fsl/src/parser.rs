use crate::ast::*;
use anyhow::{anyhow, Context, Result};

/// Hand-written recursive descent parser for FSL
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    kind: TokenKind,
    text: String,
    line: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Schema,
    Type,
    Message,
    Enum,
    List,
    Optional,
    Gateway,
    Fix,
    Rest,
    Values,
    True,
    False,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenAngle,
    CloseAngle,
    At,
    Colon,
    Comma,
    Dot,
    Arrow,
    Ident,
    StringLit,
    Number,
    Eof,
}

impl Parser {
    pub fn parse(input: &str) -> Result<Schema> {
        let tokens = Self::tokenize(input)?;
        let mut parser = Parser { tokens, pos: 0 };
        parser.parse_schema()
    }

    // ── Tokenizer ──────────────────────────────────────────────

    fn tokenize(input: &str) -> Result<Vec<Token>> {
        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let mut i = 0;
        let mut line = 1;
        let mut tokens = Vec::new();

        while i < len {
            let ch = chars[i];

            // Whitespace
            if ch.is_whitespace() {
                if ch == '\n' {
                    line += 1;
                }
                i += 1;
                continue;
            }

            // Line comment
            if ch == '/' && i + 1 < len && chars[i + 1] == '/' {
                i += 2;
                while i < len && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }

            // String literal
            if ch == '"' {
                i += 1;
                let start = i;
                while i < len && chars[i] != '"' {
                    if chars[i] == '\n' {
                        line += 1;
                    }
                    if chars[i] == '\\' && i + 1 < len {
                        // Skip escaped char
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                if i >= len {
                    return Err(anyhow!("line {}: unterminated string literal", line));
                }
                let text: String = chars[start..i].iter().collect();
                i += 1; // skip closing "
                tokens.push(Token {
                    kind: TokenKind::StringLit,
                    text,
                    line,
                });
                continue;
            }

            // Hex number or decimal number
            if ch.is_ascii_digit() {
                if ch == '0' && i + 1 < len && chars[i + 1] == 'x' {
                    i += 2; // skip "0x"
                    let start = i;
                    while i < len && chars[i].is_ascii_hexdigit() {
                        i += 1;
                    }
                    let text = format!("0x{}", &chars[start..i].iter().collect::<String>());
                    tokens.push(Token {
                        kind: TokenKind::Number,
                        text,
                        line,
                    });
                } else {
                    let start = i;
                    while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        i += 1;
                    }
                    let text: String = chars[start..i].iter().collect();
                    tokens.push(Token {
                        kind: TokenKind::Number,
                        text,
                        line,
                    });
                }
                continue;
            }

            // Identifier or keyword
            if ch.is_ascii_alphabetic() || ch == '_' {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '.')
                {
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();
                let kind = match text.as_str() {
                    "schema" => TokenKind::Schema,
                    "type" => TokenKind::Type,
                    "message" => TokenKind::Message,
                    "enum" => TokenKind::Enum,
                    "list" => TokenKind::List,
                    "optional" => TokenKind::Optional,
                    "gateway" => TokenKind::Gateway,
                    "fix" => TokenKind::Fix,
                    "rest" => TokenKind::Rest,
                    "values" => TokenKind::Values,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    _ => TokenKind::Ident,
                };
                tokens.push(Token { kind, text, line });
                continue;
            }

            // Arrow "->"
            if ch == '-' && i + 1 < len && chars[i + 1] == '>' {
                tokens.push(Token {
                    kind: TokenKind::Arrow,
                    text: "->".to_string(),
                    line,
                });
                i += 2;
                continue;
            }

            // Single-character tokens
            let kind = match ch {
                '{' => TokenKind::OpenBrace,
                '}' => TokenKind::CloseBrace,
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                '<' => TokenKind::OpenAngle,
                '>' => TokenKind::CloseAngle,
                '@' => TokenKind::At,
                ':' => TokenKind::Colon,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                c => {
                    return Err(anyhow!("line {}: unexpected character '{}'", line, c));
                }
            };
            tokens.push(Token {
                kind,
                text: ch.to_string(),
                line,
            });
            i += 1;
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            text: String::new(),
            line,
        });
        Ok(tokens)
    }

    // ── Helpers ────────────────────────────────────────────────

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().kind == *kind
    }

    fn check_ident(&self, text: &str) -> bool {
        self.peek().kind == TokenKind::Ident && self.peek().text == text
    }

    fn check_kw(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn line(&self) -> usize {
        self.peek().line
    }

    fn error(&self, msg: &str) -> anyhow::Error {
        anyhow!("line {}: {}", self.line(), msg)
    }

    fn error_at(&self, line: usize, msg: &str) -> anyhow::Error {
        anyhow!("line {}: {}", line, msg)
    }

    fn expect(&mut self, kind: TokenKind, ctx: &str) -> Result<Token> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(self.error(&format!(
                "expected {:?} for {}, got {:?}",
                kind,
                ctx,
                self.peek().kind
            )))
        }
    }

    fn expect_ident(&mut self, ctx: &str) -> Result<String> {
        if self.peek().kind == TokenKind::Ident {
            Ok(self.advance().text)
        } else {
            Err(self.error(&format!(
                "expected identifier for {}, got {:?}",
                ctx,
                self.peek().kind
            )))
        }
    }

    fn expect_kw(&mut self, kind: TokenKind, ctx: &str) -> Result<Token> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(self.error(&format!("expected {:?} for {}", kind, ctx)))
        }
    }

    /// Collect raw token texts until a stop token kind or Eof
    fn collect_until(&mut self, stops: &[TokenKind]) -> String {
        let mut parts = Vec::new();
        while !stops.contains(&self.peek().kind) && !self.check(&TokenKind::Eof) {
            parts.push(self.advance().text);
        }
        parts.join(" ")
    }

    /// Collect raw token texts until we see the 'message' keyword or '}' or Eof
    fn collect_until_message_or_close(&mut self) -> String {
        let mut parts = Vec::new();
        while !self.check(&TokenKind::Eof)
            && !self.check(&TokenKind::CloseBrace)
            && !self.check_kw(TokenKind::Message)
        {
            parts.push(self.advance().text);
        }
        parts.join(" ")
    }

    // ── Top-level ──────────────────────────────────────────────

    fn parse_schema(&mut self) -> Result<Schema> {
        self.expect_kw(TokenKind::Schema, "schema declaration")?;
        let name = self.expect_ident("schema name")?;
        let version = self.expect_ident("schema version")?;
        self.expect(TokenKind::OpenBrace, "schema body")?;

        let mut description = None;
        let mut well_known_id = None;
        let mut type_defs = Vec::new();
        let mut messages = Vec::new();
        let mut gateway_mappings = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.check(&TokenKind::Eof) {
            if self.check_ident("description") {
                self.advance(); // skip 'description'
                self.expect(TokenKind::Colon, "description colon")?;
                let val = self.expect(TokenKind::StringLit, "description value")?;
                description = Some(val.text);
            } else if self.check_ident("well_known_id") {
                self.advance(); // skip 'well_known_id'
                self.expect(TokenKind::Colon, "well_known_id colon")?;
                let hex = self.expect(TokenKind::Number, "well_known_id hex value")?;
                // Parse hex: "0x01" -> 1
                let val = hex.text.trim_start_matches("0x").trim_start_matches("0X");
                let id = u8::from_str_radix(val, 16)
                    .with_context(|| format!("well_known_id: invalid hex '{}'", hex.text))?;
                well_known_id = Some(id);
            } else if self.check_kw(TokenKind::Type) {
                type_defs.push(self.parse_type_def()?);
            } else if self.check_kw(TokenKind::Message) {
                messages.push(self.parse_message()?);
            } else if self.check_kw(TokenKind::Gateway) {
                gateway_mappings.push(self.parse_gateway_mapping()?);
            } else {
                return Err(self.error(&format!(
                    "unexpected token {:?} ('{}') in schema body",
                    self.peek().kind,
                    self.peek().text
                )));
            }
        }
        self.expect(TokenKind::CloseBrace, "schema close")?;

        Ok(Schema {
            name,
            version,
            description,
            well_known_id,
            type_defs,
            messages,
            gateway_mappings,
        })
    }

    // ── Type definitions ──────────────────────────────────────

    fn parse_type_def(&mut self) -> Result<TypeDef> {
        let line = self.line();
        self.advance(); // skip 'type'
        let name = self.expect_ident("type name")?;

        if self.check(&TokenKind::Colon) {
            // Simple alias: type Name: BaseType(constraints)
            self.advance(); // skip ':'
            let base_type = self.parse_base_type()?;
            let constraints = if self.check(&TokenKind::OpenParen) {
                self.parse_constraints()?
            } else {
                Vec::new()
            };
            Ok(TypeDef {
                name,
                base_type: Some(base_type),
                constraints,
                fields: None,
            })
        } else if self.check(&TokenKind::OpenBrace) {
            // Struct-like: type Name { fields... }
            self.advance(); // skip '{'
            let mut fields = Vec::new();
            while !self.check(&TokenKind::CloseBrace) && !self.check(&TokenKind::Eof) {
                fields.push(self.parse_struct_field()?);
            }
            self.expect(TokenKind::CloseBrace, "closing } for struct type")?;
            Ok(TypeDef {
                name,
                base_type: None,
                constraints: Vec::new(),
                fields: Some(fields),
            })
        } else {
            Err(self.error_at(
                line,
                &format!(
                    "expected ':' or '{{' after type name '{}', got {:?}",
                    name,
                    self.peek().kind
                ),
            ))
        }
    }

    /// Parse a field inside a struct-like type definition (no @number)
    fn parse_struct_field(&mut self) -> Result<Field> {
        let name = self.expect_ident("field name")?;
        self.expect(TokenKind::Colon, "field type colon")?;
        let field_type = self.parse_field_type()?;
        let optional = if self.check_kw(TokenKind::Optional) {
            self.advance();
            true
        } else {
            false
        };
        Ok(Field {
            name,
            field_type,
            number: None,
            optional,
        })
    }

    /// Parse a base type name (string, int32, decimal64, etc.)
    fn parse_base_type(&mut self) -> Result<BaseType> {
        let ident = self.expect_ident("base type")?;
        Self::base_type_from_str(&ident)
            .ok_or_else(|| self.error(&format!("unknown base type '{}'", ident)))
    }

    fn base_type_from_str(s: &str) -> Option<BaseType> {
        match s {
            "string" => Some(BaseType::String),
            "int8" => Some(BaseType::Int8),
            "int16" => Some(BaseType::Int16),
            "int32" => Some(BaseType::Int32),
            "int64" => Some(BaseType::Int64),
            "uint8" => Some(BaseType::UInt8),
            "uint16" => Some(BaseType::UInt16),
            "uint32" => Some(BaseType::UInt32),
            "uint64" => Some(BaseType::UInt64),
            "float32" => Some(BaseType::Float32),
            "float64" => Some(BaseType::Float64),
            "decimal64" => Some(BaseType::Decimal64),
            "bool" => Some(BaseType::Bool),
            "bytes" => Some(BaseType::Bytes),
            _ => None,
        }
    }

    fn is_base_type_name(s: &str) -> bool {
        Self::base_type_from_str(s).is_some()
    }

    /// Parse constraints inside parentheses: (max_len: 20, pattern: "...")
    fn parse_constraints(&mut self) -> Result<Vec<Constraint>> {
        self.advance(); // skip '('
        let mut constraints = Vec::new();
        if !self.check(&TokenKind::CloseParen) {
            loop {
                let name = self.expect_ident("constraint name")?;
                self.expect(TokenKind::Colon, "constraint colon")?;
                let value = self.parse_literal_value()?;
                let constraint = self.make_constraint(&name, value)?;
                constraints.push(constraint);
                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::CloseParen, "closing ')' for constraints")?;
        Ok(constraints)
    }

    fn make_constraint(&mut self, name: &str, value: serde_json::Value) -> Result<Constraint> {
        match name {
            "max_len" => {
                let n = value
                    .as_u64()
                    .or_else(|| value.as_i64().map(|v| v as u64))
                    .ok_or_else(|| self.error("max_len expects an integer"))?;
                Ok(Constraint::MaxLen(n as usize))
            }
            "min" => Ok(Constraint::Min(value)),
            "max" => Ok(Constraint::Max(value)),
            "precision" => {
                let n = value
                    .as_u64()
                    .or_else(|| value.as_i64().map(|v| v as u64))
                    .ok_or_else(|| self.error("precision expects an integer"))?;
                Ok(Constraint::Precision(n as u8))
            }
            "pattern" => {
                let s = value
                    .as_str()
                    .ok_or_else(|| self.error("pattern expects a string"))?;
                Ok(Constraint::Pattern(s.to_string()))
            }
            "uppercase" => {
                let b = value
                    .as_bool()
                    .ok_or_else(|| self.error("uppercase expects a boolean"))?;
                Ok(Constraint::Uppercase(b))
            }
            _ => Err(self.error(&format!("unknown constraint '{}'", name))),
        }
    }

    /// Parse a literal value: number, string, true, false
    fn parse_literal_value(&mut self) -> Result<serde_json::Value> {
        if self.check(&TokenKind::StringLit) {
            let s = self.advance().text;
            Ok(serde_json::Value::String(s))
        } else if self.check(&TokenKind::Number) {
            let t = self.advance();
            // Parse as number
            let s = &t.text;
            if s.starts_with("0x") || s.starts_with("0X") {
                let n =
                    u64::from_str_radix(s.trim_start_matches("0x").trim_start_matches("0X"), 16)
                        .map_err(|_| self.error(&format!("invalid hex literal '{}'", s)))?;
                Ok(serde_json::Value::Number(n.into()))
            } else if s.contains('.') {
                let n: f64 = s
                    .parse()
                    .map_err(|_| self.error(&format!("invalid float literal '{}'", s)))?;
                Ok(serde_json::json!(n))
            } else {
                let n: i64 = s
                    .parse()
                    .map_err(|_| self.error(&format!("invalid integer literal '{}'", s)))?;
                Ok(serde_json::Value::Number(n.into()))
            }
        } else if self.check_kw(TokenKind::True) {
            self.advance();
            Ok(serde_json::Value::Bool(true))
        } else if self.check_kw(TokenKind::False) {
            self.advance();
            Ok(serde_json::Value::Bool(false))
        } else {
            Err(self.error(&format!(
                "expected literal value, got {:?}",
                self.peek().kind
            )))
        }
    }

    // ── Messages ───────────────────────────────────────────────

    fn parse_message(&mut self) -> Result<Message> {
        self.advance(); // skip 'message'
        let name = self.expect_ident("message name")?;
        self.expect(TokenKind::OpenBrace, "message body")?;

        let mut channel_type = ChannelType::RequestResponse;
        let mut correlation_field = None;
        let mut priority = Priority::Medium;
        let mut idempotent = true; // default true
        let mut fields = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.check(&TokenKind::Eof) {
            // Each item starts with an identifier
            let key = self.expect_ident("message body item")?;
            self.expect(TokenKind::Colon, "colon after key")?;

            // Parse what looks like a type/value
            let value_token = self.peek().clone();
            let _value_text = value_token.text.clone();

            // Check if the value is a known annotation key
            if key == "channel_type" {
                channel_type = self.parse_channel_type()?;
            } else if key == "correlation_field" {
                correlation_field = Some(self.expect_ident("correlation field")?);
            } else if key == "priority" {
                priority = self.parse_priority()?;
            } else if key == "idempotent" {
                idempotent = self.parse_bool_literal()?;
            } else {
                // It's a field: name: type @number optional?
                let field_type = self.parse_field_type_after_colon(&value_token)?;

                // Expect @number for message fields
                self.expect(TokenKind::At, "@field_number")?;
                let num_token = self.expect(TokenKind::Number, "field number")?;
                let number: u32 = num_token
                    .text
                    .parse()
                    .with_context(|| format!("invalid field number '{}'", num_token.text))?;

                let optional = if self.check_kw(TokenKind::Optional) {
                    self.advance();
                    true
                } else {
                    false
                };

                fields.push(Field {
                    name: key,
                    field_type,
                    number: Some(number),
                    optional,
                });
            }
        }
        self.expect(TokenKind::CloseBrace, "message close")?;

        Ok(Message {
            name,
            channel_type,
            correlation_field,
            priority,
            idempotent,
            fields,
        })
    }

    fn parse_channel_type(&mut self) -> Result<ChannelType> {
        let val = self.expect_ident("channel_type value")?;
        match val.as_str() {
            "request_response" => Ok(ChannelType::RequestResponse),
            "stream_item" => Ok(ChannelType::StreamItem),
            "bidirectional_stream" => Ok(ChannelType::BidirectionalStream),
            "fire_and_forget" => Ok(ChannelType::FireAndForget),
            "pub_sub" => Ok(ChannelType::PubSub),
            _ => Err(self.error(&format!("unknown channel_type '{}'", val))),
        }
    }

    fn parse_priority(&mut self) -> Result<Priority> {
        let val = self.expect_ident("priority value")?;
        match val.as_str() {
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(self.error(&format!("unknown priority '{}'", val))),
        }
    }

    fn parse_bool_literal(&mut self) -> Result<bool> {
        if self.check_kw(TokenKind::True) {
            self.advance();
            Ok(true)
        } else if self.check_kw(TokenKind::False) {
            self.advance();
            Ok(false)
        } else {
            Err(self.error(&format!("expected true/false, got {:?}", self.peek().kind)))
        }
    }

    /// Parse field type after we've already consumed the colon.
    /// The value_token is the next token (peeked but not consumed).
    fn parse_field_type_after_colon(&mut self, _value_token: &Token) -> Result<FieldType> {
        self.parse_field_type()
    }

    // ── Field type parsing ─────────────────────────────────────

    /// Parse the type part of a field (after the colon)
    fn parse_field_type(&mut self) -> Result<FieldType> {
        // Check for list<...>
        if self.check_kw(TokenKind::List) {
            self.advance(); // skip 'list'
            self.expect(TokenKind::OpenAngle, "list open angle")?;
            let inner = self.parse_field_type()?;
            self.expect(TokenKind::CloseAngle, "list close angle")?;
            return Ok(FieldType::List(Box::new(inner)));
        }

        // Check for enum { ... }
        if self.check_kw(TokenKind::Enum) {
            return Ok(FieldType::Enum(self.parse_enum_def()?));
        }

        // Check for { (inline struct)
        if self.check(&TokenKind::OpenBrace) {
            self.advance(); // skip '{'
            let mut fields = Vec::new();
            while !self.check(&TokenKind::CloseBrace) && !self.check(&TokenKind::Eof) {
                fields.push(self.parse_struct_field()?);
            }
            self.expect(TokenKind::CloseBrace, "inline struct close")?;
            return Ok(FieldType::InlineStruct(fields));
        }

        // Must be an identifier: either a named type or inline base type
        let ident = self.expect_ident("field type")?;

        // Check if it's a base type (with or without constraints)
        if Self::is_base_type_name(&ident) {
            let base_type = Self::base_type_from_str(&ident).unwrap();
            let constraints = if self.check(&TokenKind::OpenParen) {
                self.parse_constraints()?
            } else {
                Vec::new()
            };
            return Ok(FieldType::InlineBase(base_type, constraints));
        }

        // Otherwise it's a named type reference
        Ok(FieldType::Named(ident))
    }

    // ── Enum ───────────────────────────────────────────────────

    fn parse_enum_def(&mut self) -> Result<EnumDef> {
        self.advance(); // skip 'enum'
        self.expect(TokenKind::OpenBrace, "enum body")?;
        let mut variants = Vec::new();
        loop {
            let variant = self.expect_ident("enum variant")?;
            variants.push(variant);
            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenKind::CloseBrace, "enum close")?;
        Ok(EnumDef { variants })
    }

    // ── Gateway mapping ────────────────────────────────────────

    fn parse_gateway_mapping(&mut self) -> Result<GatewayMapping> {
        self.advance(); // skip 'gateway'

        let protocol = if self.check_kw(TokenKind::Fix) {
            self.advance();
            GatewayProtocol::Fix
        } else if self.check_kw(TokenKind::Rest) {
            self.advance();
            GatewayProtocol::Rest
        } else {
            return Err(self.error("expected 'fix' or 'rest' after 'gateway'"));
        };

        self.expect(TokenKind::OpenBrace, "gateway body")?;
        let mut message_mappings = Vec::new();

        while !self.check(&TokenKind::CloseBrace) && !self.check(&TokenKind::Eof) {
            self.expect_kw(TokenKind::Message, "gateway message mapping")?;
            let message_name = self.expect_ident("message name in gateway")?;
            self.expect(TokenKind::Arrow, "-> in gateway mapping")?;

            let target = match protocol {
                GatewayProtocol::Fix => {
                    // Collect until '{' (start of field mappings)
                    self.collect_until(&[TokenKind::OpenBrace])
                }
                GatewayProtocol::Rest => {
                    // Collect until next 'message' or '}'
                    self.collect_until_message_or_close()
                }
            };

            let field_mappings = if matches!(protocol, GatewayProtocol::Fix) {
                self.expect(TokenKind::OpenBrace, "fix field mappings body")?;
                let mut mappings = Vec::new();
                while !self.check(&TokenKind::CloseBrace) && !self.check(&TokenKind::Eof) {
                    mappings.push(self.parse_field_mapping()?);
                }
                self.expect(TokenKind::CloseBrace, "fix field mappings close")?;
                mappings
            } else {
                Vec::new()
            };

            message_mappings.push(MessageMapping {
                message_name,
                target,
                field_mappings,
            });
        }
        self.expect(TokenKind::CloseBrace, "gateway close")?;

        Ok(GatewayMapping {
            protocol,
            message_mappings,
        })
    }

    fn parse_field_mapping(&mut self) -> Result<FieldMapping> {
        let source_field = self.expect_ident("source field name")?;
        self.expect(TokenKind::Arrow, "-> in field mapping")?;
        let target =
            self.collect_until(&[TokenKind::Values, TokenKind::CloseBrace, TokenKind::Eof]);

        let value_mappings = if self.check_kw(TokenKind::Values) {
            self.advance(); // skip 'values'
            self.expect(TokenKind::Colon, "values colon")?;
            self.expect(TokenKind::OpenBrace, "values body")?;

            let mut vals = Vec::new();
            loop {
                let key = self.expect_ident("value mapping key")?;
                self.expect(TokenKind::Colon, "value mapping colon")?;
                let val = self.expect(TokenKind::StringLit, "value mapping string")?;
                vals.push((key, val.text));
                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.expect(TokenKind::CloseBrace, "values close")?;
            Some(vals)
        } else {
            None
        };

        Ok(FieldMapping {
            source_field,
            target,
            value_mappings,
        })
    }
}

/// Load and merge the split trading schema fragments used by codegen.
pub fn load_merged_trading_schema() -> Schema {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    let fragments = [
        "orders.fsl",
        "marketdata.fsl",
        "instruments.fsl",
        "account.fsl",
    ];
    let mut schemas = Vec::new();
    for name in fragments {
        let input =
            std::fs::read_to_string(root.join(name)).unwrap_or_else(|e| panic!("read {name}: {e}"));
        schemas.push(Parser::parse(&input).unwrap_or_else(|e| panic!("parse {name}: {e}")));
    }
    merge_schemas(schemas)
}

pub fn merge_schemas(mut schemas: Vec<Schema>) -> Schema {
    let mut base = schemas.remove(0);
    for other in schemas {
        for td in other.type_defs {
            if !base.type_defs.iter().any(|t| t.name == td.name) {
                base.type_defs.push(td);
            }
        }
        for msg in other.messages {
            if !base.messages.iter().any(|m| m.name == msg.name) {
                base.messages.push(msg);
            }
        }
        base.gateway_mappings.extend(other.gateway_mappings);
    }
    base
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let input = "schema test v1.0 { type Foo: string(max_len: 10) message Bar { x: Foo @1 } }";
        let tokens = Parser::tokenize(input).unwrap();
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert!(kinds.contains(&TokenKind::Schema));
        assert!(kinds.contains(&TokenKind::Type));
        assert!(kinds.contains(&TokenKind::Message));
        assert!(kinds.contains(&TokenKind::At));
    }

    #[test]
    fn test_tokenize_comments() {
        let input = "// this is a comment\nschema test v1.0 {}";
        let tokens = Parser::tokenize(input).unwrap();
        let texts: Vec<String> = tokens.iter().map(|t| t.text.clone()).collect();
        assert!(!texts.contains(&"this".to_string()));
        assert!(texts.contains(&"schema".to_string()));
    }

    #[test]
    fn test_tokenize_string() {
        let input = r#"description: "hello world""#;
        let tokens = Parser::tokenize(input).unwrap();
        let string_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::StringLit)
            .collect();
        assert_eq!(string_tokens.len(), 1);
        assert_eq!(string_tokens[0].text, "hello world");
    }

    #[test]
    fn test_parse_simple_schema() {
        let input = r#"
            schema test v1.0.0 {
                description: "A test schema"
                well_known_id: 0x01
                type Name: string(max_len: 32)
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        assert_eq!(schema.name, "test");
        assert_eq!(schema.version, "v1.0.0");
        assert_eq!(schema.description.as_deref(), Some("A test schema"));
        assert_eq!(schema.well_known_id, Some(0x01));
        assert_eq!(schema.type_defs.len(), 1);
        assert_eq!(schema.type_defs[0].name, "Name");
    }

    #[test]
    fn test_parse_type_alias() {
        let input = "schema x v1 { type Price: decimal64(precision: 8, min: 0) }";
        let schema = Parser::parse(input).unwrap();
        let td = &schema.type_defs[0];
        assert_eq!(td.name, "Price");
        assert!(matches!(td.base_type, Some(BaseType::Decimal64)));
        assert_eq!(td.constraints.len(), 2);
    }

    #[test]
    fn test_parse_struct_type() {
        let input = "schema x v1 {
            type PriceLevel {
                price: Price
                qty: Quantity
                order_count: uint32 optional
            }
        }";
        let schema = Parser::parse(input).unwrap();
        let td = &schema.type_defs[0];
        assert_eq!(td.name, "PriceLevel");
        assert!(td.base_type.is_none());
        let fields = td.fields.as_ref().unwrap();
        assert_eq!(fields.len(), 3);
        assert!(fields[2].optional);
        assert_eq!(fields[2].name, "order_count");
    }

    #[test]
    fn test_parse_message() {
        let input = r#"
            schema x v1 {
                type Id: string(max_len: 20)
                message NewOrder {
                    channel_type: request_response
                    correlation_field: id
                    id: Id @1
                    side: enum { buy, sell } @2
                    qty: decimal64(precision: 0, min: 0) @3
                    priority: high
                    idempotent: false
                }
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        assert_eq!(schema.messages.len(), 1);
        let msg = &schema.messages[0];
        assert_eq!(msg.name, "NewOrder");
        assert!(matches!(msg.channel_type, ChannelType::RequestResponse));
        assert_eq!(msg.correlation_field.as_deref(), Some("id"));
        assert!(matches!(msg.priority, Priority::High));
        assert!(!msg.idempotent);
        assert_eq!(msg.fields.len(), 3);
        assert_eq!(msg.fields[0].name, "id");
        assert_eq!(msg.fields[0].number, Some(1));
        assert!(matches!(msg.fields[1].field_type, FieldType::Enum(_)));
        assert!(matches!(
            msg.fields[2].field_type,
            FieldType::InlineBase(_, _)
        ));
    }

    #[test]
    fn test_parse_optional_field() {
        let input = "schema x v1 {
            type P: decimal64(precision: 8)
            message M {
                channel_type: stream_item
                price: P @1 optional
                name: string(max_len: 32) @2 optional
            }
        }";
        let schema = Parser::parse(input).unwrap();
        let msg = &schema.messages[0];
        assert!(msg.fields[0].optional);
        assert!(msg.fields[1].optional);
    }

    #[test]
    fn test_parse_list_field() {
        let input = "schema x v1 {
            type PL { price: P qty: Q }
            message M {
                channel_type: stream_item
                bids: list<PL> @1
                asks: list<PL> @2 optional
            }
        }";
        let schema = Parser::parse(input).unwrap();
        let msg = &schema.messages[0];
        assert!(matches!(msg.fields[0].field_type, FieldType::List(_)));
        assert!(matches!(msg.fields[1].field_type, FieldType::List(_)));
    }

    #[test]
    fn test_parse_gateway_fix() {
        let input = r#"
            schema x v1 {
                message N {
                    channel_type: request_response
                    id: string(max_len: 20) @1
                }
                gateway fix {
                    message N -> MsgType: "D" {
                        id -> tag: 11
                    }
                }
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        assert_eq!(schema.gateway_mappings.len(), 1);
        let gm = &schema.gateway_mappings[0];
        assert!(matches!(gm.protocol, GatewayProtocol::Fix));
        assert_eq!(gm.message_mappings.len(), 1);
        assert_eq!(gm.message_mappings[0].message_name, "N");
        assert!(gm.message_mappings[0].target.contains("MsgType"));
        assert_eq!(gm.message_mappings[0].field_mappings.len(), 1);
    }

    #[test]
    fn test_parse_gateway_fix_values() {
        let input = r#"
            schema x v1 {
                message N {
                    channel_type: request_response
                    side: enum { buy, sell } @1
                }
                gateway fix {
                    message N -> MsgType: "D" {
                        side -> tag: 54 values: { buy: "1", sell: "2" }
                    }
                }
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        let fm = &schema.gateway_mappings[0].message_mappings[0].field_mappings[0];
        assert_eq!(fm.source_field, "side");
        let vals = fm.value_mappings.as_ref().unwrap();
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], ("buy".to_string(), "1".to_string()));
    }

    #[test]
    fn test_parse_gateway_rest() {
        let input = r#"
            schema x v1 {
                message GetOrder {
                    channel_type: request_response
                    id: string(max_len: 32) @1
                }
                gateway rest {
                    message GetOrder -> method: GET path: "/orders/{id}"
                }
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        let gm = &schema.gateway_mappings[0];
        assert!(matches!(gm.protocol, GatewayProtocol::Rest));
        let mm = &gm.message_mappings[0];
        assert_eq!(mm.message_name, "GetOrder");
        assert!(mm.target.contains("GET"));
        assert!(mm.target.contains("/orders/{id}"));
        assert_eq!(mm.field_mappings.len(), 0);
    }

    #[test]
    fn test_parse_default_priority_idempotent() {
        let input = r#"
            schema x v1 {
                message M {
                    channel_type: stream_item
                    x: string(max_len: 10) @1
                }
            }
        "#;
        let schema = Parser::parse(input).unwrap();
        let msg = &schema.messages[0];
        assert!(matches!(msg.priority, Priority::Medium));
        assert!(msg.idempotent);
        assert!(msg.correlation_field.is_none());
    }

    #[test]
    fn test_parse_full_orders_fsl() {
        let schema = load_merged_trading_schema();
        assert_eq!(schema.name, "trading.orders");
        assert_eq!(schema.version, "v1.0.0");
        assert_eq!(schema.well_known_id, Some(0x01));
        assert_eq!(
            schema.description.as_deref(),
            Some("Standard order entry and execution messages")
        );
        assert!(schema.type_defs.len() >= 7);
        assert!(schema.messages.len() >= 7);
        assert_eq!(schema.gateway_mappings.len(), 2);
    }
}
