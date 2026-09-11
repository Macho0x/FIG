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

Use `fig_gateways::rest`, `fix_session`, `ws_catalog`, `backend::proxy_frame` (REST
GET), and `backend::BackendSession` (live WS subscribe) from your own Tokio
service. See [docs/GATEWAY.md](../../docs/GATEWAY.md) and
[docs/DEPLOYMENT.md](../../docs/DEPLOYMENT.md).
