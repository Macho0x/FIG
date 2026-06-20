const std = @import("std");
const c = @cImport({
    @cInclude("fig.h");
});

pub const FigBuffer = c.FigBuffer;
pub const FigFrameList = c.FigFrameList;

pub fn version() [:0]const u8 {
    return std.mem.span(c.fig_version());
}

pub fn connect(addr: [:0]const u8, server_name: ?[:0]const u8) !*c.FigClientHandle {
    var out: ?*c.FigClientHandle = null;
    const rc = c.fig_client_connect(
        addr.ptr,
        if (server_name) |n| n.ptr else null,
        @ptrCast(&out),
    );
    if (rc != 0) return error.ConnectFailed;
    return out.?;
}

pub fn close(handle: *c.FigClientHandle) void {
    c.fig_client_close(handle);
}

pub fn ping(handle: *c.FigClientHandle) !void {
    if (c.fig_client_ping(handle) != 0) return error.PingFailed;
}

const alloc = std.heap.page_allocator;

pub fn compressPayload(data: []const u8) ![]u8 {
    var out = FigBuffer{ .data = null, .len = 0 };
    const rc = c.fig_payload_compress(data.ptr, data.len, &out);
    if (rc != 0) return error.CompressFailed;
    defer c.fig_buffer_free(out);
    return alloc.dupe(u8, out.data[0..out.len]);
}

pub fn encodeSubscribeAuth(
    channel_id: u16,
    stream_seq: u32,
    routing_key: [:0]const u8,
    channel_path: [:0]const u8,
    auth_token: ?[:0]const u8,
) ![]u8 {
    var out = FigBuffer{ .data = null, .len = 0 };
    const rc = c.fig_frame_encode_subscribe_auth(
        channel_id,
        stream_seq,
        routing_key.ptr,
        channel_path.ptr,
        if (auth_token) |t| t.ptr else null,
        &out,
    );
    if (rc != 0) return error.EncodeFailed;
    defer c.fig_buffer_free(out);
    return alloc.dupe(u8, out.data[0..out.len]);
}
