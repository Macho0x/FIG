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
  ocamlopt -ccopt "-I$root/crates/fig-ffi/include" \
    -ccopt "-L$root/target/release" \
    -cclib -lfig_ffi -cclib -lpthread -cclib -ldl \
    -o fig_ocaml_smoke \
    version_stub.c version.ml
  ./fig_ocaml_smoke
)
