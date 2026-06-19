# FIG Getting Started Tutorial

This guide walks through connecting to the FIG exchange simulator, placing
an order, and subscribing to market data.

## Prerequisites

- Rust 1.75+ (`rustup` recommended)
- Linux, macOS, or Windows (CI-tested on all three)

```bash
git clone https://github.com/Macho0x/fig.git
cd fig
cargo build --workspace
cargo test --workspace
```

## 1. Start the exchange simulator

```bash
cargo run -p fig-exchange-sim
```

The server binds to **`127.0.0.1:8443`** (UDP/TREE) by default and persists
sessions under `/tmp/fig-exchange-sessions`.

Optional mTLS mode:

```bash
FIG_MTLS=1 cargo run -p fig-exchange-sim
```

## 2. Run the native client

In another terminal:

```bash
cargo run -p fig-cli
```

The client connects to `127.0.0.1:8443` and demonstrates order entry, market
data subscription, account query, and PING/PONG over a single TREE connection.

## 3. Run the legacy gateway (optional)

Translate FIX and REST into FIG frames without a native FIG client:

```bash
cargo run -p fig-gateways --bin fig-gateway
```

- REST gateway: `http://127.0.0.1:8080`
- FIX gateway: `tcp://127.0.0.1:9876`

See [GATEWAY.md](GATEWAY.md) for deployment details. The gateway currently
translates frames in-process; wire it to your FIG backend for production use.

Example REST request:

```bash
curl -X POST http://127.0.0.1:8080/trading/orders \
  -H 'Content-Type: application/json' \
  -d '{"cl_ord_id":"ORD-1","symbol":"AAPL","side":"Buy","order_qty":100,"price":150.25}'
```

## 4. Prometheus metrics

Export FIG counters for scraping:

```bash
cargo run -p fig-core --features observability-bin --bin fig-observability
curl http://127.0.0.1:9090/metrics
```

## 5. Compile FSL schemas

```bash
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.usl --lang rust --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.usl --lang go --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.usl --lang proto --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- validate schemas/orders.usl
```

Supported `--lang` values:

`rust`, `sbe`, `go`, `proto`, `sbe-xml`, `cpp`, `csharp`, `python`, `typescript`, `ocaml`, `zig`, `json-schema`, `fix-yaml`

## 6. TCP downgrade mode

For environments without TREE/QUIC:

```rust
use fig_core::tcp::{FigTcpConnection, FigTcpServer};

// Server accepts FIG\x01 magic prefix, single-stream multiplexing
```

See `fig_core::tcp` module documentation and unit tests for round-trip examples.

## 7. Authentication

### Bearer / JWT

```rust
use fig_core::jwt::{encode_jwt, verify_jwt_bearer, FigJwtClaims};

let claims = FigJwtClaims::new("trader-1", exp_unix_secs, vec!["orders:write".into()]);
let token = encode_jwt(&claims, "shared-secret")?;
let auth = verify_jwt_bearer(&token, "shared-secret")?;
```

### OAuth2 (dev introspection)

```rust
use fig_core::oauth::{OAuthValidator, OAuthTokenInfo};

let validator = OAuthValidator::new("https://auth.example.com")
    .register_token("access-token", OAuthTokenInfo { /* … */ });
let auth = validator.validate("access-token")?;
```

### Per-channel permissions

```rust
use fig_core::ChannelAuthPolicy;

let policy = ChannelAuthPolicy::new()
    .require_for_channel(5, vec!["marketdata:read".into()]);
assert!(policy.authorize(5, &auth));
```

## 8. Run benchmarks

```bash
cargo bench -p fig-bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
```

Benchmark groups include frame codec, TREE round-trip latency, gateway
adapter comparison (FIX vs REST vs native FIG), matching engine throughput,
and allocation patterns.

## 9. Docker

```bash
docker build -t fig-exchange-sim .
docker run --rm -p 8443:8443/udp fig-exchange-sim
```

## Next steps

- Read [SPEC.md](../SPEC.md) for the full protocol specification
- Read [PROTOCOL.md](PROTOCOL.md) for design rationale and migration patterns
- See [API.md](API.md) for the module index
- Run integration tests: `cargo test -p fig-exchange-sim --test integration`
