//! TLS acceptor for the FIX TCP gateway.

use std::sync::Arc;

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

/// Re-export for gateway wiring.
pub type FixTlsAcceptor = TlsAcceptor;

/// Errors building or using the FIX TLS acceptor.
#[derive(Error, Debug)]
pub enum FixTlsError {
    #[error("failed to build TLS server config: {0}")]
    Config(String),

    #[error("TLS accept failed: {0}")]
    Accept(String),
}

/// Build a TLS acceptor from a certificate and private key.
pub fn build_tls_acceptor(
    cert: CertificateDer<'static>,
    key: PrivateKeyDer<'static>,
) -> Result<TlsAcceptor, FixTlsError> {
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)
        .map_err(|e| FixTlsError::Config(e.to_string()))?;
    Ok(TlsAcceptor::from(Arc::new(config)))
}

/// Accept a TLS connection over an already-established TCP stream.
pub async fn accept_tls(
    acceptor: &TlsAcceptor,
    stream: TcpStream,
) -> Result<TlsStream<TcpStream>, FixTlsError> {
    acceptor
        .accept(stream)
        .await
        .map_err(|e| FixTlsError::Accept(e.to_string()))
}

/// Type alias for a FIX gateway connection (plain or TLS).
pub enum FixGatewayStream {
    Plain(TcpStream),
    Tls(TlsStream<TcpStream>),
}

impl FixGatewayStream {
    pub fn from_plain(stream: TcpStream) -> Self {
        Self::Plain(stream)
    }

    pub fn from_tls(stream: TlsStream<TcpStream>) -> Self {
        Self::Tls(stream)
    }
}

impl AsyncRead for FixGatewayStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match &mut *self {
            FixGatewayStream::Plain(s) => std::pin::Pin::new(s).poll_read(cx, buf),
            FixGatewayStream::Tls(s) => std::pin::Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for FixGatewayStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        match &mut *self {
            FixGatewayStream::Plain(s) => std::pin::Pin::new(s).poll_write(cx, buf),
            FixGatewayStream::Tls(s) => std::pin::Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match &mut *self {
            FixGatewayStream::Plain(s) => std::pin::Pin::new(s).poll_flush(cx),
            FixGatewayStream::Tls(s) => std::pin::Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        match &mut *self {
            FixGatewayStream::Plain(s) => std::pin::Pin::new(s).poll_shutdown(cx),
            FixGatewayStream::Tls(s) => std::pin::Pin::new(s).poll_shutdown(cx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::transport::generate_self_signed_cert;

    #[test]
    fn test_build_tls_acceptor() {
        let (cert, key) = generate_self_signed_cert().expect("cert");
        let acceptor = build_tls_acceptor(cert, key).expect("acceptor");
        let _ = acceptor;
    }
}
