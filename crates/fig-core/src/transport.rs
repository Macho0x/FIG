//! TREE transport layer for FIG.
//!
//! Wraps [quinn] (RFC 9000) as the TREE transport implementation, providing
//! certificate generation, ALPN negotiation, and a [`FigConnection`] that
//! maps FIG channels to TREE streams.
//!
//! # Security note
//!
//! The default client configuration [`client_config`] verifies the server
//! certificate against the built-in WebPKI roots (or a private CA supplied via
//! [`client_config_from_ca_pem`] / the `FIG_TLS_CA` environment variable).
//! [`client_config_insecure`] disables verification and is **for development
//! and tests only**.
//!
//! Session resumption tokens are authenticated with HMAC-SHA256 by
//! [`crate::session_token::SessionSigner`]; servers reject any unsigned or
//! forged token (SPEC §10.1).

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use quinn::crypto::rustls::QuicClientConfig as TreeClientConfig;
use quinn::crypto::rustls::QuicServerConfig as TreeServerConfig;
use quinn::{ClientConfig, Connection, Endpoint, ServerConfig};
use tokio::sync::Mutex;

use crate::channel::{ChannelDirection, ChannelManager, ChannelMode};
use crate::dos::{DoSGuard, DEFAULT_CHANNEL_MSG_PER_SEC};
use crate::error::{ChannelError, FigError, FrameError, SessionError};
use crate::ext::ExtensionTag;
use crate::frame::{Frame, FrameDecoder, MIN_FRAME_SIZE};
use crate::migration::{
    apply_migration, prepare_migration, reconstruct_channels, validate_migration, MigrationToken,
};
use crate::observability::{span_session_create, span_session_resume};
use crate::rate_limit::ChannelRateLimiter;
use crate::session::{MemorySessionStore, Session, SessionStore};
use crate::session_token::SessionSigner;

/// Receive half of a per-channel TREE stream (decoder + `RecvStream`).
///
/// Send and recv live in separate maps so `send_frame` and `recv_frame` can
/// run concurrently on the same connection (live push + gateway WS).
struct RecvHalf {
    recv: quinn::RecvStream,
    decoder: FrameDecoder,
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
    Box<dyn std::error::Error + Send + Sync>,
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
) -> Result<ServerConfig, Box<dyn std::error::Error + Send + Sync>> {
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;

    // QUIC requires `max_early_data_size` to be 0 (no 0-RTT) or `u32::MAX`.
    // Enable TREE 0-RTT; replay is rejected at the authenticated
    // session-token layer (SPEC §10.1).
    server_crypto.max_early_data_size = u32::MAX;
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
/// Client certificates are verified against `client_ca` (one or more PEM/DER
/// CA certificates). This is the production mTLS entry point: clients whose
/// chain does not validate against the supplied CAs are rejected during the
/// handshake.
///
/// For a development-only variant that accepts any client certificate, see
/// [`server_config_mtls_insecure`].
pub fn server_config_mtls_with_ca(
    cert: rustls::pki_types::CertificateDer<'static>,
    key: rustls::pki_types::PrivateKeyDer<'static>,
    client_ca: Vec<rustls::pki_types::CertificateDer<'static>>,
) -> Result<ServerConfig, Box<dyn std::error::Error + Send + Sync>> {
    let mut roots = rustls::RootCertStore::empty();
    for ca in client_ca {
        roots.add(ca)?;
    }
    if roots.is_empty() {
        return Err("mTLS requires at least one client CA certificate".into());
    }

    let verifier = rustls::server::WebPkiClientVerifier::builder(Arc::new(roots))
        .build()
        .map_err(|e| format!("failed to build client certificate verifier: {e}"))?;

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(vec![cert], key)?;

    server_crypto.max_early_data_size = u32::MAX;
    server_crypto.alpn_protocols = vec![ALPN_FIG.to_vec()];

    let server_config = TreeServerConfig::try_from(server_crypto)?;
    let mut transport = quinn::TransportConfig::default();
    transport.max_concurrent_bidi_streams(65535u32.into());

    let mut config = ServerConfig::with_crypto(Arc::new(server_config));
    config.transport_config(Arc::new(transport));

    Ok(config)
}

/// Create a TREE server configuration requiring mTLS but **accepting any
/// client certificate**.
///
/// **Do not use in production** — this provides the appearance of mTLS
/// without client authentication. For development and tests only.
pub fn server_config_mtls_insecure(
    cert: rustls::pki_types::CertificateDer<'static>,
    key: rustls::pki_types::PrivateKeyDer<'static>,
) -> Result<ServerConfig, Box<dyn std::error::Error + Send + Sync>> {
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_client_cert_verifier(Arc::new(NoClientVerification))
        .with_single_cert(vec![cert], key)?;

    server_crypto.max_early_data_size = u32::MAX;
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

    /// Build a fresh mTLS server config (insecure verifier) from the current certificate.
    pub fn server_config_mtls_insecure(&self) -> Result<ServerConfig, FigError> {
        let guard = self
            .certs
            .lock()
            .map_err(|_| FigError::ConnectionFailed("cert lock poisoned".into()))?;
        server_config_mtls_insecure(guard.0.clone(), guard.1.clone_key())
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))
    }

    /// Build a fresh mTLS server config verifying client certs against `client_ca`.
    pub fn server_config_mtls_with_ca(
        &self,
        client_ca: Vec<rustls::pki_types::CertificateDer<'static>>,
    ) -> Result<ServerConfig, FigError> {
        let guard = self
            .certs
            .lock()
            .map_err(|_| FigError::ConnectionFailed("cert lock poisoned".into()))?;
        server_config_mtls_with_ca(guard.0.clone(), guard.1.clone_key(), client_ca)
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))
    }
}

/// Create a TREE client configuration that verifies the server certificate
/// against the built-in WebPKI root store.
///
/// This is the **secure default**: the TLS handshake fails if the server
/// presents a certificate that does not chain to a trusted public CA and
/// match `server_name`.
///
/// For private PKIs (including self-signed development certificates), use
/// [`client_config_with_roots`] or [`client_config_from_ca_pem`]. For a
/// development-only configuration that skips verification entirely, see
/// [`client_config_insecure`].
///
/// Sets ALPN to `"fig/1"` and enables TREE 0-RTT.
pub fn client_config() -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync>> {
    let roots = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    client_config_with_roots(roots)
}

/// Create a TREE client configuration that verifies the server certificate
/// against an explicit root store.
///
/// Sets ALPN to `"fig/1"` and enables TREE 0-RTT.
pub fn client_config_with_roots(
    roots: rustls::RootCertStore,
) -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync>> {
    let verifier = rustls::client::WebPkiServerVerifier::builder(Arc::new(roots))
        .build()
        .map_err(|e| format!("failed to build server certificate verifier: {e}"))?;
    build_client_config(verifier)
}

/// Load all certificates from a PEM file.
///
/// Used for client CA bundles (mTLS) and for building root stores.
pub fn load_certs_pem(
    path: impl AsRef<std::path::Path>,
) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>, Box<dyn std::error::Error + Send + Sync>>
{
    let file = std::fs::File::open(path.as_ref())?;
    let mut reader = std::io::BufReader::new(file);
    let mut certs = Vec::new();
    for cert in rustls_pemfile::certs(&mut reader) {
        certs.push(cert?);
    }
    if certs.is_empty() {
        return Err(format!("no certificates found in {}", path.as_ref().display()).into());
    }
    Ok(certs)
}

/// Create a TREE client configuration that trusts the CA certificate(s) in a
/// PEM file (or any PEM file with a `CERTIFICATE` block).
///
/// Useful for private PKIs and for pinning a deployment's own CA.
pub fn client_config_from_ca_pem(
    path: impl AsRef<std::path::Path>,
) -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync>> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in load_certs_pem(&path)? {
        roots.add(cert)?;
    }
    client_config_with_roots(roots)
}

/// Create a TREE client configuration honoring the `FIG_TLS_CA` and
/// `FIG_TLS_INSECURE` environment variables, falling back to the built-in
/// WebPKI roots.
///
/// - `FIG_TLS_CA=<path>` — verify the server against the CA bundle in that PEM
///   file (private PKI / pinned CA). This is the intended production entry
///   point for venues with their own PKI.
/// - `FIG_TLS_INSECURE=1` — **development only**: skip certificate
///   verification entirely. A loud warning is logged.
/// - neither set — verify against the built-in public WebPKI roots.
pub fn client_config_from_env() -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync>> {
    if std::env::var("FIG_TLS_INSECURE").ok().as_deref() == Some("1") {
        tracing::warn!(
            "FIG_TLS_INSECURE=1: TLS server certificate verification is DISABLED. \
             Never use this in production."
        );
        return client_config_insecure();
    }
    match std::env::var("FIG_TLS_CA") {
        Ok(path) if !path.trim().is_empty() => client_config_from_ca_pem(path.trim()),
        _ => client_config(),
    }
}

/// Create a TREE client configuration that **skips certificate verification**.
///
/// **Warning:** this accepts any server certificate without validation, making
/// the connection trivially MITM-able. Intended for development and tests
/// only. Production deployments MUST use [`client_config`],
/// [`client_config_with_roots`], or [`client_config_from_ca_pem`].
///
/// Sets ALPN to `"fig/1"` and enables TREE 0-RTT.
pub fn client_config_insecure() -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync>> {
    let mut client_crypto = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoServerVerification))
        .with_no_client_auth();

    client_crypto.enable_early_data = true;
    client_crypto.alpn_protocols = vec![ALPN_FIG.to_vec()];

    let client_config = TreeClientConfig::try_from(client_crypto)?;
    let mut transport = quinn::TransportConfig::default();
    transport.max_concurrent_bidi_streams(65535u32.into());

    let mut config = ClientConfig::new(Arc::new(client_config));
    config.transport_config(Arc::new(transport));

    Ok(config)
}

/// Assemble a client config around a certificate verifier, enabling ALPN and
/// TREE 0-RTT.
fn build_client_config(
    verifier: Arc<dyn rustls::client::danger::ServerCertVerifier>,
) -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync>> {
    let mut client_crypto = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(verifier)
        .with_no_client_auth();

    // Required for `quinn::Connecting::into_0rtt` to send early data.
    client_crypto.enable_early_data = true;
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
/// Each channel gets one bidirectional TREE stream. Send and recv halves
/// are stored in separate maps keyed by channel ID so live `STREAM_ITEM`
/// writes do not block a concurrent read on another (or the same) channel.
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
    send_streams: Mutex<HashMap<u16, quinn::SendStream>>,
    recv_streams: Mutex<HashMap<u16, RecvHalf>>,
    /// Connection-level resource guard (channel count, frame size, msg rate).
    dos: Arc<DoSGuard>,
    /// Per-channel message rate limiter.
    channel_limits: Arc<ChannelRateLimiter>,
    /// Last server-issued resumption token observed on the wire.
    resumption_token: Mutex<Option<Vec<u8>>>,
}

/// Outcome of peeking the first application frame for 0-RTT restore.
///
/// The frame is always returned when one arrives so the caller can dispatch
/// it (subscribe/request) even when no resumption token is present.
#[derive(Debug)]
pub struct FirstFrameRestore {
    pub session: Option<Session>,
    pub frame: Option<Frame>,
}

impl FigConnection {
    /// Wrap an established TREE connection.
    pub fn from_tree(conn: Connection, is_server: bool) -> Self {
        Self {
            conn,
            channels: Arc::new(Mutex::new(ChannelManager::new(is_server))),
            session_store: MemorySessionStore::new(),
            send_streams: Mutex::new(HashMap::new()),
            recv_streams: Mutex::new(HashMap::new()),
            dos: Arc::new(DoSGuard::with_defaults()),
            channel_limits: Arc::new(ChannelRateLimiter::new(DEFAULT_CHANNEL_MSG_PER_SEC)),
            resumption_token: Mutex::new(None),
        }
    }

    /// Replace the connection's DoS guard and per-channel rate limiter.
    pub fn with_limits(
        mut self,
        dos: Arc<DoSGuard>,
        channel_limits: Arc<ChannelRateLimiter>,
    ) -> Self {
        self.dos = dos;
        self.channel_limits = channel_limits;
        self
    }

    /// Access the connection's DoS guard.
    pub fn dos_guard(&self) -> &Arc<DoSGuard> {
        &self.dos
    }

    /// Validate an inbound frame against the connection's resource limits.
    fn admit_frame(&self, frame: &Frame) -> Result<(), FrameError> {
        self.dos.check_frame_size(frame.encoded_size())?;
        if !self.dos.allow_message() {
            return Err(FrameError::IoError(std::io::Error::other(
                "connection message rate exceeded",
            )));
        }
        if !self.channel_limits.allow(frame.channel_id) {
            return Err(FrameError::IoError(std::io::Error::other(
                "channel message rate exceeded",
            )));
        }
        Ok(())
    }

    /// Record a server-issued resumption token if this frame carries one.
    async fn observe_resumption_token(&self, frame: &Frame) {
        if let Some(token) = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::RedirectToken)
            .map(|e| e.value.as_bytes().to_vec())
            .filter(|t| !t.is_empty())
        {
            *self.resumption_token.lock().await = Some(token);
        }
    }

    /// The last server-issued resumption token seen on this connection.
    ///
    /// Clients echo this opaque token on reconnect (see
    /// [`FigClient::connect_0rtt`]); servers mint it via
    /// [`FigServer::issue_resumption_token`].
    pub async fn resumption_token(&self) -> Option<Vec<u8>> {
        self.resumption_token.lock().await.clone()
    }

    /// Store a server-issued resumption token for later reconnect.
    pub async fn set_resumption_token(&self, token: Vec<u8>) {
        *self.resumption_token.lock().await = Some(token);
    }

    async fn insert_bi(
        &self,
        channel_id: u16,
        send: quinn::SendStream,
        recv: quinn::RecvStream,
        decoder: FrameDecoder,
    ) {
        self.send_streams.lock().await.insert(channel_id, send);
        self.recv_streams
            .lock()
            .await
            .insert(channel_id, RecvHalf { recv, decoder });
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

        // Reopen TREE streams for each channel that does not already have one
        // (the first application frame may already occupy that channel).
        for &channel_id in &session.channels {
            if self.send_streams.lock().await.contains_key(&channel_id) {
                continue;
            }
            let (send, recv) = self
                .conn
                .open_bi()
                .await
                .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
            self.insert_bi(channel_id, send, recv, FrameDecoder::new())
                .await;
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
        self.send_streams.lock().await.clear();
        self.recv_streams.lock().await.clear();

        for &channel_id in &session.channels {
            let (send, recv) = self
                .conn
                .open_bi()
                .await
                .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
            self.insert_bi(channel_id, send, recv, FrameDecoder::new())
                .await;
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
        // Reserve a channel slot against the connection limit.
        self.dos.allow_open_channel()?;
        let channel_id = {
            let mut mgr = self.channels.lock().await;
            match mgr.open_unidirectional_channel(mode, direction, schema_id) {
                Ok(id) => id,
                Err(e) => {
                    self.dos.channel_closed();
                    return Err(e);
                }
            }
        };

        match self.conn.open_bi().await {
            Ok((send, recv)) => {
                self.insert_bi(channel_id, send, recv, FrameDecoder::new())
                    .await;
                Ok(channel_id)
            }
            Err(e) => {
                let mut mgr = self.channels.lock().await;
                let _ = mgr.force_close_channel(channel_id);
                self.dos.channel_closed();
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
        // Reserve a channel slot against the connection limit.
        self.dos.allow_open_channel()?;
        let channel_id = {
            let mut mgr = self.channels.lock().await;
            match mgr.open_channel(mode, schema_id) {
                Ok(id) => id,
                Err(e) => {
                    self.dos.channel_closed();
                    return Err(e);
                }
            }
        };

        // Open a TREE bidirectional stream for this channel.
        match self.conn.open_bi().await {
            Ok((send, recv)) => {
                self.insert_bi(channel_id, send, recv, FrameDecoder::new())
                    .await;
                Ok(channel_id)
            }
            Err(e) => {
                // Roll back the channel on failure
                let mut mgr = self.channels.lock().await;
                let _ = mgr.force_close_channel(channel_id);
                self.dos.channel_closed();
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

        if !self.send_streams.lock().await.contains_key(&channel_id) {
            let (send, recv) = self
                .conn
                .open_bi()
                .await
                .map_err(|e| FrameError::IoError(std::io::Error::other(e.to_string())))?;
            self.insert_bi(channel_id, send, recv, FrameDecoder::new())
                .await;
        }
        let mut send = self
            .send_streams
            .lock()
            .await
            .remove(&channel_id)
            .ok_or_else(|| FrameError::IoError(std::io::Error::other("missing send stream")))?;
        let write = send.write_all(&data).await;
        self.send_streams.lock().await.insert(channel_id, send);
        match write {
            Ok(()) => {}
            Err(quinn::WriteError::Stopped(error_code)) => {
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
        if !self.recv_streams.lock().await.contains_key(&channel_id) {
            let (send, recv) = self.conn.accept_bi().await.map_err(|e| {
                FrameError::IoError(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    e.to_string(),
                ))
            })?;
            self.insert_bi(channel_id, send, recv, FrameDecoder::new())
                .await;
        }

        let mut buf = vec![0u8; 65536];
        loop {
            let mut ch = self
                .recv_streams
                .lock()
                .await
                .remove(&channel_id)
                .ok_or_else(|| FrameError::IoError(std::io::Error::other("missing recv stream")))?;
            if let Some(result) = ch.decoder.decode_next() {
                self.recv_streams.lock().await.insert(channel_id, ch);
                let frame = result?;
                self.admit_frame(&frame)?;
                self.observe_resumption_token(&frame).await;
                return Ok(frame);
            }
            match ch.recv.read(&mut buf).await {
                Ok(Some(n)) => {
                    ch.decoder.feed(&buf[..n]);
                    self.recv_streams.lock().await.insert(channel_id, ch);
                }
                Ok(None) => {
                    let leftover = ch.decoder.decode_next();
                    let buffered = ch.decoder.buffered_len();
                    self.recv_streams.lock().await.insert(channel_id, ch);
                    if let Some(result) = leftover {
                        let frame = result?;
                        self.admit_frame(&frame)?;
                        self.observe_resumption_token(&frame).await;
                        return Ok(frame);
                    }
                    return Err(FrameError::BufferTooShort {
                        expected: MIN_FRAME_SIZE,
                        actual: buffered,
                    });
                }
                Err(quinn::ReadError::Reset(error_code)) => {
                    self.recv_streams.lock().await.insert(channel_id, ch);
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
                    self.recv_streams.lock().await.insert(channel_id, ch);
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

        // Enforce resource limits before admitting a new channel or frame.
        if channel_id != crate::frame::CONTROL_CHANNEL
            && !self.send_streams.lock().await.contains_key(&channel_id)
        {
            self.dos.allow_open_channel().map_err(|e| {
                FrameError::IoError(std::io::Error::other(format!(
                    "channel limit exceeded: {e}"
                )))
            })?;
        }
        self.admit_frame(&frame)?;
        self.observe_resumption_token(&frame).await;

        self.insert_bi(channel_id, send, recv, decoder).await;

        Ok((channel_id, frame))
    }

    /// On 0-RTT reconnect, read the first application frame (if any) and restore session state.
    ///
    /// The resumption token is accepted **only** if it is a valid
    /// server-signed token verified by `signer` (SPEC §10.1). Unsigned,
    /// tampered, or forged tokens restore nothing — the connection proceeds
    /// as a fresh session.
    pub async fn try_restore_session_from_first_frame(
        &self,
        signer: &SessionSigner,
    ) -> FirstFrameRestore {
        let result = tokio::time::timeout(Duration::from_millis(250), self.accept_frame()).await;
        let Ok(Ok((_, frame))) = result else {
            return FirstFrameRestore {
                session: None,
                frame: None,
            };
        };

        let token = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::RedirectToken)
            .map(|e| e.value.as_bytes().to_vec())
            .filter(|b| !b.is_empty())
            .or_else(|| {
                if frame.payload.is_empty() {
                    None
                } else {
                    Some(frame.payload.clone())
                }
            });

        let session = token.and_then(|token| {
            // Reject replays within the window, then authenticate the token.
            crate::replay::validate_resumption_token(&token).ok()?;
            signer.verify(&token).ok()
        });

        FirstFrameRestore {
            session,
            frame: Some(frame),
        }
    }

    /// Drain buffered frames then read until the peer half-closes this channel.
    pub async fn recv_until_closed(&self, channel_id: u16) -> Vec<Frame> {
        let mut out = self.recv_buffered_frames(channel_id).await;
        while let Ok(frame) = self.recv_frame(channel_id).await {
            out.push(frame);
        }
        out
    }

    /// Drain additional frames already buffered on a channel decoder.
    pub async fn recv_buffered_frames(&self, channel_id: u16) -> Vec<Frame> {
        let mut out = Vec::new();
        let mut recvs = self.recv_streams.lock().await;
        if let Some(ch) = recvs.get_mut(&channel_id) {
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
        if let Some(mut send) = self.send_streams.lock().await.remove(&channel_id) {
            let _ = send.finish();
        }
        self.recv_streams.lock().await.remove(&channel_id);
        self.dos.channel_closed();
        Ok(())
    }

    /// Close a channel gracefully.
    pub async fn close_channel(&self, channel_id: u16) -> Result<(), ChannelError> {
        let result = {
            let mut mgr = self.channels.lock().await;
            mgr.close_channel(channel_id)
        };
        if result.is_ok() {
            self.dos.channel_closed();
        }
        result
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
    signer: Arc<SessionSigner>,
}

impl FigServer {
    /// Bind a new FIG server to the given address.
    pub async fn bind(addr: SocketAddr, server_config: ServerConfig) -> Result<Self, FigError> {
        let endpoint = Endpoint::server(server_config, addr)
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        Ok(Self {
            endpoint,
            session_store: MemorySessionStore::new(),
            signer: Arc::new(SessionSigner::from_env()),
        })
    }

    /// Bind with an explicit resumption-token signer.
    ///
    /// Use this to share one signing key across HA nodes (see
    /// [`SessionSigner`] and `FIG_RESUMPTION_KEY`).
    pub async fn bind_with_signer(
        addr: SocketAddr,
        server_config: ServerConfig,
        signer: Arc<SessionSigner>,
    ) -> Result<Self, FigError> {
        let endpoint = Endpoint::server(server_config, addr)
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;
        Ok(Self {
            endpoint,
            session_store: MemorySessionStore::new(),
            signer,
        })
    }

    /// The signer used to authenticate resumption tokens.
    pub fn signer(&self) -> &Arc<SessionSigner> {
        &self.signer
    }

    /// Issue a signed resumption token for `session`.
    ///
    /// The token is opaque to clients; a venue hands it to the client (e.g. on
    /// a control channel) and the client echoes it on reconnect. Only tokens
    /// signed by a signer holding the same key are accepted by
    /// [`FigServer::accept_0rtt`].
    pub fn issue_resumption_token(&self, session: &Session) -> Result<Vec<u8>, SessionError> {
        self.signer.issue(session)
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
    /// A session is restored only if the client presents a token that
    /// [`SessionSigner::verify`] accepts (i.e. one this server, or a node
    /// sharing the same key, issued). Unsigned, tampered, replayed, or forged
    /// tokens restore nothing — the connection proceeds as a fresh session.
    /// If 0-RTT is rejected by the transport layer, falls back to a full
    /// handshake.
    ///
    /// Restored state prefers the server's authoritative session store when it
    /// holds the session, falling back to the (authenticated) token payload
    /// (SPEC §10.1).
    pub async fn accept_0rtt(
        &self,
    ) -> Result<(FigConnection, Option<Session>, Option<Frame>), FigError> {
        let incoming = self
            .endpoint
            .accept()
            .await
            .ok_or_else(|| FigError::ConnectionFailed("endpoint closed".into()))?;

        let conn = incoming
            .await
            .map_err(|e| FigError::ConnectionFailed(e.to_string()))?;

        let fig_conn = FigConnection::from_tree(conn, true);

        let FirstFrameRestore { session, frame } = fig_conn
            .try_restore_session_from_first_frame(&self.signer)
            .await;

        // Prefer authoritative server-side state over the token payload.
        let session = session.map(
            |restored| match self.session_store.get(&restored.session_id) {
                Ok(Some(stored)) => stored,
                _ => restored,
            },
        );

        if let Some(restored) = &session {
            let span = span_session_resume(&restored.session_id.to_string());
            let _guard = span.enter();
            tracing::info!(
                session_id = %restored.session_id,
                "session restored from authenticated 0-RTT resumption token"
            );
            let _ = fig_conn.reconstruct_from_session(restored).await;
        }

        Ok((fig_conn, session, frame))
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
        let _config = client_config_insecure().expect("client config creation should succeed");
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
        let _config =
            server_config_mtls_insecure(cert, key).expect("mTLS server config should succeed");
    }

    #[test]
    fn test_rotating_server_certs_reload() {
        let (cert1, key1) = generate_self_signed_cert().unwrap();
        let rotating = RotatingServerCerts::new(cert1, key1);
        let _cfg1 = rotating.server_config().unwrap();

        let (cert2, key2) = generate_self_signed_cert().unwrap();
        rotating.reload(cert2, key2).unwrap();
        let _cfg2 = rotating.server_config_mtls_insecure().unwrap();
    }

    #[tokio::test]
    async fn test_tree_ping_pong_round_trip() {
        let (cert, key) = generate_self_signed_cert().unwrap();
        let server_cfg = server_config(cert, key).unwrap();
        let client_cfg = client_config_insecure().unwrap();

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

    /// A server must restore a session only from a token it issued (HMAC
    /// verified). An unsigned/forged token must restore nothing.
    #[tokio::test]
    async fn test_accept_0rtt_rejects_forged_token_accepts_signed() {
        let _ = tracing_subscriber::fmt::try_init();

        let (cert, key) = generate_self_signed_cert().unwrap();
        let server_cfg = server_config(cert, key).unwrap();
        let signer = Arc::new(
            SessionSigner::from_key_bytes(b"unit-test-signing-key-32-bytes!!".to_vec()).unwrap(),
        );

        let server = Arc::new(
            FigServer::bind_with_signer("127.0.0.1:0".parse().unwrap(), server_cfg, signer.clone())
                .await
                .unwrap(),
        );
        let addr = server.local_addr().unwrap();

        let mut session = Session::new();
        session.add_channel(5);
        let signed = signer.issue(&session).unwrap();
        // Client-forged, unsigned CBOR with the same session contents.
        let forged = session.resumption_token().unwrap();
        assert_ne!(signed, forged);

        let accept_server = server.clone();
        let handle = tokio::spawn(async move {
            let (_, first, first_frame) = accept_server.accept_0rtt().await.unwrap();
            let (_, second, second_frame) = accept_server.accept_0rtt().await.unwrap();
            (first, second, first_frame.is_some(), second_frame.is_some())
        });

        // Connection 1: forged (unsigned) token.
        let mut endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap()).unwrap();
        endpoint.set_default_client_config(client_config_insecure().unwrap());
        let conn = endpoint.connect(addr, "localhost").unwrap().await.unwrap();
        let fig = FigConnection::from_tree(conn, false);
        let frame = Frame::new(crate::frame::FrameType::Request, 5).with_extension(
            crate::ext::Extension::binary(ExtensionTag::RedirectToken, forged),
        );
        fig.send_frame(5, &frame).await.unwrap();
        // Give the server a moment to read the first frame before reconnecting.
        tokio::time::sleep(Duration::from_millis(400)).await;

        // Connection 2: server-issued signed token.
        let mut endpoint2 = Endpoint::client("0.0.0.0:0".parse().unwrap()).unwrap();
        endpoint2.set_default_client_config(client_config_insecure().unwrap());
        let conn2 = endpoint2.connect(addr, "localhost").unwrap().await.unwrap();
        let fig2 = FigConnection::from_tree(conn2, false);
        let frame2 = Frame::new(crate::frame::FrameType::Request, 5).with_extension(
            crate::ext::Extension::binary(ExtensionTag::RedirectToken, signed),
        );
        fig2.send_frame(5, &frame2).await.unwrap();
        tokio::time::sleep(Duration::from_millis(400)).await;

        let (first, second, got_first_frame, got_second_frame) = handle.await.unwrap();
        assert!(
            got_first_frame && got_second_frame,
            "both frames must arrive"
        );
        assert!(
            first.is_none(),
            "forged unsigned token must not restore a session"
        );
        let restored = second.expect("server-issued token must restore a session");
        assert_eq!(restored.session_id, session.session_id);
        assert_eq!(restored.channels, vec![5]);
    }

    /// The default client config must refuse a server certificate that does not
    /// chain to a trusted root (this is what makes the connection non-MITM-able).
    #[tokio::test]
    async fn test_default_client_rejects_untrusted_server_certificate() {
        // Negative: WebPKI roots must not validate a self-signed certificate.
        let (cert, key) = generate_self_signed_cert().unwrap();
        let server_cfg = server_config(cert, key).unwrap();
        let server = Arc::new(
            FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg)
                .await
                .unwrap(),
        );
        let addr = server.local_addr().unwrap();

        let accept_server = server.clone();
        let h = tokio::spawn(async move {
            let _ = tokio::time::timeout(Duration::from_secs(5), accept_server.accept()).await;
        });

        let client = FigClient::new(client_config().unwrap()).unwrap();
        assert!(
            client.connect(addr, "localhost").await.is_err(),
            "default client config must not trust a self-signed server certificate"
        );
        let _ = h.await;

        // Positive control: the explicitly-insecure dev config connects.
        let (cert2, key2) = generate_self_signed_cert().unwrap();
        let server2 = Arc::new(
            FigServer::bind(
                "127.0.0.1:0".parse().unwrap(),
                server_config(cert2, key2).unwrap(),
            )
            .await
            .unwrap(),
        );
        let addr2 = server2.local_addr().unwrap();

        let accept_server2 = server2.clone();
        let h2 = tokio::spawn(async move { accept_server2.accept().await });

        let insecure = FigClient::new(client_config_insecure().unwrap()).unwrap();
        assert!(
            insecure.connect(addr2, "localhost").await.is_ok(),
            "insecure dev config should accept a self-signed certificate"
        );
        let _ = h2.await;
    }

    #[test]
    fn test_mtls_with_ca_requires_a_ca() {
        let (cert, key) = generate_self_signed_cert().unwrap();

        // No CA bundle → refuse to build (would be an mTLS config that verifies nothing).
        assert!(
            server_config_mtls_with_ca(cert.clone(), key.clone_key(), vec![]).is_err(),
            "mTLS without a client CA must be rejected"
        );

        // With a CA bundle the config builds using the WebPKI client verifier.
        assert!(
            server_config_mtls_with_ca(cert.clone(), key, vec![cert]).is_ok(),
            "mTLS with a client CA should build"
        );
    }
}
