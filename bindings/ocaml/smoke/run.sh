#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
if ! command -v ocamlfind >/dev/null 2>&1; then
  echo "ocamlfind is required for the OCaml smoke" >&2
  exit 1
fi
export LD_LIBRARY_PATH="$root/target/release:${LD_LIBRARY_PATH:-}"
ocamlfind opt -package ctypes,ctypes.foreign -linkpkg \
  -cclib -L"$root/target/release" \
  -cclib -lfig_ffi \
  -cclib -lpthread \
  -cclib -ldl \
  -o /tmp/fig_ocaml_smoke \
  bindings/ocaml/smoke/version.ml
/tmp/fig_ocaml_smoke
