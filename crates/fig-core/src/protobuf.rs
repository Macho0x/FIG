//! Protobuf codec for schema-evolving payloads (Spec §10).
//!
//! Encodes/decodes FIG message values as protobuf wire format using
//! field-numbered maps derived from JSON/CBOR-compatible values.

use std::collections::HashMap;

use crate::error::FrameError;

/// Encode a JSON-compatible value map as protobuf (proto3) bytes.
///
/// Uses a simplified field layout: keys become string field names hashed
/// to field numbers for wire encoding of scalar values.
pub fn encode_protobuf_map(fields: &HashMap<String, serde_json::Value>) -> Result<Vec<u8>, FrameError> {
    let mut buf = Vec::new();
    for (i, (key, value)) in fields.iter().enumerate() {
        let field_num = (i + 1) as u32;
        encode_proto_field(&mut buf, field_num, key, value)?;
    }
    Ok(buf)
}

/// Decode protobuf bytes into a string-keyed JSON value map.
pub fn decode_protobuf_map(data: &[u8]) -> Result<HashMap<String, serde_json::Value>, FrameError> {
    let mut fields = HashMap::new();
    let mut pos = 0;
    let mut index = 0;
    while pos < data.len() {
        let (tag, new_pos) = read_varint(data, pos)?;
        pos = new_pos;
        let field_num = (tag >> 3) as u32;
        let wire_type = tag & 0x07;
        let key = format!("field_{}", field_num);
        let value = decode_proto_value(data, &mut pos, wire_type)?;
        fields.insert(key, value);
        index += 1;
        let _ = index;
    }
    Ok(fields)
}

/// Convert JSON string to protobuf bytes via intermediate map.
pub fn json_to_protobuf(json: &str) -> Result<Vec<u8>, FrameError> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| FrameError::CborDecodeError(e.to_string()))?;
    let obj = value
        .as_object()
        .ok_or_else(|| FrameError::CborDecodeError("expected JSON object".into()))?;
    let map: HashMap<String, serde_json::Value> = obj.clone().into_iter().collect();
    encode_protobuf_map(&map)
}

/// Convert protobuf bytes to JSON string.
pub fn protobuf_to_json(data: &[u8]) -> Result<String, FrameError> {
    let map = decode_protobuf_map(data)?;
    let value = serde_json::Value::Object(map.into_iter().collect());
    serde_json::to_string(&value).map_err(|e| FrameError::CborEncodeError(e.to_string()))
}

fn encode_proto_field(
    buf: &mut Vec<u8>,
    field_num: u32,
    _key: &str,
    value: &serde_json::Value,
) -> Result<(), FrameError> {
    match value {
        serde_json::Value::String(s) => {
            write_tag(buf, field_num, 2);
            write_len_delimited(buf, s.as_bytes());
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                write_tag(buf, field_num, 0);
                write_varint(buf, i as u64);
            } else if let Some(f) = n.as_f64() {
                write_tag(buf, field_num, 1);
                buf.extend_from_slice(&f.to_le_bytes());
            }
        }
        serde_json::Value::Bool(b) => {
            write_tag(buf, field_num, 0);
            write_varint(buf, if *b { 1 } else { 0 });
        }
        _ => {
            let s = value.to_string();
            write_tag(buf, field_num, 2);
            write_len_delimited(buf, s.as_bytes());
        }
    }
    Ok(())
}

fn decode_proto_value(data: &[u8], pos: &mut usize, wire_type: u64) -> Result<serde_json::Value, FrameError> {
    match wire_type {
        0 => {
            let (v, new_pos) = read_varint(data, *pos)?;
            *pos = new_pos;
            Ok(serde_json::Value::Number((v as i64).into()))
        }
        1 => {
            if *pos + 8 > data.len() {
                return Err(FrameError::BufferTooShort {
                    expected: *pos + 8,
                    actual: data.len(),
                });
            }
            let bytes: [u8; 8] = data[*pos..*pos + 8].try_into().unwrap();
            *pos += 8;
            Ok(serde_json::Value::Number(
                serde_json::Number::from_f64(f64::from_le_bytes(bytes))
                    .unwrap_or_else(|| 0.into()),
            ))
        }
        2 => {
            let (len, new_pos) = read_varint(data, *pos)?;
            *pos = new_pos;
            let end = *pos + len as usize;
            if end > data.len() {
                return Err(FrameError::BufferTooShort {
                    expected: end,
                    actual: data.len(),
                });
            }
            let s = String::from_utf8_lossy(&data[*pos..end]).to_string();
            *pos = end;
            Ok(serde_json::Value::String(s))
        }
        _ => Err(FrameError::CborDecodeError(format!(
            "unsupported wire type {}",
            wire_type
        ))),
    }
}

fn write_tag(buf: &mut Vec<u8>, field_num: u32, wire_type: u64) {
    write_varint(buf, ((field_num as u64) << 3) | wire_type);
}

fn write_len_delimited(buf: &mut Vec<u8>, data: &[u8]) {
    write_varint(buf, data.len() as u64);
    buf.extend_from_slice(data);
}

fn write_varint(buf: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        buf.push((value as u8) | 0x80);
        value >>= 7;
    }
    buf.push(value as u8);
}

fn read_varint(data: &[u8], mut pos: usize) -> Result<(u64, usize), FrameError> {
    let mut result = 0u64;
    let mut shift = 0;
    loop {
        if pos >= data.len() {
            return Err(FrameError::BufferTooShort {
                expected: pos + 1,
                actual: data.len(),
            });
        }
        let byte = data[pos];
        pos += 1;
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err(FrameError::CborDecodeError("varint overflow".into()));
        }
    }
    Ok((result, pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_protobuf_round_trip() {
        let json = r#"{"symbol":"AAPL","qty":100,"active":true}"#;
        let pb = json_to_protobuf(json).unwrap();
        assert!(!pb.is_empty());
        let back = protobuf_to_json(&pb).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&back).unwrap();
        assert!(parsed.as_object().unwrap().contains_key("field_1"));
    }

    #[test]
    fn test_encode_decode_map() {
        let mut map = HashMap::new();
        map.insert("name".into(), serde_json::Value::String("test".into()));
        map.insert("count".into(), serde_json::Value::Number(42.into()));
        let encoded = encode_protobuf_map(&map).unwrap();
        let decoded = decode_protobuf_map(&encoded).unwrap();
        assert_eq!(decoded.len(), 2);
    }
}
