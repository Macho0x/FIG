use serde::{Deserialize, Serialize};

/// Top-level FSL schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub well_known_id: Option<u8>,
    pub type_defs: Vec<TypeDef>,
    pub messages: Vec<Message>,
    pub gateway_mappings: Vec<GatewayMapping>,
}

/// Custom type definition (alias or struct)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub base_type: Option<BaseType>,
    pub constraints: Vec<Constraint>,
    pub fields: Option<Vec<Field>>,
}

/// Base/scalar types in FSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaseType {
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Decimal64,
    Bool,
    Bytes,
    List(Box<BaseType>),
}

/// Constraints on a type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint {
    MaxLen(usize),
    Min(serde_json::Value),
    Max(serde_json::Value),
    Precision(u8),
    Pattern(String),
    Uppercase(bool),
}

/// Message definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub name: String,
    pub channel_type: ChannelType,
    pub correlation_field: Option<String>,
    pub priority: Priority,
    pub idempotent: bool,
    pub fields: Vec<Field>,
}

/// Channel interaction pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    RequestResponse,
    StreamItem,
    BidirectionalStream,
    FireAndForget,
    PubSub,
}

/// Message priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// A field within a message or struct type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub number: Option<u32>,
    pub optional: bool,
}

/// The type of a field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Named(String),
    Enum(EnumDef),
    List(Box<FieldType>),
    InlineStruct(Vec<Field>),
    InlineBase(BaseType, Vec<Constraint>),
}

/// Inline enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDef {
    pub variants: Vec<String>,
}

/// Gateway mapping block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayMapping {
    pub protocol: GatewayProtocol,
    pub message_mappings: Vec<MessageMapping>,
}

/// Gateway protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GatewayProtocol {
    Fix,
    Rest,
}

/// Mapping for a message to a gateway protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMapping {
    pub message_name: String,
    pub target: String,
    pub field_mappings: Vec<FieldMapping>,
}

/// Mapping for a single field within a gateway message mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub source_field: String,
    pub target: String,
    pub value_mappings: Option<Vec<(String, String)>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip() {
        let schema = Schema {
            name: "test".to_string(),
            version: "v1.0.0".to_string(),
            description: Some("Test schema".to_string()),
            well_known_id: Some(0x01),
            type_defs: vec![TypeDef {
                name: "Price".to_string(),
                base_type: Some(BaseType::Decimal64),
                constraints: vec![
                    Constraint::Precision(8),
                    Constraint::Min(serde_json::json!(0)),
                ],
                fields: None,
            }],
            messages: vec![Message {
                name: "Order".to_string(),
                channel_type: ChannelType::RequestResponse,
                correlation_field: Some("id".to_string()),
                priority: Priority::High,
                idempotent: false,
                fields: vec![
                    Field {
                        name: "id".to_string(),
                        field_type: FieldType::Named("ClientOrderId".to_string()),
                        number: Some(1),
                        optional: false,
                    },
                    Field {
                        name: "side".to_string(),
                        field_type: FieldType::Enum(EnumDef {
                            variants: vec!["buy".to_string(), "sell".to_string()],
                        }),
                        number: Some(2),
                        optional: false,
                    },
                ],
            }],
            gateway_mappings: vec![],
        };

        let json = serde_json::to_string_pretty(&schema).unwrap();
        let parsed: Schema = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "test");
        assert_eq!(parsed.messages.len(), 1);
        assert_eq!(parsed.messages[0].fields.len(), 2);
    }
}
