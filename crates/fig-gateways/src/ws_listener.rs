//! Minimal WebSocket HTTP upgrade handler for the FIG gateway.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha1::{Digest, Sha1};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::rest::parse_http_request;
use crate::ws::{parse_ws_frame, serialize_ws_frame, WsFrame, WsOpcode};

const WS_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub fn is_websocket_upgrade(raw: &[u8]) -> bool {
    std::str::from_utf8(raw)
        .ok()
        .and_then(|s| s.lines().next())
        .map(|line| line.contains("Upgrade: websocket") || line.contains("GET "))
        .unwrap_or(false)
        && std::str::from_utf8(raw)
            .ok()
            .is_some_and(|s| s.to_ascii_lowercase().contains("upgrade: websocket"))
}

pub fn websocket_accept_key(sec_key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(sec_key.as_bytes());
    hasher.update(WS_GUID.as_bytes());
    STANDARD.encode(hasher.finalize())
}

pub async fn accept_websocket(stream: &mut TcpStream, raw: &[u8]) -> anyhow::Result<()> {
    let request = parse_http_request(raw)?;
    let sec_key = request
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("sec-websocket-key"))
        .map(|(_, v)| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing Sec-WebSocket-Key"))?;
    let accept = websocket_accept_key(sec_key);
    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
    );
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

pub async fn read_ws_text_or_binary(stream: &mut TcpStream) -> anyhow::Result<WsFrame> {
    let mut buf = vec![0u8; 65536];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            anyhow::bail!("websocket closed");
        }
        if let Ok((frame, _)) = parse_ws_frame(&buf[..n]) {
            return Ok(frame);
        }
    }
}

pub async fn write_ws_json(stream: &mut TcpStream, json: &str) -> anyhow::Result<()> {
    let frame = WsFrame {
        fin: true,
        opcode: WsOpcode::Text,
        masked: false,
        payload: json.as_bytes().to_vec(),
    };
    stream.write_all(&serialize_ws_frame(&frame)).await?;
    Ok(())
}

pub async fn write_ws_pong(stream: &mut TcpStream, payload: &[u8]) -> anyhow::Result<()> {
    let frame = WsFrame {
        fin: true,
        opcode: WsOpcode::Pong,
        masked: false,
        payload: payload.to_vec(),
    };
    stream.write_all(&serialize_ws_frame(&frame)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn websocket_accept_key_is_deterministic() {
        let key = websocket_accept_key("dGhlIHNhbXBsZSBub25jZQ==");
        assert_eq!(key, "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }
}
