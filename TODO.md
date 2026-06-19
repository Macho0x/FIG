# TODO — FIG Protocol Roadmap

Tracking work to reach production-grade coverage of the
[SPEC.md](SPEC.md). **Sections 1–15 (Rust core) are complete.** Section 16
tracks multi-language SDK parity — the active roadmap. See [API.md](docs/API.md)
for the module index and [PROTOCOL.md](docs/PROTOCOL.md) for integration guidance.

**Legend:** ✅ Done · 🔶 Partial · ⬜ Not started

---

## 1. Transport (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | TREE transport via quinn 0.11 | — | ALPN `fig/1`, self-signed certs, 0-RTT |
| ✅ | FrameDecoder for streaming TREE reads | — | Buffered, handles partial frames |
| ✅ | FigConnection wrapper (open/send/recv/close) | — | Per-channel TREE streams |
| ✅ | 0-RTT session resumption end-to-end | — | FigClient::connect_0rtt + FigServer::accept_0rtt with rejection fallback; TODO: production replay protection |
| ✅ | TCP downgrade mode (`FIG\x01` magic prefix) | Medium | Spec §2.1; FigTcpConnection/FigTcpServer over plain TCP |
| ✅ | Connection migration handling | Low | `migration` module + `FigConnection::prepare_migration` / `apply_migration` |
| ✅ | TREE stream reset → channel CLOSED transition | — | StreamReset/StreamStopped errors detected, force_close_channel transitions to Closed |

---

## 2. Wire Format (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | 16-byte fixed header encode/decode | — | All fields, round-trip tested |
| ✅ | TLV extension encode/decode | — | All 29 extension tags |
| ✅ | All 13 frame types | — | CONTROL through REDIRECT |
| ✅ | Frame flags (ACK_REQUESTED, COMPRESSED, FRAGMENTED, LAST_FRAGMENT) | — | Defined and tested |
| ✅ | Payload fragmentation | Medium | FRAGMENTED + LAST_FRAGMENT split/reassembly via FragmentReassembler |
| ✅ | Payload compression | Medium | zstd compression via compression module; COMPRESSED flag wired |
| ✅ | Reserved field validation (must be 0) | Low | Spec §3.1; decoder rejects non-zero reserved and flag bit 7 |

---

## 3. Control Frames (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | PING / PONG | — | Heartbeat |
| ✅ | GOAWAY | — | Graceful shutdown with reason |
| ✅ | SETTINGS | — | Tier advertisement |
| ✅ | AUTH_REFRESH | — | Frame constructor + token extraction + verify_refresh_token + control dispatcher |
| ✅ | SEQ_RESET | — | Frame constructor + payload parsing + control dispatcher + ChannelManager::reset_seq |
| ✅ | RESEND | Medium | Frame::resend + control dispatcher + FIX ResendRequest conversion |

---

## 4. Channel Management (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Channel open/close/remove | — | 15 tests |
| ✅ | Sequence numbers (increment, wraparound, duplicate detection) | — | Spec-compliant |
| ✅ | TREE stream ID mapping (client/server parity) | — | `channel_id * 4 + offset` |
| ✅ | Credit-based flow control | — | Per-channel credits, consume/grant, exhaustion error |
| ✅ | Channel reconstruction after reconnect | — | ChannelManager::reconstruct from stored session, reopen TREE streams |
| ✅ | Channel ID leak prevention on stream reset | High | force_close_channel on StreamReset/StreamStopped in FigConnection |
| ✅ | Unidirectional channels | Low | `ChannelDirection` + `open_unidirectional_channel` + direction checks |

---

## 5. Session Management (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Session struct (UUID, auth token, channels, seq tracking) | — | 14 tests |
| ✅ | MemorySessionStore | — | HashMap-backed |
| ✅ | FileSessionStore | — | JSON file persistence, 6 tests |
| ✅ | 0-RTT resumption integration | — | TREE 0-RTT wired through transport; FileSessionStore persists sessions |
| ✅ | Redis/etcd SessionStore | Low | `RedisSessionStore` + `EtcdSessionStore` trait impls (in-memory backend for CI) |
| ✅ | Session expiry / TTL | Medium | Session::is_expired + store TTL on get/purge_expired |
| ✅ | Session migration (connection migration) | Low | `migration::apply_migration` restores session seq state across IP changes |

---

## 6. Codec (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | CBOR encode/decode via ciborium | — | 6 tests |
| ✅ | SBE encode/decode for trading messages | — | 79 tests, zero-alloc |
| ✅ | SBE encode: NewOrderSingle, ExecutionReport, CancelRequest | — | Hot path |
| ✅ | SBE encode: remaining message types | — | CancelReplace, MarketDataSnapshot, MarketDataIncrementalRefresh, CancelReject (generated from FSL) |
| ✅ | SBE message header (schema ID, version, template ID) | — | Generated SBE headers compliant with Spec §10 |
| ✅ | Protobuf codec | Low | `protobuf` module: json_to_protobuf / protobuf_to_json wire encoding |
| ✅ | JSON codec (for REST gateway) | Medium | json_to_cbor/cbor_to_json in fig-core codec module |

---

## 7. Authentication (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | AuthToken enum (Token, Mtls, DevToken) | — | 12 tests |
| ✅ | Token verification | — | Constant-time comparison |
| ✅ | mTLS certificate CN extraction | — | |
| ✅ | Dev tokens for testing | — | |
| ✅ | JWT support | Medium | HS256 encode/decode + verify_jwt_bearer in jwt module |
| ✅ | Per-channel auth | Medium | ChannelAuthPolicy with per-channel permission requirements |
| ✅ | AUTH_REFRESH flow | — | Token refresh (client→server), verify_refresh_token, control dispatcher |
| ✅ | OAuth2 / OIDC integration | Low | `OAuthValidator` with dev token introspection registry |

---

## 8. Gateway Adapters (fig-gateways)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | FIX 4.4 parse/serialize | — | 8 tests, checksum computation |
| ✅ | FIX → FIG message conversion | — | NewOrderSingle, Cancel, ExecutionReport |
| ✅ | REST HTTP/1.1 parse/serialize | — | 7 tests |
| ✅ | WebSocket RFC 6455 frame parse/serialize | — | 14 tests, all opcodes, masking |
| ✅ | FIX session state machine | — | FixSession with states (LoggedOut/LogonSent/LoggedIn/LogoutSent), MsgSeqNum tracking, Heartbeat, ResendRequest, GapFill |
| ✅ | REST JSON ↔ CBOR body conversion | Medium | Delegates to fig_core::codec json_to_cbor/cbor_to_json |
| ✅ | WebSocket → FIG stream mapping | Medium | ws_to_fig_frame / fig_to_ws_frame with CONTENT_TYPE |
| ✅ | FIX Logon (35=A) → STREAM_OPEN + AUTH | — | logon_to_stream_open + stream_open_to_logon conversion functions |
| ✅ | FIX ResendRequest (35=2) → CONTROL(RESEND) | Medium | resend_request_to_control + control_to_resend_request |
| ✅ | REST SSE → STREAM_ITEM streaming | Low | `sse` module: parse_sse_chunk / sse_to_fig_stream_item |
| ✅ | Gateway process (standalone binary) | Medium | fig-gateway binary: REST + FIX TCP listeners |

---

## 9. FSL Schema Language (fig-fsl)

| Status | Item | Priority | Notes |
|---|----|----|----|
| ✅ | FSL parser (tokenizer + recursive descent) | — | 12 tests, full orders.fsl |
| ✅ | AST with all node types | — | Serde round-trip |
| ✅ | Rust codegen | — | 5 tests |
| ✅ | ftlc CLI (compile, validate) | — | 10 tests |
| ✅ | Enum codegen | — | from_value/to_value with explicit discriminators |
| ✅ | Inline struct codegen | — | ENCODED_LEN constant, fixed-size struct generation |
| ✅ | Go codegen target | Medium | GoCodegen in target_codegen.rs + ftlc --lang go |
| ✅ | Python codegen target | Low | PythonCodegen + ftlc --lang python |
| ✅ | TypeScript codegen target | Low | TypeScriptCodegen + ftlc --lang typescript |
| ✅ | C++ codegen target | Medium | CppCodegen → generated.hpp |
| ✅ | C# codegen target | Medium | CsharpCodegen → Generated.cs |
| ✅ | OCaml codegen target | Low | OcamlCodegen + ftlc --lang ocaml |
| ✅ | Zig codegen target | Low | ZigCodegen + ftlc --lang zig |
| ✅ | Protobuf `.proto` codegen | Medium | ProtoCodegen + ftlc --lang proto |
| ✅ | SBE `.xml` codegen | Medium | SbeXmlCodegen + ftlc --lang sbe-xml |
| ✅ | JSON Schema `.json` codegen | Low | JsonSchemaCodegen + ftlc --lang json-schema |
| ✅ | FIX mapping `.yaml` codegen | Low | FixYamlCodegen + ftlc --lang fix-yaml |
| ✅ | FSL → SBE Rust encode/decode impls | — | Generated from FSL alongside hand-written, cross-validated, 7 messages |

---

## 10. Exchange Simulator (fig-exchange-sim)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Order book (price-time priority) | — | 8 tests |
| ✅ | Matching engine (market + limit orders) | — | 8 tests |
| ✅ | FIG server over TREE | — | Order entry, cancel, market data, account query |
| ✅ | Integration tests (end-to-end) | — | 6 tests |
| ✅ | Market data streaming (push updates) | Medium | Subscription registry + build_market_data_push on trade |
| ✅ | CancelReplace (order modification) | Low | `/replace` path wired to `process_replace` |
| ✅ | Order book depth streaming | Low | `push_book_depth` on every book change |
| ✅ | Multi-symbol support in server | Low | Symbol from routing key/order; no hardcoded default |
| ✅ | Session resumption in server | — | FileSessionStore wired into handle_connection, sessions persist across restarts |

---

## 11. Observability (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Tracing spans (8 span constructors) | — | encode, decode, channel, session, connection, gateway |
| ✅ | Atomic metrics counters (10 counters) | — | frames_sent/recv, channels, sessions, errors, etc. |
| ✅ | Metrics snapshot | — | For Prometheus-style export |
| ✅ | OpenTelemetry integration | Medium | init_tracing() + tracing-subscriber env-filter; OTel-ready |
| ✅ | Prometheus metrics endpoint | Medium | fig-observability binary serves /metrics |
| ✅ | Frame-level tracing (per-frame span) | Low | `span_encode`/`span_decode` in Frame::encode/decode hot path |
| ✅ | Distributed trace context propagation | Medium | W3C traceparent via trace module + TRACE_ID extension |

---

## 12. Benchmarks (fig-bench)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Frame encode/decode benchmarks | — | 4 benchmarks |
| ✅ | CBOR vs SBE codec benchmarks | — | 6 benchmarks |
| ✅ | Gateway adapter benchmarks | — | 8 benchmarks |
| ✅ | Matching engine benchmarks | — | 3 benchmarks |
| ✅ | TREE transport benchmarks (round-trip latency) | Medium | transport_bench.rs ping/pong round-trip; CI smoke-tests this only |
| ✅ | Comparison benchmarks vs FIX/REST/WS | Medium | protocol_comparison_new_order group in gateway_bench |
| 🔶 | Throughput benchmarks (msgs/sec) | Medium | `tree_throughput` removed from transport_bench pending server-side multi-frame read |
| ✅ | Memory allocation benchmarks | Low | `alloc_bench.rs`; run with `--features alloc` |

If you want the full transport throughput benchmark back in CI later, we'll need a proper server-side multi-frame read path first.

---

## 13. Documentation

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | README.md | — | Full description, benchmarks, architecture |
| ✅ | SPEC.md | — | RFC-style protocol specification (553 lines) |
| ✅ | CONTRIBUTING.md | — | Development guidelines |
| ✅ | schemas/orders.fsl | — | Complete example schema |
| ✅ | API docs (rustdoc) | Medium | docs/API.md + module index; cargo doc --workspace |
| ✅ | Tutorial / getting started guide | Medium | docs/TUTORIAL.md step-by-step guide |
| ✅ | Protocol guide (deep dive) | Low | docs/PROTOCOL.md |
| ✅ | Gateway deployment guide | Low | docs/GATEWAY.md |
| ✅ | Architecture Decision Records (ADRs) | Low | docs/adr/README.md + 0004 FSL source of truth |

---

## 14. CI/CD

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | GitHub Actions CI | — | build, test, clippy, fmt --check |
| ✅ | Cross-platform CI (macOS, Windows) | Low | `cross-platform` matrix job in ci.yml |
| ✅ | Benchmark regression CI | Medium | CI smoke-runs frame/transport (ping-pong only)/gateway/alloc benches |
| ✅ | Coverage reporting | Medium | cargo llvm-cov job in CI workflow |
| ✅ | Release workflow | Low | `.github/workflows/release.yml` on version tags |
| ✅ | Docker image | Low | Dockerfile for fig-exchange-sim |

---

## 15. Security

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | TLS 1.3 via TREE | — | Mandatory, integrated |
| ✅ | Self-signed cert generation | — | For development |
| ✅ | mTLS | Medium | server_config_mtls + FIG_MTLS=1 in exchange-sim |
| ✅ | Certificate rotation | Medium | RotatingServerCerts with reload + rebuild config |
| ✅ | Rate limiting | Medium | ChannelRateLimiter token-bucket per channel |
| ✅ | DoS protection | Low | `DoSGuard` + `FloodDetector` in dos module |
| ✅ | Security audit | Low | docs/SECURITY_AUDIT.md pre-1.0 checklist |

---

## Priority Summary

### High Priority (blocking production use)
All high-priority items complete ✅

### Medium Priority (important for adoption)
All medium-priority items complete ✅

### Low Priority (nice to have)
All low-priority items complete ✅

**Roadmap status: Rust core (§1–15) — 100% SPEC coverage complete. Multi-language SDK parity (§16) — not started.**

---

## 16. Multi-Language SDK Parity

Bring non-Rust languages to full SPEC parity with Rust. Today Rust has (1) rich
**FSL codegen** (`RustCodegen` + `--lang sbe`) and (2) a complete **`fig-core`
runtime** (~20 modules). All other `ftlc --lang` targets emit flat message shapes
only — no typed enums, nested types, serializers, or protocol client.

Schema evolution policy: [ADR 0004](docs/adr/0004-fsl-single-source-of-truth.md).

### Parity gap (current state)

| Capability | Rust | Python / C++ / C# / OCaml / Zig / Go / TS |
|---|---|---|
| FSL typed enums | ✅ | ❌ (int/string) |
| Nested struct types | ✅ | ❌ |
| Type aliases + constraints | ✅ | ❌ |
| Message metadata constants | ✅ | ❌ (`CHANNEL_TYPE`, `CORRELATION_FIELD`, …) |
| CBOR / SBE / Protobuf serializers | ✅ | ❌ |
| SBE encode/decode codegen | ✅ (`--lang sbe`) | ❌ |
| Protocol runtime (frames, channels, transport) | ✅ `fig-core` | ❌ |
| Native client SDK | ✅ `fig-cli` | ❌ |

SPEC §11.2 overclaims for some targets until §16.2 lands — see
[ADR 0004](docs/adr/0004-fsl-single-source-of-truth.md) for schema evolution policy.

### Definition of "full spec"

Align with [SPEC.md §13 Compliance Tiers](SPEC.md):

| Tier | Capability | Deliverable |
|---|---|---|
| **1 — Core** | Frame encode/decode, REQUEST/RESPONSE, extensions | Frame + ext library |
| **2 — Session** | STREAM_OPEN/CLOSE, seq nums, CONTROL, auth | Channel + session + control |
| **3 — Pub/Sub** | SUBSCRIBE, ROUTING_KEY, STREAM_ITEM | Streaming client API |
| **4 — Advanced** | ACK_RANGE, FLOW_CONTROL, FRAGMENT, COMPRESSION, migration | Full transport features |

**Full spec parity** = Tier 4 client + FSL codegen parity + passes cross-language
conformance tests against Rust reference vectors.

### Strategy (two tracks)

**Track A — FSL codegen parity** (extend `ftlc` / `target_codegen.rs`):
typed enums, nested types, type aliases, message metadata constants, per-language
CBOR/SBE/Protobuf serializers. Do not rewrite `fig-core` nine times without shared
conformance vectors.

**Track B — Protocol runtime parity** (pick per language):

1. **Rust core + FFI bindings** (recommended default) — `fig-ffi` (cbindgen → C ABI)
   + PyO3 / P/Invoke / cgo / ctypes / `@cImport`. Fastest path to behavioral parity.
2. **Generated protocol library** (pure per language) — frame/ext/channel/control
   generated from normative spec. Best for C++ / Zig (performance-sensitive, FFI-averse).
3. **Gateway proxy** (operational parity, not native) — Rust service embeds
   `fig-gateways`, forwards to FIG backend. Good for migration; not Tier 4 native FIG.

---

### 16.1 Conformance Foundation

Language-neutral test vectors that every SDK must pass. Rust `fig-exchange-sim`
integration tests are the reference behavior.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | Conformance test vector format spec | High | Hex fixtures for frames, CBOR, SBE; document in `tests/conformance/` |
| ⬜ | Frame encode/decode vectors | High | Round-trip against `fig-core::frame` golden output |
| ⬜ | CBOR payload vectors | High | `NewOrderSingle`, `ExecutionReport`, … — snake_case fields, serde enum strings (`"Buy"`) |
| ⬜ | SBE payload vectors | High | `schema_id=0x01`, template IDs; verify vs `sbe_generated.rs` |
| ⬜ | Channel stream-ID mapping vectors | Medium | `channel_id * 4 + offset` client/server cases |
| ⬜ | End-to-end integration scripts | Medium | connect → order → execution report; language-agnostic driver |
| ⬜ | CI matrix for conformance runners | Medium | Rust reference + per-language SDK jobs |

---

### 16.2 FSL Codegen Parity (Track A)

Mirror [`RustCodegen`](crates/fig-fsl/src/codegen.rs) for every `--lang` target.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | Typed enum generation (all targets) | High | Named variants, not bare `int` |
| ⬜ | Nested struct types (all targets) | High | `PriceLevel`, `MarketDataUpdate`, inline structs |
| ⬜ | Type aliases with constraint docs | Medium | `ClientOrderId`, `Price`, `Quantity`, … |
| ⬜ | Message metadata constants | Medium | `CHANNEL_TYPE`, `CORRELATION_FIELD`, `PRIORITY`, `IDEMPOTENT` per message |
| ⬜ | Python: dataclasses + CBOR serializers | High | `cbor2` or `msgspec`; field names match Rust serde |
| ⬜ | Go: structs + JSON/CBOR tags | High | `encoding/json` + CBOR lib |
| ⬜ | C++: structs + CBOR/JSON helpers | High | nlohmann/json or custom CBOR |
| ⬜ | C#: classes + System.Text.Json + CBOR | High | Nullable reference types for optionals |
| ⬜ | TypeScript: interfaces + CBOR encode/decode | Medium | Browser vs Node packaging |
| ⬜ | OCaml: records + variant enums + CBOR | Low | yojson/cbor ppx or hand-rolled |
| ⬜ | Zig: structs + CBOR helpers | Medium | std/json or `@cImport` to shared C codec |
| ⬜ | SBE encode/decode codegen per language | Medium | Extend beyond Rust-only `sbe_codegen.rs`, or document + verify `sbe-xml` → SBE tool pipeline |
| ⬜ | Verify `sbe-xml` vs Rust `--lang sbe` wire compatibility | Medium | Do not assume identical without cross-validation |
| ⬜ | Java codegen target (`--lang java`) | Low | Listed in SPEC §11.2 but not in `ftlc` today |
| ✅ | Update SPEC §11.2 claims to match reality | Low | Done — see ADR 0004 + SPEC §11.2; track serializer parity in §16.2 |
| ⬜ | Codegen tests per target (orders.fsl) | High | Enums, nested types, serializers in CI |

---

### 16.3 `fig-ffi` + Language Bindings (Track B, Option 1)

Wrap `fig-core` once; expose stable C ABI; bind per language.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | `crates/fig-ffi` crate | High | cbindgen → `fig.h`; staticlib + cdylib |
| ⬜ | FFI API surface spec | High | `fig_client_connect`, `fig_frame_encode/decode`, send/recv/close, … |
| ⬜ | `fig-python` (PyO3 / maturin) | High | Reference SDK; `pip install fig` target |
| ⬜ | `fig-csharp` (P/Invoke) | Medium | Common in trading desks |
| ⬜ | `fig-go` (cgo) | Medium | cgo wrapper over `fig-ffi` |
| ⬜ | `fig-cpp` (header + link staticlib) | Medium | Low-latency client path |
| ⬜ | `fig-ocaml` (ctypes) | Low | ctypes binding over `fig-ffi` |
| ⬜ | `fig-zig` (`@cImport fig.h`) | Low | Comptime-friendly thin wrapper |
| ⬜ | TypeScript / Node native addon or WASM | Medium | Browser → gateway; Node → addon or WASM build of `fig-core` |
| ⬜ | Binding conformance tests | High | Each binding passes §16.1 vectors |

High-level client API (all bindings):

```text
FigClient::connect(addr, tls_config)
FigClient::open_channel(path, schema_id)
FigClient::send_request(channel, payload, content_type)
FigClient::recv_stream() → stream of frames
FigClient::ping() / close()
```

---

### 16.4 SDK Feature Rollout (SPEC Tiers)

Roll out incrementally per binding; do not expose all 20 `fig-core` modules at once.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | Tier 1 — frames, REQUEST/RESPONSE, CBOR payloads | High | MVP: trade against exchange-sim |
| ⬜ | Tier 2 — STREAM_OPEN/CLOSE, JWT auth, PING/PONG, seq nums | High | Session parity with FIX tier |
| ⬜ | Tier 3 — SUBSCRIBE, market data streams, correlation | Medium | Pub/sub parity |
| ⬜ | Tier 4 — 0-RTT resumption, migration, fragmentation, zstd | Low | Full protocol parity |

---

### 16.5 Server + Gateway (multi-codec & proxy)

End-to-end parity requires server and gateway changes, not just client SDKs.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | Exchange-sim multi-codec dispatch | High | Today CBOR-only; negotiate via `ContentType` extension |
| ⬜ | SBE payload decode in exchange-sim | High | Dispatch on `application/fig+sbe` + `schema_id` |
| ⬜ | Protobuf payload decode in exchange-sim | Medium | Dispatch on `application/x-protobuf` + `schema_id` |
| ⬜ | Multi-codec integration tests | High | CBOR × SBE × Protobuf × each language binding |
| ⬜ | Wire `fig-gateway` to proxy to FIG backend | Medium | Today translates in-process only; does not forward to `:8443` |
| ⬜ | Production gateway service template | Medium | Embed `fig-gateways` adapters; REST/WS/FIX → native FIG |

---

### 16.6 Pure Protocol Libraries (Track B, Option 2)

For languages where FFI is unacceptable — optional alternative to §16.3.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | Generated frame encode/decode (spec-driven) | Medium | 16-byte header + TLV extensions |
| ⬜ | Generated extension tag library | Medium | All 29 tags from `ext.rs` |
| ⬜ | Generated channel manager | Medium | Stream ID formula, seq nums |
| ⬜ | Generated control frame dispatcher | Medium | PING/PONG, AUTH_REFRESH, SEQ_RESET, … |
| ⬜ | TREE transport binding per language | Low | Native FIG client over UDP/TLS (ALPN `fig/1`); or TCP downgrade (`FIG\x01`) where TREE is unavailable |
| ⬜ | C++ pure protocol library | Medium | HFT / low-latency path without Rust runtime dep |
| ⬜ | Zig pure protocol library | Low | `@cImport` or comptime-generated frame codec |

---

### 16.7 Per-Language Recommended Path

| Language | Codegen (§16.2) | Runtime path | Notes |
|---|---|---|---|
| Python | enums + CBOR serializers | PyO3 → `fig-core` | Best ROI; ship first |
| C# | classes + System.Text.Json | P/Invoke → `fig-ffi` | Common in trading |
| C++ | full structs + SBE | Generated protocol or cbindgen | Low-latency HFT |
| Go | structs + tags + CBOR | cgo or pure Go port | `--lang go` exists |
| TypeScript | interfaces + CBOR | WASM or Node native addon | Browser uses gateway |
| OCaml | variant enums + CBOR | ctypes → `fig-ffi` | Quant/research |
| Zig | struct layout + comptime | `@cImport fig.h` or pure Zig | Comptime-friendly codegen |
| Java | add `--lang java` first | JNI → `fig-ffi` | SPEC lists; not in `ftlc` yet |

---

### 16.8 Anti-Patterns (do not do)

- **Do not** treat REST gateway as native FIG parity — gateway demo does not proxy to exchange-sim.
- **Do not** hand-port `fig-core` eight times without §16.1 conformance vectors — wire drift is guaranteed.
- **Do not** stop at flat dataclasses/structs — that is ~5% of Rust capability.
- **Do not** assume `sbe-xml` and Rust `--lang sbe` are wire-identical without verification.

---

### 16.9 Target End State

For each non-Rust language:

```text
ftlc --lang {lang}  →  types + enums + nested types + CBOR/SBE codecs + message constants
{fig}-{lang} SDK    →  Tier 4 client (connect, channels, auth, streaming, migration)
conformance tests   →  same vectors as Rust, CI green
exchange-sim        →  accepts CBOR + SBE + Protobuf from all SDKs
```

**Minimal viable parity** (recommended first milestone):

```text
FSL codegen (full) + PyO3/FFI client (Tier 2) + CBOR only + conformance tests
```

---

### 16.10 Priority Summary (§16)

| Priority | Items |
|---|---|
| **High** | §16.1 conformance vectors · §16.2 enum/nested/serializer codegen · §16.3 `fig-ffi` + Python binding · §16.4 Tier 1–2 SDK · §16.5 multi-codec exchange-sim |
| **Medium** | C# / Go / C++ bindings · SBE per-language codegen · gateway proxy · TypeScript SDK · pure protocol libs (C++) |
| **Low** | OCaml / Zig bindings · Tier 4 advanced features · Java target · pure Zig protocol lib |

**Suggested implementation order:**

1. Conformance test vectors (§16.1)
2. Extend `target_codegen` — enums, nested types, serializers (§16.2)
3. `fig-ffi` + Python binding (§16.3) — reference SDK
4. Wire gateway to exchange-sim (§16.5) — HTTP path for langs without a native TREE client yet
5. Multi-codec server dispatch (§16.5)
6. C++ / C# / Go bindings (§16.3)
7. Pure-generated protocol libs where FFI is unacceptable (§16.6)
