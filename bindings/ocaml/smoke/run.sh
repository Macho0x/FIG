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
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT
cp bindings/ocaml/smoke/version.ml bindings/ocaml/smoke/version_stub.c "$out/"
(
  cd "$out"
  gcc -c -I"$root/crates/fig-ffi/include" -I"$(ocamlc -where)" version_stub.c -o version_stub.o
  # Ubuntu ocamlopt can pass -lfig_ffi before objects; GNU ld then drops the
  # .so (--as-needed). Force the symbol and link the static archive last.
  ocamlopt -ccopt "-Wl,-u,fig_version" \
    -ccopt "$root/target/release/libfig_ffi.a" \
    -cclib -lpthread -cclib -ldl -cclib -lm \
    -o fig_ocaml_smoke \
    version_stub.o version.ml
  ./fig_ocaml_smoke
)
