//! SoupBinTCP session framing (OUCH/ITCH).
//!
//! Packet: `u16` big-endian length (type + payload) | ASCII type | payload.
//! This is the NASDAQ SoupBinTCP layout; FIG uses it as the session for
//! OUCH 4.2-shaped order entry and ITCH L2 egress ([ADR 0008](../../../docs/adr/0008-named-gateway-adapters.md)).

use thiserror::Error;

pub const LOGIN_REQUEST: u8 = b'L';
pub const LOGIN_ACCEPTED: u8 = b'A';
pub const LOGIN_REJECTED: u8 = b'J';
pub const SEQUENCED_DATA: u8 = b'S';
pub const UNSEQUENCED_DATA: u8 = b'U';
pub const SERVER_HEARTBEAT: u8 = b'H';
pub const CLIENT_HEARTBEAT: u8 = b'R';
pub const LOGOUT_REQUEST: u8 = b'O';
pub const END_OF_SESSION: u8 = b'Z';

const LOGIN_USER_LEN: usize = 6;
const LOGIN_PASS_LEN: usize = 10;
const LOGIN_SESSION_LEN: usize = 10;
const LOGIN_SEQ_LEN: usize = 20;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SoupError {
    #[error("truncated SoupBinTCP packet")]
    Truncated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoupPacket {
    pub packet_type: u8,
    pub payload: Vec<u8>,
}

pub fn encode_packet(packet_type: u8, payload: &[u8]) -> Vec<u8> {
    let len = 1 + payload.len();
    let mut buf = Vec::with_capacity(2 + len);
    buf.extend_from_slice(&(len as u16).to_be_bytes());
    buf.push(packet_type);
    buf.extend_from_slice(payload);
    buf
}

pub fn pad_ascii(s: &str, n: usize) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.truncate(n);
    v.resize(n, b' ');
    v
}

pub fn trim_ascii(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

pub fn encode_login_request(username: &str, password: &str, session: &str, seq: u64) -> Vec<u8> {
    let mut payload = Vec::with_capacity(46);
    payload.extend_from_slice(&pad_ascii(username, LOGIN_USER_LEN));
    payload.extend_from_slice(&pad_ascii(password, LOGIN_PASS_LEN));
    payload.extend_from_slice(&pad_ascii(session, LOGIN_SESSION_LEN));
    payload.extend_from_slice(&pad_ascii(&seq.to_string(), LOGIN_SEQ_LEN));
    encode_packet(LOGIN_REQUEST, &payload)
}

pub fn encode_login_accepted(session: &str, next_seq: u64) -> Vec<u8> {
    let mut payload = Vec::with_capacity(30);
    payload.extend_from_slice(&pad_ascii(session, LOGIN_SESSION_LEN));
    payload.extend_from_slice(format!("{next_seq:>20}").as_bytes());
    encode_packet(LOGIN_ACCEPTED, &payload)
}

pub fn encode_login_rejected(reason: u8) -> Vec<u8> {
    encode_packet(LOGIN_REJECTED, &[reason])
}

pub fn encode_sequenced(payload: &[u8]) -> Vec<u8> {
    encode_packet(SEQUENCED_DATA, payload)
}

pub fn encode_unsequenced(payload: &[u8]) -> Vec<u8> {
    encode_packet(UNSEQUENCED_DATA, payload)
}

pub fn encode_server_heartbeat() -> Vec<u8> {
    encode_packet(SERVER_HEARTBEAT, &[])
}

pub fn parse_login_request(payload: &[u8]) -> (String, String, String, u64) {
    let user = payload
        .get(..LOGIN_USER_LEN)
        .map(trim_ascii)
        .unwrap_or_default();
    let pass = payload
        .get(LOGIN_USER_LEN..LOGIN_USER_LEN + LOGIN_PASS_LEN)
        .map(trim_ascii)
        .unwrap_or_default();
    let session = payload
        .get(LOGIN_USER_LEN + LOGIN_PASS_LEN..LOGIN_USER_LEN + LOGIN_PASS_LEN + LOGIN_SESSION_LEN)
        .map(trim_ascii)
        .unwrap_or_default();
    let seq_off = LOGIN_USER_LEN + LOGIN_PASS_LEN + LOGIN_SESSION_LEN;
    let seq = payload
        .get(seq_off..seq_off + LOGIN_SEQ_LEN)
        .map(trim_ascii)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    (user, pass, session, seq)
}

/// Streaming SoupBinTCP decoder.
#[derive(Debug, Default)]
pub struct SoupDecoder {
    buf: Vec<u8>,
}

impl SoupDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    pub fn next_packet(&mut self) -> Option<Result<SoupPacket, SoupError>> {
        if self.buf.len() < 3 {
            return None;
        }
        let len = u16::from_be_bytes([self.buf[0], self.buf[1]]) as usize;
        if len == 0 {
            self.buf.drain(..2);
            return Some(Err(SoupError::Truncated));
        }
        if self.buf.len() < 2 + len {
            return None;
        }
        let packet_type = self.buf[2];
        let payload = self.buf[3..2 + len].to_vec();
        self.buf.drain(..2 + len);
        Some(Ok(SoupPacket {
            packet_type,
            payload,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_round_trip() {
        let wire = encode_login_request("alice", "secret", "SESSION", 7);
        let mut dec = SoupDecoder::new();
        dec.feed(&wire);
        let pkt = dec.next_packet().unwrap().unwrap();
        assert_eq!(pkt.packet_type, LOGIN_REQUEST);
        let (user, pass, session, seq) = parse_login_request(&pkt.payload);
        assert_eq!(user, "alice");
        assert_eq!(pass, "secret");
        assert_eq!(session, "SESSION");
        assert_eq!(seq, 7);
    }

    #[test]
    fn sequenced_payload_round_trip() {
        let wire = encode_sequenced(b"HELLO");
        let mut dec = SoupDecoder::new();
        dec.feed(&wire);
        let pkt = dec.next_packet().unwrap().unwrap();
        assert_eq!(pkt.packet_type, SEQUENCED_DATA);
        assert_eq!(pkt.payload, b"HELLO");
        assert!(dec.next_packet().is_none());
    }

    #[test]
    fn split_tcp_chunks() {
        let a = encode_unsequenced(b"AB");
        let b = encode_server_heartbeat();
        let mut all = a.clone();
        all.extend_from_slice(&b);
        let mut dec = SoupDecoder::new();
        dec.feed(&all[..3]);
        assert!(dec.next_packet().is_none());
        dec.feed(&all[3..]);
        assert_eq!(dec.next_packet().unwrap().unwrap().payload, b"AB");
        assert_eq!(
            dec.next_packet().unwrap().unwrap().packet_type,
            SERVER_HEARTBEAT
        );
    }

    #[test]
    fn login_rejected_round_trip() {
        let wire = encode_login_rejected(b'A');
        let mut dec = SoupDecoder::new();
        dec.feed(&wire);
        let pkt = dec.next_packet().unwrap().unwrap();
        assert_eq!(pkt.packet_type, LOGIN_REJECTED);
        assert_eq!(pkt.payload, b"A");
    }
}
