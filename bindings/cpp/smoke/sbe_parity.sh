#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
out="$(mktemp -d)"
trap 'rm -rf "$out"' EXIT
g++ -std=c++20 -I bindings/cpp/include bindings/cpp/smoke/sbe_parity.cpp -o "$out/sbe_parity"
"$out/sbe_parity"
