# OCaml bindings

ctypes wrapper over [`fig.h`](../../crates/fig-ffi/include/fig.h).

## Quick check (same as CI)

```bash
# Ubuntu: ocaml ocaml-findlib libctypes-ocaml-dev
bash bindings/ocaml/smoke/run.sh
# prints: fig-ocaml smoke OK <crate-version>
```

The smoke uses `ctypes.foreign` against `libfig_ffi`. See [`fig.ml`](fig.ml)
for connect, compression, and subscribe-auth encode entry points.
