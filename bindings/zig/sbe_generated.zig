// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' vv1.0.0
// Standard order entry and execution messages

const std = @import("std");

pub const SCHEMA_ID: u16 = 0x01;

fn writeU16BE(buf: *std.ArrayList(u8), v: u16) !void { try buf.append(@intCast(v >> 8)); try buf.append(@intCast(v & 0xFF)); }
fn writeU32BE(buf: *std.ArrayList(u8), v: u32) !void { try buf.append(@intCast(v >> 24)); try buf.append(@intCast(v >> 16)); try buf.append(@intCast(v >> 8)); try buf.append(@intCast(v & 0xFF)); }
fn writeU64BE(buf: *std.ArrayList(u8), v: u64) !void { var i: i32 = 7; while (i >= 0) : (i -= 1) try buf.append(@intCast((v >> @intCast(i * 8)) & 0xFF)); }
fn writeF64BE(buf: *std.ArrayList(u8), v: f64) !void { try writeU64BE(buf, @bitCast(v)); }
fn writeI64BE(buf: *std.ArrayList(u8), v: i64) !void { try writeU64BE(buf, @bitCast(v)); }
fn writeString(buf: *std.ArrayList(u8), s: []const u8) !void { try writeU16BE(buf, @intCast(s.len)); try buf.appendSlice(s); }
fn readU16BE(buf: []const u8, pos: *usize) u16 { const v = (@as(u16, buf[pos.*]) << 8) | buf[pos.* + 1]; pos.* += 2; return v; }
fn readU32BE(buf: []const u8, pos: *usize) u32 { const v = (@as(u32, buf[pos.*]) << 24) | (@as(u32, buf[pos.* + 1]) << 16) | (@as(u32, buf[pos.* + 2]) << 8) | buf[pos.* + 3]; pos.* += 4; return v; }
fn readU64BE(buf: []const u8, pos: *usize) u64 { var v: u64 = 0; var i: usize = 0; while (i < 8) : (i += 1) v = (v << 8) | buf[pos.* + i]; pos.* += 8; return v; }
fn readF64BE(buf: []const u8, pos: *usize) f64 { return @bitCast(readU64BE(buf, pos)); }
fn readI64BE(buf: []const u8, pos: *usize) i64 { return @bitCast(readU64BE(buf, pos)); }
fn readString(buf: []const u8, pos: *usize, allocator: std.mem.Allocator) ![]u8 { const len = readU16BE(buf, pos); const s = try allocator.dupe(u8, buf[pos.* .. pos.* + len]); pos.* += len; return s; }

pub const NewOrderSingleSide = enum(u8) {
    Buy,
    Sell,
    SellShort,
    SellShortExempt,
};

pub fn NewOrderSingleSideFromValue(v: u8) !NewOrderSingleSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        3 => .SellShort,
        4 => .SellShortExempt,
        else => error.InvalidEnumValue,
    };
}

pub fn NewOrderSingleSideToValue(e: NewOrderSingleSide) u8 { return @intFromEnum(e); }

pub const NewOrderSingleOrderType = enum(u8) {
    Market,
    Limit,
    Stop,
    StopLimit,
    MarketOnClose,
    LimitOnClose,
    Pegged,
};

pub fn NewOrderSingleOrderTypeFromValue(v: u8) !NewOrderSingleOrderType {
    return switch (v) {
        1 => .Market,
        2 => .Limit,
        3 => .Stop,
        4 => .StopLimit,
        5 => .MarketOnClose,
        6 => .LimitOnClose,
        7 => .Pegged,
        else => error.InvalidEnumValue,
    };
}

pub fn NewOrderSingleOrderTypeToValue(e: NewOrderSingleOrderType) u8 { return @intFromEnum(e); }

pub const NewOrderSingleTimeInForce = enum(u8) {
    Day,
    Gtc,
    Ioc,
    Fok,
    Gtd,
};

pub fn NewOrderSingleTimeInForceFromValue(v: u8) !NewOrderSingleTimeInForce {
    return switch (v) {
        1 => .Day,
        2 => .Gtc,
        3 => .Ioc,
        4 => .Fok,
        5 => .Gtd,
        else => error.InvalidEnumValue,
    };
}

pub fn NewOrderSingleTimeInForceToValue(e: NewOrderSingleTimeInForce) u8 { return @intFromEnum(e); }

pub const NewOrderSingleIdSource = enum(u8) {
    Cusip,
    Sedol,
    Isin,
    Ric,
    ExchangeSymbol,
};

pub fn NewOrderSingleIdSourceFromValue(v: u8) !NewOrderSingleIdSource {
    return switch (v) {
        1 => .Cusip,
        2 => .Sedol,
        3 => .Isin,
        4 => .Ric,
        5 => .ExchangeSymbol,
        else => error.InvalidEnumValue,
    };
}

pub fn NewOrderSingleIdSourceToValue(e: NewOrderSingleIdSource) u8 { return @intFromEnum(e); }

pub const CancelRequestSide = enum(u8) {
    Buy,
    Sell,
    SellShort,
};

pub fn CancelRequestSideFromValue(v: u8) !CancelRequestSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        3 => .SellShort,
        else => error.InvalidEnumValue,
    };
}

pub fn CancelRequestSideToValue(e: CancelRequestSide) u8 { return @intFromEnum(e); }

pub const CancelReplaceRequestSide = enum(u8) {
    Buy,
    Sell,
    SellShort,
};

pub fn CancelReplaceRequestSideFromValue(v: u8) !CancelReplaceRequestSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        3 => .SellShort,
        else => error.InvalidEnumValue,
    };
}

pub fn CancelReplaceRequestSideToValue(e: CancelReplaceRequestSide) u8 { return @intFromEnum(e); }

pub const ExecutionReportExecType = enum(u8) {
    New,
    PartialFill,
    Fill,
    DoneForDay,
    Canceled,
    Replaced,
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    PendingNew,
    Expired,
};

pub fn ExecutionReportExecTypeFromValue(v: u8) !ExecutionReportExecType {
    return switch (v) {
        1 => .New,
        2 => .PartialFill,
        3 => .Fill,
        4 => .DoneForDay,
        5 => .Canceled,
        6 => .Replaced,
        7 => .PendingCancel,
        8 => .Stopped,
        9 => .Rejected,
        10 => .Suspended,
        11 => .PendingNew,
        12 => .Expired,
        else => error.InvalidEnumValue,
    };
}

pub fn ExecutionReportExecTypeToValue(e: ExecutionReportExecType) u8 { return @intFromEnum(e); }

pub const ExecutionReportOrdStatus = enum(u8) {
    New,
    PartiallyFilled,
    Filled,
    DoneForDay,
    Canceled,
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    PendingNew,
    Expired,
    Replaced,
};

pub fn ExecutionReportOrdStatusFromValue(v: u8) !ExecutionReportOrdStatus {
    return switch (v) {
        1 => .New,
        2 => .PartiallyFilled,
        3 => .Filled,
        4 => .DoneForDay,
        5 => .Canceled,
        6 => .PendingCancel,
        7 => .Stopped,
        8 => .Rejected,
        9 => .Suspended,
        10 => .PendingNew,
        11 => .Expired,
        12 => .Replaced,
        else => error.InvalidEnumValue,
    };
}

pub fn ExecutionReportOrdStatusToValue(e: ExecutionReportOrdStatus) u8 { return @intFromEnum(e); }

pub const ExecutionReportSide = enum(u8) {
    Buy,
    Sell,
    SellShort,
};

pub fn ExecutionReportSideFromValue(v: u8) !ExecutionReportSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        3 => .SellShort,
        else => error.InvalidEnumValue,
    };
}

pub fn ExecutionReportSideToValue(e: ExecutionReportSide) u8 { return @intFromEnum(e); }

pub const CancelRejectRejectReason = enum(u8) {
    OrderNotFound,
    AlreadyCanceled,
    AlreadyFilled,
    TooLateToCancel,
};

pub fn CancelRejectRejectReasonFromValue(v: u8) !CancelRejectRejectReason {
    return switch (v) {
        1 => .OrderNotFound,
        2 => .AlreadyCanceled,
        3 => .AlreadyFilled,
        4 => .TooLateToCancel,
        else => error.InvalidEnumValue,
    };
}

pub fn CancelRejectRejectReasonToValue(e: CancelRejectRejectReason) u8 { return @intFromEnum(e); }

pub const BalanceUpdateReason = enum(u8) {
    Trade,
    Deposit,
    Withdrawal,
    Transfer,
    Fee,
};

pub fn BalanceUpdateReasonFromValue(v: u8) !BalanceUpdateReason {
    return switch (v) {
        1 => .Trade,
        2 => .Deposit,
        3 => .Withdrawal,
        4 => .Transfer,
        5 => .Fee,
        else => error.InvalidEnumValue,
    };
}

pub fn BalanceUpdateReasonToValue(e: BalanceUpdateReason) u8 { return @intFromEnum(e); }

pub const OrderListStatusStatus = enum(u8) {
    Executing,
    AllDone,
    Reject,
};

pub fn OrderListStatusStatusFromValue(v: u8) !OrderListStatusStatus {
    return switch (v) {
        1 => .Executing,
        2 => .AllDone,
        3 => .Reject,
        else => error.InvalidEnumValue,
    };
}

pub fn OrderListStatusStatusToValue(e: OrderListStatusStatus) u8 { return @intFromEnum(e); }

pub const LedgerUpdateKind = enum(u8) {
    Deposit,
    Withdrawal,
    Transfer,
    Fee,
    Funding,
};

pub fn LedgerUpdateKindFromValue(v: u8) !LedgerUpdateKind {
    return switch (v) {
        1 => .Deposit,
        2 => .Withdrawal,
        3 => .Transfer,
        4 => .Fee,
        5 => .Funding,
        else => error.InvalidEnumValue,
    };
}

pub fn LedgerUpdateKindToValue(e: LedgerUpdateKind) u8 { return @intFromEnum(e); }

pub const AggregateTradeSide = enum(u8) {
    Buy,
    Sell,
};

pub fn AggregateTradeSideFromValue(v: u8) !AggregateTradeSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        else => error.InvalidEnumValue,
    };
}

pub fn AggregateTradeSideToValue(e: AggregateTradeSide) u8 { return @intFromEnum(e); }

pub const LiquidationTradeSide = enum(u8) {
    Buy,
    Sell,
};

pub fn LiquidationTradeSideFromValue(v: u8) !LiquidationTradeSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        else => error.InvalidEnumValue,
    };
}

pub fn LiquidationTradeSideToValue(e: LiquidationTradeSide) u8 { return @intFromEnum(e); }

pub const MarketDataUpdateSide = enum(u8) {
    Buy,
    Sell,
};

pub fn MarketDataUpdateSideFromValue(v: u8) !MarketDataUpdateSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        else => error.InvalidEnumValue,
    };
}

pub fn MarketDataUpdateSideToValue(e: MarketDataUpdateSide) u8 { return @intFromEnum(e); }

pub const MarketDataUpdateAction = enum(u8) {
    New,
    Change,
    Delete,
};

pub fn MarketDataUpdateActionFromValue(v: u8) !MarketDataUpdateAction {
    return switch (v) {
        1 => .New,
        2 => .Change,
        3 => .Delete,
        else => error.InvalidEnumValue,
    };
}

pub fn MarketDataUpdateActionToValue(e: MarketDataUpdateAction) u8 { return @intFromEnum(e); }

pub const PublicTradeSide = enum(u8) {
    Buy,
    Sell,
};

pub fn PublicTradeSideFromValue(v: u8) !PublicTradeSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        else => error.InvalidEnumValue,
    };
}

pub fn PublicTradeSideToValue(e: PublicTradeSide) u8 { return @intFromEnum(e); }

pub const CapabilityPathPattern = enum(u8) {
    PubSub,
    RequestResponse,
    RequestStream,
};

pub fn CapabilityPathPatternFromValue(v: u8) !CapabilityPathPattern {
    return switch (v) {
        1 => .PubSub,
        2 => .RequestResponse,
        3 => .RequestStream,
        else => error.InvalidEnumValue,
    };
}

pub fn CapabilityPathPatternToValue(e: CapabilityPathPattern) u8 { return @intFromEnum(e); }

/// SBE encoder for NewOrderSingle
pub const NewOrderSingleEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id []const u8, side NewOrderSingleSide, order_qty f64, price ?f64, stop_price ?f64, symbol []const u8, order_type NewOrderSingleOrderType, time_in_force NewOrderSingleTimeInForce, expire_time ?i64, account ?[]const u8, strategy_id ?[]const u8, security_id ?[]const u8, id_source ?NewOrderSingleIdSource, security_exchange ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 1);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 36);

        // Fixed fields
        try writeString(&buf, cl_ord_id);
        try buf.append(NewOrderSingleSideToValue(side));
        try writeF64BE(&buf, order_qty);
        try writeF64BE(&buf, price orelse 0);
        try buf.append(if (price != null) 1 else 0);
        try writeF64BE(&buf, stop_price orelse 0);
        try buf.append(if (stop_price != null) 1 else 0);
        try writeString(&buf, symbol);
        try buf.append(NewOrderSingleOrderTypeToValue(order_type));
        try buf.append(NewOrderSingleTimeInForceToValue(time_in_force));
        if (expire_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (account) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }
        if (strategy_id) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }
        if (security_id) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }
        try buf.append(if (id_source) |v| NewOrderSingleIdSourceToValue(v) else 0);
        if (security_exchange) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for NewOrderSingle
pub const NewOrderSingleDecoder = struct {
    cl_ord_id: []const u8,
    side: NewOrderSingleSide,
    order_qty: f64,
    price: ?f64,
    stop_price: ?f64,
    symbol: []const u8,
    order_type: NewOrderSingleOrderType,
    time_in_force: NewOrderSingleTimeInForce,
    expire_time: ?i64,
    account: ?[]const u8,
    strategy_id: ?[]const u8,
    security_id: ?[]const u8,
    id_source: ?NewOrderSingleIdSource,
    security_exchange: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 1) return error.InvalidTemplateId;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const side_raw = buf[pos];
        pos += 1;
        const side = try NewOrderSingleSideFromValue(side_raw);
        const order_qty = readF64BE(buf, &pos);
        const price_raw = readF64BE(buf, &pos);
        var price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; price = price_raw; } else { pos += 1; }
        const stop_price_raw = readF64BE(buf, &pos);
        var stop_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; stop_price = stop_price_raw; } else { pos += 1; }
        const symbol = try readString(buf, &pos, allocator);
        const order_type_raw = buf[pos];
        pos += 1;
        const order_type = try NewOrderSingleOrderTypeFromValue(order_type_raw);
        const time_in_force_raw = buf[pos];
        pos += 1;
        const time_in_force = try NewOrderSingleTimeInForceFromValue(time_in_force_raw);
        var expire_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; expire_time = readI64BE(buf, &pos); } else { pos += 1; }
        var account: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; account = try readString(buf, &pos, allocator); } else { pos += 1; }
        var strategy_id: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; strategy_id = try readString(buf, &pos, allocator); } else { pos += 1; }
        var security_id: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; security_id = try readString(buf, &pos, allocator); } else { pos += 1; }
        const id_source_raw = buf[pos];
        pos += 1;
        const id_source = try NewOrderSingleIdSourceFromValue(id_source_raw);
        var security_exchange: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; security_exchange = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .cl_ord_id = cl_ord_id,
            .side = side,
            .order_qty = order_qty,
            .price = price,
            .stop_price = stop_price,
            .symbol = symbol,
            .order_type = order_type,
            .time_in_force = time_in_force,
            .expire_time = expire_time,
            .account = account,
            .strategy_id = strategy_id,
            .security_id = security_id,
            .id_source = id_source,
            .security_exchange = security_exchange,
        };
    }
};

/// SBE encoder for CancelRequest
pub const CancelRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id []const u8, orig_cl_ord_id []const u8, symbol []const u8, side CancelRequestSide, order_qty ?f64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 2);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 9);

        // Fixed fields
        try writeString(&buf, cl_ord_id);
        try writeString(&buf, orig_cl_ord_id);
        try writeString(&buf, symbol);
        try buf.append(CancelRequestSideToValue(side));
        try writeF64BE(&buf, order_qty orelse 0);
        try buf.append(if (order_qty != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CancelRequest
pub const CancelRequestDecoder = struct {
    cl_ord_id: []const u8,
    orig_cl_ord_id: []const u8,
    symbol: []const u8,
    side: CancelRequestSide,
    order_qty: ?f64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 2) return error.InvalidTemplateId;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const side_raw = buf[pos];
        pos += 1;
        const side = try CancelRequestSideFromValue(side_raw);
        const order_qty_raw = readF64BE(buf, &pos);
        var order_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; order_qty = order_qty_raw; } else { pos += 1; }
        return .{
            .cl_ord_id = cl_ord_id,
            .orig_cl_ord_id = orig_cl_ord_id,
            .symbol = symbol,
            .side = side,
            .order_qty = order_qty,
        };
    }
};

/// SBE encoder for CancelReplaceRequest
pub const CancelReplaceRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id []const u8, orig_cl_ord_id []const u8, symbol []const u8, side CancelReplaceRequestSide, order_qty f64, price ?f64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 3);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 17);

        // Fixed fields
        try writeString(&buf, cl_ord_id);
        try writeString(&buf, orig_cl_ord_id);
        try writeString(&buf, symbol);
        try buf.append(CancelReplaceRequestSideToValue(side));
        try writeF64BE(&buf, order_qty);
        try writeF64BE(&buf, price orelse 0);
        try buf.append(if (price != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CancelReplaceRequest
pub const CancelReplaceRequestDecoder = struct {
    cl_ord_id: []const u8,
    orig_cl_ord_id: []const u8,
    symbol: []const u8,
    side: CancelReplaceRequestSide,
    order_qty: f64,
    price: ?f64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 3) return error.InvalidTemplateId;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const side_raw = buf[pos];
        pos += 1;
        const side = try CancelReplaceRequestSideFromValue(side_raw);
        const order_qty = readF64BE(buf, &pos);
        const price_raw = readF64BE(buf, &pos);
        var price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; price = price_raw; } else { pos += 1; }
        return .{
            .cl_ord_id = cl_ord_id,
            .orig_cl_ord_id = orig_cl_ord_id,
            .symbol = symbol,
            .side = side,
            .order_qty = order_qty,
            .price = price,
        };
    }
};

/// SBE encoder for ExecutionReport
pub const ExecutionReportEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id []const u8, order_id []const u8, exec_id []const u8, exec_type ExecutionReportExecType, ord_status ExecutionReportOrdStatus, side ExecutionReportSide, last_qty ?f64, last_price ?f64, leaves_qty f64, cum_qty f64, avg_price f64, symbol []const u8, transact_time i64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 4);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 51);

        // Fixed fields
        try writeString(&buf, cl_ord_id);
        try writeString(&buf, order_id);
        try writeString(&buf, exec_id);
        try buf.append(ExecutionReportExecTypeToValue(exec_type));
        try buf.append(ExecutionReportOrdStatusToValue(ord_status));
        try buf.append(ExecutionReportSideToValue(side));
        try writeF64BE(&buf, last_qty orelse 0);
        try buf.append(if (last_qty != null) 1 else 0);
        try writeF64BE(&buf, last_price orelse 0);
        try buf.append(if (last_price != null) 1 else 0);
        try writeF64BE(&buf, leaves_qty);
        try writeF64BE(&buf, cum_qty);
        try writeF64BE(&buf, avg_price);
        try writeString(&buf, symbol);
        try writeI64BE(&buf, transact_time);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for ExecutionReport
pub const ExecutionReportDecoder = struct {
    cl_ord_id: []const u8,
    order_id: []const u8,
    exec_id: []const u8,
    exec_type: ExecutionReportExecType,
    ord_status: ExecutionReportOrdStatus,
    side: ExecutionReportSide,
    last_qty: ?f64,
    last_price: ?f64,
    leaves_qty: f64,
    cum_qty: f64,
    avg_price: f64,
    symbol: []const u8,
    transact_time: i64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 4) return error.InvalidTemplateId;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const order_id = try readString(buf, &pos, allocator);
        const exec_id = try readString(buf, &pos, allocator);
        const exec_type_raw = buf[pos];
        pos += 1;
        const exec_type = try ExecutionReportExecTypeFromValue(exec_type_raw);
        const ord_status_raw = buf[pos];
        pos += 1;
        const ord_status = try ExecutionReportOrdStatusFromValue(ord_status_raw);
        const side_raw = buf[pos];
        pos += 1;
        const side = try ExecutionReportSideFromValue(side_raw);
        const last_qty_raw = readF64BE(buf, &pos);
        var last_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; last_qty = last_qty_raw; } else { pos += 1; }
        const last_price_raw = readF64BE(buf, &pos);
        var last_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; last_price = last_price_raw; } else { pos += 1; }
        const leaves_qty = readF64BE(buf, &pos);
        const cum_qty = readF64BE(buf, &pos);
        const avg_price = readF64BE(buf, &pos);
        const symbol = try readString(buf, &pos, allocator);
        const transact_time = readI64BE(buf, &pos);
        return .{
            .cl_ord_id = cl_ord_id,
            .order_id = order_id,
            .exec_id = exec_id,
            .exec_type = exec_type,
            .ord_status = ord_status,
            .side = side,
            .last_qty = last_qty,
            .last_price = last_price,
            .leaves_qty = leaves_qty,
            .cum_qty = cum_qty,
            .avg_price = avg_price,
            .symbol = symbol,
            .transact_time = transact_time,
        };
    }
};

/// SBE encoder for CancelReject
pub const CancelRejectEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id []const u8, orig_cl_ord_id []const u8, reject_reason CancelRejectRejectReason, symbol []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 5);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, cl_ord_id);
        try writeString(&buf, orig_cl_ord_id);
        try buf.append(CancelRejectRejectReasonToValue(reject_reason));
        try writeString(&buf, symbol);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CancelReject
pub const CancelRejectDecoder = struct {
    cl_ord_id: []const u8,
    orig_cl_ord_id: []const u8,
    reject_reason: CancelRejectRejectReason,
    symbol: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 5) return error.InvalidTemplateId;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const reject_reason_raw = buf[pos];
        pos += 1;
        const reject_reason = try CancelRejectRejectReasonFromValue(reject_reason_raw);
        const symbol = try readString(buf, &pos, allocator);
        return .{
            .cl_ord_id = cl_ord_id,
            .orig_cl_ord_id = orig_cl_ord_id,
            .reject_reason = reject_reason,
            .symbol = symbol,
        };
    }
};

/// SBE encoder for MarketDataSnapshot
pub const MarketDataSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, exchange []const u8, bids [][]const u8, asks [][]const u8, timestamp i64, sequence ?i64, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 6);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 17);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeString(&buf, exchange);
        try writeU32BE(&buf, @intCast(bids.len));
        for (bids) |item| {
            try PriceLevel.encode(allocator, item, &buf);
        }
        try writeU32BE(&buf, @intCast(asks.len));
        for (asks) |item| {
            try PriceLevel.encode(allocator, item, &buf);
        }
        try writeI64BE(&buf, timestamp);
        if (sequence) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarketDataSnapshot
pub const MarketDataSnapshotDecoder = struct {
    symbol: []const u8,
    exchange: []const u8,
    bids: [][]const u8,
    asks: [][]const u8,
    timestamp: i64,
    sequence: ?i64,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 6) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const exchange = try readString(buf, &pos, allocator);
        const bids_count = readU32BE(buf, &pos);
        var bids = try allocator.alloc(PriceLevel, bids_count);
        var idx: usize = 0;
        while (idx < bids_count) : (idx += 1) {
            const item = try PriceLevelDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            bids[idx] = item;
        }
        const asks_count = readU32BE(buf, &pos);
        var asks = try allocator.alloc(PriceLevel, asks_count);
        var idx: usize = 0;
        while (idx < asks_count) : (idx += 1) {
            const item = try PriceLevelDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            asks[idx] = item;
        }
        const timestamp = readI64BE(buf, &pos);
        var sequence: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = readI64BE(buf, &pos); } else { pos += 1; }
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .symbol = symbol,
            .exchange = exchange,
            .bids = bids,
            .asks = asks,
            .timestamp = timestamp,
            .sequence = sequence,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for MarketDataIncrementalRefresh
pub const MarketDataIncrementalRefreshEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, updates [][]const u8, timestamp i64, sequence ?i64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 7);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 16);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeU32BE(&buf, @intCast(updates.len));
        for (updates) |item| {
            try MarketDataUpdate.encode(allocator, item, &buf);
        }
        try writeI64BE(&buf, timestamp);
        if (sequence) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarketDataIncrementalRefresh
pub const MarketDataIncrementalRefreshDecoder = struct {
    symbol: []const u8,
    updates: [][]const u8,
    timestamp: i64,
    sequence: ?i64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 7) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const updates_count = readU32BE(buf, &pos);
        var updates = try allocator.alloc(MarketDataUpdate, updates_count);
        var idx: usize = 0;
        while (idx < updates_count) : (idx += 1) {
            const item = try MarketDataUpdateDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            updates[idx] = item;
        }
        const timestamp = readI64BE(buf, &pos);
        var sequence: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = readI64BE(buf, &pos); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .updates = updates,
            .timestamp = timestamp,
            .sequence = sequence,
        };
    }
};

/// SBE encoder for OrderBookRequest
pub const OrderBookRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, depth ?u32, at_time ?i64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 8);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 12);

        // Fixed fields
        try writeString(&buf, symbol);
        // TODO: encode depth as u32
        if (at_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderBookRequest
pub const OrderBookRequestDecoder = struct {
    symbol: []const u8,
    depth: ?u32,
    at_time: ?i64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 8) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        // TODO: decode depth
        var at_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; at_time = readI64BE(buf, &pos); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .depth = depth,
            .at_time = at_time,
        };
    }
};

/// SBE encoder for OrderBookSnapshot
pub const OrderBookSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, exchange []const u8, bids [][]const u8, asks [][]const u8, timestamp i64, sequence ?i64, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 9);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 17);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeString(&buf, exchange);
        try writeU32BE(&buf, @intCast(bids.len));
        for (bids) |item| {
            try PriceLevel.encode(allocator, item, &buf);
        }
        try writeU32BE(&buf, @intCast(asks.len));
        for (asks) |item| {
            try PriceLevel.encode(allocator, item, &buf);
        }
        try writeI64BE(&buf, timestamp);
        if (sequence) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderBookSnapshot
pub const OrderBookSnapshotDecoder = struct {
    symbol: []const u8,
    exchange: []const u8,
    bids: [][]const u8,
    asks: [][]const u8,
    timestamp: i64,
    sequence: ?i64,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 9) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const exchange = try readString(buf, &pos, allocator);
        const bids_count = readU32BE(buf, &pos);
        var bids = try allocator.alloc(PriceLevel, bids_count);
        var idx: usize = 0;
        while (idx < bids_count) : (idx += 1) {
            const item = try PriceLevelDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            bids[idx] = item;
        }
        const asks_count = readU32BE(buf, &pos);
        var asks = try allocator.alloc(PriceLevel, asks_count);
        var idx: usize = 0;
        while (idx < asks_count) : (idx += 1) {
            const item = try PriceLevelDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            asks[idx] = item;
        }
        const timestamp = readI64BE(buf, &pos);
        var sequence: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = readI64BE(buf, &pos); } else { pos += 1; }
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .symbol = symbol,
            .exchange = exchange,
            .bids = bids,
            .asks = asks,
            .timestamp = timestamp,
            .sequence = sequence,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for OrderBookDelta
pub const OrderBookDeltaEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, updates [][]const u8, timestamp i64, sequence ?i64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 10);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 16);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeU32BE(&buf, @intCast(updates.len));
        for (updates) |item| {
            try MarketDataUpdate.encode(allocator, item, &buf);
        }
        try writeI64BE(&buf, timestamp);
        if (sequence) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderBookDelta
pub const OrderBookDeltaDecoder = struct {
    symbol: []const u8,
    updates: [][]const u8,
    timestamp: i64,
    sequence: ?i64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 10) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const updates_count = readU32BE(buf, &pos);
        var updates = try allocator.alloc(MarketDataUpdate, updates_count);
        var idx: usize = 0;
        while (idx < updates_count) : (idx += 1) {
            const item = try MarketDataUpdateDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            updates[idx] = item;
        }
        const timestamp = readI64BE(buf, &pos);
        var sequence: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = readI64BE(buf, &pos); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .updates = updates,
            .timestamp = timestamp,
            .sequence = sequence,
        };
    }
};

/// SBE encoder for AggregateTradeEvent
pub const AggregateTradeEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, trade []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 11);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, trade);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AggregateTradeEvent
pub const AggregateTradeEventDecoder = struct {
    trade: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 11) return error.InvalidTemplateId;
        const trade = try readString(buf, &pos, allocator);
        return .{
            .trade = trade,
        };
    }
};

/// SBE encoder for AggregateTradeRequest
pub const AggregateTradeRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, start_time ?i64, end_time ?i64, limit ?u32, cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 12);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 20);

        // Fixed fields
        try writeString(&buf, symbol);
        if (start_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (end_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        // TODO: encode limit as u32
        if (cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AggregateTradeRequest
pub const AggregateTradeRequestDecoder = struct {
    symbol: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 12) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = readI64BE(buf, &pos); } else { pos += 1; }
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = readI64BE(buf, &pos); } else { pos += 1; }
        // TODO: decode limit
        var cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .start_time = start_time,
            .end_time = end_time,
            .limit = limit,
            .cursor = cursor,
        };
    }
};

/// SBE encoder for AggregateTradeBatch
pub const AggregateTradeBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, trades [][]const u8, has_more u8, next_cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 13);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeU32BE(&buf, @intCast(trades.len));
        for (trades) |item| {
            try AggregateTrade.encode(allocator, item, &buf);
        }
        try buf.append(has_more);
        if (next_cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AggregateTradeBatch
pub const AggregateTradeBatchDecoder = struct {
    symbol: []const u8,
    trades: [][]const u8,
    has_more: u8,
    next_cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 13) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const trades_count = readU32BE(buf, &pos);
        var trades = try allocator.alloc(AggregateTrade, trades_count);
        var idx: usize = 0;
        while (idx < trades_count) : (idx += 1) {
            const item = try AggregateTradeDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            trades[idx] = item;
        }
        const has_more = buf[pos];
        pos += 1;
        var next_cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; next_cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .trades = trades,
            .has_more = has_more,
            .next_cursor = next_cursor,
        };
    }
};

/// SBE encoder for MiniTicker
pub const MiniTickerEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, last_price f64, volume f64, timestamp i64, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 14);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 25);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeF64BE(&buf, last_price);
        try writeF64BE(&buf, volume);
        try writeI64BE(&buf, timestamp);
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MiniTicker
pub const MiniTickerDecoder = struct {
    symbol: []const u8,
    last_price: f64,
    volume: f64,
    timestamp: i64,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 14) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const last_price = readF64BE(buf, &pos);
        const volume = readF64BE(buf, &pos);
        const timestamp = readI64BE(buf, &pos);
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .symbol = symbol,
            .last_price = last_price,
            .volume = volume,
            .timestamp = timestamp,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for AllMidsRequest
pub const AllMidsRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, ) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 15);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AllMidsRequest
pub const AllMidsRequestDecoder = struct {
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 15) return error.InvalidTemplateId;
        return .{

        };
    }
};

/// SBE encoder for AllMidsBatch
pub const AllMidsBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, tickers [][]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 16);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeU32BE(&buf, @intCast(tickers.len));
        for (tickers) |item| {
            try MiniTicker.encode(allocator, item, &buf);
        }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AllMidsBatch
pub const AllMidsBatchDecoder = struct {
    tickers: [][]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 16) return error.InvalidTemplateId;
        const tickers_count = readU32BE(buf, &pos);
        var tickers = try allocator.alloc(MiniTicker, tickers_count);
        var idx: usize = 0;
        while (idx < tickers_count) : (idx += 1) {
            const item = try MiniTickerDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            tickers[idx] = item;
        }
        return .{
            .tickers = tickers,
        };
    }
};

/// SBE encoder for MarkPriceUpdate
pub const MarkPriceUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, mark_price f64, index_price ?f64, funding_rate ?f64, timestamp i64, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 17);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 33);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeF64BE(&buf, mark_price);
        try writeF64BE(&buf, index_price orelse 0);
        try buf.append(if (index_price != null) 1 else 0);
        if (funding_rate) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        try writeI64BE(&buf, timestamp);
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarkPriceUpdate
pub const MarkPriceUpdateDecoder = struct {
    symbol: []const u8,
    mark_price: f64,
    index_price: ?f64,
    funding_rate: ?f64,
    timestamp: i64,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 17) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const mark_price = readF64BE(buf, &pos);
        const index_price_raw = readF64BE(buf, &pos);
        var index_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; index_price = index_price_raw; } else { pos += 1; }
        var funding_rate: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; funding_rate = readI64BE(buf, &pos); } else { pos += 1; }
        const timestamp = readI64BE(buf, &pos);
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .symbol = symbol,
            .mark_price = mark_price,
            .index_price = index_price,
            .funding_rate = funding_rate,
            .timestamp = timestamp,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for MarkPriceRequest
pub const MarkPriceRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 18);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, symbol);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarkPriceRequest
pub const MarkPriceRequestDecoder = struct {
    symbol: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 18) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        return .{
            .symbol = symbol,
        };
    }
};

/// SBE encoder for LiquidationTradeEvent
pub const LiquidationTradeEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, trade []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 19);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, trade);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LiquidationTradeEvent
pub const LiquidationTradeEventDecoder = struct {
    trade: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 19) return error.InvalidTemplateId;
        const trade = try readString(buf, &pos, allocator);
        return .{
            .trade = trade,
        };
    }
};

/// SBE encoder for CandleBarEvent
pub const CandleBarEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, bar []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 20);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, bar);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CandleBarEvent
pub const CandleBarEventDecoder = struct {
    bar: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 20) return error.InvalidTemplateId;
        const bar = try readString(buf, &pos, allocator);
        return .{
            .bar = bar,
        };
    }
};

/// SBE encoder for CandleBarRequest
pub const CandleBarRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, interval []const u8, start_time ?i64, end_time ?i64, limit ?u32, cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 21);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 20);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeString(&buf, interval);
        if (start_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (end_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        // TODO: encode limit as u32
        if (cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CandleBarRequest
pub const CandleBarRequestDecoder = struct {
    symbol: []const u8,
    interval: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 21) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const interval = try readString(buf, &pos, allocator);
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = readI64BE(buf, &pos); } else { pos += 1; }
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = readI64BE(buf, &pos); } else { pos += 1; }
        // TODO: decode limit
        var cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .interval = interval,
            .start_time = start_time,
            .end_time = end_time,
            .limit = limit,
            .cursor = cursor,
        };
    }
};

/// SBE encoder for CandleBarBatch
pub const CandleBarBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, interval []const u8, bars [][]const u8, has_more u8, next_cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 22);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeString(&buf, interval);
        try writeU32BE(&buf, @intCast(bars.len));
        for (bars) |item| {
            try CandleBar.encode(allocator, item, &buf);
        }
        try buf.append(has_more);
        if (next_cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CandleBarBatch
pub const CandleBarBatchDecoder = struct {
    symbol: []const u8,
    interval: []const u8,
    bars: [][]const u8,
    has_more: u8,
    next_cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 22) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const interval = try readString(buf, &pos, allocator);
        const bars_count = readU32BE(buf, &pos);
        var bars = try allocator.alloc(CandleBar, bars_count);
        var idx: usize = 0;
        while (idx < bars_count) : (idx += 1) {
            const item = try CandleBarDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            bars[idx] = item;
        }
        const has_more = buf[pos];
        pos += 1;
        var next_cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; next_cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .interval = interval,
            .bars = bars,
            .has_more = has_more,
            .next_cursor = next_cursor,
        };
    }
};

/// SBE encoder for PublicTradeEvent
pub const PublicTradeEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, trade []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 23);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, trade);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PublicTradeEvent
pub const PublicTradeEventDecoder = struct {
    trade: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 23) return error.InvalidTemplateId;
        const trade = try readString(buf, &pos, allocator);
        return .{
            .trade = trade,
        };
    }
};

/// SBE encoder for TradeHistoryRequest
pub const TradeHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, start_time ?i64, end_time ?i64, limit ?u32, cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 24);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 20);

        // Fixed fields
        try writeString(&buf, symbol);
        if (start_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (end_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        // TODO: encode limit as u32
        if (cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for TradeHistoryRequest
pub const TradeHistoryRequestDecoder = struct {
    symbol: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 24) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = readI64BE(buf, &pos); } else { pos += 1; }
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = readI64BE(buf, &pos); } else { pos += 1; }
        // TODO: decode limit
        var cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .start_time = start_time,
            .end_time = end_time,
            .limit = limit,
            .cursor = cursor,
        };
    }
};

/// SBE encoder for PublicTradeBatch
pub const PublicTradeBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, trades [][]const u8, has_more u8, next_cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 25);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeU32BE(&buf, @intCast(trades.len));
        for (trades) |item| {
            try PublicTrade.encode(allocator, item, &buf);
        }
        try buf.append(has_more);
        if (next_cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PublicTradeBatch
pub const PublicTradeBatchDecoder = struct {
    symbol: []const u8,
    trades: [][]const u8,
    has_more: u8,
    next_cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 25) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const trades_count = readU32BE(buf, &pos);
        var trades = try allocator.alloc(PublicTrade, trades_count);
        var idx: usize = 0;
        while (idx < trades_count) : (idx += 1) {
            const item = try PublicTradeDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            trades[idx] = item;
        }
        const has_more = buf[pos];
        pos += 1;
        var next_cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; next_cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .symbol = symbol,
            .trades = trades,
            .has_more = has_more,
            .next_cursor = next_cursor,
        };
    }
};

/// SBE encoder for BestBidOffer
pub const BestBidOfferEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, bid_price ?f64, bid_qty ?f64, ask_price ?f64, ask_qty ?f64, timestamp i64, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 26);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 41);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeF64BE(&buf, bid_price orelse 0);
        try buf.append(if (bid_price != null) 1 else 0);
        try writeF64BE(&buf, bid_qty orelse 0);
        try buf.append(if (bid_qty != null) 1 else 0);
        try writeF64BE(&buf, ask_price orelse 0);
        try buf.append(if (ask_price != null) 1 else 0);
        try writeF64BE(&buf, ask_qty orelse 0);
        try buf.append(if (ask_qty != null) 1 else 0);
        try writeI64BE(&buf, timestamp);
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for BestBidOffer
pub const BestBidOfferDecoder = struct {
    symbol: []const u8,
    bid_price: ?f64,
    bid_qty: ?f64,
    ask_price: ?f64,
    ask_qty: ?f64,
    timestamp: i64,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 26) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const bid_price_raw = readF64BE(buf, &pos);
        var bid_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; bid_price = bid_price_raw; } else { pos += 1; }
        const bid_qty_raw = readF64BE(buf, &pos);
        var bid_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; bid_qty = bid_qty_raw; } else { pos += 1; }
        const ask_price_raw = readF64BE(buf, &pos);
        var ask_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; ask_price = ask_price_raw; } else { pos += 1; }
        const ask_qty_raw = readF64BE(buf, &pos);
        var ask_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; ask_qty = ask_qty_raw; } else { pos += 1; }
        const timestamp = readI64BE(buf, &pos);
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .symbol = symbol,
            .bid_price = bid_price,
            .bid_qty = bid_qty,
            .ask_price = ask_price,
            .ask_qty = ask_qty,
            .timestamp = timestamp,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for SymbolTicker
pub const SymbolTickerEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8, last_price f64, price_change f64, price_change_pct f64, volume f64, high f64, low f64, open f64, timestamp i64, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 27);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 65);

        // Fixed fields
        try writeString(&buf, symbol);
        try writeF64BE(&buf, last_price);
        try writeI64BE(&buf, price_change);
        try writeI64BE(&buf, price_change_pct);
        try writeF64BE(&buf, volume);
        try writeF64BE(&buf, high);
        try writeF64BE(&buf, low);
        try writeF64BE(&buf, open);
        try writeI64BE(&buf, timestamp);
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for SymbolTicker
pub const SymbolTickerDecoder = struct {
    symbol: []const u8,
    last_price: f64,
    price_change: f64,
    price_change_pct: f64,
    volume: f64,
    high: f64,
    low: f64,
    open: f64,
    timestamp: i64,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 27) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        const last_price = readF64BE(buf, &pos);
        const price_change = readI64BE(buf, &pos);
        const price_change_pct = readI64BE(buf, &pos);
        const volume = readF64BE(buf, &pos);
        const high = readF64BE(buf, &pos);
        const low = readF64BE(buf, &pos);
        const open = readF64BE(buf, &pos);
        const timestamp = readI64BE(buf, &pos);
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .symbol = symbol,
            .last_price = last_price,
            .price_change = price_change,
            .price_change_pct = price_change_pct,
            .volume = volume,
            .high = high,
            .low = low,
            .open = open,
            .timestamp = timestamp,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for TickerRequest
pub const TickerRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 28);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, symbol);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for TickerRequest
pub const TickerRequestDecoder = struct {
    symbol: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 28) return error.InvalidTemplateId;
        const symbol = try readString(buf, &pos, allocator);
        return .{
            .symbol = symbol,
        };
    }
};

/// SBE encoder for AccountSummary
pub const AccountSummaryEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, balance f64, buying_power f64, currency []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 29);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 16);

        // Fixed fields
        try writeString(&buf, account);
        try writeI64BE(&buf, balance);
        try writeI64BE(&buf, buying_power);
        try writeString(&buf, currency);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AccountSummary
pub const AccountSummaryDecoder = struct {
    account: []const u8,
    balance: f64,
    buying_power: f64,
    currency: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 29) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const balance = readI64BE(buf, &pos);
        const buying_power = readI64BE(buf, &pos);
        const currency = try readString(buf, &pos, allocator);
        return .{
            .account = account,
            .balance = balance,
            .buying_power = buying_power,
            .currency = currency,
        };
    }
};

/// SBE encoder for MarginSummary
pub const MarginSummaryEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, balance f64, buying_power f64, equity f64, margin_used f64, available f64, currency []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 30);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 40);

        // Fixed fields
        try writeString(&buf, account);
        try writeI64BE(&buf, balance);
        try writeI64BE(&buf, buying_power);
        try writeI64BE(&buf, equity);
        try writeI64BE(&buf, margin_used);
        try writeI64BE(&buf, available);
        try writeString(&buf, currency);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarginSummary
pub const MarginSummaryDecoder = struct {
    account: []const u8,
    balance: f64,
    buying_power: f64,
    equity: f64,
    margin_used: f64,
    available: f64,
    currency: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 30) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const balance = readI64BE(buf, &pos);
        const buying_power = readI64BE(buf, &pos);
        const equity = readI64BE(buf, &pos);
        const margin_used = readI64BE(buf, &pos);
        const available = readI64BE(buf, &pos);
        const currency = try readString(buf, &pos, allocator);
        return .{
            .account = account,
            .balance = balance,
            .buying_power = buying_power,
            .equity = equity,
            .margin_used = margin_used,
            .available = available,
            .currency = currency,
        };
    }
};

/// SBE encoder for BalanceSnapshot
pub const BalanceSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, balances [][]const u8, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 31);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeU32BE(&buf, @intCast(balances.len));
        for (balances) |item| {
            try BalanceEntry.encode(allocator, item, &buf);
        }
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for BalanceSnapshot
pub const BalanceSnapshotDecoder = struct {
    account: []const u8,
    balances: [][]const u8,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 31) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const balances_count = readU32BE(buf, &pos);
        var balances = try allocator.alloc(BalanceEntry, balances_count);
        var idx: usize = 0;
        while (idx < balances_count) : (idx += 1) {
            const item = try BalanceEntryDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            balances[idx] = item;
        }
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .account = account,
            .balances = balances,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for BalanceUpdate
pub const BalanceUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, asset []const u8, delta f64, total f64, available f64, reason BalanceUpdateReason) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 32);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 25);

        // Fixed fields
        try writeString(&buf, account);
        try writeString(&buf, asset);
        try writeI64BE(&buf, delta);
        try writeI64BE(&buf, total);
        try writeI64BE(&buf, available);
        try buf.append(BalanceUpdateReasonToValue(reason));

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for BalanceUpdate
pub const BalanceUpdateDecoder = struct {
    account: []const u8,
    asset: []const u8,
    delta: f64,
    total: f64,
    available: f64,
    reason: BalanceUpdateReason,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 32) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const asset = try readString(buf, &pos, allocator);
        const delta = readI64BE(buf, &pos);
        const total = readI64BE(buf, &pos);
        const available = readI64BE(buf, &pos);
        const reason_raw = buf[pos];
        pos += 1;
        const reason = try BalanceUpdateReasonFromValue(reason_raw);
        return .{
            .account = account,
            .asset = asset,
            .delta = delta,
            .total = total,
            .available = available,
            .reason = reason,
        };
    }
};

/// SBE encoder for PositionSnapshot
pub const PositionSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, positions [][]const u8, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 33);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeU32BE(&buf, @intCast(positions.len));
        for (positions) |item| {
            try PositionEntry.encode(allocator, item, &buf);
        }
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PositionSnapshot
pub const PositionSnapshotDecoder = struct {
    account: []const u8,
    positions: [][]const u8,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 33) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const positions_count = readU32BE(buf, &pos);
        var positions = try allocator.alloc(PositionEntry, positions_count);
        var idx: usize = 0;
        while (idx < positions_count) : (idx += 1) {
            const item = try PositionEntryDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            positions[idx] = item;
        }
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .account = account,
            .positions = positions,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for PositionUpdate
pub const PositionUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, symbol []const u8, qty f64, entry_price f64, unrealized_pnl f64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 34);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 24);

        // Fixed fields
        try writeString(&buf, account);
        try writeString(&buf, symbol);
        try writeF64BE(&buf, qty);
        try writeF64BE(&buf, entry_price);
        try writeI64BE(&buf, unrealized_pnl);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PositionUpdate
pub const PositionUpdateDecoder = struct {
    account: []const u8,
    symbol: []const u8,
    qty: f64,
    entry_price: f64,
    unrealized_pnl: f64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 34) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const qty = readF64BE(buf, &pos);
        const entry_price = readF64BE(buf, &pos);
        const unrealized_pnl = readI64BE(buf, &pos);
        return .{
            .account = account,
            .symbol = symbol,
            .qty = qty,
            .entry_price = entry_price,
            .unrealized_pnl = unrealized_pnl,
        };
    }
};

/// SBE encoder for MarginUpdate
pub const MarginUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, summary []const u8, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 35);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeString(&buf, summary);
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarginUpdate
pub const MarginUpdateDecoder = struct {
    account: []const u8,
    summary: []const u8,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 35) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const summary = try readString(buf, &pos, allocator);
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .account = account,
            .summary = summary,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for UserLiquidation
pub const UserLiquidationEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, symbol []const u8, qty f64, price f64, timestamp i64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 36);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 24);

        // Fixed fields
        try writeString(&buf, account);
        try writeString(&buf, symbol);
        try writeF64BE(&buf, qty);
        try writeF64BE(&buf, price);
        try writeI64BE(&buf, timestamp);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for UserLiquidation
pub const UserLiquidationDecoder = struct {
    account: []const u8,
    symbol: []const u8,
    qty: f64,
    price: f64,
    timestamp: i64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 36) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const qty = readF64BE(buf, &pos);
        const price = readF64BE(buf, &pos);
        const timestamp = readI64BE(buf, &pos);
        return .{
            .account = account,
            .symbol = symbol,
            .qty = qty,
            .price = price,
            .timestamp = timestamp,
        };
    }
};

/// SBE encoder for OrderListStatus
pub const OrderListStatusEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, list_id []const u8, status OrderListStatusStatus, symbol ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 37);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeString(&buf, list_id);
        try buf.append(OrderListStatusStatusToValue(status));
        if (symbol) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderListStatus
pub const OrderListStatusDecoder = struct {
    account: []const u8,
    list_id: []const u8,
    status: OrderListStatusStatus,
    symbol: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 37) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const list_id = try readString(buf, &pos, allocator);
        const status_raw = buf[pos];
        pos += 1;
        const status = try OrderListStatusStatusFromValue(status_raw);
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; symbol = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .list_id = list_id,
            .status = status,
            .symbol = symbol,
        };
    }
};

/// SBE encoder for FillHistoryRequest
pub const FillHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, symbol ?[]const u8, start_time ?i64, end_time ?i64, limit ?u32, cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 38);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 20);

        // Fixed fields
        try writeString(&buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }
        if (start_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (end_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        // TODO: encode limit as u32
        if (cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FillHistoryRequest
pub const FillHistoryRequestDecoder = struct {
    account: []const u8,
    symbol: ?[]const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 38) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; symbol = try readString(buf, &pos, allocator); } else { pos += 1; }
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = readI64BE(buf, &pos); } else { pos += 1; }
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = readI64BE(buf, &pos); } else { pos += 1; }
        // TODO: decode limit
        var cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .symbol = symbol,
            .start_time = start_time,
            .end_time = end_time,
            .limit = limit,
            .cursor = cursor,
        };
    }
};

/// SBE encoder for FillHistoryBatch
pub const FillHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, fills [][]const u8, has_more u8, next_cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 39);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeU32BE(&buf, @intCast(fills.len));
        for (fills) |item| {
            try ExecutionReport.encode(allocator, item, &buf);
        }
        try buf.append(has_more);
        if (next_cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FillHistoryBatch
pub const FillHistoryBatchDecoder = struct {
    account: []const u8,
    fills: [][]const u8,
    has_more: u8,
    next_cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 39) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const fills_count = readU32BE(buf, &pos);
        var fills = try allocator.alloc(ExecutionReport, fills_count);
        var idx: usize = 0;
        while (idx < fills_count) : (idx += 1) {
            const item = try ExecutionReportDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            fills[idx] = item;
        }
        const has_more = buf[pos];
        pos += 1;
        var next_cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; next_cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .fills = fills,
            .has_more = has_more,
            .next_cursor = next_cursor,
        };
    }
};

/// SBE encoder for FundingPayment
pub const FundingPaymentEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, symbol ?[]const u8, amount f64, rate f64, timestamp i64) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 40);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 24);

        // Fixed fields
        try writeString(&buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }
        try writeI64BE(&buf, amount);
        try writeI64BE(&buf, rate);
        try writeI64BE(&buf, timestamp);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FundingPayment
pub const FundingPaymentDecoder = struct {
    account: []const u8,
    symbol: ?[]const u8,
    amount: f64,
    rate: f64,
    timestamp: i64,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 40) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; symbol = try readString(buf, &pos, allocator); } else { pos += 1; }
        const amount = readI64BE(buf, &pos);
        const rate = readI64BE(buf, &pos);
        const timestamp = readI64BE(buf, &pos);
        return .{
            .account = account,
            .symbol = symbol,
            .amount = amount,
            .rate = rate,
            .timestamp = timestamp,
        };
    }
};

/// SBE encoder for FundingHistoryRequest
pub const FundingHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, start_time ?i64, end_time ?i64, limit ?u32, cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 41);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 20);

        // Fixed fields
        try writeString(&buf, account);
        if (start_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (end_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        // TODO: encode limit as u32
        if (cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FundingHistoryRequest
pub const FundingHistoryRequestDecoder = struct {
    account: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 41) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = readI64BE(buf, &pos); } else { pos += 1; }
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = readI64BE(buf, &pos); } else { pos += 1; }
        // TODO: decode limit
        var cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .start_time = start_time,
            .end_time = end_time,
            .limit = limit,
            .cursor = cursor,
        };
    }
};

/// SBE encoder for FundingHistoryBatch
pub const FundingHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, payments [][]const u8, has_more u8, next_cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 42);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeU32BE(&buf, @intCast(payments.len));
        for (payments) |item| {
            try FundingPayment.encode(allocator, item, &buf);
        }
        try buf.append(has_more);
        if (next_cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FundingHistoryBatch
pub const FundingHistoryBatchDecoder = struct {
    account: []const u8,
    payments: [][]const u8,
    has_more: u8,
    next_cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 42) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const payments_count = readU32BE(buf, &pos);
        var payments = try allocator.alloc(FundingPayment, payments_count);
        var idx: usize = 0;
        while (idx < payments_count) : (idx += 1) {
            const item = try FundingPaymentDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            payments[idx] = item;
        }
        const has_more = buf[pos];
        pos += 1;
        var next_cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; next_cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .payments = payments,
            .has_more = has_more,
            .next_cursor = next_cursor,
        };
    }
};

/// SBE encoder for LedgerUpdate
pub const LedgerUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, asset []const u8, delta f64, kind LedgerUpdateKind, timestamp i64, reference_id ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 43);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 17);

        // Fixed fields
        try writeString(&buf, account);
        try writeString(&buf, asset);
        try writeI64BE(&buf, delta);
        try buf.append(LedgerUpdateKindToValue(kind));
        try writeI64BE(&buf, timestamp);
        if (reference_id) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LedgerUpdate
pub const LedgerUpdateDecoder = struct {
    account: []const u8,
    asset: []const u8,
    delta: f64,
    kind: LedgerUpdateKind,
    timestamp: i64,
    reference_id: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 43) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const asset = try readString(buf, &pos, allocator);
        const delta = readI64BE(buf, &pos);
        const kind_raw = buf[pos];
        pos += 1;
        const kind = try LedgerUpdateKindFromValue(kind_raw);
        const timestamp = readI64BE(buf, &pos);
        var reference_id: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; reference_id = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .asset = asset,
            .delta = delta,
            .kind = kind,
            .timestamp = timestamp,
            .reference_id = reference_id,
        };
    }
};

/// SBE encoder for LedgerHistoryRequest
pub const LedgerHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, start_time ?i64, end_time ?i64, limit ?u32, cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 44);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 20);

        // Fixed fields
        try writeString(&buf, account);
        if (start_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (end_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        // TODO: encode limit as u32
        if (cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LedgerHistoryRequest
pub const LedgerHistoryRequestDecoder = struct {
    account: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 44) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = readI64BE(buf, &pos); } else { pos += 1; }
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = readI64BE(buf, &pos); } else { pos += 1; }
        // TODO: decode limit
        var cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .start_time = start_time,
            .end_time = end_time,
            .limit = limit,
            .cursor = cursor,
        };
    }
};

/// SBE encoder for LedgerHistoryBatch
pub const LedgerHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, entries [][]const u8, has_more u8, next_cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 45);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeU32BE(&buf, @intCast(entries.len));
        for (entries) |item| {
            try LedgerUpdate.encode(allocator, item, &buf);
        }
        try buf.append(has_more);
        if (next_cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LedgerHistoryBatch
pub const LedgerHistoryBatchDecoder = struct {
    account: []const u8,
    entries: [][]const u8,
    has_more: u8,
    next_cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 45) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const entries_count = readU32BE(buf, &pos);
        var entries = try allocator.alloc(LedgerUpdate, entries_count);
        var idx: usize = 0;
        while (idx < entries_count) : (idx += 1) {
            const item = try LedgerUpdateDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            entries[idx] = item;
        }
        const has_more = buf[pos];
        pos += 1;
        var next_cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; next_cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .entries = entries,
            .has_more = has_more,
            .next_cursor = next_cursor,
        };
    }
};

/// SBE encoder for OpenOrdersRequest
pub const OpenOrdersRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, symbol ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 46);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OpenOrdersRequest
pub const OpenOrdersRequestDecoder = struct {
    account: []const u8,
    symbol: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 46) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; symbol = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .symbol = symbol,
        };
    }
};

/// SBE encoder for OpenOrdersSnapshot
pub const OpenOrdersSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, orders [][]const u8, is_snapshot ?u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 47);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeU32BE(&buf, @intCast(orders.len));
        for (orders) |item| {
            try ExecutionReport.encode(allocator, item, &buf);
        }
        try buf.append(is_snapshot orelse 0);
        try buf.append(if (is_snapshot != null) 1 else 0);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OpenOrdersSnapshot
pub const OpenOrdersSnapshotDecoder = struct {
    account: []const u8,
    orders: [][]const u8,
    is_snapshot: ?u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 47) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const orders_count = readU32BE(buf, &pos);
        var orders = try allocator.alloc(ExecutionReport, orders_count);
        var idx: usize = 0;
        while (idx < orders_count) : (idx += 1) {
            const item = try ExecutionReportDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            orders[idx] = item;
        }
        const v = buf[pos];
        pos += 1;
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = v; } else { pos += 1; }
        return .{
            .account = account,
            .orders = orders,
            .is_snapshot = is_snapshot,
        };
    }
};

/// SBE encoder for OrderHistoryRequest
pub const OrderHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, symbol ?[]const u8, start_time ?i64, end_time ?i64, limit ?u32, cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 48);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 20);

        // Fixed fields
        try writeString(&buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }
        if (start_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        if (end_time) |v| { try buf.append(1); try writeI64BE(&buf, v); } else { try buf.append(0); }
        // TODO: encode limit as u32
        if (cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderHistoryRequest
pub const OrderHistoryRequestDecoder = struct {
    account: []const u8,
    symbol: ?[]const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 48) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; symbol = try readString(buf, &pos, allocator); } else { pos += 1; }
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = readI64BE(buf, &pos); } else { pos += 1; }
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = readI64BE(buf, &pos); } else { pos += 1; }
        // TODO: decode limit
        var cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .symbol = symbol,
            .start_time = start_time,
            .end_time = end_time,
            .limit = limit,
            .cursor = cursor,
        };
    }
};

/// SBE encoder for OrderHistoryBatch
pub const OrderHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8, orders [][]const u8, has_more u8, next_cursor ?[]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 49);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 1);

        // Fixed fields
        try writeString(&buf, account);
        try writeU32BE(&buf, @intCast(orders.len));
        for (orders) |item| {
            try ExecutionReport.encode(allocator, item, &buf);
        }
        try buf.append(has_more);
        if (next_cursor) |s| { try buf.append(1); try writeString(&buf, s); } else { try buf.append(0); }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderHistoryBatch
pub const OrderHistoryBatchDecoder = struct {
    account: []const u8,
    orders: [][]const u8,
    has_more: u8,
    next_cursor: ?[]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 49) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        const orders_count = readU32BE(buf, &pos);
        var orders = try allocator.alloc(ExecutionReport, orders_count);
        var idx: usize = 0;
        while (idx < orders_count) : (idx += 1) {
            const item = try ExecutionReportDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            orders[idx] = item;
        }
        const has_more = buf[pos];
        pos += 1;
        var next_cursor: ?[]u8 = null;
        if (buf[pos] == 1) { pos += 1; next_cursor = try readString(buf, &pos, allocator); } else { pos += 1; }
        return .{
            .account = account,
            .orders = orders,
            .has_more = has_more,
            .next_cursor = next_cursor,
        };
    }
};

/// SBE encoder for CapabilitiesRequest
pub const CapabilitiesRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, ) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 50);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CapabilitiesRequest
pub const CapabilitiesRequestDecoder = struct {
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 50) return error.InvalidTemplateId;
        return .{

        };
    }
};

/// SBE encoder for CapabilitiesResponse
pub const CapabilitiesResponseEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, schema_ids [][]const u8, paths [][]const u8, symbols [][]const u8, intervals [][]const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 51);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeU32BE(&buf, @intCast(schema_ids.len));
        for (schema_ids) |item| {
            try u8.encode(allocator, item, &buf);
        }
        try writeU32BE(&buf, @intCast(paths.len));
        for (paths) |item| {
            try CapabilityPath.encode(allocator, item, &buf);
        }
        try writeU32BE(&buf, @intCast(symbols.len));
        for (symbols) |item| {
            try Symbol.encode(allocator, item, &buf);
        }
        try writeU32BE(&buf, @intCast(intervals.len));
        for (intervals) |item| {
            try CandleInterval.encode(allocator, item, &buf);
        }

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CapabilitiesResponse
pub const CapabilitiesResponseDecoder = struct {
    schema_ids: [][]const u8,
    paths: [][]const u8,
    symbols: [][]const u8,
    intervals: [][]const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 51) return error.InvalidTemplateId;
        const schema_ids_count = readU32BE(buf, &pos);
        var schema_ids = try allocator.alloc(u8, schema_ids_count);
        var idx: usize = 0;
        while (idx < schema_ids_count) : (idx += 1) {
            const item = try u8Decoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            schema_ids[idx] = item;
        }
        const paths_count = readU32BE(buf, &pos);
        var paths = try allocator.alloc(CapabilityPath, paths_count);
        var idx: usize = 0;
        while (idx < paths_count) : (idx += 1) {
            const item = try CapabilityPathDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            paths[idx] = item;
        }
        const symbols_count = readU32BE(buf, &pos);
        var symbols = try allocator.alloc(Symbol, symbols_count);
        var idx: usize = 0;
        while (idx < symbols_count) : (idx += 1) {
            const item = try SymbolDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            symbols[idx] = item;
        }
        const intervals_count = readU32BE(buf, &pos);
        var intervals = try allocator.alloc(CandleInterval, intervals_count);
        var idx: usize = 0;
        while (idx < intervals_count) : (idx += 1) {
            const item = try CandleIntervalDecoder.decode(allocator, buf[pos..]);
            pos += item.encodedLen();
            intervals[idx] = item;
        }
        return .{
            .schema_ids = schema_ids,
            .paths = paths,
            .symbols = symbols,
            .intervals = intervals,
        };
    }
};

/// SBE encoder for PositionRequest
pub const PositionRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account []const u8) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        defer buf.deinit();
        // SBE Message Header (8 bytes)
        try writeU16BE(&buf, SCHEMA_ID);
        try writeU16BE(&buf, 52);
        try writeU16BE(&buf, 0);
        try writeU16BE(&buf, 0);

        // Fixed fields
        try writeString(&buf, account);

        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PositionRequest
pub const PositionRequestDecoder = struct {
    account: []const u8,
    pub fn encodedLen(self: @This()) usize { return 0; }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !@This() {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos); _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 52) return error.InvalidTemplateId;
        const account = try readString(buf, &pos, allocator);
        return .{
            .account = account,
        };
    }
};

