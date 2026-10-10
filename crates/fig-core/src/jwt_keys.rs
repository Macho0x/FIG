//! Asymmetric (RS256) JWT verification and key rotation.
//!
//! [`crate::jwt`] ships an HS256 verifier for development and for venues that
//! share a symmetric secret. Production identity systems normally sign with an
//! asymmetric key held in a KMS/HSM and publish the public half as a JWKS
//! document. This module verifies those tokens **without** the private key:
//!
//! - [`RsaKeySet`] verifies RS256 tokens against one or more RSA public keys,
//!   selected by the JWT `kid` header. Keeping several keys configured at once
//!   is what makes rotation non-disruptive (see `docs/JWT_KEY_ROTATION.md`).
//! - [`FnVerifier`] adapts an arbitrary closure, e.g. a KMS `Verify` call, to
//!   the [`JwtVerifier`] trait.
//!
//! ```no_run
//! use fig_core::jwt_keys::{JwtVerifier, RsaKeySet};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Configure every key that may currently sign a live token (rotation overlap).
//! let jwks = std::fs::read_to_string("jwks.json")?;
//! let keys = RsaKeySet::from_jwks_json(&jwks)?;
//! let token = std::env::var("FIG_TOKEN")?;
//! let claims = keys.verify(&token)?;
//! println!("subject = {}", claims.sub);
//! # Ok(())
//! # }
//! ```
//!
//! Claims are validated exactly as in [`crate::jwt::decode_jwt`]: `alg` must be
//! RS256, the signature must verify, and `sub`/`exp` must be present and valid.

use std::sync::Arc;

use base64::Engine;
use ring::signature::{RsaPublicKeyComponents, RSA_PKCS1_2048_8192_SHA256};
use serde::Deserialize;
use serde_json::Value;

use crate::jwt::{FigJwtClaims, JwtError};

/// A pluggable bearer-token verifier (local keys, JWKS, or a KMS/HSM call).
pub trait JwtVerifier: Send + Sync {
    /// Verify `token` and return its claims.
    fn verify(&self, token: &str) -> Result<FigJwtClaims, JwtError>;
}

/// Environment variable holding a JWKS document or a path to one.
pub const JWKS_ENV: &str = "FIG_JWT_JWKS";

/// Adapt a closure into a [`JwtVerifier`] (e.g. wrapping a KMS verify call).
pub struct FnVerifier<F> {
    f: F,
    name: &'static str,
}

impl<F> FnVerifier<F>
where
    F: Fn(&str) -> Result<FigJwtClaims, JwtError> + Send + Sync,
{
    /// Wrap `f` as a verifier. `name` appears in diagnostics only.
    pub fn new(name: &'static str, f: F) -> Self {
        Self { f, name }
    }

    /// Human-readable name of the backing verifier.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl<F> JwtVerifier for FnVerifier<F>
where
    F: Fn(&str) -> Result<FigJwtClaims, JwtError> + Send + Sync,
{
    fn verify(&self, token: &str) -> Result<FigJwtClaims, JwtError> {
        (self.f)(token)
    }
}

/// One RSA public key with an optional key id.
#[derive(Clone)]
pub struct JwksKey {
    /// `kid` from the JWKS entry (absent for PEM-loaded keys).
    pub kid: Option<String>,
    /// RSA modulus, big-endian bytes.
    pub n: Vec<u8>,
    /// RSA public exponent, big-endian bytes.
    pub e: Vec<u8>,
}

impl std::fmt::Debug for JwksKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwksKey")
            .field("kid", &self.kid)
            .field("modulus_bits", &(self.n.len() * 8))
            .finish()
    }
}

#[derive(Deserialize)]
struct JwkEntry {
    kty: String,
    #[serde(default)]
    kid: Option<String>,
    n: Option<String>,
    e: Option<String>,
}

#[derive(Deserialize)]
struct JwksDocument {
    keys: Vec<JwkEntry>,
}

/// A set of trusted RSA public keys for RS256 verification.
///
/// Configure every key that may currently have signed a live token; a token is
/// accepted if its `kid` matches one of them. During rotation, publish (and
/// configure) both the old and the new key until all old tokens have expired.
#[derive(Debug, Default, Clone)]
pub struct RsaKeySet {
    keys: Vec<JwksKey>,
}

impl RsaKeySet {
    /// An empty key set (rejects every token).
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    /// Add a key.
    pub fn add(&mut self, key: JwksKey) -> &mut Self {
        self.keys.push(key);
        self
    }

    /// Number of configured keys.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Whether no keys are configured.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// The configured key ids.
    pub fn kids(&self) -> Vec<&str> {
        self.keys.iter().filter_map(|k| k.kid.as_deref()).collect()
    }

    /// Parse a JWKS document (`{"keys":[{"kty":"RSA","kid":..,"n":..,"e":..}]}`).
    ///
    /// Non-RSA entries are skipped; entries missing `n`/`e` are rejected.
    pub fn from_jwks_json(json: &str) -> Result<Self, JwtError> {
        let doc: JwksDocument = serde_json::from_str(json).map_err(|_| JwtError::InvalidFormat)?;
        let mut keys = Vec::new();
        for entry in doc.keys {
            if !entry.kty.eq_ignore_ascii_case("RSA") {
                continue;
            }
            let (Some(n), Some(e)) = (entry.n, entry.e) else {
                return Err(JwtError::InvalidFormat);
            };
            keys.push(JwksKey {
                kid: entry.kid,
                n: b64url_bytes(&n)?,
                e: b64url_bytes(&e)?,
            });
        }
        Ok(Self { keys })
    }

    /// Add every RSA key in a JWKS document.
    pub fn add_jwks_json(&mut self, json: &str) -> Result<&mut Self, JwtError> {
        self.keys.extend(Self::from_jwks_json(json)?.keys);
        Ok(self)
    }

    /// Add a key from a PEM SubjectPublicKeyInfo (`-----BEGIN PUBLIC KEY-----`).
    ///
    /// The RSA modulus and exponent are extracted from the DER, so PEM and
    /// JWKS keys verify through the same code path.
    pub fn add_pem(&mut self, pem: &str, kid: Option<&str>) -> Result<&mut Self, JwtError> {
        let der = pem_spki_der(pem)?;
        let (n, e) = parse_spki_rsa(&der)?;
        self.keys.push(JwksKey {
            kid: kid.map(str::to_string),
            n,
            e,
        });
        Ok(self)
    }

    /// Load keys from either a JWKS document or a filesystem path to one.
    ///
    /// A value beginning with `{` is treated as inline JSON; anything else is
    /// read as a file path. This is what [`RsaKeySet::from_env`] uses.
    pub fn from_jwks_or_path(value: &str) -> Result<Self, JwtError> {
        let trimmed = value.trim();
        if trimmed.starts_with('{') {
            Self::from_jwks_json(trimmed)
        } else {
            let json = std::fs::read_to_string(trimmed).map_err(|_| JwtError::InvalidFormat)?;
            Self::from_jwks_json(&json)
        }
    }

    /// Load keys from `FIG_JWT_JWKS` (inline JSON or a path to a JWKS file).
    ///
    /// Returns `None` when the variable is unset or empty, so a venue can opt
    /// into RS256 while keeping HS256 as the default.
    pub fn from_env() -> Result<Option<Self>, JwtError> {
        match std::env::var(JWKS_ENV) {
            Ok(v) if !v.trim().is_empty() => Ok(Some(Self::from_jwks_or_path(&v)?)),
            _ => Ok(None),
        }
    }

    fn verify_with_key(key: &JwksKey, signing_input: &[u8], signature: &[u8]) -> bool {
        let components = RsaPublicKeyComponents {
            n: &key.n,
            e: &key.e,
        };
        components
            .verify(&RSA_PKCS1_2048_8192_SHA256, signing_input, signature)
            .is_ok()
    }
}

impl JwtVerifier for RsaKeySet {
    /// Verify an RS256 JWT.
    ///
    /// The `kid` header selects the key; a token without `kid` is tried against
    /// every configured key. Anything else (missing/duplicate claims, a wrong
    /// signature, `alg` other than RS256, or an expired token) is rejected.
    fn verify(&self, token: &str) -> Result<FigJwtClaims, JwtError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::InvalidFormat);
        }

        let header_bytes = b64url_bytes(parts[0])?;
        let header: Value =
            serde_json::from_slice(&header_bytes).map_err(|_| JwtError::InvalidHeader)?;
        match header.get("alg").and_then(Value::as_str) {
            Some("RS256") => {}
            _ => return Err(JwtError::InvalidHeader),
        }

        let signature = b64url_bytes(parts[2]).map_err(|_| JwtError::SignatureMismatch)?;
        let signing_input = format!("{}.{}", parts[0], parts[1]);

        let kid = header.get("kid").and_then(Value::as_str);
        let mut verified = false;
        for key in &self.keys {
            if let (Some(expected), Some(actual)) = (kid, key.kid.as_deref()) {
                if expected != actual {
                    continue;
                }
            }
            if RsaKeySet::verify_with_key(key, signing_input.as_bytes(), &signature) {
                verified = true;
                break;
            }
        }
        if !verified {
            return Err(JwtError::SignatureMismatch);
        }

        // Claims validation matches `decode_jwt`.
        let payload_bytes = b64url_bytes(parts[1])?;
        let claims: FigJwtClaims =
            serde_json::from_slice(&payload_bytes).map_err(|_| JwtError::InvalidPayload)?;
        if claims.sub.is_empty() {
            return Err(JwtError::MissingSubject);
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if claims.is_expired(now) {
            return Err(JwtError::Expired);
        }
        Ok(claims)
    }
}

/// Wrap a key set in an `Arc` for shared use as a [`JwtVerifier`].
pub fn shared(keys: RsaKeySet) -> Arc<dyn JwtVerifier> {
    Arc::new(keys)
}

fn b64url_bytes(s: &str) -> Result<Vec<u8>, JwtError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| JwtError::InvalidFormat)
}

/// Extract the DER bytes of a PEM block (`PUBLIC KEY` or `CERTIFICATE`).
fn pem_spki_der(pem: &str) -> Result<Vec<u8>, JwtError> {
    let body: String = pem
        .lines()
        .filter(|l| !l.trim_start().starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");
    if body.is_empty() {
        return Err(JwtError::InvalidFormat);
    }
    base64::engine::general_purpose::STANDARD
        .decode(body)
        .map_err(|_| JwtError::InvalidFormat)
}

/// Extract `(n, e)` from an RSA SubjectPublicKeyInfo (or PKCS#1) DER blob.
///
/// Hand-rolled minimal DER walk: `SEQUENCE { AlgorithmIdentifier, BIT STRING
/// { SEQUENCE { INTEGER n, INTEGER e } } }`. Accepts the PKCS#1 form
/// (`SEQUENCE { INTEGER n, INTEGER e }`) too.
fn parse_spki_rsa(der: &[u8]) -> Result<(Vec<u8>, Vec<u8>), JwtError> {
    fn read_tlv(input: &[u8], pos: &mut usize) -> Result<(u8, Vec<u8>), JwtError> {
        let tag = *input.get(*pos).ok_or(JwtError::InvalidFormat)?;
        *pos += 1;
        let first = *input.get(*pos).ok_or(JwtError::InvalidFormat)?;
        *pos += 1;
        let len = if first & 0x80 == 0 {
            first as usize
        } else {
            let n = (first & 0x7f) as usize;
            if n == 0 || n > 4 {
                return Err(JwtError::InvalidFormat);
            }
            let mut v = 0usize;
            for _ in 0..n {
                v = (v << 8) | *input.get(*pos).ok_or(JwtError::InvalidFormat)? as usize;
                *pos += 1;
            }
            v
        };
        let end = pos.checked_add(len).ok_or(JwtError::InvalidFormat)?;
        let body = input.get(*pos..end).ok_or(JwtError::InvalidFormat)?;
        *pos = end;
        Ok((tag, body.to_vec()))
    }

    /// Strip the leading 0x00 DER INTEGER padding byte.
    fn integer(bytes: Vec<u8>) -> Vec<u8> {
        match bytes.split_first() {
            Some((0, rest)) if !rest.is_empty() => rest.to_vec(),
            _ => bytes,
        }
    }

    let tag_sequence = 0x30;
    let tag_bit_string = 0x03;
    let tag_integer = 0x02;

    let mut pos = 0;
    let (tag, spki_body) = read_tlv(der, &mut pos)?;
    if tag != tag_sequence {
        return Err(JwtError::InvalidFormat);
    }

    // Try PKCS#1 first: SEQUENCE { INTEGER n, INTEGER e }.
    {
        let mut p = 0;
        if let Ok((t1, n)) = read_tlv(&spki_body, &mut p) {
            if t1 == tag_integer {
                if let Ok((t2, e)) = read_tlv(&spki_body, &mut p) {
                    if t2 == tag_integer {
                        let (n, e) = (integer(n), integer(e));
                        if n.len() >= 128 && !e.is_empty() {
                            return Ok((n, e));
                        }
                    }
                }
            }
        }
    }

    // SPKI: skip AlgorithmIdentifier, then read the BIT STRING.
    let mut p = 0;
    let (t_alg, _alg) = read_tlv(&spki_body, &mut p)?;
    if t_alg != tag_sequence {
        return Err(JwtError::InvalidFormat);
    }
    let (t_bits, bits) = read_tlv(&spki_body, &mut p)?;
    if t_bits != tag_bit_string || bits.is_empty() {
        return Err(JwtError::InvalidFormat);
    }
    // BIT STRING: first byte is the number of unused bits (0 for DER keys).
    let inner = bits
        .split_first()
        .filter(|(unused, _): &(&u8, &[u8])| **unused == 0)
        .map(|(_, rest)| rest)
        .ok_or(JwtError::InvalidFormat)?;

    let mut q = 0;
    let (t_seq, seq) = read_tlv(inner, &mut q)?;
    if t_seq != tag_sequence {
        return Err(JwtError::InvalidFormat);
    }
    let mut r = 0;
    let (t_n, n) = read_tlv(&seq, &mut r)?;
    let (t_e, e) = read_tlv(&seq, &mut r)?;
    if t_n != tag_integer || t_e != tag_integer {
        return Err(JwtError::InvalidFormat);
    }
    let (n, e) = (integer(n), integer(e));
    if n.len() < 128 || e.is_empty() {
        return Err(JwtError::InvalidFormat);
    }
    Ok((n, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODULUS_OLD: &str = "xrVHy6TgDj1yhElRN5CWrl3xUVONd_jiW6Hv4iHYjK0tl5e1Y0ClhWK-VdAS9viwGovrxs7Y6rwiCvpvuLBilnS6xIVcb8Zpea2p71ydpxJKAKY34e75SM1t0OGerLE6P1Wvq2Rv3OprGr6JWdtcvslsn-kdOqu6Rn8yv_F4F5cGbkxCjLSv0tsYOsnuedwfSXZl_BRfUD7EkBoKDEQPB-sHu0PjYum-L8-PGtafFqXTsJLvNt93BNbmZdEvvbYMy85mVC0ITgVV1525odF8XJJdKEhhXr37nlNb_1z-D7QhnUi4rNl4j_IFzEy53D6h-53G2vXWTxZPyMB7r1o_xw";
    const MODULUS_NEW: &str = "j7Lt30m4W4opLPO27OXEKciua5DoSERHHcaI89cKK9DP-kNtPIJA_O4574dqUkY_UXVHqVYFxElPoVHrPWxLHTcVxRozz16iCX30c_hFnZHQAYvJ7nPpO52HMuwBUXUM-67xaN5eqbe2xUtSQck5_hYMBF43fzU9QnkHwm8F1l6H7T7om2H12pLUNP1X_ie3j1CmlctkVy36ukpbtUhzsKgTP-pM2ayprb06R-WnYXoxj6bOAtvd3lDJwTd5uU362YtA0CwUpc8B3B2mTRAW6ojpnfpaTXfMVAUf1-GZfSg3Hs9UlS7Nswhz95iaEifChjDLPqOZJNGXFkG7Frl26w";
    const EXPONENT: &str = "AQAB"; // 65537

    /// Public half of the "new" key (SubjectPublicKeyInfo PEM).
    const PUBLIC_KEY_NEW_PEM: &str = "\
-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAj7Lt30m4W4opLPO27OXE\nKciua5DoSERHHcaI89cKK9DP+kNtPIJA/O4574dqUkY/UXVHqVYFxElPoVHrPWxL\nHTcVxRozz16iCX30c/hFnZHQAYvJ7nPpO52HMuwBUXUM+67xaN5eqbe2xUtSQck5\n/hYMBF43fzU9QnkHwm8F1l6H7T7om2H12pLUNP1X/ie3j1CmlctkVy36ukpbtUhz\nsKgTP+pM2ayprb06R+WnYXoxj6bOAtvd3lDJwTd5uU362YtA0CwUpc8B3B2mTRAW\n6ojpnfpaTXfMVAUf1+GZfSg3Hs9UlS7Nswhz95iaEifChjDLPqOZJNGXFkG7Frl2\n6wIDAQAB\n-----END PUBLIC KEY-----";

    /// RS256 JWT signed by the "old" key (`kid=k-old`); `exp` in 2100.
    const TOKEN_OLD: &str = include_str!("../tests/data/jwt_rs256_old.txt");
    /// RS256 JWT signed by the "new" key (`kid=k-new`).
    const TOKEN_NEW: &str = include_str!("../tests/data/jwt_rs256_new.txt");
    /// Valid signature, `exp` in 2001.
    const TOKEN_EXPIRED: &str = include_str!("../tests/data/jwt_rs256_expired.txt");
    /// `alg: none`, empty signature.
    const TOKEN_ALG_NONE: &str = include_str!("../tests/data/jwt_rs256_alg_none.txt");

    fn jwks(kids: &[(&str, &str)]) -> String {
        let keys: Vec<Value> = kids
            .iter()
            .map(|(kid, n)| {
                serde_json::json!({
                    "kty": "RSA",
                    "kid": kid,
                    "alg": "RS256",
                    "use": "sig",
                    "n": n,
                    "e": EXPONENT,
                })
            })
            .collect();
        serde_json::json!({ "keys": keys }).to_string()
    }

    #[test]
    fn verifies_token_with_matching_kid() {
        let set = RsaKeySet::from_jwks_json(&jwks(&[("k-old", MODULUS_OLD)])).unwrap();
        let claims = set.verify(TOKEN_OLD.trim()).expect("token verifies");
        assert_eq!(claims.sub, "trader-1");
        assert_eq!(claims.permissions, vec!["trading:orders:write"]);
        assert_eq!(
            claims.session_id.as_deref(),
            Some("11111111-1111-1111-1111-111111111111")
        );
    }

    #[test]
    fn rejects_token_signed_by_unknown_key() {
        // Only the old key is configured, but the token is signed by the new one.
        let set = RsaKeySet::from_jwks_json(&jwks(&[("k-old", MODULUS_OLD)])).unwrap();
        assert!(matches!(
            set.verify(TOKEN_NEW.trim()),
            Err(JwtError::SignatureMismatch)
        ));
    }

    #[test]
    fn rotation_overlap_accepts_both_keys() {
        // Both keys published during rotation: old and new tokens both verify.
        let set =
            RsaKeySet::from_jwks_json(&jwks(&[("k-old", MODULUS_OLD), ("k-new", MODULUS_NEW)]))
                .unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.verify(TOKEN_OLD.trim()).is_ok());
        assert!(set.verify(TOKEN_NEW.trim()).is_ok());
    }

    #[test]
    fn rotation_retirement_rejects_old_key() {
        // Old key retired: old tokens stop verifying, new ones still do.
        let set = RsaKeySet::from_jwks_json(&jwks(&[("k-new", MODULUS_NEW)])).unwrap();
        assert!(matches!(
            set.verify(TOKEN_OLD.trim()),
            Err(JwtError::SignatureMismatch)
        ));
        assert!(set.verify(TOKEN_NEW.trim()).is_ok());
    }

    #[test]
    fn rejects_tampered_payload() {
        let set = RsaKeySet::from_jwks_json(&jwks(&[("k-old", MODULUS_OLD)])).unwrap();
        let token = TOKEN_OLD.trim();
        let mut parts: Vec<&str> = token.split('.').collect();
        let forged = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(br#"{"sub":"attacker","exp":4102444800}"#);
        parts[1] = &forged;
        let tampered = parts.join(".");
        assert!(matches!(
            set.verify(&tampered),
            Err(JwtError::SignatureMismatch)
        ));
    }

    #[test]
    fn rejects_expired_token() {
        let set = RsaKeySet::from_jwks_json(&jwks(&[("k-old", MODULUS_OLD)])).unwrap();
        // Signature is valid, but `exp` is in 2001.
        assert!(matches!(
            set.verify(TOKEN_EXPIRED.trim()),
            Err(JwtError::Expired)
        ));
    }

    #[test]
    fn rejects_alg_none() {
        let set = RsaKeySet::from_jwks_json(&jwks(&[("k-old", MODULUS_OLD)])).unwrap();
        assert!(matches!(
            set.verify(TOKEN_ALG_NONE.trim()),
            Err(JwtError::InvalidHeader)
        ));
    }

    #[test]
    fn empty_key_set_rejects_everything() {
        let set = RsaKeySet::new();
        assert!(set.is_empty());
        assert!(set.verify(TOKEN_OLD.trim()).is_err());
    }

    #[test]
    fn malformed_jwks_is_rejected() {
        assert!(RsaKeySet::from_jwks_json("not json").is_err());
        assert!(RsaKeySet::from_jwks_json(r#"{"keys":[{"kty":"RSA","kid":"x"}]}"#).is_err());
        // Non-RSA keys are skipped rather than failing.
        let ok = RsaKeySet::from_jwks_json(r#"{"keys":[{"kty":"EC","kid":"x","crv":"P-256"}]}"#)
            .unwrap();
        assert!(ok.is_empty());
    }

    #[test]
    fn fn_verifier_adapter() {
        let v = FnVerifier::new("test-kms", |token: &str| {
            if token == "good" {
                Ok(FigJwtClaims::new("svc", u64::MAX, vec![]))
            } else {
                Err(JwtError::SignatureMismatch)
            }
        });
        assert_eq!(v.name(), "test-kms");
        assert!(v.verify("good").is_ok());
        assert!(v.verify("bad").is_err());
    }

    /// The same token must verify when the key is supplied as a PEM public key
    /// (the form a venue's KMS/HSM would export) instead of a JWKS entry.
    #[test]
    fn verifies_token_loaded_from_pem() {
        let mut set = RsaKeySet::new();
        set.add_pem(PUBLIC_KEY_NEW_PEM, Some("k-new")).unwrap();
        assert_eq!(set.kids(), vec!["k-new"]);
        assert!(set.verify(TOKEN_NEW.trim()).is_ok());
        // A token signed by a different key must not verify against it.
        assert!(matches!(
            set.verify(TOKEN_OLD.trim()),
            Err(JwtError::SignatureMismatch)
        ));
    }

    #[test]
    fn loads_keys_from_inline_json_or_path() {
        let inline = RsaKeySet::from_jwks_or_path(&jwks(&[("k-old", MODULUS_OLD)])).unwrap();
        assert!(inline.verify(TOKEN_OLD.trim()).is_ok());

        let dir = std::env::temp_dir().join(format!("fig-jwks-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("jwks.json");
        std::fs::write(&path, jwks(&[("k-new", MODULUS_NEW)])).unwrap();
        let from_file = RsaKeySet::from_jwks_or_path(path.to_str().unwrap()).unwrap();
        assert!(from_file.verify(TOKEN_NEW.trim()).is_ok());
        let _ = std::fs::remove_dir_all(&dir);

        assert!(RsaKeySet::from_jwks_or_path("/nonexistent/jwks.json").is_err());
    }
}
