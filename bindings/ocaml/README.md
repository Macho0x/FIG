# OCaml bindings

ctypes wrapper over [`fig.h`](../../crates/fig-ffi/include/fig.h).

```bash
cargo build -p fig-ffi
export LD_LIBRARY_PATH=$PWD/target/debug
ocamlfind opt -package ctypes -linkpkg -o fig_test fig.ml
```

See [`fig.ml`](fig.ml) for connect, compression, and subscribe-auth encode entry points.
