//! TCP downgrade transport for FIG.
//!
//! Spec §2.1: legacy environments without TREE use plain TCP with a
//! `FIG\x01` magic prefix for protocol detection. All channels are
//! serialized over a single TCP stream (multiplexing via channel IDs
//! in frame headers).

use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::channel::{ChannelManager, ChannelMode};
use crate::error::{ChannelError, FigError, FrameError};
use crate::frame::{Frame, FrameDecoder, MIN_FRAME_SIZE};
use crate::session::MemorySessionStore;

// ─── Constants ───────────────────────────────────────────────────

/// Magic prefix sent as the first bytes on a TCP downgrade connection.
pub const FIG_TCP_MAGIC: &[u8] = b"FIG\x01";

/// Length of the TCP magic prefix in bytes.
pub const FIG_TCP_MAGIC_LEN: usize = FIG_TCP_MAGIC.len();

// ─── FigTcpConnection ────────────────────────────────────────────

/// A FIG connection over plain TCP (downgrade mode).
///
/// Unlike [`crate::transport::FigConnection`], all frames share one TCP
/// stream. Channel multiplexing is handled via the channel ID field in
/// each frame header.
pub struct FigTcpConnection {
    stream: TcpStream,
    channels: ChannelManager,
    decoder: FrameDecoder,
    session_store: MemorySessionStore,
    read_buf: Vec<u8>,
}

impl FigTcpConnection {
    fn new(stream: TcpStream, is_server: bool) -> Self {
        Self {
            stream,
            channels: ChannelManager::new(is_server),
            decoder: FrameDecoder::new(),
            session_store: MemorySessionStore::new(),
            read_buf: vec![0u8; 65536],
        }
    }

    /// Connect to a FIG server over TCP downgrade mode.
    ///
    /// Sends the magic prefix immediately after the TCP handshake.
    pub async fn connect(addr: SocketAddr) -> Result<Self, FigError> {
        let mut stream = TcpStream::connect(addr)
            .await
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        stream
            .write_all(FIG_TCP_MAGIC)
            .await
            .map_err(FigError::IoError)?;
        Ok(Self::new(stream, false))
    }

    /// Accept an incoming TCP connection after validating the magic prefix.
    pub async fn from_accepted(mut stream: TcpStream) -> Result<Self, FigError> {
        let mut magic = [0u8; FIG_TCP_MAGIC_LEN];
        stream
            .read_exact(&mut magic)
            .await
            .map_err(|e| FigError::HandshakeFailed(e.to_string()))?;
        if magic != *FIG_TCP_MAGIC {
            return Err(FigError::HandshakeFailed(format!(
                "invalid TCP magic prefix: expected {:?}, got {:?}",
                FIG_TCP_MAGIC, magic
            )));
        }
        Ok(Self::new(stream, true))
    }

    /// Open a new logical channel (no separate TCP stream).
    pub fn open_channel(
        &mut self,
        mode: ChannelMode,
        schema_id: Option<u8>,
    ) -> Result<u16, ChannelError> {
        self.channels.open_channel(mode, schema_id)
    }

    /// Send a frame on the shared TCP stream.
    pub async fn send_frame(&mut self, frame: &Frame) -> Result<(), FrameError> {
        let data = frame.encode()?;
        self.stream
            .write_all(&data)
            .await
            .map_err(FrameError::IoError)?;

        let _ = self.channels.next_send_seq(frame.channel_id);
        Ok(())
    }

    /// Receive the next frame from the shared TCP stream.
    pub async fn recv_frame(&mut self) -> Result<Frame, FrameError> {
        loop {
            if let Some(result) = self.decoder.decode_next() {
                return result;
            }

            let n = self
                .stream
                .read(&mut self.read_buf)
                .await
                .map_err(FrameError::IoError)?;
            if n == 0 {
                if let Some(result) = self.decoder.decode_next() {
                    return result;
                }
                return Err(FrameError::BufferTooShort {
                    expected: MIN_FRAME_SIZE,
                    actual: self.decoder.buffered_len(),
                });
            }
            self.decoder.feed(&self.read_buf[..n]);
        }
    }

    /// Close a channel gracefully.
    pub fn close_channel(&mut self, channel_id: u16) -> Result<(), ChannelError> {
        self.channels.close_channel(channel_id)
    }

    /// Permanently close and remove a channel.
    pub fn force_close_channel(&mut self, channel_id: u16) -> Result<(), ChannelError> {
        self.channels.force_close_channel(channel_id)
    }

    /// Access the session store.
    pub fn session_store(&self) -> &MemorySessionStore {
        &self.session_store
    }

    /// Access the session store mutably.
    pub fn session_store_mut(&mut self) -> &mut MemorySessionStore {
        &mut self.session_store
    }

    /// Access the underlying TCP stream.
    pub fn inner(&self) -> &TcpStream {
        &self.stream
    }
}

// ─── FigTcpServer ────────────────────────────────────────────────

/// A FIG server listening for TCP downgrade connections.
pub struct FigTcpServer {
    listener: TcpListener,
}

impl FigTcpServer {
    /// Bind to the given address.
    pub async fn bind(addr: SocketAddr) -> Result<Self, FigError> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        Ok(Self { listener })
    }

    /// Accept the next TCP downgrade connection.
    pub async fn accept(&self) -> Result<FigTcpConnection, FigError> {
        let (stream, _) = self
            .listener
            .accept()
            .await
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        FigTcpConnection::from_accepted(stream).await
    }

    /// Return the local address the server is bound to.
    pub fn local_addr(&self) -> Result<SocketAddr, FigError> {
        self.listener
            .local_addr()
            .map_err(|e| FigError::IoError(std::io::Error::other(e.to_string())))
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{FrameType, CONTROL_CHANNEL};

    #[tokio::test]
    async fn test_tcp_round_trip_single_frame() {
        let server = FigTcpServer::bind("127.0.0.1:0".parse().unwrap())
            .await
            .unwrap();
        let addr = server.local_addr().unwrap();

        let server_handle = tokio::spawn(async move {
            let mut conn = server.accept().await.unwrap();
            let frame = conn.recv_frame().await.unwrap();
            assert_eq!(frame.frame_type, FrameType::Control);
            assert_eq!(frame.control_subtype(), Some(crate::frame::ControlSubtype::Ping));
            assert_eq!(frame.channel_id, CONTROL_CHANNEL);
            conn.send_frame(&Frame::pong()).await.unwrap();
        });

        let mut client = FigTcpConnection::connect(addr).await.unwrap();
        client.send_frame(&Frame::ping()).await.unwrap();
        let pong = client.recv_frame().await.unwrap();
        assert_eq!(pong.control_subtype(), Some(crate::frame::ControlSubtype::Pong));

        server_handle.await.unwrap();
    }

    #[tokio::test]
    async fn test_tcp_multiplexed_channels() {
        let server = FigTcpServer::bind("127.0.0.1:0".parse().unwrap())
            .await
            .unwrap();
        let addr = server.local_addr().unwrap();

        let server_handle = tokio::spawn(async move {
            let mut conn = server.accept().await.unwrap();
            let ch1 = conn.open_channel(ChannelMode::Stateless, None).unwrap();
            let ch2 = conn.open_channel(ChannelMode::Stateless, None).unwrap();

            let f1 = conn.recv_frame().await.unwrap();
            let f2 = conn.recv_frame().await.unwrap();
            assert_eq!(f1.channel_id, ch1);
            assert_eq!(f2.channel_id, ch2);

            conn.send_frame(
                &Frame::new(FrameType::Response, ch1)
                    .with_payload(b"ch1-response".to_vec()),
            )
            .await
            .unwrap();
            conn.send_frame(
                &Frame::new(FrameType::Response, ch2)
                    .with_payload(b"ch2-response".to_vec()),
            )
            .await
            .unwrap();
        });

        let mut client = FigTcpConnection::connect(addr).await.unwrap();
        let ch1 = client.open_channel(ChannelMode::Stateless, None).unwrap();
        let ch2 = client.open_channel(ChannelMode::Stateless, None).unwrap();

        client
            .send_frame(
                &Frame::new(FrameType::Request, ch1).with_payload(b"ch1-request".to_vec()),
            )
            .await
            .unwrap();
        client
            .send_frame(
                &Frame::new(FrameType::Request, ch2).with_payload(b"ch2-request".to_vec()),
            )
            .await
            .unwrap();

        let r1 = client.recv_frame().await.unwrap();
        let r2 = client.recv_frame().await.unwrap();
        assert_eq!(r1.channel_id, ch1);
        assert_eq!(r2.channel_id, ch2);
        assert_eq!(r1.payload, b"ch1-response");
        assert_eq!(r2.payload, b"ch2-response");

        server_handle.await.unwrap();
    }

    #[tokio::test]
    async fn test_tcp_rejects_invalid_magic_prefix() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let result = FigTcpConnection::from_accepted(stream).await;
            assert!(result.is_err());
        });

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(b"BAD\x01").await.unwrap();

        server_handle.await.unwrap();
    }

    #[tokio::test]
    async fn test_tcp_partial_frame_reads() {
        let server = FigTcpServer::bind("127.0.0.1:0".parse().unwrap())
            .await
            .unwrap();
        let addr = server.local_addr().unwrap();

        let server_handle = tokio::spawn(async move {
            let mut conn = server.accept().await.unwrap();
            let frame = conn.recv_frame().await.unwrap();
            assert_eq!(frame.payload, b"partial-read-test");
        });

        let mut client = FigTcpConnection::connect(addr).await.unwrap();
        let frame = Frame::new(FrameType::Request, 1).with_payload(b"partial-read-test".to_vec());
        let encoded = frame.encode().unwrap();

        // Send one byte at a time to exercise FrameDecoder buffering.
        for byte in encoded {
            client.stream.write_all(&[byte]).await.unwrap();
        }
        client.stream.flush().await.unwrap();

        server_handle.await.unwrap();
    }

    #[test]
    fn test_tcp_magic_constant() {
        assert_eq!(FIG_TCP_MAGIC, b"FIG\x01");
        assert_eq!(FIG_TCP_MAGIC_LEN, 4);
    }
}
