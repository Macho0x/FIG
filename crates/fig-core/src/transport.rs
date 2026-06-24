//! TREE transport layer for FIG.
//!
//! Wraps [quinn] (RFC 9000) as the TREE transport implementation, providing
//! certificate generation, ALPN negotiation, and a [`FigConnection`] that
//! maps FIG channels to TREE streams.
//!
//! # Security note
//!
//! The client configuration provided by [`client_config`] skips TLS
//! certificate verification. This is intended **for development only**.
//! Production deployments must use proper certificate chains and mTLS.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use quinn::crypto::rustls::QuicClientConfig as TreeClientConfig;
use quinn::crypto::rustls::QuicServerConfig as TreeServerConfig;
use quinn::{ClientConfig, Connection, Endpoint, ServerConfig};
use tokio::sync::Mutex;

use crate::channel::{ChannelDirection, ChannelManager, ChannelMode};
use crate::error::{ChannelError, FigError, FrameError};
use crate::ext::ExtensionTag;
use crate::frame::{Frame, FrameDecoder, MIN_FRAME_SIZE};
use crate::migration::{
    apply_migration, prepare_migration, reconstruct_channels, validate_migration, MigrationToken,
};
use crate::observability::{span_session_create, span_session_resume};
use crate::session::{MemorySessionStore, Session};

/// Per-channel TREE stream state including a persistent frame decoder.
struct ChannelStream {
    send: quinn::SendStream,
    recv: quinn::RecvStream,
    decoder: FrameDecoder,
}

impl ChannelStream {
    fn new(send: quinn::SendStream, recv: quinn::RecvStream) -> Self {
        Self {
            send,
            recv,
            decoder: FrameDecoder::new(),
        }
    }
}

// ─── Constants ───────────────────────────────────────────────────

/// ALPN identifier for FIG. Both client and server must advertise this.
pub const ALPN_FIG: &[u8] = b"fig/1";

// ─── Certificate Generation ──────────────────────────────────────

/// Generate a self-signed TLS certificate for local development.
///
/// The returned certificate and private key are suitable for use with
/// [`server_config`]. The certificate includes `localhost` in its
/// subject alternative names.
pub fn generate_self_signed_cert() -> Result<
    (
        rustls::pki_types::CertificateDer<'static>,
        rustls::pki_types::PrivateKeyDer<'static>,
    ),
    Box<dyn std::error::Error>,
> {
    let subject_alt_names = vec!["localhost".to_string()];

    let cert = rcgen::generate_simple_self_signed(subject_alt_names)?;

    let cert_der = cert.cert.der().clone();
    let key_der_vec = cert.key_pair.serialize_der();

    let priv_key = rustls::pki_types::PrivateKeyDer::Pkcs8(
        rustls::pki_types::PrivatePkcs8KeyDer::from(key_der_vec),
    );

    Ok((cert_der, priv_key))
}

/// Create a TREE server configuration with a self-signed certificate.
///
/// Sets ALPN to `"fig/1"`. Callers should use
/// [`generate_self_signed_cert`] to obtain the certificate and key,
/// or provide their own.
pub fn server_config(
    cert: rustls::pki_types::CertificateDer<'static>,
    key: rustls::pki_types::PrivateKeyDer<'static>,
) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;

    server_crypto.alpn_protocols = vec![ALPN_FIG.to_vec()];

    let server_config = TreeServerConfig::try_from(server_crypto)?;
    let mut transport = quinn::TransportConfig::default();
    transport.max_concurrent_bidi_streams(65535u32.into());

    let mut config = ServerConfig::with_crypto(Arc::new(server_config));
    config.transport_config(Arc::new(transport));

    Ok(config)
}

/// Create a TREE server configuration requiring mTLS client certificates.
///
/// **Development mode:** accepts any client certificate without CA validation.
/// Production deployments MUST use [`server_config_mtls_with_ca`] with a
/// proper client CA root store.
pub fn server_config_mtls(
    cert: rustls::pki_types::CertificateDer<'static>,
    key: rustls::pki_types::PrivateKeyDer<'static>,
) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_client_cert_verifier(Arc::new(NoClientVerification))
        .with_single_cert(vec![cert], key)?;

    server_crypto.alpn_protocols = vec![ALPN_FIG.to_vec()];

    let server_config = TreeServerConfig::try_from(server_crypto)?;
    let mut transport = quinn::TransportConfig::default();
    transport.max_concurrent_bidi_streams(65535u32.into());

    let mut config = ServerConfig::with_crypto(Arc::new(server_config));
    config.transport_config(Arc::new(transport));

    Ok(config)
}

/// Rotating TLS identity for runtime certificate reload (Spec §15).
#[derive(Debug)]
pub struct RotatingServerCerts {
    certs: std::sync::Mutex<(
        rustls::pki_types::CertificateDer<'static>,
        rustls::pki_types::PrivateKeyDer<'static>,
    )>,
}

impl RotatingServerCerts {
    /// Store an initial certificate and private key pair.
    pub fn new(
        cert: rustls::pki_types::CertificateDer<'static>,
        key: rustls::pki_types::PrivateKeyDer<'static>,
    ) -> Self {
        Self {
            certs: std::sync::Mutex::new((cert, key)),
        }
    }

    /// Replace the active certificate and key (hot reload).
    pub fn reload(
        &self,
        cert: rustls::pki_types::CertificateDer<'static>,
        key: rustls::pki_types::PrivateKeyDer<'static>,
    ) -> Result<(), FigError> {
        let mut guard = self
            .certs
            .lock()
            .map_err(|_| FigError::ConnectionFailed("cert lock poisoned".into()))?;
        *guard = (cert, key);
        Ok(())
    }

    /// Build a fresh server config from the current certificate.
    pub fn server_config(&self) -> Result<ServerConfig, FigError> {
        let guard = self
            .certs
            .lock()
            .map_err(|_| FigError::ConnectionFailed("cert lock poisoned".into()))?;
        server_config(guard.0.clone(), guard.1.clone_key())
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))
    }

    /// Build a fresh mTLS server config from the current certificate.
    pub fn server_config_mtls(&self) -> Result<ServerConfig, FigError> {
        let guard = self
            .certs
            .lock()
            .map_err(|_| FigError::ConnectionFailed("cert lock poisoned".into()))?;
        server_config_mtls(guard.0.clone(), guard.1.clone_key())
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))
    }
}

/// Create a TREE client configuration that **skips certificate verification**.
///
/// **Warning:** This accepts any server certificate without validation.
/// Intended for development and testing only. Production deployments
/// MUST replace this with proper certificate verification.
///
/// Sets ALPN to `"fig/1"`.
pub fn client_config() -> Result<ClientConfig, Box<dyn std::error::Error>> {
    let mut client_crypto = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoServerVerification))
        .with_no_client_auth();

    client_crypto.alpn_protocols = vec![ALPN_FIG.to_vec()];

    let client_config = TreeClientConfig::try_from(client_crypto)?;
    let mut transport = quinn::TransportConfig::default();
    transport.max_concurrent_bidi_streams(65535u32.into());

    let mut config = ClientConfig::new(Arc::new(client_config));
    config.transport_config(Arc::new(transport));

    Ok(config)
}

// ─── Certificate Verification (dev only) ─────────────────────────

/// A certificate verifier that accepts any server certificate.
///
/// **Do not use in production.**
#[derive(Debug)]
struct NoServerVerification;

impl rustls::client::danger::ServerCertVerifier for NoServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

/// A client certificate verifier that accepts any client certificate.
///
/// **Do not use in production.**
#[derive(Debug)]
struct NoClientVerification;

impl rustls::server::danger::ClientCertVerifier for NoClientVerification {
    fn verify_client_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::server::danger::ClientCertVerified, rustls::Error> {
        Ok(rustls::server::danger::ClientCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ED25519,
        ]
    }

    fn root_hint_subjects(&self) -> &[rustls::DistinguishedName] {
        &[]
    }
}

// ─── FigConnection ──────────────────────────────────────────────

/// A FIG connection wrapping a TREE connection.
///
/// Manages channels, session state, and frame send/receive over the
/// underlying TREE transport. Persists TREE stream handles per channel
/// for reuse across multiple frame send/receive operations.
///
/// # Stream reuse
///
/// Each channel gets one bidirectional TREE stream. The `streams` map
/// stores `(SendStream, RecvStream)` keyed by channel ID. On first use
/// of a channel (send or receive), a new bidirectional stream is opened
/// and stored. Subsequent operations reuse the existing stream.
///
/// # Production notes
///
/// This is reference-quality. Production code should:
/// - Implement proper flow control and backpressure.
/// - Handle bidirectional streams for server-initiated messages.
pub struct FigConnection {
    conn: Connection,
    channels: Arc<Mutex<ChannelManager>>,
    session_store: MemorySessionStore,
    /// Persistent TREE bidirectional streams per channel.
    streams: Mutex<HashMap<u16, ChannelStream>>,
}

impl FigConnection {
    /// Wrap an established TREE connection.
    pub fn from_tree(conn: Connection, is_server: bool) -> Self {
        Self {
            conn,
            channels: Arc::new(Mutex::new(ChannelManager::new(is_server))),
            session_store: MemorySessionStore::new(),
            streams: Mutex::new(HashMap::new()),
        }
    }

    /// Reconstruct channel state from a restored session after 0-RTT resumption.
    ///
    /// Rebuilds the channel manager and opens TREE streams for each channel.
    pub async fn reconstruct_from_session(&self, session: &Session) -> Result<(), FigError> {
        // Rebuild the channel manager from the session's channel list.
        {
            let new_mgr = ChannelManager::reconstruct(session, false);
            let mut mgr = self.channels.lock().await;
            *mgr = new_mgr;
        }

        // Reopen TREE streams for each channel in the session.
        for &channel_id in &session.channels {
            let (send, recv) = self
                .conn
                .open_bi()
                .await
                .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
            self.streams
                .lock()
                .await
                .insert(channel_id, ChannelStream::new(send, recv));
        }

        tracing::info!(
            session_id = %session.session_id,
            "session resumed with {} channels",
            session.channels.len()
        );

        Ok(())
    }

    /// Capture migratable state before a TREE connection migration.
    pub async fn prepare_migration(&self, session: &Session) -> MigrationToken {
        let mgr = self.channels.lock().await;
        prepare_migration(session, &mgr)
    }

    /// Apply migration state after reconnecting on a new network path.
    pub async fn apply_migration(
        &self,
        token: &MigrationToken,
        session: &mut Session,
        is_server: bool,
    ) -> Result<(), FigError> {
        validate_migration(token, &session.session_id)?;
        apply_migration(token, session).map_err(|e| FigError::HandshakeFailed(e.to_string()))?;

        {
            let mut mgr = self.channels.lock().await;
            *mgr = reconstruct_channels(token, is_server);
        }
        self.streams.lock().await.clear();

        for &channel_id in &session.channels {
            let (send, recv) = self
                .conn
                .open_bi()
                .await
                .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
            self.streams
                .lock()
                .await
                .insert(channel_id, ChannelStream::new(send, recv));
        }

        tracing::info!(
            session_id = %session.session_id,
            channels = session.channels.len(),
            "connection migration applied"
        );
        Ok(())
    }

    /// Open a unidirectional channel with a dedicated TREE stream.
    pub async fn open_unidirectional_channel(
        &self,
        mode: ChannelMode,
        direction: ChannelDirection,
        schema_id: Option<u8>,
    ) -> Result<u16, ChannelError> {
        let channel_id = {
            let mut mgr = self.channels.lock().await;
            mgr.open_unidirectional_channel(mode, direction, schema_id)?
        };

        match self.conn.open_bi().await {
            Ok((send, recv)) => {
                self.streams
                    .lock()
                    .await
                    .insert(channel_id, ChannelStream::new(send, recv));
                Ok(channel_id)
            }
            Err(e) => {
                let mut mgr = self.channels.lock().await;
                let _ = mgr.force_close_channel(channel_id);
                Err(ChannelError::StreamError(channel_id, e.to_string()))
            }
        }
    }

    /// Open a new channel.
    ///
    /// Allocates a channel ID, opens a TREE bidirectional stream, and
    /// stores the stream handles for subsequent send/recv operations.
    /// Returns the assigned channel ID.
    pub async fn open_channel(
        &self,
        mode: ChannelMode,
        schema_id: Option<u8>,
    ) -> Result<u16, ChannelError> {
        let channel_id = {
            let mut mgr = self.channels.lock().await;
            mgr.open_channel(mode, schema_id)?
        };

        // Open a TREE bidirectional stream for this channel.
        match self.conn.open_bi().await {
            Ok((send, recv)) => {
                self.streams
                    .lock()
                    .await
                    .insert(channel_id, ChannelStream::new(send, recv));
                Ok(channel_id)
            }
            Err(e) => {
                // Roll back the channel on failure
                let mut mgr = self.channels.lock().await;
                let _ = mgr.force_close_channel(channel_id);
                Err(ChannelError::StreamError(channel_id, e.to_string()))
            }
        }
    }

    /// Send a frame on a channel.
    ///
    /// Reuses the persistent `SendStream` for this channel. Opens a new
    /// bidirectional stream on first use and stores it in the stream map.
    ///
    /// TODO: Enforce flow-control credits before sending. The ChannelManager
    /// has `has_credits()` / `consume_credit()`, but they are not yet wired
    /// here. Sending currently ignores credit exhaustion.
    pub async fn send_frame(&self, channel_id: u16, frame: &Frame) -> Result<(), FrameError> {
        let data = frame.encode()?;

        // Get or create the SendStream for this channel.
        let mut streams = self.streams.lock().await;
        if let std::collections::hash_map::Entry::Vacant(entry) = streams.entry(channel_id) {
            let (send, recv) = self
                .conn
                .open_bi()
                .await
                .map_err(|e| FrameError::IoError(std::io::Error::other(e.to_string())))?;
            entry.insert(ChannelStream::new(send, recv));
        }
        let ch = streams.get_mut(&channel_id).unwrap();

        match ch.send.write_all(&data).await {
            Ok(()) => {}
            Err(quinn::WriteError::Stopped(error_code)) => {
                // Stream stopped by peer — mark channel as errored.
                let code: u64 = error_code.into();
                return Err(FrameError::IoError(std::io::Error::new(
                    std::io::ErrorKind::ConnectionReset,
                    format!(
                        "stream stopped on channel {}: error_code={}",
                        channel_id, code
                    ),
                )));
            }
            Err(e) => {
                return Err(FrameError::IoError(std::io::Error::other(e.to_string())));
            }
        }

        // Do NOT call finish() — keep the stream open for subsequent frames.

        // Advance the channel's send sequence number.
        {
            let mut mgr = self.channels.lock().await;
            let _ = mgr.next_send_seq(channel_id);
        }

        Ok(())
    }

    /// Receive a frame from a specific channel's persistent RecvStream.
    ///
    /// **Client-only.** This method assumes the caller knows which channel
    /// a stream belongs to. On the server side, use `accept_frame()` instead,
    /// which reads the first frame to determine the actual channel ID.
    /// Calling `recv_frame` on the server may silently map an incoming stream
    /// to the wrong channel.
    ///
    /// If no stream exists for this channel yet, accepts a new incoming
    /// bidirectional stream and maps it to the given channel.
    pub async fn recv_frame(&self, channel_id: u16) -> Result<Frame, FrameError> {
        // Get or create the RecvStream for this channel.
        let mut streams = self.streams.lock().await;
        if let std::collections::hash_map::Entry::Vacant(entry) = streams.entry(channel_id) {
            let (send, recv) = self.conn.accept_bi().await.map_err(|e| {
                FrameError::IoError(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    e.to_string(),
                ))
            })?;
            entry.insert(ChannelStream::new(send, recv));
        }
        let ch = streams.get_mut(&channel_id).unwrap();

        let mut buf = vec![0u8; 65536];

        loop {
            if let Some(result) = ch.decoder.decode_next() {
                return result;
            }
            match ch.recv.read(&mut buf).await {
                Ok(Some(n)) => {
                    ch.decoder.feed(&buf[..n]);
                }
                Ok(None) => {
                    if let Some(result) = ch.decoder.decode_next() {
                        return result;
                    }
                    return Err(FrameError::BufferTooShort {
                        expected: MIN_FRAME_SIZE,
                        actual: ch.decoder.buffered_len(),
                    });
                }
                Err(quinn::ReadError::Reset(error_code)) => {
                    let code: u64 = error_code.into();
                    return Err(FrameError::IoError(std::io::Error::new(
                        std::io::ErrorKind::ConnectionReset,
                        format!(
                            "stream reset on channel {}: error_code={}",
                            channel_id, code
                        ),
                    )));
                }
                Err(e) => {
                    return Err(FrameError::IoError(std::io::Error::other(e.to_string())));
                }
            }
        }
    }

    /// Send one frame on a fresh bidirectional stream and read all response frames.
    ///
    /// Used by gateway proxies: half-closes the send side after the request so the
    /// server can flush responses and close the stream.
    pub async fn request_and_recv_all(&self, frame: Frame) -> Result<Vec<Frame>, FrameError> {
        let (mut send, mut recv) = self
            .conn
            .open_bi()
            .await
            .map_err(|e| FrameError::IoError(std::io::Error::other(e.to_string())))?;
        let data = frame.encode()?;
        send.write_all(&data)
            .await
            .map_err(|e| FrameError::IoError(std::io::Error::other(e.to_string())))?;
        send.finish()
            .map_err(|e| FrameError::IoError(std::io::Error::other(e.to_string())))?;

        let mut decoder = FrameDecoder::new();
        let mut buf = vec![0u8; 65536];
        let mut frames = Vec::new();
        loop {
            match recv.read(&mut buf).await {
                Ok(Some(n)) => {
                    decoder.feed(&buf[..n]);
                    while let Some(result) = decoder.decode_next() {
                        frames.push(result?);
                    }
                }
                Ok(None) => {
                    while let Some(result) = decoder.decode_next() {
                        frames.push(result?);
                    }
                    break;
                }
                Err(e) => {
                    return Err(FrameError::IoError(std::io::Error::other(e.to_string())));
                }
            }
        }
        Ok(frames)
    }

    /// Accept an incoming bidirectional stream, read the first frame to determine
    /// the channel ID, store the stream mapping, and return the frame.
    ///
    /// This is the server-side entry point for receiving frames on new channels.
    pub async fn accept_frame(&self) -> Result<(u16, Frame), FrameError> {
        let (send, mut recv) = self.conn.accept_bi().await.map_err(|e| {
            FrameError::IoError(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                e.to_string(),
            ))
        })?;

        // Read the first frame to determine the channel ID.
        let mut decoder = FrameDecoder::new();
        let mut buf = vec![0u8; 65536];

        let frame = loop {
            match recv.read(&mut buf).await {
                Ok(Some(n)) => {
                    decoder.feed(&buf[..n]);
                    if let Some(result) = decoder.decode_next() {
                        break result?;
                    }
                }
                Ok(None) => {
                    if let Some(result) = decoder.decode_next() {
                        break result?;
                    }
                    return Err(FrameError::BufferTooShort {
                        expected: MIN_FRAME_SIZE,
                        actual: decoder.buffered_len(),
                    });
                }
                Err(quinn::ReadError::Reset(error_code)) => {
                    let code: u64 = error_code.into();
                    return Err(FrameError::IoError(std::io::Error::new(
                        std::io::ErrorKind::ConnectionReset,
                        format!("stream reset: error_code={}", code),
                    )));
                }
                Err(e) => {
                    return Err(FrameError::IoError(std::io::Error::other(e.to_string())));
                }
            }
        };

        let channel_id = frame.channel_id;

        let mut ch = ChannelStream::new(send, recv);
        ch.decoder = decoder;
        self.streams.lock().await.insert(channel_id, ch);

        Ok((channel_id, frame))
    }

    /// On 0-RTT reconnect, read the first application frame (if any) and restore session state.
    pub async fn try_restore_session_from_first_frame(&self) -> Option<Session> {
        let result = tokio::time::timeout(Duration::from_millis(250), self.accept_frame()).await;
        let Ok(Ok((_, frame))) = result else {
            return None;
        };

        let token = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::RedirectToken)
            .map(|e| e.value.as_bytes())
            .filter(|b| !b.is_empty())
            .or_else(|| {
                if frame.payload.is_empty() {
                    None
                } else {
                    Some(frame.payload.clone())
                }
            })?;

        crate::replay::validate_resumption_token(&token).ok()?;
        Session::from_resumption_token(&token).ok()
    }

    /// Drain additional frames already buffered on a channel decoder.
    pub async fn recv_buffered_frames(&self, channel_id: u16) -> Vec<Frame> {
        let mut out = Vec::new();
        let mut streams = self.streams.lock().await;
        if let Some(ch) = streams.get_mut(&channel_id) {
            while let Some(Ok(frame)) = ch.decoder.decode_next() {
                out.push(frame);
            }
        }
        out
    }

    /// Close the TREE stream for a channel and remove it from the stream map.
    ///
    /// Finishes the send stream and cleans up the HashMap entry.
    pub async fn close_channel_stream(&self, channel_id: u16) -> Result<(), ChannelError> {
        let mut streams = self.streams.lock().await;
        if let Some(mut ch) = streams.remove(&channel_id) {
            let _ = ch.send.finish();
        }
        Ok(())
    }

    /// Close a channel gracefully.
    pub async fn close_channel(&self, channel_id: u16) -> Result<(), ChannelError> {
        let mut mgr = self.channels.lock().await;
        mgr.close_channel(channel_id)
    }

    /// Permanently close and remove a channel (used after stream reset).
    pub async fn force_close_channel(&self, channel_id: u16) -> Result<(), ChannelError> {
        let mut mgr = self.channels.lock().await;
        mgr.force_close_channel(channel_id)
    }

    /// Close the entire connection with an error code and reason.
    pub fn close(&self, code: u32, reason: &[u8]) {
        self.conn.close(quinn::VarInt::from_u32(code), reason);
    }

    /// Access the underlying TREE connection.
    pub fn inner(&self) -> &Connection {
        &self.conn
    }

    /// Access the session store.
    pub fn session_store(&self) -> &MemorySessionStore {
        &self.session_store
    }

    /// Access the session store mutably (for wiring into server persistence).
    pub fn session_store_mut(&mut self) -> &mut MemorySessionStore {
        &mut self.session_store
    }
}

// ─── FigServer ──────────────────────────────────────────────────

/// A FIG server endpoint.
///
/// Wraps a TREE [`Endpoint`] for accepting FIG connections. Supports
/// 0-RTT session resumption via [`FigServer::accept_0rtt`].
pub struct FigServer {
    endpoint: Endpoint,
    session_store: MemorySessionStore,
}

impl FigServer {
    /// Bind a new FIG server to the given address.
    pub async fn bind(addr: SocketAddr, server_config: ServerConfig) -> Result<Self, FigError> {
        let endpoint = Endpoint::server(server_config, addr)
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        Ok(Self {
            endpoint,
            session_store: MemorySessionStore::new(),
        })
    }

    /// Accept a normal (full handshake) FIG connection.
    pub async fn accept(&self) -> Result<FigConnection, FigError> {
        let incoming = self
            .endpoint
            .accept()
            .await
            .ok_or_else(|| FigError::ConnectionFailed("endpoint closed".into()))?;
        let conn = incoming
            .await
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        Ok(FigConnection::from_tree(conn, true))
    }

    /// Accept a 0-RTT connection and validate a resumption token.
    ///
    /// If the client presents a valid 0-RTT token, the session is restored
    /// from the session store and the connection proceeds with 0-RTT data.
    /// If 0-RTT is rejected by the transport layer, falls back to a full
    /// handshake.
    ///
    /// Call [`validate_resumption_token`] on any resumption token bytes before
    /// restoring session state (SPEC §10.1).
    pub async fn accept_0rtt(&self) -> Result<(FigConnection, Option<Session>), FigError> {
        let incoming = self
            .endpoint
            .accept()
            .await
            .ok_or_else(|| FigError::ConnectionFailed("endpoint closed".into()))?;

        let conn = incoming
            .await
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;

        let fig_conn = FigConnection::from_tree(conn, true);

        let restored_session = fig_conn.try_restore_session_from_first_frame().await;
        if let Some(ref session) = restored_session {
            let span = span_session_resume(&session.session_id.to_string());
            let _guard = span.enter();
            tracing::info!(
                session_id = %session.session_id,
                "session restored from 0-RTT resumption token"
            );
            let _ = fig_conn.reconstruct_from_session(session).await;
        }

        Ok((fig_conn, restored_session))
    }

    /// Return the local address the server is bound to.
    pub fn local_addr(&self) -> Result<SocketAddr, FigError> {
        self.endpoint
            .local_addr()
            .map_err(|e| FigError::IoError(std::io::Error::other(e.to_string())))
    }

    /// Access the session store.
    pub fn session_store(&self) -> &MemorySessionStore {
        &self.session_store
    }

    /// Access the underlying TREE endpoint.
    pub fn inner(&self) -> &Endpoint {
        &self.endpoint
    }
}

// ─── FigClient ──────────────────────────────────────────────────

/// A FIG client connection manager.
///
/// Wraps a TREE [`Endpoint`] for initiating FIG connections. Supports
/// 0-RTT session resumption via [`FigClient::connect_0rtt`].
pub struct FigClient {
    endpoint: Endpoint,
    client_config: ClientConfig,
}

impl FigClient {
    /// Create a new FIG client.
    pub fn new(client_config: ClientConfig) -> Result<Self, FigError> {
        let endpoint = Endpoint::client(
            "0.0.0.0:0"
                .parse::<SocketAddr>()
                .map_err(|e| FigError::IoError(std::io::Error::other(e.to_string())))?,
        )
        .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        Ok(Self {
            endpoint,
            client_config,
        })
    }

    /// Connect to a FIG server with a full handshake.
    pub async fn connect(
        &self,
        server_addr: SocketAddr,
        server_name: &str,
    ) -> Result<FigConnection, FigError> {
        let conn = self
            .endpoint
            .connect_with(self.client_config.clone(), server_addr, server_name)
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?
            .await
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;

        let span = span_session_create("");
        let _guard = span.enter();

        Ok(FigConnection::from_tree(conn, false))
    }

    /// Connect to a FIG server attempting 0-RTT resumption.
    ///
    /// Uses [`quinn::Connecting::into_0rtt`] to send data on the first
    /// flight. If the server rejects 0-RTT, falls back to a full handshake.
    ///
    /// Returns the [`FigConnection`] and an optional restored [`Session`]
    /// if the server accepted 0-RTT.
    ///
    /// Rejects duplicate resumption tokens within [`DEFAULT_REPLAY_WINDOW`]
    /// (SPEC §10.1).
    pub async fn connect_0rtt(
        &self,
        server_addr: SocketAddr,
        server_name: &str,
        resumption_token: Option<&[u8]>,
    ) -> Result<(FigConnection, Option<Session>), FigError> {
        let connecting = self
            .endpoint
            .connect_with(self.client_config.clone(), server_addr, server_name)
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;

        let conn = match connecting.into_0rtt() {
            Ok((conn, _zero_rtt)) => {
                tracing::info!("0-RTT connection accepted");

                let restored_session = if let Some(token) = resumption_token {
                    crate::replay::validate_resumption_token(token)?;
                    Some(
                        Session::from_resumption_token(token)
                            .map_err(|e| FigError::HandshakeFailed(e.to_string()))?,
                    )
                } else {
                    None
                };

                let fig_conn = FigConnection::from_tree(conn, false);

                if let Some(ref session) = restored_session {
                    let span = span_session_resume(&session.session_id.to_string());
                    let _guard = span.enter();
                    tracing::info!(
                        session_id = %session.session_id,
                        "session resumed via 0-RTT"
                    );
                    fig_conn.reconstruct_from_session(session).await?;
                }

                return Ok((fig_conn, restored_session));
            }
            Err(connecting) => {
                // 0-RTT rejected — fall back to full handshake.
                tracing::info!("0-RTT rejected, falling back to full handshake");
                connecting
                    .await
                    .map_err(|e| FigError::ConnectionFailed(e.to_string()))?
            }
        };

        let span = span_session_create("");
        let _guard = span.enter();

        Ok((FigConnection::from_tree(conn, false), None))
    }

    /// Access the underlying TREE endpoint.
    pub fn inner(&self) -> &Endpoint {
        &self.endpoint
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_self_signed_cert() {
        let (cert, key) = generate_self_signed_cert().expect("cert generation should succeed");
        // Certificate DER should not be empty.
        assert!(!cert.as_ref().is_empty());
        // Private key should not be empty.
        match &key {
            rustls::pki_types::PrivateKeyDer::Pkcs8(pkcs8) => {
                assert!(!pkcs8.secret_pkcs8_der().is_empty());
            }
            _ => panic!("expected Pkcs8 private key"),
        }
    }

    #[test]
    fn test_server_config_creation() {
        let (cert, key) = generate_self_signed_cert().expect("cert generation should succeed");
        let _config = server_config(cert, key).expect("server config creation should succeed");
        // Config created successfully — transport config was set to allow
        // up to 65535 concurrent bidirectional streams.
    }

    #[test]
    fn test_client_config_creation() {
        let _config = client_config().expect("client config creation should succeed");
        // Config created successfully with dev-mode cert verification skipped.
    }

    #[test]
    fn test_cert_generation_multiple_certs_differ() {
        let (cert1, key1) = generate_self_signed_cert().unwrap();
        let (cert2, key2) = generate_self_signed_cert().unwrap();

        // Each generation should produce different certs and keys.
        assert_ne!(cert1.as_ref(), cert2.as_ref());
        let pk1 = match &key1 {
            rustls::pki_types::PrivateKeyDer::Pkcs8(p) => p.secret_pkcs8_der().to_vec(),
            _ => vec![],
        };
        let pk2 = match &key2 {
            rustls::pki_types::PrivateKeyDer::Pkcs8(p) => p.secret_pkcs8_der().to_vec(),
            _ => vec![],
        };
        assert_ne!(pk1, pk2);
    }

    #[test]
    fn test_server_config_mtls_creation() {
        let (cert, key) = generate_self_signed_cert().unwrap();
        let _config = server_config_mtls(cert, key).expect("mTLS server config should succeed");
    }

    #[test]
    fn test_rotating_server_certs_reload() {
        let (cert1, key1) = generate_self_signed_cert().unwrap();
        let rotating = RotatingServerCerts::new(cert1, key1);
        let _cfg1 = rotating.server_config().unwrap();

        let (cert2, key2) = generate_self_signed_cert().unwrap();
        rotating.reload(cert2, key2).unwrap();
        let _cfg2 = rotating.server_config_mtls().unwrap();
    }

    #[tokio::test]
    async fn test_tree_ping_pong_round_trip() {
        let (cert, key) = generate_self_signed_cert().unwrap();
        let server_cfg = server_config(cert, key).unwrap();
        let client_cfg = client_config().unwrap();

        let server = FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg)
            .await
            .unwrap();
        let addr = server.local_addr().unwrap();

        let client_handle = tokio::spawn(async move {
            let client = FigClient::new(client_cfg).unwrap();
            let conn = client.connect(addr, "localhost").await.unwrap();
            conn.send_frame(0, &Frame::ping()).await.unwrap();
            let pong = conn.recv_frame(0).await.unwrap();
            assert_eq!(
                pong.control_subtype(),
                Some(crate::frame::ControlSubtype::Pong)
            );
        });

        let conn = server.accept().await.unwrap();
        let (ch, frame) = conn.accept_frame().await.unwrap();
        assert_eq!(
            frame.control_subtype(),
            Some(crate::frame::ControlSubtype::Ping)
        );
        conn.send_frame(ch, &Frame::pong()).await.unwrap();
        client_handle.await.unwrap();
    }
}
