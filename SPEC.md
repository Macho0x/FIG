# FIG — Protocol Specification

**Version:** 1.0.0
**Status:** Stable

---

## 1. Introduction

FIG (Fast Interchange Gateway) is a binary, multiplexed,
schema-native protocol designed for high-performance trading systems and
real-time financial applications. It runs over TREE and unifies the
interaction patterns of FIX (session-oriented order flow), REST
(resource-oriented request-response), and WebSocket (bidirectional
streaming) into a single wire format.

### 1.1 Goals

- **One connection, all patterns.** Request-response, streaming, pub/sub,
  and session-oriented messaging over a single TREE connection with
  per-stream flow control.
- **Schema-native.** Every frame carries a Schema ID. Messages are
  validated at the protocol layer.
- **Free observability.** Trace IDs, correlation IDs, and nanosecond
  timestamps are protocol-level header fields.
- **Zero-RTT session continuity.** Sessions survive disconnects and
  resume with TREE 0-RTT.
- **Backwards compatible.** Gateways translate to/from FIX, REST, and
  WebSocket for migration.

### 1.2 Terminology

| Term | Expansion | What it is |
|------|-----------|------------|
| **FIG** | Fast Interchange Gateway | The protocol — a schema-native, multiplexed binary protocol for trading systems. Unifies FIX, REST, and WebSocket semantics over a single transport. |
| **TREE** | Trunked Reliable Encrypted Exchange | FIG's mandatory transport layer (implementation: quinn crate). Provides 0-RTT resumption, 65,535 concurrent channels per connection, and mandatory TLS 1.3 encryption. |
| **FSL** | Fig Schema Language | FIG's schema definition language (IDL). Defines messages, channels, and gateway mappings. Compiles to Rust, Go, SBE, JSON Schema, FIX mappings, and other type-only targets. |
| **Connection** | — | A TREE connection between a client and server. |
| **Channel** | — | A logical conversation within a connection, mapped 1:1 to a TREE stream. |
| **Frame** | — | The unit of communication — a typed binary message on a channel. |
| **Session** | — | A durable, migratable logical entity identified by SESSION_ID. |
| **Extension** | — | A typed key-value pair in the extension header (TLV). |
| **Schema** | — | A versioned message definition in FSL, identified by a Schema ID. |

### 1.3 Scope

FIG is a **messaging protocol** — framing, transport, schemas, and wire-level
security — at the same layer as FIX, REST, and WebSocket. It does **not**
define account onboarding, API key generation, credential storage, matching
engines, or ledger logic. Venues own that infrastructure; FIG defines how an
already-issued credential is carried on frames, validated, and scoped to
`ChannelPath` segments (see §9.3). This is analogous to FIX Logon carrying
username/password without specifying how passwords are created.

---

## 2. Transport

FIG uses **TREE** as its mandatory native transport.

- **ALPN:** `fig/1`
- **TLS:** 1.3 mandatory (provided by TREE)
- **Stream mapping:** Each FIG channel maps 1:1 to a TREE stream.
  - Bidirectional channels → bidirectional TREE streams
  - Unidirectional channels → unidirectional TREE streams
- **Channel 0** is reserved for connection-level control frames
  (PING, PONG, GOAWAY, SETTINGS, AUTH_REFRESH).

TREE provides per-stream multiplexing (no head-of-line blocking across
channels), 0-RTT session resumption, connection migration (NAT rebinding
survival), and integrated TLS 1.3.

### 2.1 TCP Downgrade

For legacy environments without TREE support, FIG defines a TCP fallback
mode using the same binary framing with a magic prefix (`FIG\x01`) for
first-byte protocol detection. This mode loses multiplexing benefits
(channels are serialized over one TCP stream) and is intended only for
gateway use.

---

## 3. Wire Format

### 3.1 Frame Layout

Every FIG frame has a **fixed 16-byte header** followed by an optional
**extension block** and an optional **payload**.

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                            Length (32)                        |  bytes 0-3
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Type (8)   |  Flags (8)   |        Channel ID (16)         |  bytes 4-7
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Stream Seq (32)                        |  bytes 8-11
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| HeaderCount  |  SchemaID    |          reserved (16)         |  bytes 12-15
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|   Extension Block (variable: HeaderCount × TLV entries)       |  bytes 16+
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|   Payload (variable)                                          |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

**`Length` (32 bits, unsigned):** Total frame size in bytes, including the
16-byte header. Minimum value: 16 (header only, no extensions, no payload).

**`Type` (8 bits):** Frame type. See §4.

**`Flags` (8 bits):** Bitfield. See §5.

**`Channel ID` (16 bits, unsigned):** Identifies the logical channel.
Channel 0 = control. Range: 0–65535.

**`Stream Seq` (32 bits, unsigned):** Monotonically increasing per-channel
sequence number. Used for ordering, gap detection, idempotency, and
deduplication. Wraps at 2^32. Reset via CONTROL(SEQ_RESET) on channel 0.

**`HeaderCount` (8 bits):** Number of TLV extension entries that follow
the fixed header. 0 = no extension block. Range: 0–255.

**`Schema ID` (8 bits):** Identifies the payload schema.
- `0x00`: No schema — payload is self-describing (CBOR) or raw bytes.
- `0x01–0xEF`: Well-known schema IDs (registered).
- `0xF0–0xFE`: Dynamic schema IDs (negotiated per-connection via SETTINGS).
- `0xFF`: Schema fingerprint in a SCHEMA_FINGERPRINT extension (64-bit hash).

**`reserved` (16 bits):** Must be 0. Reserved for future use.

### 3.2 Extension Block

When `HeaderCount > 0`, the extension block follows the fixed header. Each
extension is a TLV (Tag-Length-Value) entry:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|          Tag (16)            |          Length (16)           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                    Value (Length bytes)                       |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

**`Tag` (16 bits):** Extension tag identifier. See §6.

**`Length` (16 bits):** Length of the Value field in bytes.

**`Value` (variable):** The extension value. Interpretation depends on the
tag. Well-known tags have defined value types (see §6).

### 3.3 Payload

The payload follows the extension block (or the fixed header if
`HeaderCount == 0`). Its length is:

```
payload_len = Length - 16 - extension_block_len
```

Payload encoding is determined by `Schema ID` and the `CONTENT_TYPE`
extension:
- Schema ID 0x00 + no CONTENT_TYPE → raw bytes
- Schema ID 0x00 + CONTENT_TYPE `application/cbor` → CBOR (default for gateway and reference broker)
- Well-known Schema ID → **CBOR** (human-readable / JSON-interop path) or **SBE**
  (`application/fig+sbe`) on the trading fast path

FIG 1.0 wire codecs are **CBOR + SBE only**. Protobuf payloads were removed from
the reference implementation; do not send `application/protobuf` on FIG frames.

---

## 4. Frame Types

| Code | Name | Channel | Semantics |
|---|---|---|---|
| `0x00` | CONTROL | 0 only | Connection-level control (PING, PONG, GOAWAY, SETTINGS, AUTH_REFRESH, SEQ_RESET) |
| `0x01` | REQUEST | any | Expects exactly one RESPONSE or STREAM_ERROR on this channel |
| `0x02` | RESPONSE | any | Final response to a REQUEST; channel closes after this |
| `0x03` | STREAM_OPEN | any | Opens a new stream; carries channel metadata (session, mode, auth) |
| `0x04` | STREAM_ITEM | any | One item in an open stream (bidirectional data) |
| `0x05` | STREAM_CLOSE | any | Graceful close of a stream |
| `0x06` | STREAM_ERROR | any | Error that terminates the stream |
| `0x07` | ONE_WAY | any | Fire-and-forget; no response expected, channel closes immediately |
| `0x08` | SUBSCRIBE | any | Subscribe to a topic/routing key for streaming updates |
| `0x09` | UNSUBSCRIBE | any | Unsubscribe from a topic/routing key |
| `0x0A` | ACK_RANGE | any | Acknowledge receipt of a range of Stream Seq numbers |
| `0x0B` | FLOW_CONTROL | any | Message-level backpressure: "accept at most N more messages" |
| `0x0C` | REDIRECT | any | Channel migrated to a different server; reconnect with token |

### 4.1 Control Sub-Types

CONTROL frames on channel 0 carry a control sub-type in the first byte of
the payload (or in a dedicated extension). Defined sub-types:

| Sub-type | Code | Purpose |
|---|---|---|
| PING | `0x00` | Heartbeat request |
| PONG | `0x01` | Heartbeat response |
| GOAWAY | `0x02` | Server is shutting down; includes grace period |
| SETTINGS | `0x03` | Exchange connection parameters (max channels, compression, schema versions) |
| AUTH_REFRESH | `0x04` | Refresh authentication token |
| SEQ_RESET | `0x05` | Reset Stream Seq for a channel (after sequence wrap or recovery) |
| RESEND | `0x06` | Request retransmission of a seq range (guaranteed-delivery channels) |

---

## 5. Flags

| Bit | Name | Meaning |
|---|---|---|
| 0 | `EXTENSIONS` | Extension block present (HeaderCount > 0). Redundant with HeaderCount but explicit. |
| 1 | `COMPRESSED` | Payload compressed (algorithm from SETTINGS negotiation) |
| 2 | `PRIORITY` | High-priority message — expedite delivery |
| 3 | `ENCRYPTED` | Payload encrypted at application layer (end-to-end through gateways) |
| 4 | `FRAGMENTED` | Message continues in subsequent frame(s) |
| 5 | `LAST_FRAGMENT` | This is the last fragment of a fragmented message |
| 6 | `ACK_REQUESTED` | Sender requests an ACK_RANGE for this message |
| 7 | `reserved` | Must be 0 |

---

## 6. Extension Tags

### 6.1 Well-Known Tags

| Tag | Name | Value Type | Purpose |
|---|---|---|---|
| `0x0001` | REQUEST_URI | UTF-8 string | Resource path: `/accounts/12345/orders` |
| `0x0002` | RESPONSE_URI | UTF-8 string | Effective URI after routing |
| `0x0003` | CONTENT_TYPE | ASCII string | MIME-like: `application/fig+sbe`, `application/cbor`, `application/json` |
| `0x0004` | STATUS_CODE | uint16 | HTTP-compatible status: 200, 404, 500, plus custom |
| `0x0005` | CORRELATION_ID | 16-byte binary | Links request → response across channels |
| `0x0006` | SEQUENCE_NUM | uint64 | FIX MsgSeqNum compatibility (distinct from Stream Seq) |
| `0x0007` | SESSION_ID | 16-byte binary | Identifies a FIX session or logical session |
| `0x0008` | TIMESTAMP | int64 | Nanoseconds since Unix epoch |
| `0x0009` | TTL_MILLIS | uint32 | Message validity window for queued/buffered delivery |
| `0x000A` | ROUTING_KEY | UTF-8 string | Pub/sub topic: `marketdata.NYSE.AAPL.quotes` |
| `0x000B` | SCHEMA_FINGERPRINT | uint64 | 64-bit hash of schema definition |
| `0x000C` | CONTENT_ENCODING | ASCII string | `zstd`, `lz4`, `snappy`, `gzip` |
| `0x000D` | METHOD | ASCII string | `GET`, `PUT`, `POST`, `DELETE`, `PATCH` (REST mapping) |
| `0x000E` | USER_AGENT | UTF-8 string | Client identification |
| `0x000F` | AUTH_TOKEN | binary | Bearer token, JWT, or custom auth blob |
| `0x0010` | ACCEPT | ASCII string | Accepted content types for response |
| `0x0011` | CACHE_CONTROL | UTF-8 string | `no-cache`, `max-age=3600` |
| `0x0012` | IDEMPOTENCY_KEY | binary | Safe retry of non-idempotent requests |
| `0x0013` | TRACE_ID | 16-byte binary | Distributed tracing correlation |
| `0x0014` | ERROR_CODE | UTF-8 string | Machine-readable: `UNAUTHORIZED`, `INVALID_ORDER_QTY` |
| `0x0015` | ERROR_MESSAGE | UTF-8 string | Human-readable error description |
| `0x0016` | CHANNEL_MODE | ASCII string | `stateless`, `session`, `affinity` |
| `0x0017` | CHANNEL_PATH | UTF-8 string | Unified channel address: `trading/accounts/123/orders` |
| `0x0018` | REDIRECT_TARGET | UTF-8 string | Target server address for REDIRECT frames |
| `0x0019` | REDIRECT_TOKEN | binary | Session token for redirect reconnection |
| `0x001A` | ACK_RANGE_START | uint32 | Start of acknowledged seq range |
| `0x001B` | ACK_RANGE_END | uint32 | End of acknowledged seq range |
| `0x001C` | FLOW_CONTROL_CREDIT | uint32 | Additional message credits granted |
| `0x001D` | SCOPE | UTF-8 string | Authorization scope: `trading:orders:write` |

### 6.2 Custom Tags

Tags `0x8000–0xFFFF` are reserved for private/custom use. They MUST NOT
collide with well-known tags. Implementations SHOULD ignore unknown
custom tags rather than erroring.

---

## 7. Channel Model

### 7.1 Channel Lifecycle

```
                    ┌─────────────┐
                    │   CLOSED    │
                    └──────┬──────┘
                           │ CONNECT (TREE handshake, ALPN "fig/1")
                    ┌──────▼──────┐
                    │  CONNECTED  │ (Channel 0 active)
                    └──┬───┬───┬──┘
                       │   │   │
          STREAM_OPEN  │   │   │  REQUEST (implicit channel)
               ┌───────▼┐  │  ┌▼──────────────┐
               │STREAMING│  │  │REQUEST_PENDING│
               │  (open) │  │  └──────┬────────┘
               └──┬───┬──┘  │         │
                  │   │     │    ┌────▼────┐
      STREAM_ITEM │   │     │    │RESPONSE │
      (either dir)│   │     │    └────┬────┘
                  │   │     │         │ (channel closes)
                  │   │     │    ┌────▼────┐
                  │   │     │    │  CLOSED │
                  │   │     │    └─────────┘
      STREAM_CLOSE│   │     │
      (graceful)  │   │     │
                  │   │     │
          ┌───────▼┐  │     │
          │ CLOSING│  │     │  (drain pending items)
          └────┬───┘  │     │
               │      │     │
          ┌────▼──────▼─────▼──┐
          │ DISCONNECTED       │ (GOAWAY or transport close)
          └────────────────────┘
```

### 7.2 Channel Modes

Negotiated at STREAM_OPEN via the CHANNEL_MODE extension:

| Mode | Semantics | Legacy equivalent |
|---|---|---|
| `stateless` | Each REQUEST is independent. No session state. Channel opens and closes per request. | REST |
| `session` | SESSION_ID establishes a durable session. Sequence numbers track ordering. State persists across reconnections. | FIX |
| `affinity` | Channel is sticky to a backend. State is implicit in the connection. | WebSocket |

### 7.3 Interaction Patterns

| Pattern | Frame sequence | Example |
|---|---|---|
| Request-Response | REQUEST → RESPONSE (channel closes) | REST GET, RPC, FIX order |
| Request-Stream | REQUEST → STREAM_ITEM × N → STREAM_CLOSE | Market data subscription |
| Bidirectional Stream | STREAM_OPEN → STREAM_ITEM ↔ STREAM_ITEM → STREAM_CLOSE | WebSocket, FIX session |
| Fire-and-forget | ONE_WAY (channel closes immediately) | Metrics, audit push |
| Pub/Sub (subscribe) | SUBSCRIBE → STREAM_ITEM × N | Market data |
| Pub/Sub (publish) | STREAM_ITEM or ONE_WAY with ROUTING_KEY | Event dissemination |

---

## 8. Session Model

A FIG session is a **durable, migratable logical entity** identified by
SESSION_ID (16-byte binary). It is NOT tied to a single connection.

### 8.1 Session Lifecycle

1. **Establish:** Client sends STREAM_OPEN with SESSION_ID (new or existing)
   and AUTH_TOKEN on a channel.
2. **Active:** Messages flow over channels associated with the session.
   Sequence numbers (Stream Seq + optional SEQUENCE_NUM extension) track
   ordering.
3. **Suspend:** Connection drops. Session state persists on the server
   (in a durable store).
4. **Resume:** Client reconnects (TREE 0-RTT), sends STREAM_OPEN with the
   same SESSION_ID and a start-seq extension. Server restores channel
   state. Outstanding orders remain alive. Subscriptions resume.
5. **Migrate:** Server sends REDIRECT with REDIRECT_TARGET and
   REDIRECT_TOKEN. Client reconnects to the new server with 0-RTT + token.
   Session continues transparently.

### 8.2 Session Store

Servers maintain session state in a durable store (memory, file, or Redis via
`fig-core` feature `session-redis`). State includes:
- Session ID and authentication context
- Open channels and their modes
- Last sent/received sequence numbers per channel
- Active subscriptions
- Outstanding orders (for trading sessions)

---

## 9. Addressing

FIG uses **unified channel paths** with type annotations:

```
channel://broker.example.com:8443/trading/accounts/12345/orders?type=request_response
channel://broker.example.com:8443/marketdata/NYSE/AAPL/quotes?type=pub_sub
channel://broker.example.com:8443/analytics/risk/VaR?portfolio=main&type=request_stream
```

- The path is hierarchical and human-readable (REST-like).
- The `type` parameter declares the interaction pattern.
- Carried in the CHANNEL_PATH extension.
- For pub/sub, ROUTING_KEY may also be used (dot-separated topics).

**Native FIG is canonical.** Every broker capability must exist as native FIG
frames on TREE first (`SUBSCRIBE` for live push, `REQUEST`/`RESPONSE` or
`REQUEST`/`STREAM_ITEM` for historical pulls). REST GET and WebSocket topics
are **gateway adapters** that translate to and from the same FSL payloads —
they must not introduce queries or streams that native FIG lacks. See
[ADR 0006](docs/adr/0006-broker-api-parity.md).

**Routing rules:**
- If CHANNEL_PATH is present → route to the path handler.
- If ROUTING_KEY is present → match against subscriptions, fan out.
- If both → route to path handler AND fan out to subscribers.
- If neither → deliver to the specific channel (WebSocket equivalent).

**Capability discovery:** `GET /.well-known/capabilities` (or
`CapabilitiesRequest` on TREE) returns the server's channel tree (available
paths, types, schemas).

### 9.1 Native stream catalog (§17)

Paths below are implemented by the reference broker (`fig-exchange-sim`) and
mapped by `fig-gateways` when `--fig-backend` is set. Gateway WS aliases refer
to Binance-style topics; Hyperliquid JSON subscriptions map via `ws_catalog.rs`.

#### Public market data (no auth)

| Native `CHANNEL_PATH` | Pattern | FSL payload(s) | Gateway WS alias (examples) |
|---|---|---|---|
| `marketdata/{symbol}/book` | pub/sub + GET | `OrderBookSnapshot`, `OrderBookDelta` | `@depth` |
| `marketdata/{symbol}/quotes` | pub/sub + GET | `MarketDataSnapshot`, incremental | `@depth` (legacy alias) |
| `marketdata/{symbol}/bbo` | pub/sub | `BestBidOffer` | `@bookTicker` |
| `marketdata/{symbol}/trades` | pub/sub + GET | `PublicTradeEvent`, `PublicTradeBatch` | `@trade` |
| `marketdata/{symbol}/aggtrades` | pub/sub + GET | `AggregateTradeEvent`, `AggregateTradeBatch` | `@aggTrade` |
| `marketdata/{symbol}/candles/{interval}` | pub/sub + GET | `CandleBarEvent`, `CandleBarBatch` | `@kline_{interval}` |
| `marketdata/{symbol}/ticker` | pub/sub + GET | `SymbolTicker` | `@ticker` |
| `marketdata/ticker/all` | pub/sub + GET | `MiniTicker`, `AllMidsBatch` | `@miniTicker`, `allMids` |
| `marketdata/{symbol}/mark` | pub/sub + GET | `MarkPriceUpdate` | `@markPrice`, `activeAssetCtx` |
| `marketdata/liquidations` | pub/sub | `LiquidationTrade` | `@forceOrder` |

Intervals use FIG names (`1m`, `5m`, `1h`, `1d`) — not Binance “klines”.

#### Private account / trading (auth required)

| Native `CHANNEL_PATH` | Pattern | FSL payload(s) | Notes |
|---|---|---|---|
| `trading/accounts/{account}/executions` | pub/sub | `ExecutionReport` | Fill fan-out on match |
| `trading/accounts/{account}/orders/open` | GET | `OpenOrdersSnapshot` | Snapshot query |
| `trading/accounts/{account}/orders` | GET (+ stream) | `OrderHistoryBatch` | `request_stream` when >50 rows |
| `accounts/{account}` | GET | `AccountSummary` | Account snapshot |
| `accounts/{account}/balances` | pub/sub | `BalanceSnapshot`, `BalanceUpdate` | Snapshot on subscribe |
| `accounts/{account}/positions` | pub/sub | `PositionSnapshot`, `PositionUpdate` | Delta on fill |
| `accounts/{account}/margin` | pub/sub + GET | `MarginSummary`, `MarginUpdate` | Live margin on subscribe/fill |
| `accounts/{account}/fills` | GET | `FillHistoryBatch` | User trade history |
| `accounts/{account}/funding` | pub/sub + GET | `FundingPayment`, `FundingHistoryBatch` | |
| `accounts/{account}/ledger` | pub/sub + GET | `LedgerUpdate`, `LedgerHistoryBatch` | Fee on fill |
| `accounts/{account}/liquidations` | pub/sub | `UserLiquidation` | Push on balance breach |
| `/.well-known/capabilities` | GET | `CapabilitiesResponse` | Exchange info / path catalog |
| `/.well-known/instruments` | GET | `InstrumentCatalogResponse` | Perp/spot instrument catalog |

`Symbol` supports up to 32 uppercase characters (equities and crypto tickers).
Instrument metadata is defined in `instruments.fsl` — see
[ADR 0007](docs/adr/0007-crypto-instrument-model.md).

`OrderListStatus` is defined in FSL; live stream wiring is available on
`trading/accounts/{account}/orderlists` in the reference broker.

#### Well-known schema IDs

| ID | FSL domain | Notes |
|---|---|---|
| `0x01` | Trading (`orders.fsl`) | Wire schema for all merged messages today |
| `0x02` | Market data (`marketdata.fsl`) | Logical domain; advertised in capabilities |
| `0x03` | Account (`account.fsl`) | Logical domain; advertised in capabilities |

All frames currently use wire **`schema_id = 0x01`** with distinct SBE template IDs
per message. Future releases may assign separate wire IDs per domain without
changing `CHANNEL_PATH` semantics (see ADR 0004).

#### ROUTING_KEY convention

- Dot-separated legacy topics: `marketdata.AAPL.quotes`, `hl.orderUpdates.alice`.
- Gateway WS adapters populate `ROUTING_KEY` from Binance `@topic` or Hyperliquid
  subscription JSON; native clients may omit it when `CHANNEL_PATH` is sufficient.
- When both are present, brokers route by path **and** fan out by routing key.

#### Correlation / subscription identity

- **`CORRELATION_ID` extension (tag `0x0005`)** carries the subscribing channel ID
  (decimal string) on `SUBSCRIBE`, subscribe ack `RESPONSE`, and `STREAM_ITEM`.
- Clients use this to multiplex many streams on one TREE session; it mirrors
  Binance/Hyperliquid subscription ack IDs in gateway egress JSON.

#### Multi-stream sessions

A single TREE connection may hold many concurrent channels (public MD, private
account, historical `REQUEST` channels). Each `SUBSCRIBE` opens a dedicated channel
with its own sequence space and `CORRELATION_ID`.

#### Snapshot-then-delta contract

Stream payloads include optional **`is_snapshot: true`** on the first item after
`SUBSCRIBE` (balances, positions, candles, BBO, ticker, order book). Subsequent
items set `is_snapshot: false`. Order book also carries monotonic **`sequence`**
on snapshots and deltas; gap-fill uses `OrderBookRequest` / `CandleBarRequest` with
time ranges (§9.2).

### 9.2 Historical query patterns (§7.3)

| Result size | Pattern | FIG frames | Example |
|---|---|---|---|
| Small (≤ few KB) | Request-Response | `REQUEST` → `RESPONSE` | `CandleBarRequest` → `CandleBarBatch` |
| Large / paginated | Request-Stream | `REQUEST` → `STREAM_ITEM` × N → `STREAM_CLOSE` | Order history (>50 rows in exchange-sim) |
| Live ongoing | Pub/Sub | `SUBSCRIBE` → `STREAM_ITEM` × N | Candle stream, balance updates |

Batch requests accept optional `start_time`, `end_time`, `limit`, and `cursor`.
Batch responses include `has_more` and optional `next_cursor` (see `PageInfo` in
`account.fsl`). Gap-fill after live disconnect: client sends `CandleBarRequest`
or `FillHistoryRequest` for the missing window, then resumes `SUBSCRIBE`.

REST `GET` paths and query parameters map to the same `ChannelPath` and CBOR
request bodies via `fig_gateways::rest_query`. See [docs/QUERY.md](docs/QUERY.md).

### 9.3 Private stream auth and scoping

**Protocol scope:** FIG specifies the `AUTH_TOKEN` extension, scoping rules, and
error semantics for private paths. **Credential issuance** (admin UI, key
rotation, JWT signing services, account databases) is venue infrastructure —
outside this specification. Gateways and brokers verify credentials their own
systems issued; FIG only defines the on-wire contract.

Private paths under `accounts/{account}/…` and `trading/accounts/{account}/…`
require authentication on every frame that opens or uses the channel:

| Mechanism | Extension / transport | Scope |
|---|---|---|
| Dev token (simulator) | `AUTH_TOKEN` = `fig-dev-{account}` | Account in path must match token suffix |
| Bearer / JWT | `AUTH_TOKEN` bearer or JWT claims | Subject must match `{account}` in path |
| mTLS | Client certificate CN | CN mapped to permitted account(s) |
| Dev bypass | Server env `FIG_DEV_OPEN=1` | Skips auth check (local testing only) |

**Scoping rules:**

1. A client authenticated as account `A` MUST NOT subscribe or query paths for
   account `B` — the broker returns `403` / `STREAM_ERROR`.
2. Public market data paths (`marketdata/…`) do not require auth unless the
   venue policy adds rate limits by tier.
3. Gateway proxies MUST forward `AUTH_TOKEN` (or terminate TLS and inject venue
   credentials) when translating private WS/REST to native FIG.
4. Cross-account reads are rejected even when the token is valid for another
   account.

See [docs/STREAMING.md](docs/STREAMING.md) and [docs/TUTORIAL.md](docs/TUTORIAL.md)
for subscribe examples with `AUTH_TOKEN`.

### 9.4 Order policy flags (venue-enforced)

`NewOrderSingle` includes optional **`post_only`** and **`reduce_only`** booleans.
Semantics are enforced by the venue matching engine; FIG carries the flags on the wire.

| Flag | Meaning | Reference sim behavior |
|---|---|---|
| `post_only: true` | Order must not take liquidity | Reject if limit would cross the book |
| `reduce_only: true` | Order must not increase absolute position | Reject if order would open or add to position |

Gateway REST POST bodies pass these fields through JSON → CBOR unchanged
(`"post_only": true`, `"reduce_only": true`). Native SBE/CBOR clients set them on
`NewOrderSingle` directly. See [docs/SBE_ORDER_PATH.md](docs/SBE_ORDER_PATH.md).

---

## 10. Security

Four-layer model, all protocol-native:

| Layer | Mechanism | Scope | Required |
|---|---|---|---|
| **Transport** | TREE mandatory TLS 1.3; mTLS for server-to-server | Connection | Yes |
| **Session** | AUTH_TOKEN (JWT/OAuth2) in STREAM_OPEN; refresh via CONTROL(AUTH_REFRESH) | Session | Yes |
| **Channel** | SCOPE extension in STREAM_OPEN: `trading:orders:write` | Channel | Optional |
| **Message** | ENCRYPTED flag + per-message symmetric key (E2E through gateways) | Payload | Optional |

Reference libraries (`fig_core::auth`, `jwt`, `oauth`) provide **validators**
for implementers (constant-time compare, JWT decode, introspection hooks).
They do not constitute a credential-issuance product. Production venues plug
their own identity systems behind the same `AUTH_TOKEN` + path-scoping rules.

### 10.1 0-RTT replay protection

TREE 0-RTT resumption can replay early client data. Implementations MUST
reject duplicate resumption tokens within a configurable replay window
(default **30 seconds**). The reference `fig_core::replay::MemoryReplayCache`
keys on a hash of the resumption token bytes.

Venues SHOULD treat idempotent read paths as 0-RTT-safe and defer
non-idempotent order submission until the handshake completes (1-RTT) unless
a venue-specific anti-replay policy allows otherwise.

---

## 11. FSL — Fig Schema Language

FSL is a domain-specific IDL that compiles to multiple targets (Rust, Go,
Python, TypeScript, Java, C#, C++, SBE, JSON Schema, FIX mappings). Optional
`.proto` export may be generated for gRPC interop documentation; it is **not**
a FIG 1.0 on-wire codec.

**`schemas/*.fsl` is the single source of truth** for message types, enums,
field numbers, and gateway mappings. Do not edit language-specific types by
hand without updating FSL and regenerating all targets. See
[ADR 0004 — FSL as Single Source of Truth](docs/adr/0004-fsl-single-source-of-truth.md).

### 11.1 Example

```
schema trading.orders v1.0.0 {
    well_known_id: 0x01

    type ClientOrderId: string(max_len: 20)
    type Price: decimal64(precision: 8, min: 0)
    type Quantity: decimal64(precision: 0, min: 0, max: 999999999)
    type Symbol: string(max_len: 8, uppercase: true)

    message NewOrderSingle {
        channel_type: request_response
        correlation_field: cl_ord_id
        cl_ord_id: ClientOrderId @1
        side: enum { buy, sell, sell_short } @2
        order_qty: Quantity @3
        price: Price @4 optional
        symbol: Symbol @5
        priority: high
    }

    message ExecutionReport {
        channel_type: stream_item
        correlation_field: cl_ord_id
        cl_ord_id: ClientOrderId @1
        exec_id: string(max_len: 20) @2
        exec_type: enum { new, fill, partial_fill, canceled, rejected } @3
        last_qty: Quantity @4 optional
        last_price: Price @5 optional
        leaves_qty: Quantity @6
        cum_qty: Quantity @7
        avg_price: Price @8
        symbol: Symbol @9
    }

    gateway fix {
        message NewOrderSingle -> MsgType: "D" {
            cl_ord_id -> tag: 11
            side -> tag: 54 values: { buy: "1", sell: "2" }
            order_qty -> tag: 38
            price -> tag: 44
            symbol -> tag: 55
        }
    }

    gateway rest {
        message NewOrderSingle -> method: POST path: "/accounts/{account}/orders"
        message ExecutionReport -> method: SSE path: "/accounts/{account}/executions"
    }
}
```

### 11.2 Codegen Targets

| Target | Output (1.0) | On-wire serializer |
|---|---|---|
| Rust | Structs + serde (`RustCodegen`); SBE encode/decode | ✅ CBOR + SBE in `fig-core` |
| SBE | Per-language `sbe_generated.*` + Rust codec | ✅ Rust reference; compile-smoke elsewhere |
| Python | PyO3 (`fig-python`) + `fig-client` merge helpers | ✅ CBOR via FFI / PyO3 |
| Go / C++ / C# / TypeScript / Zig | Generated types + thin FFI wrapper | ✅ Wire via `fig-ffi` (CBOR + frames) |
| OCaml | FSL types + ctypes | 🔶 Types; wire via `fig-ffi` |
| JSON Schema | `.schema.json` (REST docs) | N/A (documentation) |
| FIX mapping | `.fix.yaml` (gateway config) | N/A (gateway translation) |
| Protobuf | — | ❌ Removed from FIG wire (was experimental) |

**1.0 rule:** multi-language clients SHOULD use **`fig-ffi`** (C ABI) or
**`fig-python`** for encode/decode/conformance-tested paths. Per-language
generated SBE is for zero-copy hot paths once hex parity CI lands — see
[bindings/README.md](bindings/README.md).

Full multi-language serializer and native TREE client parity: [TODO.md §16](TODO.md).
Schema change policy: [ADR 0004](docs/adr/0004-fsl-single-source-of-truth.md).

---

## 12. Gateway Mapping

Gateways translate FIG ↔ legacy protocols. They are migration tools, not
the protocol's identity.

### 12.1 FIX ↔ FIG

| FIX concept | FIG equivalent |
|---|---|
| MsgSeqNum (34=) | Stream Seq + SEQUENCE_NUM extension |
| MsgType (35=) | Schema ID |
| SenderCompID/TargetCompID | SESSION_ID |
| Logon (35=A) | STREAM_OPEN + AUTH_TOKEN |
| Logout (35=5) | STREAM_CLOSE |
| Heartbeat (35=0) | CONTROL(PING/PONG) |
| ResendRequest (35=2) | CONTROL(RESEND) |
| NewOrderSingle (35=D) | STREAM_ITEM, Schema ID 0x01, SBE payload |
| ExecutionReport (35=8) | STREAM_ITEM, Schema ID 0x02 |

### 12.2 REST ↔ FIG

| HTTP concept | FIG equivalent |
|---|---|
| Method | METHOD extension |
| URI | CHANNEL_PATH / REQUEST_URI extension |
| Headers | Extension tags |
| Body | Payload (CBOR ↔ JSON) |
| Status code | STATUS_CODE extension |
| Chunked transfer | STREAM_ITEM frames |
| SSE | STREAM_ITEM with CONTENT_TYPE |

### 12.3 WebSocket ↔ FIG

| WebSocket concept | FIG equivalent |
|---|---|
| Upgrade handshake | TREE handshake + ALPN |
| Text frame | STREAM_ITEM, CONTENT_TYPE "text/plain" |
| Binary frame | STREAM_ITEM, CONTENT_TYPE "application/octet-stream" |
| Close frame | STREAM_CLOSE |
| Ping/Pong | CONTROL(PING/PONG) |

---

## 13. Compliance Tiers

| Tier | Required features | Covers |
|---|---|---|
| **1 — Core** | Fixed header, REQUEST/RESPONSE, CHANNEL_PATH, STATUS_CODE, CONTENT_TYPE | REST parity |
| **2 — Session** | + SESSION_ID, SEQUENCE_NUM, TIMESTAMP, STREAM_OPEN/CLOSE, CONTROL | FIX parity |
| **3 — Pub/Sub** | + ROUTING_KEY, SUBSCRIBE/UNSUBSCRIBE, STREAM_ITEM streaming | Market data |
| **4 — Advanced** | + ACK_RANGE, FLOW_CONTROL, REDIRECT, FRAGMENTED, COMPRESSION | Full protocol |

Servers advertise their tier during CONNECT via SETTINGS.

---

## 14. Performance Characteristics

| Metric | FIG (native) | FIX ASCII | HTTP/1.1+JSON | WebSocket |
|---|---|---|---|---|
| Min header overhead | 16 bytes | 200-500 bytes | 200-800 bytes | 2-10 bytes |
| Parse speed | ~0.5-2μs (SBE zero-copy) | ~5-20μs (ASCII parse) | ~10-50μs (JSON) | N/A |
| Handshake RTTs | 1 (TREE) / 0 (resumed) | 4 (TCP+TLS+Logon) | 3-5 (DNS+TCP+TLS+HTTP) | 2-3 (upgrade+TLS) |
| Multiplexing | 65535 channels/conn | 1 session/conn | 6 (browser) / HTTP/2 | 1/conn |
| Session resumption | 0-RTT | Full reconnect | N/A (stateless) | Full reconnect |

---

## 15. IANA Considerations

- **ALPN identifier:** `fig/1`
- **Well-known Schema IDs:** 0x01–0xEF (registry TBD)
- **Well-known Extension Tags:** 0x0001–0x001D (this document); future tags via registry
- **Content Types:** `application/fig+sbe`, `application/cbor`

---

## References

- RFC 9000 — the transport foundation TREE builds upon
- RFC 9001 — TLS for TREE encryption
- RFC 9114 — HTTP/3
- RFC 7049 — Concise Binary Object Representation (CBOR)
- FIX Protocol 4.4 / 5.0 SP2 / FIXT.1.1
- FIX Simple Binary Encoding (SBE) Standard
- RFC 6455 — The WebSocket Protocol

---

*End of specification.*

