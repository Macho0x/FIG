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
import { sbeEncodeNewOrderSingle, version } from "./bindings/typescript/fig.ts";
const v = version();
if (!v) throw new Error("empty fig_version");
const bytes = sbeEncodeNewOrderSingle("CONF-001", "AAPL", true, 100, 50.25);
if (!bytes.length) throw new Error("empty SBE");
const root = process.env.FIG_REPO_ROOT ?? process.cwd();
const json = JSON.parse(readFileSync(root + "/tests/conformance/vectors/v1.json", "utf8"));
const expected = json.vectors.find((x) => x.id === "sbe.new_order_single.limit_buy").expected_hex;
const got = bytes.toString("hex");
if (got !== expected) throw new Error("SBE hex mismatch expected=" + expected + " got=" + got);
console.log("fig-ts smoke OK", v);
'

# Per-language SBE parity: encode with the *generated* TypeScript code and compare
# against the shared conformance vectors (produced by the Rust/FFI encoder).
bun -e '
import { readFileSync } from "fs";
import {
  NewOrderSingleEncoderEncode, NewOrderSingleSide, NewOrderSingleOrderType, NewOrderSingleTimeInForce,
  OrderBookSnapshotEncoderEncode, NewOrderSingleDecoderDecode,
  CapabilitiesResponseEncoderEncode, CapabilityPathPattern,
} from "./bindings/typescript/sbe_generated.ts";

const root = process.env.FIG_REPO_ROOT ?? process.cwd();
const vectors = JSON.parse(readFileSync(root + "/tests/conformance/vectors/v1.json", "utf8")).vectors;
const want = (id) => vectors.find((v) => v.id === id).expected_hex;
const hex = (b) => Buffer.from(b).toString("hex");

const order = NewOrderSingleEncoderEncode(
  "CONF-001", NewOrderSingleSide.Buy, 100, 50.25, null, "AAPL",
  NewOrderSingleOrderType.Limit, NewOrderSingleTimeInForce.Day,
  null, null, null, null, null, null, null, null,
);
if (hex(order) !== want("sbe.new_order_single.limit_buy")) {
  throw new Error("new_order_single parity failed\n got " + hex(order) + "\nwant " + want("sbe.new_order_single.limit_buy"));
}

const snapshot = OrderBookSnapshotEncoderEncode(
  "AAPL", "SIM", [{ price: 100, qty: 5, order_count: 1 }], [],
  1700000000000000000n, 42n, 1,
);
if (hex(snapshot) !== want("sbe.order_book_snapshot.demo")) {
  throw new Error("order_book_snapshot parity failed\n got " + hex(snapshot) + "\nwant " + want("sbe.order_book_snapshot.demo"));
}

const round = NewOrderSingleDecoderDecode(order);
if (round.cl_ord_id !== "CONF-001" || round.symbol !== "AAPL" || round.order_qty !== 100 || round.price !== 50.25) {
  throw new Error("round trip failed: " + JSON.stringify(round));
}
const caps = CapabilitiesResponseEncoderEncode(
  [1, 2, 3],
  [{ path: "marketdata/AAPL/candles/5m", pattern: CapabilityPathPattern.PubSub, auth_required: 0 }],
  ["AAPL"], ["5m"],
);
if (hex(caps) !== want("sbe.capabilities_response.demo")) {
  throw new Error("capabilities_response parity failed\n got " + hex(caps) + "\nwant " + want("sbe.capabilities_response.demo"));
}

console.log("fig-ts sbe parity OK");
'
