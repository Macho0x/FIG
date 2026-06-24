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
                    └──────────────┬──────────────────────┘
                                   │
         ┌─────────────────────────┼─────────────────────────┐
         │ optional edge           │                         │
  FIX :9876 ──► fig-gateway ────────┘                         │
  REST :8080 ──► fig-gateway ──► --fig-backend host:8443     │
  WS :8090 ────► fig-gateway                                 │
         └───────────────────────────────────────────────────┘

  Prometheus ──scrape──► fig-observability :9090
                      or fig-gateway --metrics-addr
```

## Single node (development)

```bash
cargo run -p fig-exchange-sim
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
```

Or use [docker-compose.yml](../docker-compose.yml):

```bash
docker compose up
```

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

```bash
cargo run -p fig-core --features observability-bin --bin fig-observability
curl http://127.0.0.1:9090/metrics
```

Wire `tracing-subscriber` in your binary to export spans to OpenTelemetry.

## Environment variables

| Variable | Purpose |
|---|---|
| `FIG_DEV_OPEN` | Skip auth on sim (dev only) |
| `FIG_MTLS` | Enable mTLS on exchange-sim |
| `REDIS_URL` | Redis session store when `--session-store redis` |
| `RUST_LOG` | Log level (`info`, `debug`) |

See also [GATEWAY.md](GATEWAY.md), [1.0-CRITERIA.md](1.0-CRITERIA.md).
