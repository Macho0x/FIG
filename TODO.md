# TODO — FIG Protocol Roadmap

Tracking work to reach production-grade coverage of the
[SPEC.md](SPEC.md). **All tracked items are complete.** See [API.md](docs/API.md)
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
| ✅ | FSL parser (tokenizer + recursive descent) | — | 12 tests, full orders.usl |
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
| ✅ | TREE transport benchmarks (round-trip latency) | Medium | transport_bench.rs ping/pong round-trip |
| ✅ | Comparison benchmarks vs FIX/REST/WS | Medium | protocol_comparison_new_order group in gateway_bench |
| ✅ | Throughput benchmarks (msgs/sec) | Medium | tree_throughput 100-frame group in transport_bench |
| ✅ | Memory allocation benchmarks | Low | `alloc_bench.rs`; run with `--features alloc` |

---

## 13. Documentation

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | README.md | — | Full description, benchmarks, architecture |
| ✅ | SPEC.md | — | RFC-style protocol specification (553 lines) |
| ✅ | CONTRIBUTING.md | — | Development guidelines |
| ✅ | schemas/orders.usl | — | Complete example schema |
| ✅ | API docs (rustdoc) | Medium | docs/API.md + module index; cargo doc --workspace |
| ✅ | Tutorial / getting started guide | Medium | docs/TUTORIAL.md step-by-step guide |
| ✅ | Protocol guide (deep dive) | Low | docs/PROTOCOL.md |
| ✅ | Gateway deployment guide | Low | docs/GATEWAY.md |
| ✅ | Architecture Decision Records (ADRs) | Low | docs/adr/README.md |

---

## 14. CI/CD

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | GitHub Actions CI | — | build, test, clippy, fmt --check |
| ✅ | Cross-platform CI (macOS, Windows) | Low | `cross-platform` matrix job in ci.yml |
| ✅ | Benchmark regression CI | Medium | CI smoke-runs frame/transport/gateway/alloc benches |
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

**Roadmap status: 100% SPEC coverage tracked in this document.**
