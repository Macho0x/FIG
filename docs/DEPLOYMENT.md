# FIG Deployment Guide

Production layout for brokers running native FIG backends with optional legacy
gateways. FIG is a **wire protocol** — you own matching, credential issuance,
and account portals.

## Topology

```text
                    ┌─────────────────────────────────────┐
  Native FIG bots   │  FIG backend (your venue or sim)     │
  (TREE :8443) ────►│  fig-exchange-sim / custom server    │
                    │  session store: memory | file | redis │
                    │  live SUBSCRIBE: held-open TREE streams │
                    └──────────────┬──────────────────────┘
                                   │
         ┌─────────────────────────┼─────────────────────────┐
         │ optional edge           │                         │
  FIX :9876 ──► fig-gateway ────────┘                         │
  REST :8080 ──► fig-gateway ──► --fig-backend host:8443     │
  WS :8090 ────► fig-gateway                                 │
         └───────────────────────────────────────────────────┘

  Prometheus ──scrape──► fig-gateway --metrics-addr :9091
                      or fig-observability :9090 (this process only)
```

## Single node (development)

```bash
cargo run -p fig-exchange-sim
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
```

Or use [docker-compose.yml](../docker-compose.yml) (Redis session store, sim,
gateway, and in-process observability exporter):

```bash
docker compose up
```

`FIG_DEV_OPEN=1` is set on the compose **demo** sim only — never in production.

Live `SUBSCRIBE` holds the TREE stream open: snapshot/ack is written immediately,
then later `STREAM_ITEM`s are pushed on that same subscriber connection. REQUEST
streams still half-close after the response.

## Multi-node HA

1. **Stateless FIG workers** — multiple backend instances behind UDP/TCP load balancer (TREE connection migration + resumption tokens).
2. **Shared session store** — enable `fig-core` feature `session-redis`:

   ```bash
   cargo run -p fig-exchange-sim --features fig-core/session-redis -- \
     --session-store redis --redis-url redis://redis:6379
   ```

3. **Gateway** — typically stateless when proxying to `--fig-backend`; use file or redis `fix-seq-store` for FIX sequence persistence across gateway restarts.
4. **0-RTT replay cache** — default in-process `MemoryReplayCache` per node. For
   strict cluster-wide replay rejection, point nodes at a shared Redis and build
   with the `session-redis` feature:

   ```bash
   FIG_REPLAY_REDIS_URL=redis://redis:6379 \
     cargo run -p fig-exchange-sim --features fig-core/session-redis
   ```

   The cache uses `SET key 1 NX PX <window>` so a token replayed to a *different*
   node is rejected inside the replay window (SPEC §10.1). Embedders can also
   install one explicitly with `fig_core::replay::set_default_replay_cache`.
   If `FIG_REPLAY_REDIS_URL` is set without the feature, startup logs an error
   and falls back to the per-node cache.

## Security

| Control | How |
|---|---|
| TLS 1.3 | Mandatory on TREE (ALPN `fig/1`) |
| Server certificate verification (clients) | On by default against the built-in WebPKI roots. Private PKI: `FIG_TLS_CA=/path/to/ca.pem` (or `client_config_from_ca_pem`). `client_config_insecure()` is for tests/dev only |
| mTLS | `FIG_MTLS=1` **and** `FIG_MTLS_CA=/path/to/client-ca.pem` to verify client certificates. `FIG_MTLS=1` without `FIG_MTLS_CA` accepts any client certificate (logged warning, development only) |
| Auth | Venue-issued `AUTH_TOKEN` on private paths ([SPEC §9.3](../SPEC.md)); token comparison is constant-time. RS256 via `FIG_JWT_JWKS`; rotation: [JWT_KEY_ROTATION.md](JWT_KEY_ROTATION.md) |
| Resumption tokens (0-RTT) | Server-signed HMAC-SHA256; set `FIG_RESUMPTION_KEY` to the **same** value on every node that must accept the same tokens. Servers reject unsigned/forged tokens ([SPEC §10.1](../SPEC.md)) |
| DoS limits | `DoSGuard` (max channels, frame size, message rate) + per-channel `ChannelRateLimiter`, enforced on every connection |
| Frame size | 1 MiB hard cap in `FrameDecoder` (`MAX_FRAME_SIZE`); oversize length prefixes are rejected before buffering |
| Dev bypass | `FIG_TLS_INSECURE=1`, `FIG_DEV_OPEN=1` — **never** in production. Set `FIG_PRODUCTION=1` so a stray `FIG_DEV_OPEN` is refused rather than silently disabling auth |

### Required production environment

```bash
FIG_PRODUCTION=1                     # refuse dev-only bypasses
FIG_MTLS=1 FIG_MTLS_CA=/etc/fig/client-ca.pem
FIG_RESUMPTION_KEY=$(openssl rand -hex 32)   # shared across HA nodes
FIG_TLS_CA=/etc/fig/venue-ca.pem     # on every FIG client
```

Rotate `FIG_RESUMPTION_KEY` on the same schedule as TLS material: rotating it
invalidates outstanding resumption tokens (clients transparently fall back to a
full handshake), so a rolling restart is safe.

## Observability

`fig-observability` (`:9090`) exports **that binary's** in-process Prometheus
text. It does not scrape other services. Scrape gateway metrics at
`fig-gateway --metrics-addr` (compose maps `:9091`).

```bash
cargo run -p fig-core --features observability-bin --bin fig-observability
curl http://127.0.0.1:9090/metrics
curl http://127.0.0.1:9091/metrics   # gateway, when --metrics-addr is set
```

Wire `tracing-subscriber` in your binary to export spans to OpenTelemetry.

## Environment variables

| Variable | Purpose |
|---|---|
| `FIG_PRODUCTION` | `1` — refuse dev-only bypasses (`FIG_DEV_OPEN`) |
| `FIG_DEV_OPEN` | Skip auth on sim (dev only) |
| `FIG_TLS_CA` | Client: verify the server against this CA bundle (private PKI) |
| `FIG_TLS_INSECURE` | Client: `1` disables TLS verification (dev only) |
| `FIG_MTLS` | Enable mTLS on exchange-sim |
| `FIG_MTLS_CA` | Server: client CA bundle to verify mTLS client certificates |
| `FIG_RESUMPTION_KEY` | Shared HMAC key for session resumption tokens (HA) |
| `FIG_REPLAY_REDIS_URL` | Shared Redis 0-RTT replay cache (requires `session-redis`) |
| `FIG_JWT_JWKS` | RS256 public keys: inline JWKS JSON or a path to a JWKS file |
| `REDIS_URL` | Redis session store when `--session-store redis` |
| `RUST_LOG` | Log level (`info`, `debug`) |

See also [GATEWAY.md](GATEWAY.md), [docs/README.md](README.md), [TODO.md §18](../TODO.md#18-fig-10-release-criteria).
