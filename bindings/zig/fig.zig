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

pub fn jwtDecodeSub(token: [:0]const u8, secret: [:0]const u8) ![]u8 {
    var sub: [*c]u8 = null;
    if (c.fig_jwt_decode_sub(token.ptr, secret.ptr, &sub) != 0) return error.JwtFailed;
    return dupCString(sub);
}

pub fn postBatch(
    handle: *c.FigClientHandle,
    account: [:0]const u8,
    payload: []const u8,
    channel_id: u16,
) ![]u8 {
    var out = FigBuffer{ .data = null, .len = 0 };
    const rc = c.fig_client_post_batch(handle, account.ptr, payload.ptr, payload.len, channel_id, &out);
    if (rc != 0) return error.RequestFailed;
    defer c.fig_buffer_free(out);
    if (out.data == null or out.len == 0) return alloc.alloc(u8, 0);
    return alloc.dupe(u8, out.data[0..out.len]);
}

fn mustApply(rc: i32) !void {
    if (rc != 0) return error.ApplyFailed;
}

pub const OrderBookState = struct {
    handle: *c.FigOrderBookHandle,
    pub fn init() OrderBookState {
        return .{ .handle = c.fig_order_book_new().? };
    }
    pub fn deinit(self: *OrderBookState) void {
        c.fig_order_book_free(self.handle);
    }
    pub fn applySnapshot(self: *OrderBookState, payload: []const u8) !void {
        try mustApply(c.fig_order_book_apply_snapshot(self.handle, payload.ptr, payload.len));
    }
    pub fn applyDelta(self: *OrderBookState, payload: []const u8) !void {
        try mustApply(c.fig_order_book_apply_delta(self.handle, payload.ptr, payload.len));
    }
    pub fn bestBid(self: OrderBookState) f64 {
        return c.fig_order_book_best_bid(self.handle);
    }
};

pub const MidsState = struct {
    handle: *c.FigMidsHandle,
    pub fn init() MidsState {
        return .{ .handle = c.fig_mids_new().? };
    }
    pub fn deinit(self: *MidsState) void {
        c.fig_mids_free(self.handle);
    }
    pub fn applyTicker(self: *MidsState, payload: []const u8) !void {
        try mustApply(c.fig_mids_apply_ticker(self.handle, payload.ptr, payload.len));
    }
    pub fn applyBatch(self: *MidsState, payload: []const u8) !void {
        try mustApply(c.fig_mids_apply_batch(self.handle, payload.ptr, payload.len));
    }
    pub fn mid(self: MidsState, symbol: [:0]const u8) f64 {
        return c.fig_mids_mid(self.handle, symbol.ptr);
    }
};

pub const BboState = struct {
    handle: *c.FigBboHandle,
    pub fn init() BboState {
        return .{ .handle = c.fig_bbo_new().? };
    }
    pub fn deinit(self: *BboState) void {
        c.fig_bbo_free(self.handle);
    }
    pub fn apply(self: *BboState, payload: []const u8) !void {
        try mustApply(c.fig_bbo_apply(self.handle, payload.ptr, payload.len));
    }
    pub fn impliedMid(self: BboState) f64 {
        return c.fig_bbo_implied_mid(self.handle);
    }
    pub fn bestBid(self: BboState) f64 {
        return c.fig_bbo_best_bid(self.handle);
    }
    pub fn bestAsk(self: BboState) f64 {
        return c.fig_bbo_best_ask(self.handle);
    }
};

pub const TradeTape = struct {
    handle: *c.FigTradeTapeHandle,
    pub fn init(capacity: usize) TradeTape {
        return .{ .handle = c.fig_trade_tape_new(capacity).? };
    }
    pub fn deinit(self: *TradeTape) void {
        c.fig_trade_tape_free(self.handle);
    }
    pub fn push(self: *TradeTape, payload: []const u8) !void {
        try mustApply(c.fig_trade_tape_push(self.handle, payload.ptr, payload.len));
    }
    pub fn len(self: TradeTape) usize {
        return c.fig_trade_tape_len(self.handle);
    }
    pub fn latestPrice(self: TradeTape) f64 {
        return c.fig_trade_tape_latest_price(self.handle);
    }
};

pub const MarkPriceState = struct {
    handle: *c.FigMarkPriceHandle,
    pub fn init() MarkPriceState {
        return .{ .handle = c.fig_mark_price_new().? };
    }
    pub fn deinit(self: *MarkPriceState) void {
        c.fig_mark_price_free(self.handle);
    }
    pub fn apply(self: *MarkPriceState, payload: []const u8) !void {
        try mustApply(c.fig_mark_price_apply(self.handle, payload.ptr, payload.len));
    }
    pub fn get(self: MarkPriceState, symbol: [:0]const u8) f64 {
        return c.fig_mark_price_get(self.handle, symbol.ptr);
    }
};

pub const OrdersState = struct {
    handle: *c.FigOrdersHandle,
    pub fn init(exec_capacity: usize) OrdersState {
        return .{ .handle = c.fig_orders_new(exec_capacity).? };
    }
    pub fn deinit(self: *OrdersState) void {
        c.fig_orders_free(self.handle);
    }
    pub fn applySnapshot(self: *OrdersState, payload: []const u8) !void {
        try mustApply(c.fig_orders_apply_snapshot(self.handle, payload.ptr, payload.len));
    }
    pub fn applyExecution(self: *OrdersState, payload: []const u8) !void {
        try mustApply(c.fig_orders_apply_execution(self.handle, payload.ptr, payload.len));
    }
    pub fn openCount(self: OrdersState) usize {
        return c.fig_orders_open_count(self.handle);
    }
    pub fn executionCount(self: OrdersState) usize {
        return c.fig_orders_execution_count(self.handle);
    }
};

pub const FundingState = struct {
    handle: *c.FigFundingHandle,
    pub fn init(capacity: usize) FundingState {
        return .{ .handle = c.fig_funding_new(capacity).? };
    }
    pub fn deinit(self: *FundingState) void {
        c.fig_funding_free(self.handle);
    }
    pub fn apply(self: *FundingState, payload: []const u8) !void {
        try mustApply(c.fig_funding_apply(self.handle, payload.ptr, payload.len));
    }
    pub fn len(self: FundingState) usize {
        return c.fig_funding_len(self.handle);
    }
    pub fn latestAmount(self: FundingState) f64 {
        return c.fig_funding_latest_amount(self.handle);
    }
};

pub const AggTradesState = struct {
    handle: *c.FigAggTradesHandle,
    pub fn init(capacity: usize) AggTradesState {
        return .{ .handle = c.fig_agg_trades_new(capacity).? };
    }
    pub fn deinit(self: *AggTradesState) void {
        c.fig_agg_trades_free(self.handle);
    }
    pub fn apply(self: *AggTradesState, payload: []const u8) !void {
        try mustApply(c.fig_agg_trades_apply(self.handle, payload.ptr, payload.len));
    }
    pub fn len(self: AggTradesState) usize {
        return c.fig_agg_trades_len(self.handle);
    }
    pub fn latestPrice(self: AggTradesState) f64 {
        return c.fig_agg_trades_latest_price(self.handle);
    }
};

pub const LedgerState = struct {
    handle: *c.FigLedgerHandle,
    pub fn init(capacity: usize) LedgerState {
        return .{ .handle = c.fig_ledger_new(capacity).? };
    }
    pub fn deinit(self: *LedgerState) void {
        c.fig_ledger_free(self.handle);
    }
    pub fn apply(self: *LedgerState, payload: []const u8) !void {
        try mustApply(c.fig_ledger_apply(self.handle, payload.ptr, payload.len));
    }
    pub fn len(self: LedgerState) usize {
        return c.fig_ledger_len(self.handle);
    }
};

pub const LiquidationState = struct {
    handle: *c.FigLiquidationHandle,
    pub fn init(capacity: usize) LiquidationState {
        return .{ .handle = c.fig_liquidation_new(capacity).? };
    }
    pub fn deinit(self: *LiquidationState) void {
        c.fig_liquidation_free(self.handle);
    }
    pub fn applyUser(self: *LiquidationState, payload: []const u8) !void {
        try mustApply(c.fig_liquidation_apply_user(self.handle, payload.ptr, payload.len));
    }
    pub fn applyPublic(self: *LiquidationState, payload: []const u8) !void {
        try mustApply(c.fig_liquidation_apply_public(self.handle, payload.ptr, payload.len));
    }
    pub fn userCount(self: LiquidationState) usize {
        return c.fig_liquidation_user_count(self.handle);
    }
};

pub const CandleState = struct {
    handle: *c.FigCandleHandle,
    pub fn init() CandleState {
        return .{ .handle = c.fig_candle_new().? };
    }
    pub fn deinit(self: *CandleState) void {
        c.fig_candle_free(self.handle);
    }
    pub fn apply(self: *CandleState, payload: []const u8) !void {
        try mustApply(c.fig_candle_apply(self.handle, payload.ptr, payload.len));
    }
    pub fn closePrice(self: CandleState) f64 {
        return c.fig_candle_close(self.handle);
    }
    pub fn isFinal(self: CandleState) i32 {
        return c.fig_candle_is_final(self.handle);
    }
    pub fn symbol(self: CandleState) ![]u8 {
        var out: [*c]u8 = null;
        if (c.fig_candle_symbol(self.handle, &out) != 0) return alloc.dupe(u8, "");
        return dupCString(out);
    }
    pub fn interval(self: CandleState) ![]u8 {
        var out: [*c]u8 = null;
        if (c.fig_candle_interval(self.handle, &out) != 0) return alloc.dupe(u8, "");
        return dupCString(out);
    }
};
