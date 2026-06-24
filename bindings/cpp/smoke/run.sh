#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
g++ -std=c++17 -I bindings/cpp/include \
  bindings/cpp/smoke/smoke.cpp \
  target/release/libfig_ffi.a \
  -lpthread -ldl -lm \
  -o /tmp/fig_cpp_smoke
/tmp/fig_cpp_smoke
