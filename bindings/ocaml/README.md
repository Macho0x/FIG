# OCaml bindings

C stubs over [`fig.h`](../../crates/fig-ffi/include/fig.h). Does not use
`ctypes-foreign` (not packaged on Ubuntu 24.04).

## Quick check (same as CI)

```bash
# Ubuntu: ocaml (ocamlopt)
bash bindings/ocaml/smoke/run.sh
# prints: fig-ocaml smoke OK <crate-version>
```

See [`fig.ml`](fig.ml) and [`fig_stubs.c`](fig_stubs.c) for `subscribe` /
`sub_next` (live SUBSCRIBE), `request_and_recv` (REQUEST only), and the
market-state helpers `request_session_status`, `request_security_status`,
and `mass_cancel`.
