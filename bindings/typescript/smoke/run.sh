#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release
export FIG_FFI_LIB="$root/target/release/libfig_ffi.so"
if [[ "$(uname -s)" == "Darwin" ]]; then
  export FIG_FFI_LIB="$root/target/release/libfig_ffi.dylib"
fi
export FIG_REPO_ROOT="$root"
if ! command -v bun >/dev/null 2>&1; then
  echo "bun is required to load fig.ts (bun:ffi)" >&2
  exit 1
fi
bun -e '
import { readFileSync } from "fs";
import { CandleState, cborDecodeMassCancelRequest, cborEncodeMassCancelRequest, jwtDecodeSub, jwtEncode, jwtVerifyBearer, sbeEncodeNewOrderSingle, version } from "./bindings/typescript/fig.ts";
const v = version();
if (!v) throw new Error("empty fig_version");
const token = jwtEncode("alice", 4000000000n, "fig-smoke-secret");
if (jwtDecodeSub(token, "fig-smoke-secret") !== "alice") throw new Error("jwt sub");
jwtVerifyBearer(token, "fig-smoke-secret");
const candle = new CandleState();
if (candle.isFinal() !== -1) throw new Error("empty candle");
candle.close();
const bytes = sbeEncodeNewOrderSingle("CONF-001", "AAPL", true, 100, 50.25);
if (!bytes.length) throw new Error("empty SBE");
const root = process.env.FIG_REPO_ROOT ?? process.cwd();
const json = JSON.parse(readFileSync(root + "/tests/conformance/vectors/v1.json", "utf8"));
const expected = json.vectors.find((x) => x.id === "sbe.new_order_single.limit_buy").expected_hex;
const got = bytes.toString("hex");
if (got !== expected) throw new Error("SBE hex mismatch expected=" + expected + " got=" + got);
const mass = cborEncodeMassCancelRequest("MC-001", "DEMO", "AAPL", 1);
const decoded = cborDecodeMassCancelRequest(mass);
if (decoded.clOrdId !== "MC-001" || decoded.account !== "DEMO" || decoded.symbol !== "AAPL" || decoded.side !== 1) {
  throw new Error("mass-cancel CBOR round trip failed");
}
console.log("fig-ts smoke OK", v);
'

# Per-language SBE parity: encode with the *generated* TypeScript code and compare
# against the shared conformance vectors (produced by the Rust/FFI encoder).
bun bindings/typescript/smoke/sbe_parity.ts
