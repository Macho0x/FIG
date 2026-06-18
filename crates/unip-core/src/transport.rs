//! QUIC transport layer for UNIP.
//!
//! Wraps [quinn] for QUIC transport, providing certificate generation,
//! ALPN negotiation, and a [`UnipConnection`] that maps UNIP channels
//! to QUIC streams.
//!
//! # Security note
//!
//! The client configuration provided by [`client_config`] skips TLS
//! certificate verification. This is intended **for development only**.
//! Production deployments must use proper certificate chains and mTLS.

use std::sync::Arc;

use quinn::{ClientConfig, Connection, ServerConfig};
use tokio::sync::Mutex;

use crate::channel::{ChannelManager, ChannelMode};
use crate::error::{ChannelError, FrameError};
use crate::frame::{Frame, FrameDecoder, MIN_FRAME_SIZE};
use crate::session::MemorySessionStore;

// ─── Constants ───────────────────────────────────────────────────

/// ALPN identifier for UNIP. Both client and server must advertise this.
pub const ALPN_UNIP: &[u8] = b"unip/1";

// ─── Certificate Generation ──────────────────────────────────────

/// Generate a self-signed TLS certificate for local development.
///
/// The returned certificate and private key are suitable for use with
/// [`server_config`]. The certificate includes `localhost` in its
/// subject alternative names.
pub fn generate_self_signed_cert(
) -> Result<
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

    let priv_key =
        rustls::pki_types::PrivateKeyDer::Pkcs8(rustls::pki_types::PrivatePkcs8KeyDer::from(
            key_der_vec,
        ));

    Ok((cert_der, priv_key))
}

/// Create a QUIC server configuration with a self-signed certificate.
///
/// Sets ALPN to `"unip/1"`. Callers should use
/// [`generate_self_signed_cert`] to obtain the certificate and key,
/// or provide their own.
pub fn server_config(
    cert: rustls::pki_types::CertificateDer<'static>,
    key: rustls::pki_types::PrivateKeyDer<'static>,
) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;

    server_crypto.alpn_protocols = vec![ALPN_UNIP.to_vec()];

    let server_config = quinn::crypto::rustls::QuicServerConfig::try_from(server_crypto)?;
    let mut transport = quinn::TransportConfig::default();
    transport.max_concurrent_bidi_streams(65535u32.into());

    let mut config = ServerConfig::with_crypto(Arc::new(server_config));
    config.transport_config(Arc::new(transport));

    Ok(config)
}

/// Create a QUIC client configuration that **skips certificate verification**.
///
/// **Warning:** This accepts any server certificate without validation.
/// Intended for development and testing only. Production deployments
/// MUST replace this with proper certificate verification.
///
/// Sets ALPN to `"unip/1"`.
pub fn client_config() -> Result<ClientConfig, Box<dyn std::error::Error>> {
    let mut client_crypto = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoServerVerification))
        .with_no_client_auth();

    client_crypto.alpn_protocols = vec![ALPN_UNIP.to_vec()];

    let client_config = quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto)?;
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

// ─── UnipConnection ──────────────────────────────────────────────

/// A UNIP connection wrapping a QUIC connection.
///
/// Manages channels, session state, and frame send/receive over the
/// underlying QUIC transport.
///
/// # Skeleton notes
///
/// This is a skeleton implementation. Production code should:
/// - Reuse QUIC streams per channel (not open one per frame).
/// - Handle bidirectional streams for server-initiated messages.
/// - Implement proper flow control and backpressure.
pub struct UnipConnection {
    conn: Connection,
    channels: Arc<Mutex<ChannelManager>>,
    session_store: MemorySessionStore,
}

impl UnipConnection {
    /// Wrap an established QUIC connection.
    pub fn from_quic(conn: Connection, is_server: bool) -> Self {
        Self {
            conn,
            channels: Arc::new(Mutex::new(ChannelManager::new(is_server))),
            session_store: MemorySessionStore::new(),
        }
    }

    /// Open a new channel.
    ///
    /// Allocates a channel ID and (in the skeleton) opens a QUIC
    /// bidirectional stream. Returns the assigned channel ID.
    pub async fn open_channel(
        &self,
        mode: ChannelMode,
        schema_id: Option<u8>,
    ) -> Result<u16, ChannelError> {
        let channel_id = {
            let mut mgr = self.channels.lock().await;
            mgr.open_channel(mode, schema_id)?
        };

        // Open a QUIC bidirectional stream for this channel.
        let (_send, _recv) = self.conn.open_bi().await.map_err(|e| {
            // Roll back the channel on failure
            ChannelError::StreamError(channel_id, e.to_string())
        })?;

        // In a full implementation, the SendStream and RecvStream would
        // be stored and reused for subsequent send/recv operations on
        // this channel. For the skeleton we let them drop — subsequent
        // send_frame calls open new streams each time.

        Ok(channel_id)
    }

    /// Send a frame on a channel.
    ///
    /// Encodes the frame and writes it to a new QUIC bidirectional stream.
    /// Skeleton limitation: opens a new stream per frame rather than
    /// reusing the channel's persistent stream.
    pub async fn send_frame(&self, channel_id: u16, frame: &Frame) -> Result<(), FrameError> {
        let data = frame.encode()?;

        let (mut send, _recv) = self.conn.open_bi().await.map_err(|e| {
            FrameError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })?;

        send.write_all(&data)
            .await
            .map_err(|e| FrameError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )))?;

        // Signal end of the send direction.
        send.finish().map_err(|e| {
            FrameError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        })?;

        // Advance the channel's send sequence number.
        {
            let mut mgr = self.channels.lock().await;
            let _ = mgr.next_send_seq(channel_id);
        }

        Ok(())
    }

    /// Receive a frame from any incoming QUIC stream.
    ///
    /// Blocks until a peer opens a new bidirectional stream, then reads
    /// frame data from it. Skeleton limitation: does not handle incoming
    /// frames on already-established streams; each call accepts a new
    /// stream from the peer.
    pub async fn recv_frame(&self) -> Result<Frame, FrameError> {
        // Accept the next incoming bidirectional stream from the peer.
        let (_send, mut recv) = self.conn.accept_bi().await.map_err(|e| {
            FrameError::IoError(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                e.to_string(),
            ))
        })?;

        // Accumulate data in a FrameDecoder to handle partial reads.
        let mut decoder = FrameDecoder::new();
        let mut buf = vec![0u8; 65536];

        loop {
            match recv.read(&mut buf).await {
                Ok(Some(n)) => {
                    decoder.feed(&buf[..n]);
                    if let Some(result) = decoder.decode_next() {
                        return result;
                    }
                    // Not enough data yet — continue reading.
                }
                Ok(None) => {
                    // Stream finished — check for any remaining frame.
                    if let Some(result) = decoder.decode_next() {
                        return result;
                    }
                    return Err(FrameError::BufferTooShort {
                        expected: MIN_FRAME_SIZE,
                        actual: decoder.buffered_len(),
                    });
                }
                Err(e) => {
                    return Err(FrameError::IoError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )));
                }
            }
        }
    }

    /// Close a channel gracefully.
    pub async fn close_channel(&self, channel_id: u16) -> Result<(), ChannelError> {
        let mut mgr = self.channels.lock().await;
        mgr.close_channel(channel_id)
    }

    /// Close the entire connection with an error code and reason.
    pub fn close(&self, code: u32, reason: &[u8]) {
        self.conn
            .close(quinn::VarInt::from_u32(code), reason);
    }

    /// Access the underlying QUIC connection.
    pub fn inner(&self) -> &Connection {
        &self.conn
    }

    /// Access the session store.
    pub fn session_store(&self) -> &MemorySessionStore {
        &self.session_store
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
}
