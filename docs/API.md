# FIG — Fast Interchange Gateway

[![CI](https://github.com/Macho0x/fig/actions/workflows/ci.yml/badge.svg)](https://github.com/Macho0x/fig/actions/workflows/ci.yml)

A schema-native, multiplexed, zero-RTT protocol for trading systems.
Unifies and supersedes FIX, REST, and WebSocket.

## Quick start

See [docs/TUTORIAL.md](docs/TUTORIAL.md) for a step-by-step guide.

```bash
cargo test --workspace
cargo run -p fig-exchange-sim
cargo run -p fig-gateways --bin fig-gateway
```

## API documentation

Generate Rust API docs:

```bash
cargo doc --workspace --no-deps --open
```

Key public modules:

| Crate | Module | Purpose |
|---|---|---|
| `fig-core` | [`frame`](crates/fig-core/src/frame.rs) | Wire format encode/decode |
| `fig-core` | [`transport`](crates/fig-core/src/transport.rs) | TREE + TCP downgrade |
| `fig-core` | [`jwt`](crates/fig-core/src/jwt.rs) | JWT bearer authentication |
| `fig-core` | [`channel_auth`](crates/fig-core/src/channel_auth.rs) | Per-channel permissions |
| `fig-core` | [`observability`](crates/fig-core/src/observability.rs) | Metrics + Prometheus export |
| `fig-gateways` | [`fix`](crates/fig-gateways/src/fix.rs) | FIX 4.4 adapter |
| `fig-gateways` | [`rest`](crates/fig-gateways/src/rest.rs) | HTTP/1.1 adapter |
| `fig-gateways` | [`ws`](crates/fig-gateways/src/ws.rs) | WebSocket RFC 6455 adapter |
| `fig-fsl` | [`target_codegen`](crates/fig-fsl/src/target_codegen.rs) | Multi-language codegen |
