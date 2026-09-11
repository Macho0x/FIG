# Production gateway template

Embed `fig_gateways` in your venue edge service or run the stock `fig-gateway` binary
behind a load balancer.

## Stock binary (compose)

```bash
docker compose up gateway exchange-sim observability
curl http://127.0.0.1:8081/healthz
curl http://127.0.0.1:9091/metrics
```

## systemd unit (sketch)

```ini
[Service]
ExecStart=/usr/local/bin/fig-gateway \
  --fig-backend 10.0.0.5:8443 \
  --rest-addr 0.0.0.0:8080 \
  --ws-addr 0.0.0.0:8090 \
  --fix-addr 0.0.0.0:9876 \
  --health-addr 127.0.0.1:8081 \
  --metrics-addr 127.0.0.1:9091
Restart=on-failure
```

## Library embed

REST GET is one-shot (`proxy_frame` — finish send, read until EOF). Live WS
subscribe keeps a `BackendSession` open:

```rust
use fig_gateways::backend::{proxy_frame, BackendSession};

// REST GET — finish send, read until EOF
let responses = proxy_frame(backend_addr, ticker_req).await?;

// WS client — one TREE connection; later STREAM_ITEMs on the same channel
let session = BackendSession::connect(backend_addr).await?;
session.send_frame(&subscribe).await?;
let item = session.recv_frame(subscribe.channel_id).await?;
```

See [docs/GATEWAY.md](../../docs/GATEWAY.md) and
[docs/DEPLOYMENT.md](../../docs/DEPLOYMENT.md).
