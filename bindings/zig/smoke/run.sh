#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
if ! command -v zig >/dev/null 2>&1; then
  echo "zig is required for the Zig smoke" >&2
  exit 1
fi
zig run bindings/zig/smoke/main.zig \
  -I crates/fig-ffi/include \
  -L target/release \
  -lfig_ffi \
  -lc \
  -lpthread
