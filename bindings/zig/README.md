# Zig bindings

`@cImport` wrapper over [`fig.h`](../../crates/fig-ffi/include/fig.h).

```bash
cargo build -p fig-ffi
zig build -Dfig-ffi=../../target/debug
```

Compile with include path to `crates/fig-ffi/include` and link `libfig_ffi`.

See [`fig.zig`](fig.zig) for connect, ping, compression, and subscribe-auth helpers.
