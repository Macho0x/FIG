// Per-language SBE parity: encode with the *generated* Zig code and compare the
// bytes with the shared conformance vectors (produced by the Rust/FFI encoder).
const std = @import("std");
const sbe = @import("sbe_generated.zig");

const order_want = "00010001000000260008434f4e462d30303101405900000000000040492000000000000100000000000000000000044141504c020100000000000000000000000000000000000000";
const snapshot_want = "000100090000001100044141504c000353494d00000001000000154059000000000000401400000000000000000001010000000017979cfe362a0000000000000000002a010101";

fn hexAlloc(allocator: std.mem.Allocator, bytes: []const u8) ![]u8 {
    const digits = "0123456789abcdef";
    const out = try allocator.alloc(u8, bytes.len * 2);
    for (bytes, 0..) |b, i| {
        out[i * 2] = digits[b >> 4];
        out[i * 2 + 1] = digits[b & 0x0f];
    }
    return out;
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    const order = try sbe.NewOrderSingleEncoder.encode(
        allocator,
        "CONF-001",
        .Buy,
        100.0,
        50.25,
        null,
        "AAPL",
        .Limit,
        .Day,
        null, null, null, null, null, null, null, null,
    );
    defer allocator.free(order);
    const order_hex = try hexAlloc(allocator, order);
    defer allocator.free(order_hex);
    if (!std.mem.eql(u8, order_hex, order_want)) {
        std.debug.print("zig sbe parity new_order_single FAILED\n got  {s}\n want {s}\n", .{ order_hex, order_want });
        return error.ParityMismatch;
    }

    var bids = [_]sbe.PriceLevel{.{ .price = 100.0, .qty = 5.0, .order_count = 1 }};
    const snapshot = try sbe.OrderBookSnapshotEncoder.encode(
        allocator, "AAPL", "SIM", &bids, &.{}, 1700000000000000000, 42, 1,
    );
    defer allocator.free(snapshot);
    const snapshot_hex = try hexAlloc(allocator, snapshot);
    defer allocator.free(snapshot_hex);
    if (!std.mem.eql(u8, snapshot_hex, snapshot_want)) {
        std.debug.print("zig sbe parity order_book_snapshot FAILED\n got  {s}\n want {s}\n", .{ snapshot_hex, snapshot_want });
        return error.ParityMismatch;
    }

    const decoded = try sbe.NewOrderSingleDecoder.decode(allocator, order);
    if (!std.mem.eql(u8, decoded.cl_ord_id, "CONF-001") or
        !std.mem.eql(u8, decoded.symbol, "AAPL") or
        decoded.order_qty != 100.0 or decoded.price == null or decoded.price.? != 50.25)
    {
        std.debug.print("zig sbe parity round_trip FAILED\n", .{});
        return error.ParityMismatch;
    }

    const caps_want = "000100350000000000000003010203000000010000001e001a6d61726b6574646174612f4141504c2f63616e646c65732f356d010000000001000000044141504c0000000100000002356d";
    const caps = try sbe.CapabilitiesResponseEncoder.encode(
        allocator, &.{1, 2, 3},
        &.{sbe.CapabilityPath{ .path = "marketdata/AAPL/candles/5m", .pattern = .PubSub, .auth_required = 0 }},
        &.{"AAPL"}, &.{"5m"},
    );
    defer allocator.free(caps);
    const caps_hex = try hexAlloc(allocator, caps);
    defer allocator.free(caps_hex);
    if (!std.mem.eql(u8, caps_hex, caps_want)) {
        std.debug.print("zig sbe parity capabilities_response FAILED\n got  {s}\n want {s}\n", .{ caps_hex, caps_want });
        return error.ParityMismatch;
    }

    std.debug.print("fig-zig sbe parity OK\n", .{});
}
