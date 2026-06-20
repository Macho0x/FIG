#!/usr/bin/env bash
# Language-agnostic §16.1 / §17 E2E driver (invokes Rust reference integration tests).
set -euo pipefail
cd "$(dirname "$0")/../.."
cargo test -p fig-exchange-sim --test integration test_e2e_driver_order_to_execution "$@"
