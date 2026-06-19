# FIG Getting Started Tutorial

This guide walks through connecting to the FIG exchange simulator, placing
an order, and subscribing to market data.

## Prerequisites

- Rust 1.75+ (`rustup` recommended)
- Linux (CI-tested; macOS/Windows may work)

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

The server binds to `127.0.0.1:7443` (UDP/TREE) by default and persists
sessions under `/tmp/fig-exchange-sessions`.

## 2. Run the legacy gateway (optional)

Translate FIX and REST into FIG frames without a native FIG client:

```bash
cargo run -p fig-gateways --bin fig-gateway
```

- REST gateway: `http://127.0.0.1:8080`
- FIX gateway: `tcp://127.0.0.1:9876`

Example REST request:

```bash
curl -X POST http://127.0.0.1:8080/trading/orders \
  -H 'Content-Type: application/json' \
  -d '{"cl_ord_id":"ORD-1","symbol":"AAPL","side":"Buy","order_qty":100,"price":150.25}'
```

## 3. Prometheus metrics

Export FIG counters for scraping:

```bash
cargo run -p fig-core --features observability-bin --bin fig-observability
curl http://127.0.0.1:9090/metrics
```

## 4. Compile FSL schemas

```bash
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.usl --lang rust --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.usl --lang go --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.usl --lang proto --out /tmp/fig-gen
```

Supported `--lang` values: `rust`, `sbe`, `go`, `proto`, `sbe-xml`, `cpp`, `csharp`.

## 5. TCP downgrade mode

For environments without TREE/QUIC:

```rust
use fig_core::tcp::{FigTcpConnection, FigTcpServer};

// Server accepts FIG\x01 magic prefix, single-stream multiplexing
```

See `fig_core::tcp` module documentation and unit tests for round-trip examples.

## 6. Authentication

### Bearer / JWT

```rust
use fig_core::jwt::{encode_jwt, verify_jwt_bearer, FigJwtClaims};

let claims = FigJwtClaims::new("trader-1", exp_unix_secs, vec!["orders:write".into()]);
let token = encode_jwt(&claims, "shared-secret")?;
let auth = verify_jwt_bearer(&token, "shared-secret")?;
```

### Per-channel permissions

```rust
use fig_core::ChannelAuthPolicy;

let policy = ChannelAuthPolicy::new()
    .require_for_channel(5, vec!["marketdata:read".into()]);
assert!(policy.authorize(5, &auth));
```

## 7. Run benchmarks

```bash
cargo bench -p fig-bench
```

Benchmark groups include frame codec, TREE round-trip latency, gateway
adapter comparison (FIX vs REST vs native FIG), and matching engine throughput.

## Next steps

- Read [SPEC.md](SPEC.md) for the full protocol specification
- See [TODO.md](TODO.md) for the implementation roadmap
- Run integration tests: `cargo test -p fig-exchange-sim --test integration`
