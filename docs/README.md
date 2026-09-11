# FIG documentation

Read **top-down**. [SPEC.md](../SPEC.md) is the only normative wire document.
Guides below are how-to: they must match the spec, not replace it.

| If you want to… | Start here |
|---|---|
| Run something in 60 seconds | [README](../README.md#try-it-in-60-seconds) → [TUTORIAL.md](TUTORIAL.md) |
| Place an order / subscribe / query from Rust | [TUTORIAL.md](TUTORIAL.md) · copy-paste in [README worked examples](../README.md#worked-examples) |
| Understand frames, channels, 0-RTT | [PROTOCOL.md](PROTOCOL.md) · [SPEC.md](../SPEC.md) |
| Live `SUBSCRIBE` (held-open TREE stream) | [STREAMING.md](STREAMING.md) |
| Historical `REQUEST` / REST GET | [QUERY.md](QUERY.md) |
| Keep FIX / REST / WebSocket clients | [GATEWAY.md](GATEWAY.md) |
| Deploy a venue | [DEPLOYMENT.md](DEPLOYMENT.md) |
| Colo SBE order entry | [SBE_ORDER_PATH.md](SBE_ORDER_PATH.md) |
| Language bindings | [bindings/README.md](../bindings/README.md) · [PUBLISHING.md](PUBLISHING.md) |
| Crate / module map | [API.md](API.md) |
| Change FSL or the protocol | [CONTRIBUTING.md](../CONTRIBUTING.md) · [AGENTS.md](../AGENTS.md) · [adr/](adr/) |

**Crate version** (workspace `Cargo.toml`) is independent of
**protocol version** ([SPEC.md](../SPEC.md) `1.0.0`). Bump SPEC only on a wire break.

## Two I/O patterns (do not mix)

| Intent | Client API | Server behavior |
|---|---|---|
| Query / order (`REQUEST`) | `FigSdkClient::send_and_read` / `request_*` | Response, then **half-close** (finish + EOF) |
| Live stream (`SUBSCRIBE`) | `subscribe_*` for snapshot, or `subscribe_live` + `LiveSubscription::next_frame` | Snapshot, then **stream stays open** |

`send_and_read` on a live `SUBSCRIBE` waits for EOF and **hangs** on the
reference broker. Gateway REST GET uses `proxy_frame` (EOF); live WebSocket
uses `BackendSession` (held open).

## Binding caveats

- **TypeScript** smoke uses **Bun** (`bun:ffi`). There is no Node `node:ffi` runtime.
- **Python** `FigPyClient.request()` is the supported query path. `subscribe()` still
  waits for stream EOF and will hang on live exchange-sim SUBSCRIBE until a
  `LiveSubscription`-style helper lands.
- **C / FFI** `fig_version()` returns the crate version (`CARGO_PKG_VERSION`).
