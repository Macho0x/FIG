#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
if ! command -v ocamlopt >/dev/null 2>&1; then
  echo "ocamlopt is required for the OCaml smoke" >&2
  exit 1
fi
export LD_LIBRARY_PATH="$root/target/release:${LD_LIBRARY_PATH:-}"
export FIG_REPO_ROOT="$root"
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT
cp bindings/ocaml/fig.ml bindings/ocaml/fig_stubs.c bindings/ocaml/smoke/main.ml "$out/"
(
  cd "$out"
  gcc -c -I"$root/crates/fig-ffi/include" -I"$(ocamlc -where)" fig_stubs.c -o fig_stubs.o
  # Ubuntu ocamlopt can pass -lfig_ffi before objects; GNU ld then drops the
  # .so (--as-needed). Force symbols and link the static archive last.
  ocamlopt -ccopt "-Wl,-u,fig_version,-u,fig_sbe_encode_new_order_single,-u,fig_client_subscribe" \
    -ccopt "$root/target/release/libfig_ffi.a" \
    -cclib -lpthread -cclib -ldl -cclib -lm \
    -o fig_ocaml_smoke \
    fig_stubs.o fig.ml main.ml
  ./fig_ocaml_smoke
)
