# JWT Signing-Key Rotation Runbook

How to rotate the key that signs FIG `AUTH_TOKEN` bearer tokens (RS256) without
dropping live sessions.

FIG **verifies** tokens; the venue **issues** them. The private key lives in the
venue's KMS/HSM and never reaches FIG. FIG only needs the *public* half, which it
accepts as a JWKS document (`FIG_JWT_JWKS` or
[`fig_core::jwt_keys::RsaKeySet`](../crates/fig-core/src/jwt_keys.rs)).

> Symmetric HS256 (`FIG_JWT_SECRET`) has no rotation story that avoids a flag day:
> every verifier shares the signing secret. Move to RS256 before you need
> rotation. HS256 remains supported for development.

## Key identities

| Item | Where it lives | Who owns it |
|---|---|---|
| Private key (signing) | Venue KMS/HSM | Venue |
| Public key (`n`, `e`) | JWKS document served to FIG | Venue publishes, FIG consumes |
| `kid` | JWT header + JWKS entry | Venue |
| Trusted key set | `FIG_JWT_JWKS` (JSON or file path) on every FIG node | Operator |

## Invariants

1. **A token is accepted only if its `kid` matches a configured key** (a token
   with no `kid` is tried against every configured key).
2. **Configure more than one key during rotation.** Both old and new must be
   present until every token signed by the old key has expired.
3. **Retire a key only after `max(token lifetime) + clock skew`** has elapsed
   since the issuer stopped using it.
4. **Never remove the last key.** An empty key set rejects every token.

## Rotation procedure (zero downtime)

Let `T` be the token lifetime (e.g. 15 min) and `S` the allowed clock skew.

| Step | Action | Verify |
|---|---|---|
| 0 | Generate a new keypair in the KMS; note its `kid` (e.g. `k-2026-10`). | `openssl rsa -pubin -noout -text` on the exported public key |
| 1 | **Publish** the new public key in the JWKS alongside the old one. | JWKS endpoint contains both `kid`s |
| 2 | **Configure both keys on every FIG node** (`FIG_JWT_JWKS`), then reload/restart. | `RsaKeySet::kids()` lists both; a token for the new `kid` verifies |
| 3 | **Wait** for config to be live everywhere: `≥ T + S`. Tokens signed by the old key must not be trusted by a node that lacks it. | Issue a token with the new `kid`; it verifies on every node |
| 4 | **Switch the issuer** to sign with the new `kid`. | New tokens carry `kid=k-2026-10` and verify |
| 5 | **Wait** `≥ T + S` so all old-key tokens expire. | No token with `kid=k-old` presented (check auth logs) |
| 6 | **Retire**: remove the old key from the JWKS and from every node's config. | Old-key tokens are rejected with `SignatureMismatch` |

Steps 2→4 are the overlap window. Reversing step 4 (issuer rolls back to the old
key) is safe as long as you have not performed step 6.

## Configuration

```bash
# A JWKS URL is fetched by the venue's deploy tooling; FIG reads the document
# itself (inline JSON, or a path to a file):
FIG_JWT_JWKS=/etc/fig/jwks.json
# or
FIG_JWT_JWKS='{"keys":[{"kty":"RSA","kid":"k-2026-10","n":"...","e":"AQAB"}]}'
```

Programmatic use (embedders, tests, KMS-backed verification):

```rust
use fig_core::jwt_keys::{JwtVerifier, RsaKeySet};

// Trusted set: keep both keys configured across a rotation.
let keys = RsaKeySet::from_jwks_json(&std::fs::read_to_string("/etc/fig/jwks.json")?)?;
let claims = keys.verify(bearer_token)?;
```

### KMS/HSM-backed verification

If the venue never exports key material, wrap its `Verify` API instead of
configuring public keys:

```rust
use fig_core::jwt_keys::FnVerifier;

let verifier = FnVerifier::new("venue-kms", |token: &str| {
    // Call the KMS `Verify` operation (kid from the token header) and map the
    // result into FigJwtClaims; return JwtError::SignatureMismatch on failure.
    venue_kms::verify_jwt(token)
});
```

An `FnVerifier` closure must still enforce `alg` (reject `none`/HS256 confusion),
`exp`, and `sub` — mirror [`fig_core::jwt::decode_jwt`](../crates/fig-core/src/jwt.rs).

## Emergency revocation

Signing-key rotation revokes *everything*, not one stolen token. For a single
compromised token:

1. Reject the offending `session_id` / `sub` at the venue's authorization layer
   (FIG re-checks private paths on every frame).
2. Shorten token lifetimes so the blast radius is bounded (`≤ 15 min` typical).
3. If the *key* is compromised, run the procedure above from step 0 with a new
   keypair and skip the overlap: publish only the new key and accept a brief
   window of failed handshakes rather than honoring attacker-signed tokens.

## Verification checklist

```bash
# JWKS parses and lists the expected kids
cargo test -p fig-core --lib jwt_keys::tests::rotation_overlap_accepts_both_keys

# A retired key is rejected
cargo test -p fig-core --lib jwt_keys::tests::rotation_retirement_rejects_old_key

# Tampered / expired / alg=none tokens are rejected
cargo test -p fig-core --lib jwt_keys::tests
```

In production, after each step confirm with a real token: the venue's token
issuer produces one, and the FIG node's auth log shows the accepted `kid`.

## Audit trail

Record, per rotation: old `kid`, new `kid`, step timestamps, the operator, and
the ticket. Retain at least one prior rotation cycle — this is the evidence that
the overlap window was respected.
