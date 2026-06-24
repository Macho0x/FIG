#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
export FIG_REPO_ROOT="$root"
g++ -std=c++17 -I bindings/cpp/include \
  bindings/cpp/smoke/sbe_hex.cpp \
  target/release/libfig_ffi.a \
  -lpthread -ldl -lm \
  -o /tmp/fig_cpp_sbe_hex
/tmp/fig_cpp_sbe_hex
