const std = @import("std");
const fig = @import("fig.zig");

// tests/conformance/vectors/v1.json id sbe.new_order_single.limit_buy
const expected_hex = "00010001000000260008434f4e462d30303101405900000000000040492000000000000100000000000000000000044141504c020100000000000000000000000000000000000000";

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
    const v = fig.version();
    if (v.len == 0) return error.EmptyVersion;

    const bytes = try fig.sbeEncodeNewOrderSingle("CONF-001", "AAPL", true, 100.0, 50.25);
    defer std.heap.page_allocator.free(bytes);
    if (bytes.len == 0) return error.EmptySbe;

    const got = try hexAlloc(std.heap.page_allocator, bytes);
    defer std.heap.page_allocator.free(got);
    if (!std.mem.eql(u8, got, expected_hex)) return error.SbeHexMismatch;

    std.debug.print("fig-zig smoke OK {s}\n", .{v});
}
