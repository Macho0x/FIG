# Gateway Deployment Guide

How to run FIG alongside legacy FIX and REST infrastructure.

## Overview

The `fig-gateway` binary is a **translation bridge** for migration testing. It
accepts legacy FIX and REST connections, converts them to FIG frames, and can
proxy GET queries to a remote FIG backend via `--fig-backend`.

Library adapters (`fig_gateways::fix`, `rest`, `ws`, `sse`) can be embedded in
your own gateway service that forwards to a FIG server.

```text
FIX clients ──► :9876 ──┐
REST clients ──► :8080 ─┼──► fig-gateway (translate in-process)
                        │
WebSocket / SSE         └── use library adapters in custom gateway
```

## Quick Start

```bash
# Terminal 1: FIG exchange simulator
cargo run -p fig-exchange-sim

# Terminal 2: Gateway (REST + FIX translation demo)
cargo run -p fig-gateways --bin fig-gateway
```

The exchange simulator listens on **`127.0.0.1:8443`** (UDP/TREE). Native FIG
clients (e.g. `fig-cli`) connect directly to that address.

## CLI options

| Flag | Default | Description |
|---|---|---|
| `--rest-addr` | `127.0.0.1:8080` | REST HTTP listener |
| `--ws-addr` | `127.0.0.1:8090` | WebSocket HTTP upgrade listener |
| `--fix-addr` | `127.0.0.1:9876` | FIX TCP listener |
| `--fig-backend` | (none) | Proxy REST GET and WS SUBSCRIBE to native FIG backend |

Example:

```bash
cargo run -p fig-gateways --bin fig-gateway -- \
  --rest-addr 0.0.0.0:8080 \
  --fix-addr 0.0.0.0:9876
```

## REST Gateway

- JSON request bodies are converted to CBOR via `json_to_cbor`.
- Responses are converted back with `cbor_to_json`.
- SSE streaming endpoints map to FIG `STREAM_ITEM` frames via `fig_gateways::sse`.

Example:

```bash
curl -X POST http://127.0.0.1:8080/trading/orders \
  -H 'Content-Type: application/json' \
  -d '{"cl_ord_id":"ORD-1","symbol":"AAPL","side":"Buy","order_qty":100,"price":150.25}'
```

## FIX Gateway

- Logon (35=A) maps to STREAM_OPEN + AUTH extension.
- ResendRequest (35=2) maps to CONTROL(RESEND) via `fix_session`.
- Application messages map to FIG Request/StreamItem frames.

Use `fig_gateways::fix_session::FixSession` for production-grade FIX session
state management (sequence numbers, heartbeats, gap fill).

## WebSocket & SSE

- **WebSocket**: `fig-gateway` listens on `--ws-addr` (default `8090`). Legacy subscribe JSON is translated via `fig_gateways::ws_catalog` and proxied when `--fig-backend` is set.
- **SSE**: use `fig_gateways::sse` (`parse_sse_chunk`, `sse_to_fig_stream_item`) for REST streaming endpoints.

## Production Checklist

- [ ] Replace self-signed certs with proper PKI or mTLS (`FIG_MTLS=1` on exchange-sim).
- [ ] Use Redis/etcd session store for multi-node deployments (`RedisSessionStore`, `EtcdSessionStore`).
- [ ] Enable rate limiting and DoS guards (`DoSGuard`, `ChannelRateLimiter`).
- [ ] Export metrics from `fig-observability` (`/metrics` on `:9090`).
- [ ] Wire gateway adapters to forward translated frames to your FIG backend.
- [ ] Run gateway and backend in separate network zones with firewall rules.

## Docker

Build and run the exchange simulator container:

```bash
docker build -t fig-exchange-sim .
docker run --rm -p 8443:8443/udp fig-exchange-sim
```

See [Dockerfile](../Dockerfile) for the multi-stage build definition.
