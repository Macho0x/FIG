#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
export FIG_FFI_LIB="$root/target/release/libfig_ffi.so"
if [[ "$(uname -s)" == "Darwin" ]]; then
  export FIG_FFI_LIB="$root/target/release/libfig_ffi.dylib"
fi
if ! command -v bun >/dev/null 2>&1; then
  echo "bun is required to load fig.ts (bun:ffi)" >&2
  exit 1
fi
bun -e 'import { version } from "./bindings/typescript/fig.ts"; const v = version(); if (!v) throw new Error("empty fig_version"); console.log("fig-ts smoke OK", v);'
