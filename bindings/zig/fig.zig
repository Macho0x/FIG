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

fn copyFrameList(list: FigFrameList) ![][]u8 {
    if (list.frames == null or list.count == 0) {
        return try alloc.alloc([]u8, 0);
    }
    const frames = list.frames[0..list.count];
    const out = try alloc.alloc([]u8, frames.len);
    for (frames, 0..) |buf, i| {
        if (buf.data == null or buf.len == 0) {
            out[i] = try alloc.alloc(u8, 0);
            continue;
        }
        out[i] = try alloc.dupe(u8, buf.data[0..buf.len]);
    }
    return out;
}

pub fn requestAndRecv(handle: *c.FigClientHandle, frame: []const u8) ![][]u8 {
    var list = FigFrameList{ .frames = null, .count = 0 };
    const rc = c.fig_client_request_and_recv(handle, frame.ptr, frame.len, &list);
    if (rc != 0) return error.RequestFailed;
    defer c.fig_frame_list_free(list);
    return copyFrameList(list);
}

pub const Subscription = struct {
    handle: *c.FigSubHandle,

    /// Next live frame. timeout_ms 0 waits forever. null on EOF.
    pub fn next(self: *const Subscription, timeout_ms: u32) !?[]u8 {
        var out = FigBuffer{ .data = null, .len = 0 };
        const rc = c.fig_client_sub_next(self.handle, timeout_ms, &out);
        if (rc == 1) return error.Timeout;
        if (rc == 2) return null;
        if (rc != 0) return error.SubNextFailed;
        defer c.fig_buffer_free(out);
        if (out.data == null or out.len == 0) return try alloc.alloc(u8, 0);
        return alloc.dupe(u8, out.data[0..out.len]);
    }

    pub fn close(self: *const Subscription) void {
        c.fig_client_sub_close(self.handle);
    }
};

pub const SubscribeResult = struct {
    snapshot: [][]u8,
    sub: Subscription,
};

/// SUBSCRIBE snapshot plus a live handle. Do not use requestAndRecv for SUBSCRIBE.
pub fn subscribe(handle: *c.FigClientHandle, frame: []const u8) !SubscribeResult {
    var list = FigFrameList{ .frames = null, .count = 0 };
    var sub: ?*c.FigSubHandle = null;
    const rc = c.fig_client_subscribe(handle, frame.ptr, frame.len, &list, &sub);
    if (rc != 0) return error.SubscribeFailed;
    defer c.fig_frame_list_free(list);
    return .{
        .snapshot = try copyFrameList(list),
        .sub = .{ .handle = sub orelse return error.SubscribeFailed },
    };
}

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

pub fn jwtEncode(sub: [:0]const u8, exp: u64, secret: [:0]const u8) ![]u8 {
    var out = FigBuffer{ .data = null, .len = 0 };
    const rc = c.fig_jwt_encode(sub.ptr, exp, secret.ptr, &out);
    if (rc != 0) return error.JwtEncodeFailed;
    defer c.fig_buffer_free(out);
    return alloc.dupe(u8, out.data[0..out.len]);
}

pub fn jwtVerifyBearer(token: [:0]const u8, secret: [:0]const u8) !void {
    if (c.fig_jwt_verify_bearer(token.ptr, secret.ptr) != 0) return error.JwtVerifyFailed;
}

pub fn sbeEncodeNewOrderSingle(
    cl_ord_id: [:0]const u8,
    symbol: [:0]const u8,
    side_buy: bool,
    qty: f64,
    price: f64,
) ![]u8 {
    var out = FigBuffer{ .data = null, .len = 0 };
    const rc = c.fig_sbe_encode_new_order_single(
        cl_ord_id.ptr,
        symbol.ptr,
        @intFromBool(side_buy),
        qty,
        price,
        -1,
        -1,
        &out,
    );
    if (rc != 0) return error.SbeEncodeFailed;
    defer c.fig_buffer_free(out);
    return alloc.dupe(u8, out.data[0..out.len]);
}

pub const MassCancelRequest = struct {
    cl_ord_id: []u8,
    account: []u8,
    symbol: ?[]u8,
    side: u8,
};

pub const TradingSessionStatus = struct {
    status: u8,
    session_id: []u8,
};

pub const SecurityStatus = struct {
    symbol: []u8,
    status: u8,
};

pub const MassCancelReport = struct {
    cl_ord_id: []u8,
    total_affected: u32,
    status: u8,
};

fn dupCString(p: [*c]u8) ![]u8 {
    if (p == null) return error.NullString;
    const span = std.mem.span(@as([*:0]const u8, @ptrCast(p)));
    const out = try alloc.dupe(u8, span);
    c.fig_string_free(p);
    return out;
}

fn dupOptional(p: [*c]u8) !?[]u8 {
    if (p == null) return null;
    return try dupCString(p);
}

/// side 0 omits the filter. symbol null omits the symbol filter.
pub fn cborEncodeMassCancelRequest(
    cl_ord_id: [:0]const u8,
    account: [:0]const u8,
    symbol: ?[:0]const u8,
    side: u8,
) ![]u8 {
    var out = FigBuffer{ .data = null, .len = 0 };
    const rc = c.fig_cbor_encode_mass_cancel_request(
        cl_ord_id.ptr,
        account.ptr,
        if (symbol) |s| s.ptr else null,
        side,
        &out,
    );
    if (rc != 0) return error.EncodeFailed;
    defer c.fig_buffer_free(out);
    return alloc.dupe(u8, out.data[0..out.len]);
}

pub fn cborDecodeMassCancelRequest(data: []const u8) !MassCancelRequest {
    var cl: [*c]u8 = null;
    var account: [*c]u8 = null;
    var symbol: [*c]u8 = null;
    var side: u8 = 0;
    const rc = c.fig_cbor_decode_mass_cancel_request(data.ptr, data.len, &cl, &account, &symbol, &side);
    if (rc != 0) return error.DecodeFailed;
    return .{
        .cl_ord_id = try dupCString(cl),
        .account = try dupCString(account),
        .symbol = try dupOptional(symbol),
        .side = side,
    };
}

pub fn cborDecodeTradingSessionStatus(data: []const u8) !TradingSessionStatus {
    var status: u8 = 0;
    var session: [*c]u8 = null;
    const rc = c.fig_cbor_decode_trading_session_status(data.ptr, data.len, &status, &session);
    if (rc != 0) return error.DecodeFailed;
    return .{ .status = status, .session_id = try dupCString(session) };
}

pub fn cborDecodeSecurityStatus(data: []const u8) !SecurityStatus {
    var symbol: [*c]u8 = null;
    var status: u8 = 0;
    const rc = c.fig_cbor_decode_security_status(data.ptr, data.len, &symbol, &status);
    if (rc != 0) return error.DecodeFailed;
    return .{ .symbol = try dupCString(symbol), .status = status };
}

pub fn cborDecodeMassCancelReport(data: []const u8) !MassCancelReport {
    var cl: [*c]u8 = null;
    var total: u32 = 0;
    var status: u8 = 0;
    const rc = c.fig_cbor_decode_mass_cancel_report(data.ptr, data.len, &cl, &total, &status);
    if (rc != 0) return error.DecodeFailed;
    return .{
        .cl_ord_id = try dupCString(cl),
        .total_affected = total,
        .status = status,
    };
}

pub fn requestSessionStatus(handle: *c.FigClientHandle, channel_id: u16) !TradingSessionStatus {
    var status: u8 = 0;
    var session: [*c]u8 = null;
    const rc = c.fig_client_request_session_status(handle, channel_id, &status, &session);
    if (rc != 0) return error.RequestFailed;
    return .{ .status = status, .session_id = try dupCString(session) };
}

pub fn requestSecurityStatus(handle: *c.FigClientHandle, symbol: [:0]const u8, channel_id: u16) !SecurityStatus {
    var out_symbol: [*c]u8 = null;
    var status: u8 = 0;
    const rc = c.fig_client_request_security_status(handle, symbol.ptr, channel_id, &out_symbol, &status);
    if (rc != 0) return error.RequestFailed;
    return .{ .symbol = try dupCString(out_symbol), .status = status };
}

pub fn massCancel(
    handle: *c.FigClientHandle,
    cl_ord_id: [:0]const u8,
    account: [:0]const u8,
    symbol: ?[:0]const u8,
    side: u8,
    channel_id: u16,
) !MassCancelReport {
    var out_cl: [*c]u8 = null;
    var total: u32 = 0;
    var status: u8 = 0;
    const rc = c.fig_client_mass_cancel(
        handle,
        cl_ord_id.ptr,
        account.ptr,
        if (symbol) |s| s.ptr else null,
        side,
        channel_id,
        &out_cl,
        &total,
        &status,
    );
    if (rc != 0) return error.RequestFailed;
    return .{
        .cl_ord_id = try dupCString(out_cl),
        .total_affected = total,
        .status = status,
    };
}
