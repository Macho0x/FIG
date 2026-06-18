# TODO — FIG Protocol Roadmap

Tracking remaining work to reach production-grade 100% coverage of the
[SPEC.md](SPEC.md). Items are grouped by area and prioritized.

**Legend:** ✅ Done · 🔶 Partial · ⬜ Not started

---

## 1. Transport (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | QUIC transport via quinn 0.11 | — | ALPN `fig/1`, self-signed certs, 0-RTT |
| ✅ | FrameDecoder for streaming QUIC reads | — | Buffered, handles partial frames |
| ✅ | UnipConnection wrapper (open/send/recv/close) | — | Per-channel QUIC streams |
| 🔶 | 0-RTT session resumption end-to-end | High | FileSessionStore exists; QUIC 0-RTT not wired through transport |
| ⬜ | TCP downgrade mode (`FIG\x01` magic prefix) | Medium | Spec §2.1; for legacy environments without QUIC |
| ⬜ | Connection migration handling | Low | QUIC supports it; FIG channel reconstruction on migration not tested |
| ⬜ | QUIC stream reset → channel CLOSED transition | High | Spec §2; STOP_SENDING/RESET_STREAM must close channel, prevent leaks |

---

## 2. Wire Format (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | 16-byte fixed header encode/decode | — | All fields, round-trip tested |
| ✅ | TLV extension encode/decode | — | All 29 extension tags |
| ✅ | All 13 frame types | — | CONTROL through REDIRECT |
| ✅ | Frame flags (ACK_REQUESTED, COMPRESSED, FRAGMENTED, LAST_FRAGMENT) | — | Defined and tested |
| 🔶 | Payload fragmentation | Medium | FRAGMENTED + LAST_FRAGMENT flags defined; reassembly logic not implemented |
| ⬜ | Payload compression | Medium | COMPRESSED flag defined; no compression algorithm wired (zstd/zlib) |
| ⬜ | Reserved field validation (must be 0) | Low | Spec §3.1; decoder should reject non-zero reserved |

---

## 3. Control Frames (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | PING / PONG | — | Heartbeat |
| ✅ | GOAWAY | — | Graceful shutdown with reason |
| ✅ | SETTINGS | — | Tier advertisement |
| 🔶 | AUTH_REFRESH | High | ControlSubtype defined; token refresh flow not implemented |
| 🔶 | SEQ_RESET | High | ControlSubtype defined; sequence reset logic not implemented |
| ⬜ | RESEND | Medium | Spec §12.1; FIX ResendRequest (35=2) equivalent |

---

## 4. Channel Management (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Channel open/close/remove | — | 15 tests |
| ✅ | Sequence numbers (increment, wraparound, duplicate detection) | — | Spec-compliant |
| ✅ | QUIC stream ID mapping (client/server parity) | — | `channel_id * 4 + offset` |
| ✅ | Credit-based flow control | — | Per-channel credits, consume/grant, exhaustion error |
| 🔶 | Channel reconstruction after reconnect | High | Session stores channel list; transport doesn't rebuild QUIC streams from it |
| ⬜ | Channel ID leak prevention on stream reset | High | Spec §2; must detect RESET_STREAM and transition to CLOSED |
| ⬜ | Unidirectional channels | Low | Spec §2; only bidirectional implemented |

---

## 5. Session Management (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Session struct (UUID, auth token, channels, seq tracking) | — | 14 tests |
| ✅ | MemorySessionStore | — | HashMap-backed |
| ✅ | FileSessionStore | — | JSON file persistence, 6 tests |
| 🔶 | 0-RTT resumption integration | High | Store works; QUIC 0-RTT not wired through transport layer |
| ⬜ | Redis/etcd SessionStore | Low | Trait is abstract; production backends not implemented |
| ⬜ | Session expiry / TTL | Medium | No automatic expiry of idle sessions |
| ⬜ | Session migration (connection migration) | Low | Spec §1.1; sessions should survive IP changes |

---

## 6. Codec (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | CBOR encode/decode via ciborium | — | 6 tests |
| ✅ | SBE encode/decode for trading messages | — | 79 tests, zero-alloc |
| ✅ | SBE encode: NewOrderSingle, ExecutionReport, CancelRequest | — | Hot path |
| ⬜ | SBE encode: remaining message types | Medium | CancelReplace, MarketDataSnapshot, MarketDataUpdate, AccountSummary |
| ⬜ | SBE message header (schema ID, version, template ID) | Medium | Spec §10; SBE standard header not fully compliant |
| ⬜ | Protobuf codec | Low | Spec §10; for schema-evolving payloads |
| ⬜ | JSON codec (for REST gateway) | Medium | REST gateway uses string manipulation; proper JSON↔CBOR not implemented |

---

## 7. Authentication (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | AuthToken enum (Token, Mtls, DevToken) | — | 12 tests |
| ✅ | Token verification | — | Constant-time comparison |
| ✅ | mTLS certificate CN extraction | — | |
| ✅ | Dev tokens for testing | — | |
| ⬜ | JWT support | Medium | Spec §6; bearer token via JWT not implemented |
| ⬜ | Per-channel auth | Medium | Spec §6; auth scoped to individual channels |
| ⬜ | AUTH_REFRESH flow | High | Token expiry → refresh → resume; not implemented |
| ⬜ | OAuth2 / OIDC integration | Low | Enterprise auth |

---

## 8. Gateway Adapters (fig-gateways)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | FIX 4.4 parse/serialize | — | 8 tests, checksum computation |
| ✅ | FIX → FIG message conversion | — | NewOrderSingle, Cancel, ExecutionReport |
| ✅ | REST HTTP/1.1 parse/serialize | — | 7 tests |
| ✅ | WebSocket RFC 6455 frame parse/serialize | — | 14 tests, all opcodes, masking |
| 🔶 | FIX session state machine | High | Parse/serialize done; QuickFIX-style state machine (Logon/Logout/Heartbeat/ResendRequest/GapFill) not implemented |
| 🔶 | REST JSON ↔ CBOR body conversion | Medium | REST adapter parses HTTP; body conversion is string-based, not proper JSON↔CBOR |
| ⬜ | WebSocket → FIG stream mapping | Medium | WS frames parse; mapping to STREAM_ITEM with content-type not wired |
| ⬜ | FIX Logon (35=A) → STREAM_OPEN + AUTH | High | Spec §12.1; critical for FIX compatibility |
| ⬜ | FIX ResendRequest (35=2) → CONTROL(RESEND) | Medium | Spec §12.1 |
| ⬜ | REST SSE → STREAM_ITEM streaming | Low | Spec §12.2 |
| ⬜ | Gateway process (standalone binary) | Medium | Adapters are libraries; no standalone gateway server binary |

---

## 9. USL Schema Language (fig-usl)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | USL parser (tokenizer + recursive descent) | — | 12 tests, full orders.usl |
| ✅ | AST with all node types | — | Serde round-trip |
| ✅ | Rust codegen | — | 5 tests |
| ✅ | uslc CLI (compile, validate) | — | 10 tests |
| 🔶 | Enum codegen | Medium | Parser handles enums; codegen has `TODO_enum` placeholder |
| 🔶 | Inline struct codegen | Medium | Parser handles inline structs; codegen has `TODO_inline_struct` placeholder |
| ⬜ | Go codegen target | Medium | Spec §11.2 |
| ⬜ | Python codegen target | Low | Spec §11.2 |
| ⬜ | TypeScript codegen target | Low | Spec §11.2 |
| ⬜ | Protobuf `.proto` codegen | Medium | Spec §11.2; for gRPC interop |
| ⬜ | SBE `.xml` codegen | Medium | Spec §11.2; for trading fast path |
| ⬜ | JSON Schema `.json` codegen | Low | Spec §11.2; for REST docs |
| ⬜ | FIX mapping `.yaml` codegen | Low | Spec §11.2; for gateway config |
| ⬜ | USL → SBE Rust encode/decode impls | High | Currently SBE is hand-written; should be generated from USL |

---

## 10. Exchange Simulator (fig-exchange-sim)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Order book (price-time priority) | — | 8 tests |
| ✅ | Matching engine (market + limit orders) | — | 8 tests |
| ✅ | FIG server over QUIC | — | Order entry, cancel, market data, account query |
| ✅ | Integration tests (end-to-end) | — | 6 tests |
| 🔶 | Market data streaming (push updates) | Medium | Snapshot on subscribe; no incremental push on trade |
| ⬜ | CancelReplace (order modification) | Low | Method exists in matching engine; not wired to server |
| ⬜ | Order book depth streaming | Low | `bid_depth`/`ask_depth` exist; not pushed on change |
| ⬜ | Multi-symbol support in server | Low | Matching engine supports it; server hardcodes AAPL |
| ⬜ | Session resumption in server | High | Server doesn't persist sessions across restarts |

---

## 11. Observability (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Tracing spans (8 span constructors) | — | encode, decode, channel, session, connection, gateway |
| ✅ | Atomic metrics counters (10 counters) | — | frames_sent/recv, channels, sessions, errors, etc. |
| ✅ | Metrics snapshot | — | For Prometheus-style export |
| ⬜ | OpenTelemetry integration | Medium | Spans use `tracing`; OTel exporter not wired |
| ⬜ | Prometheus metrics endpoint | Medium | `Metrics::snapshot()` exists; HTTP exporter not implemented |
| ⬜ | Frame-level tracing (per-frame span) | Low | Spans exist; not instrumented in frame encode/decode hot path |
| ⬜ | Distributed trace context propagation | Medium | Trace ID in header; W3C TraceContext propagation not implemented |

---

## 12. Benchmarks (fig-bench)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Frame encode/decode benchmarks | — | 4 benchmarks |
| ✅ | CBOR vs SBE codec benchmarks | — | 6 benchmarks |
| ✅ | Gateway adapter benchmarks | — | 8 benchmarks |
| ✅ | Matching engine benchmarks | — | 3 benchmarks |
| ⬜ | QUIC transport benchmarks (round-trip latency) | Medium | End-to-end frame round-trip over localhost QUIC |
| ⬜ | Comparison benchmarks vs FIX/REST/WS | Medium | Same message through FIG native vs gateway vs raw FIX |
| ⬜ | Throughput benchmarks (msgs/sec) | Medium | Sustained throughput over 60s |
| ⬜ | Memory allocation benchmarks | Low | `cargo bench` with `--features alloc` |

---

## 13. Documentation

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | README.md | — | Full description, benchmarks, architecture |
| ✅ | SPEC.md | — | RFC-style protocol specification (553 lines) |
| ✅ | CONTRIBUTING.md | — | Development guidelines |
| ✅ | schemas/orders.usl | — | Complete example schema |
| ⬜ | API docs (rustdoc) | Medium | `cargo doc` works; no custom documentation |
| ⬜ | Tutorial / getting started guide | Medium | Step-by-step for new users |
| ⬜ | Protocol guide (deep dive) | Low | Beyond SPEC; design rationale, examples |
| ⬜ | Gateway deployment guide | Low | How to run FIG alongside legacy FIX/REST |
| ⬜ | Architecture Decision Records (ADRs) | Low | Key design decisions and trade-offs |

---

## 14. CI/CD

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | GitHub Actions CI | — | build, test, clippy, fmt --check |
| ⬜ | Cross-platform CI (macOS, Windows) | Low | Currently Linux only |
| ⬜ | Benchmark regression CI | Medium | Compare bench results against baseline |
| ⬜ | Coverage reporting | Medium | `cargo tarpaulin` or `cargo llvm-cov` |
| ⬜ | Release workflow | Low | Tagged releases with changelog |
| ⬜ | Docker image | Low | Containerized fig-exchange-sim |

---

## 15. Security

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | TLS 1.3 via QUIC | — | Mandatory, integrated |
| ✅ | Self-signed cert generation | — | For development |
| 🔶 | mTLS | Medium | Auth module supports it; server doesn't require client certs |
| ⬜ | Certificate rotation | Medium | No runtime cert reload |
| ⬜ | Rate limiting | Medium | No per-channel or per-connection rate limiting |
| ⬜ | DoS protection | Low | QUIC provides some; no FIG-level protection |
| ⬜ | Security audit | Low | Pre-1.0 audit |

---

## Priority Summary

### High Priority (blocking production use)
1. 0-RTT session resumption end-to-end (transport + session + server)
2. Channel reconstruction after reconnect
3. Channel ID leak prevention on QUIC stream reset
4. FIX session state machine (Logon/Logout/Heartbeat/ResendRequest)
5. FIX Logon → STREAM_OPEN + AUTH
6. AUTH_REFRESH flow
7. SEQ_RESET control frame logic
8. USL → SBE Rust codegen (replace hand-written SBE)
9. Session resumption in exchange-sim server

### Medium Priority (important for adoption)
1. TCP downgrade mode
2. Payload fragmentation + reassembly
3. Payload compression (zstd)
4. SBE encode for remaining message types
5. JSON codec for REST gateway
6. WebSocket → FIG stream mapping
7. JWT auth support
8. Per-channel auth
9. USL enum + inline struct codegen
10. Go/Protobuf/SBE codegen targets
11. OpenTelemetry + Prometheus export
12. QUIC transport benchmarks
13. API docs + tutorial
14. Coverage reporting in CI
15. mTLS enforcement in server
16. Market data streaming (push updates)
17. Gateway standalone binary

### Low Priority (nice to have)
1. Connection migration
2. Unidirectional channels
3. Redis/etcd session store
4. Session expiry/TTL
5. Protobuf codec
6. OAuth2/OIDC
7. REST SSE streaming
8. Python/TypeScript/JSON Schema/FIX mapping codegen
9. CancelReplace in server
10. Multi-symbol server
11. Frame-level tracing instrumentation
12. Memory allocation benchmarks
13. Protocol deep-dive guide
14. ADRs
15. Cross-platform CI
16. Docker image
17. Rate limiting / DoS protection
18. Security audit
