#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
export FIG_REPO_ROOT="$root"
if ! command -v zig >/dev/null 2>&1; then
  echo "zig is required for the Zig smoke" >&2
  exit 1
fi
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT
# build-obj + gcc: Zig 0.16 lld rejects gcc 16 crt .sframe relocs; CI gcc is fine.
# smoke.zig sits next to fig.zig so `@import("fig.zig")` works on 0.14 and 0.16.
zig build-obj bindings/zig/smoke.zig \
  -I crates/fig-ffi/include \
  -lc \
  -femit-bin="$out/smoke.o"
gcc "$out/smoke.o" target/release/libfig_ffi.a \
  -lpthread -ldl -lm \
  -o "$out/fig_zig_smoke"
"$out/fig_zig_smoke"
