# FIG Protocol Guide

Deep dive beyond [SPEC.md](../SPEC.md): design rationale, channel model, and
worked examples for production integrations.

## Design Goals

FIG unifies FIX session semantics, REST resource routing, and WebSocket
streaming over a single multiplexed TREE transport. Key goals:

- **Zero-RTT resumption** — sessions survive disconnects via resumption tokens.
- **Schema-native payloads** — CBOR for flexibility, SBE for zero-alloc hot paths.
- **Channel isolation** — each logical stream maps to a TREE bidirectional or
  unidirectional stream with independent sequence numbers.

## Connection Lifecycle

1. TLS 1.3 handshake over TREE (ALPN `fig/1`).
2. Optional 0-RTT session resumption with stored resumption token.
3. Control channel (ID 0) for PING/PONG, SETTINGS, AUTH_REFRESH, SEQ_RESET.
4. Application channels opened via STREAM_OPEN or implicit gateway mapping.

## Channel Directions

| Direction | Client offset | Server offset | Use case |
|---|---|---|---|
| Bidirectional | 0 | 1 | Request/response, FIX sessions |
| Send-only | 2 | 3 | Client order entry firehose |
| Recv-only | 3 | 2 | Server market data push |

Stream ID formula: `channel_id × 4 + offset`.

## Connection Migration

When the underlying IP changes (mobile clients, load balancer failover):

1. Call `FigConnection::prepare_migration()` before disconnect.
2. Reconnect via TREE migration or fresh handshake with resumption token.
3. Call `FigConnection::apply_migration()` to restore channel sequence state.

See `fig_core::migration` for token format and validation. Session stores
(`MemorySessionStore`, `FileSessionStore`, `RedisSessionStore`, `EtcdSessionStore`)
persist channel and sequence state across restarts.

## Security Primitives

| Module | Purpose |
|---|---|
| `fig_core::jwt` | HS256 JWT bearer tokens |
| `fig_core::oauth` | OAuth2/OIDC dev token introspection |
| `fig_core::channel_auth` | Per-channel permission requirements |
| `fig_core::rate_limit` | Token-bucket per-channel rate limiting |
| `fig_core::dos` | Connection-level DoS guard and flood detection |

## Worked Example: Order Entry

```text
Client                              Server
  | STREAM_OPEN (channel 1)            |
  |---------------------------------->|
  | REQUEST /orders (SBE payload)    |
  |---------------------------------->|
  | STREAM_ITEM (ExecutionReport)      |
  |<----------------------------------|
  | RESPONSE 200                       |
  |<----------------------------------|
```

## Codec Selection

| Payload | When to use |
|---|---|
| SBE | Production trading hot path (fixed schema) |
| CBOR | Self-describing, gateway translation |
| Protobuf | Schema-evolving enterprise integrations |
| JSON | REST gateway edge only |

## Session resume and gap fill

- **Resume:** persisted subscriptions are replayed when the client sends `SUBSCRIBE` with `Method: RESUME` and `ChannelPath: .well-known/resume` (same `SESSION_ID` as before disconnect when available).
- **Gap fill:** after a sequence gap on a live book or candle stream, issue `OrderBookRequest` or `CandleBarRequest` for a snapshot, then resume the subscription.
- **`request_stream`:** large historical responses (order history, candles) may arrive as multiple `STREAM_ITEM` frames plus `STREAM_CLOSE` instead of one `RESPONSE`.

## Further Reading

- [SPEC.md](../SPEC.md) — normative wire format
- [TUTORIAL.md](TUTORIAL.md) — getting started
- [GATEWAY.md](GATEWAY.md) — legacy protocol deployment
- [adr/](adr/) — architecture decision records ([0004: FSL source of truth](adr/0004-fsl-single-source-of-truth.md))
