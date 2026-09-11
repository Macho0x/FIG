# OCaml bindings

ctypes wrapper over [`fig.h`](../../crates/fig-ffi/include/fig.h).

## Quick check (same as CI)

```bash
# Ubuntu: ocaml ocaml-findlib libctypes-ocaml-dev
bash bindings/ocaml/smoke/run.sh
# prints: fig-ocaml smoke OK <crate-version>
```

# OCaml bindings

ctypes wrapper over [`fig.h`](../../crates/fig-ffi/include/fig.h).

## Quick check (same as CI)

```bash
bash bindings/ocaml/smoke/run.sh
# prints: fig-ocaml smoke OK <crate-version>
```

The smoke uses a tiny C stub (`caml_fig_version`) so it does not depend on
`ctypes-foreign` (not packaged on Ubuntu 24.04). See [`fig.ml`](fig.ml) for the
ctypes client bindings.

