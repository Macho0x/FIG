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
4. **0-RTT replay cache** — default in-process `MemoryReplayCache` per node; for strict cluster-wide replay rejection, front with a shared cache (document venue policy in SPEC §security).

## Security

| Control | How |
|---|---|
| TLS 1.3 | Mandatory on TREE (ALPN `fig/1`) |
| mTLS | `FIG_MTLS=1` on sim; production PKI via `server_config_mtls` |
| Auth | Venue-issued `AUTH_TOKEN` on private paths ([SPEC §9.3](../SPEC.md)) |
| Rate limits | `ChannelRateLimiter`, `DoSGuard` in `fig-core` |
| Dev bypass | `FIG_DEV_OPEN=1` — **never** in production |

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
| `FIG_DEV_OPEN` | Skip auth on sim (dev only) |
| `FIG_MTLS` | Enable mTLS on exchange-sim |
| `REDIS_URL` | Redis session store when `--session-store redis` |
| `RUST_LOG` | Log level (`info`, `debug`) |

See also [GATEWAY.md](GATEWAY.md), [TODO.md §18](../TODO.md#18-fig-10-release-criteria).
