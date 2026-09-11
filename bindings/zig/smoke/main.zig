const std = @import("std");
const c = @cImport({
    @cInclude("fig.h");
});

pub fn main() !void {
    const v = std.mem.span(c.fig_version());
    if (v.len == 0) {
        return error.EmptyVersion;
    }
    std.debug.print("fig-zig smoke OK {s}\n", .{v});
}
