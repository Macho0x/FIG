# Gateway Deployment Guide

How to run FIG alongside legacy FIX, REST, and WebSocket infrastructure.

## Overview

The `fig-gateway` binary translates legacy protocols to FIG frames and forwards
them to a FIG backend (e.g. `fig-exchange-sim` or your production server).

```text
FIX clients ──► :9876 ──┐
REST clients ──► :8080 ─┼──► fig-gateway ──► FIG/TREE backend
WS clients  ──► :8081 ──┘
```

## Quick Start

```bash
# Terminal 1: FIG exchange simulator
cargo run -p fig-exchange-sim

# Terminal 2: Gateway (REST + FIX)
cargo run -p fig-gateways --bin fig-gateway
```

Environment variables:

| Variable | Default | Description |
|---|---|---|
| `FIG_REST_ADDR` | `0.0.0.0:8080` | REST HTTP listener |
| `FIG_FIX_ADDR` | `0.0.0.0:9876` | FIX TCP listener |
| `FIG_BACKEND` | `127.0.0.1:4433` | FIG server address |

## REST Gateway

- JSON request bodies are converted to CBOR via `json_to_cbor`.
- Responses are converted back with `cbor_to_json`.
- SSE streaming endpoints map to FIG `STREAM_ITEM` frames via `fig_gateways::sse`.

## FIX Gateway

- Logon (35=A) maps to STREAM_OPEN + AUTH extension.
- ResendRequest (35=2) maps to CONTROL(RESEND).
- Application messages map to FIG Request/StreamItem frames.

## WebSocket Gateway

- Text/binary WS frames map to FIG StreamItem/Request via `ws_to_fig_frame`.
- Ping/pong maps to CONTROL(PING/PONG).

## Production Checklist

- [ ] Replace self-signed certs with proper PKI or mTLS (`FIG_MTLS=1`).
- [ ] Use Redis/etcd session store for multi-node deployments.
- [ ] Enable rate limiting and DoS guards (`DoSGuard`, `ChannelRateLimiter`).
- [ ] Export metrics from `fig-observability` (`/metrics` endpoint).
- [ ] Run gateway and backend in separate network zones with firewall rules.

## Docker

See [Dockerfile](../Dockerfile) for a containerized exchange simulator suitable
for staging environments.
