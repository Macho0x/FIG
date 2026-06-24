#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
export LD_LIBRARY_PATH="$root/target/release:${LD_LIBRARY_PATH:-}"
export FIG_REPO_ROOT="$root"
dotnet run --project bindings/csharp/smoke/FigSmoke.csproj
