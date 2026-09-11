# Zig bindings

`@cImport` wrapper over [`fig.h`](../../crates/fig-ffi/include/fig.h).

## Quick check (same as CI)

```bash
bash bindings/zig/smoke/run.sh
# prints: fig-zig smoke OK <crate-version>
```

CI uses Zig 0.14. See [`fig.zig`](fig.zig) for `subscribe` / `Subscription.next`
(live SUBSCRIBE) and `requestAndRecv` (REQUEST only).
