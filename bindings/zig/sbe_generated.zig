// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' v1.0.0
// Standard order entry and execution messages

const std = @import("std");

pub const SCHEMA_ID: u16 = 0x01;

fn writeStringU32(buf: *std.ArrayList(u8), s: []const u8) !void { try writeU32BE(buf, @intCast(s.len)); try buf.appendSlice(s); }
fn readStringU32(buf: []const u8, pos: *usize, allocator: std.mem.Allocator) ![]u8 { const n = readU32BE(buf, pos); const s = try allocator.dupe(u8, buf[pos.* .. pos.* + n]); pos.* += n; return s; }
fn readU8(buf: []const u8, pos: *usize) u8 { const v = buf[pos.*]; pos.* += 1; return v; }
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
    Buy = 1,
    Sell = 2,
    SellShort = 3,
    SellShortExempt = 4,
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
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLimit = 4,
    MarketOnClose = 5,
    LimitOnClose = 6,
    Pegged = 7,
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
    Day = 1,
    Gtc = 2,
    Ioc = 3,
    Fok = 4,
    Gtd = 5,
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
    Cusip = 1,
    Sedol = 2,
    Isin = 3,
    Ric = 4,
    ExchangeSymbol = 5,
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
    Buy = 1,
    Sell = 2,
    SellShort = 3,
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
    Buy = 1,
    Sell = 2,
    SellShort = 3,
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
    New = 1,
    PartialFill = 2,
    Fill = 3,
    DoneForDay = 4,
    Canceled = 5,
    Replaced = 6,
    PendingCancel = 7,
    Stopped = 8,
    Rejected = 9,
    Suspended = 10,
    PendingNew = 11,
    Expired = 12,
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
    New = 1,
    PartiallyFilled = 2,
    Filled = 3,
    DoneForDay = 4,
    Canceled = 5,
    PendingCancel = 6,
    Stopped = 7,
    Rejected = 8,
    Suspended = 9,
    PendingNew = 10,
    Expired = 11,
    Replaced = 12,
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
    Buy = 1,
    Sell = 2,
    SellShort = 3,
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
    OrderNotFound = 1,
    AlreadyCanceled = 2,
    AlreadyFilled = 3,
    TooLateToCancel = 4,
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
    Trade = 1,
    Deposit = 2,
    Withdrawal = 3,
    Transfer = 4,
    Fee = 5,
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
    Executing = 1,
    AllDone = 2,
    Reject = 3,
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
    Deposit = 1,
    Withdrawal = 2,
    Transfer = 3,
    Fee = 4,
    Funding = 5,
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

pub const TradingSessionStatusStatus = enum(u8) {
    PreOpen = 1,
    Open = 2,
    Break = 3,
    Close = 4,
    Halt = 5,
    Auction = 6,
};

pub fn TradingSessionStatusStatusFromValue(v: u8) !TradingSessionStatusStatus {
    return switch (v) {
        1 => .PreOpen,
        2 => .Open,
        3 => .Break,
        4 => .Close,
        5 => .Halt,
        6 => .Auction,
        else => error.InvalidEnumValue,
    };
}

pub fn TradingSessionStatusStatusToValue(e: TradingSessionStatusStatus) u8 { return @intFromEnum(e); }

pub const TradingSessionStatusHaltReason = enum(u8) {
    None = 1,
    News = 2,
    MatchingEngine = 3,
    Regulatory = 4,
    Auction = 5,
    Other = 6,
};

pub fn TradingSessionStatusHaltReasonFromValue(v: u8) !TradingSessionStatusHaltReason {
    return switch (v) {
        1 => .None,
        2 => .News,
        3 => .MatchingEngine,
        4 => .Regulatory,
        5 => .Auction,
        6 => .Other,
        else => error.InvalidEnumValue,
    };
}

pub fn TradingSessionStatusHaltReasonToValue(e: TradingSessionStatusHaltReason) u8 { return @intFromEnum(e); }

pub const SecurityStatusStatus = enum(u8) {
    Trading = 1,
    Halt = 2,
    Auction = 3,
    Suspended = 4,
};

pub fn SecurityStatusStatusFromValue(v: u8) !SecurityStatusStatus {
    return switch (v) {
        1 => .Trading,
        2 => .Halt,
        3 => .Auction,
        4 => .Suspended,
        else => error.InvalidEnumValue,
    };
}

pub fn SecurityStatusStatusToValue(e: SecurityStatusStatus) u8 { return @intFromEnum(e); }

pub const SecurityStatusHaltReason = enum(u8) {
    None = 1,
    News = 2,
    MatchingEngine = 3,
    Regulatory = 4,
    Auction = 5,
    Other = 6,
};

pub fn SecurityStatusHaltReasonFromValue(v: u8) !SecurityStatusHaltReason {
    return switch (v) {
        1 => .None,
        2 => .News,
        3 => .MatchingEngine,
        4 => .Regulatory,
        5 => .Auction,
        6 => .Other,
        else => error.InvalidEnumValue,
    };
}

pub fn SecurityStatusHaltReasonToValue(e: SecurityStatusHaltReason) u8 { return @intFromEnum(e); }

pub const MassCancelRequestSide = enum(u8) {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
    SellShortExempt = 4,
};

pub fn MassCancelRequestSideFromValue(v: u8) !MassCancelRequestSide {
    return switch (v) {
        1 => .Buy,
        2 => .Sell,
        3 => .SellShort,
        4 => .SellShortExempt,
        else => error.InvalidEnumValue,
    };
}

pub fn MassCancelRequestSideToValue(e: MassCancelRequestSide) u8 { return @intFromEnum(e); }

pub const MassCancelReportStatus = enum(u8) {
    Accepted = 1,
    Completed = 2,
    Rejected = 3,
};

pub fn MassCancelReportStatusFromValue(v: u8) !MassCancelReportStatus {
    return switch (v) {
        1 => .Accepted,
        2 => .Completed,
        3 => .Rejected,
        else => error.InvalidEnumValue,
    };
}

pub fn MassCancelReportStatusToValue(e: MassCancelReportStatus) u8 { return @intFromEnum(e); }

pub const AggregateTradeSide = enum(u8) {
    Buy = 1,
    Sell = 2,
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
    Buy = 1,
    Sell = 2,
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
    Buy = 1,
    Sell = 2,
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
    New = 1,
    Change = 2,
    Delete = 3,
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
    Buy = 1,
    Sell = 2,
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
    PubSub = 1,
    RequestResponse = 2,
    RequestStream = 3,
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

/// SBE struct AggregateTrade
pub const AggregateTrade = struct {
    symbol: []const u8,
    agg_trade_id: []const u8,
    price: f64,
    qty: f64,
    side: AggregateTradeSide,
    first_trade_id: []const u8,
    last_trade_id: []const u8,
    timestamp: i64,

    pub fn encode(allocator: std.mem.Allocator, value: AggregateTrade, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeString(buf, value.agg_trade_id);
        try writeF64BE(buf, value.price);
        try writeF64BE(buf, value.qty);
        try buf.append(AggregateTradeSideToValue(value.side));
        try writeString(buf, value.first_trade_id);
        try writeString(buf, value.last_trade_id);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !AggregateTrade {
        var value: AggregateTrade = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const agg_trade_id = try readString(buf, &pos, allocator);
        const price = readF64BE(buf, &pos);
        const qty = readF64BE(buf, &pos);
        const side = try AggregateTradeSideFromValue(readU8(buf, &pos));
        const first_trade_id = try readString(buf, &pos, allocator);
        const last_trade_id = try readString(buf, &pos, allocator);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.symbol = symbol;
        value.agg_trade_id = agg_trade_id;
        value.price = price;
        value.qty = qty;
        value.side = side;
        value.first_trade_id = first_trade_id;
        value.last_trade_id = last_trade_id;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE struct LiquidationTrade
pub const LiquidationTrade = struct {
    symbol: []const u8,
    side: LiquidationTradeSide,
    price: f64,
    qty: f64,
    timestamp: i64,

    pub fn encode(allocator: std.mem.Allocator, value: LiquidationTrade, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try buf.append(LiquidationTradeSideToValue(value.side));
        try writeF64BE(buf, value.price);
        try writeF64BE(buf, value.qty);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !LiquidationTrade {
        var value: LiquidationTrade = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const side = try LiquidationTradeSideFromValue(readU8(buf, &pos));
        const price = readF64BE(buf, &pos);
        const qty = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.symbol = symbol;
        value.side = side;
        value.price = price;
        value.qty = qty;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE struct PriceLevel
pub const PriceLevel = struct {
    price: f64,
    qty: f64,
    order_count: ?u32,

    pub fn encode(allocator: std.mem.Allocator, value: PriceLevel, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeF64BE(buf, value.price);
        try writeF64BE(buf, value.qty);
        try writeU32BE(buf, @intCast(value.order_count orelse 0));
        try buf.append(if (value.order_count != null) 1 else 0);
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PriceLevel {
        _ = allocator;
        var value: PriceLevel = undefined;
        const price = readF64BE(buf, &pos);
        const qty = readF64BE(buf, &pos);
        const order_count_v = readU32BE(buf, &pos);
        var order_count: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; order_count = order_count_v; } else { pos += 1; }
        value.price = price;
        value.qty = qty;
        value.order_count = order_count;
        return value;
    }
};

/// SBE struct MarketDataUpdate
pub const MarketDataUpdate = struct {
    side: MarketDataUpdateSide,
    action: MarketDataUpdateAction,
    price: f64,
    qty: f64,

    pub fn encode(allocator: std.mem.Allocator, value: MarketDataUpdate, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try buf.append(MarketDataUpdateSideToValue(value.side));
        try buf.append(MarketDataUpdateActionToValue(value.action));
        try writeF64BE(buf, value.price);
        try writeF64BE(buf, value.qty);
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MarketDataUpdate {
        _ = allocator;
        var value: MarketDataUpdate = undefined;
        const side = try MarketDataUpdateSideFromValue(readU8(buf, &pos));
        const action = try MarketDataUpdateActionFromValue(readU8(buf, &pos));
        const price = readF64BE(buf, &pos);
        const qty = readF64BE(buf, &pos);
        value.side = side;
        value.action = action;
        value.price = price;
        value.qty = qty;
        return value;
    }
};

/// SBE struct CandleBar
pub const CandleBar = struct {
    symbol: []const u8,
    interval: []const u8,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    bar_start: i64,
    bar_end: i64,
    is_final: u8,
    is_snapshot: ?u8,

    pub fn encode(allocator: std.mem.Allocator, value: CandleBar, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeString(buf, value.interval);
        try writeF64BE(buf, value.open);
        try writeF64BE(buf, value.high);
        try writeF64BE(buf, value.low);
        try writeF64BE(buf, value.close);
        try writeF64BE(buf, value.volume);
        try writeU64BE(buf, @as(u64, @bitCast(value.bar_start)));
        try writeU64BE(buf, @as(u64, @bitCast(value.bar_end)));
        try buf.append(@intCast(value.is_final));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CandleBar {
        var value: CandleBar = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const interval = try readString(buf, &pos, allocator);
        const open = readF64BE(buf, &pos);
        const high = readF64BE(buf, &pos);
        const low = readF64BE(buf, &pos);
        const close = readF64BE(buf, &pos);
        const volume = readF64BE(buf, &pos);
        const bar_start = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const bar_end = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_final = readU8(buf, &pos);
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.interval = interval;
        value.open = open;
        value.high = high;
        value.low = low;
        value.close = close;
        value.volume = volume;
        value.bar_start = bar_start;
        value.bar_end = bar_end;
        value.is_final = is_final;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE struct PublicTrade
pub const PublicTrade = struct {
    symbol: []const u8,
    trade_id: []const u8,
    price: f64,
    qty: f64,
    side: PublicTradeSide,
    timestamp: i64,

    pub fn encode(allocator: std.mem.Allocator, value: PublicTrade, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeString(buf, value.trade_id);
        try writeF64BE(buf, value.price);
        try writeF64BE(buf, value.qty);
        try buf.append(PublicTradeSideToValue(value.side));
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PublicTrade {
        var value: PublicTrade = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const trade_id = try readString(buf, &pos, allocator);
        const price = readF64BE(buf, &pos);
        const qty = readF64BE(buf, &pos);
        const side = try PublicTradeSideFromValue(readU8(buf, &pos));
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.symbol = symbol;
        value.trade_id = trade_id;
        value.price = price;
        value.qty = qty;
        value.side = side;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE struct InstrumentMetadata
pub const InstrumentMetadata = struct {
    instrument_id: []const u8,
    symbol: []const u8,
    product_kind: []const u8,
    margin_asset: ?[]const u8,
    display_name: ?[]const u8,
    tick_size: ?f64,
    lot_size: ?f64,

    pub fn encode(allocator: std.mem.Allocator, value: InstrumentMetadata, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.instrument_id);
        try writeString(buf, value.symbol);
        try writeString(buf, value.product_kind);
        if (value.margin_asset) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        if (value.display_name) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeF64BE(buf, value.tick_size orelse 0);
        try buf.append(if (value.tick_size != null) 1 else 0);
        try writeF64BE(buf, value.lot_size orelse 0);
        try buf.append(if (value.lot_size != null) 1 else 0);
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !InstrumentMetadata {
        var value: InstrumentMetadata = undefined;
        var margin_asset: ?[]u8 = null;
        var display_name: ?[]u8 = null;
        const instrument_id = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const product_kind = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            margin_asset = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        if (buf[pos] == 1) {
            pos += 1;
            display_name = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const tick_size_v = readF64BE(buf, &pos);
        var tick_size: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; tick_size = tick_size_v; } else { pos += 1; }
        const lot_size_v = readF64BE(buf, &pos);
        var lot_size: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; lot_size = lot_size_v; } else { pos += 1; }
        value.instrument_id = instrument_id;
        value.symbol = symbol;
        value.product_kind = product_kind;
        value.margin_asset = margin_asset;
        value.display_name = display_name;
        value.tick_size = tick_size;
        value.lot_size = lot_size;
        return value;
    }
};

/// SBE struct BalanceEntry
pub const BalanceEntry = struct {
    asset: []const u8,
    total: f64,
    available: f64,
    hold: f64,

    pub fn encode(allocator: std.mem.Allocator, value: BalanceEntry, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.asset);
        try writeF64BE(buf, value.total);
        try writeF64BE(buf, value.available);
        try writeF64BE(buf, value.hold);
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !BalanceEntry {
        var value: BalanceEntry = undefined;
        const asset = try readString(buf, &pos, allocator);
        const total = readF64BE(buf, &pos);
        const available = readF64BE(buf, &pos);
        const hold = readF64BE(buf, &pos);
        value.asset = asset;
        value.total = total;
        value.available = available;
        value.hold = hold;
        return value;
    }
};

/// SBE struct PositionEntry
pub const PositionEntry = struct {
    symbol: []const u8,
    qty: f64,
    entry_price: f64,
    unrealized_pnl: f64,

    pub fn encode(allocator: std.mem.Allocator, value: PositionEntry, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.qty);
        try writeF64BE(buf, value.entry_price);
        try writeF64BE(buf, value.unrealized_pnl);
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PositionEntry {
        var value: PositionEntry = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const qty = readF64BE(buf, &pos);
        const entry_price = readF64BE(buf, &pos);
        const unrealized_pnl = readF64BE(buf, &pos);
        value.symbol = symbol;
        value.qty = qty;
        value.entry_price = entry_price;
        value.unrealized_pnl = unrealized_pnl;
        return value;
    }
};

/// SBE struct PageInfo
pub const PageInfo = struct {
    has_more: u8,
    next_cursor: ?[]const u8,

    pub fn encode(allocator: std.mem.Allocator, value: PageInfo, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PageInfo {
        var value: PageInfo = undefined;
        var next_cursor: ?[]u8 = null;
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE struct CapabilityPath
pub const CapabilityPath = struct {
    path: []const u8,
    pattern: CapabilityPathPattern,
    auth_required: u8,

    pub fn encode(allocator: std.mem.Allocator, value: CapabilityPath, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.path);
        try buf.append(CapabilityPathPatternToValue(value.pattern));
        try buf.append(@intCast(value.auth_required));
    }

    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CapabilityPath {
        var value: CapabilityPath = undefined;
        const path = try readString(buf, &pos, allocator);
        const pattern = try CapabilityPathPatternFromValue(readU8(buf, &pos));
        const auth_required = readU8(buf, &pos);
        value.path = path;
        value.pattern = pattern;
        value.auth_required = auth_required;
        return value;
    }
};

/// SBE message NewOrderSingle
pub const NewOrderSingle = struct {
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
    post_only: ?u8,
    reduce_only: ?u8,
};

/// SBE body encoder for NewOrderSingle (fields only, no header).
pub const NewOrderSingleBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: NewOrderSingle, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.cl_ord_id);
        try buf.append(NewOrderSingleSideToValue(value.side));
        try writeF64BE(buf, value.order_qty);
        try writeF64BE(buf, value.price orelse 0);
        try buf.append(if (value.price != null) 1 else 0);
        try writeF64BE(buf, value.stop_price orelse 0);
        try buf.append(if (value.stop_price != null) 1 else 0);
        try writeString(buf, value.symbol);
        try buf.append(NewOrderSingleOrderTypeToValue(value.order_type));
        try buf.append(NewOrderSingleTimeInForceToValue(value.time_in_force));
        try writeU64BE(buf, @as(u64, @bitCast(value.expire_time orelse 0)));
        try buf.append(if (value.expire_time != null) 1 else 0);
        if (value.account) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        if (value.strategy_id) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        if (value.security_id) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try buf.append(if (value.id_source) |e| NewOrderSingleIdSourceToValue(e) else 0);
        try buf.append(if (value.id_source != null) 1 else 0);
        if (value.security_exchange) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try buf.append(@intCast(value.post_only orelse 0));
        try buf.append(if (value.post_only != null) 1 else 0);
        try buf.append(@intCast(value.reduce_only orelse 0));
        try buf.append(if (value.reduce_only != null) 1 else 0);
    }
};

/// SBE body decoder for NewOrderSingle: reads fields at `pos`.
pub const NewOrderSingleBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !NewOrderSingle {
        var value: NewOrderSingle = undefined;
        var account: ?[]u8 = null;
        var strategy_id: ?[]u8 = null;
        var security_id: ?[]u8 = null;
        var security_exchange: ?[]u8 = null;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const side = try NewOrderSingleSideFromValue(readU8(buf, &pos));
        const order_qty = readF64BE(buf, &pos);
        const price_v = readF64BE(buf, &pos);
        var price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; price = price_v; } else { pos += 1; }
        const stop_price_v = readF64BE(buf, &pos);
        var stop_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; stop_price = stop_price_v; } else { pos += 1; }
        const symbol = try readString(buf, &pos, allocator);
        const order_type = try NewOrderSingleOrderTypeFromValue(readU8(buf, &pos));
        const time_in_force = try NewOrderSingleTimeInForceFromValue(readU8(buf, &pos));
        const expire_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var expire_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; expire_time = expire_time_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            account = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        if (buf[pos] == 1) {
            pos += 1;
            strategy_id = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        if (buf[pos] == 1) {
            pos += 1;
            security_id = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const id_source_raw = readU8(buf, &pos);
        var id_source: ?NewOrderSingleIdSource = null;
        if (buf[pos] == 1) {
            pos += 1;
            id_source = try NewOrderSingleIdSourceFromValue(id_source_raw);
        } else {
            pos += 1;
        }
        if (buf[pos] == 1) {
            pos += 1;
            security_exchange = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const post_only_v = readU8(buf, &pos);
        var post_only: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; post_only = post_only_v; } else { pos += 1; }
        const reduce_only_v = readU8(buf, &pos);
        var reduce_only: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; reduce_only = reduce_only_v; } else { pos += 1; }
        value.cl_ord_id = cl_ord_id;
        value.side = side;
        value.order_qty = order_qty;
        value.price = price;
        value.stop_price = stop_price;
        value.symbol = symbol;
        value.order_type = order_type;
        value.time_in_force = time_in_force;
        value.expire_time = expire_time;
        value.account = account;
        value.strategy_id = strategy_id;
        value.security_id = security_id;
        value.id_source = id_source;
        value.security_exchange = security_exchange;
        value.post_only = post_only;
        value.reduce_only = reduce_only;
        return value;
    }
};

/// SBE encoder for NewOrderSingle
pub const NewOrderSingleEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id: []const u8, side: NewOrderSingleSide, order_qty: f64, price: ?f64, stop_price: ?f64, symbol: []const u8, order_type: NewOrderSingleOrderType, time_in_force: NewOrderSingleTimeInForce, expire_time: ?i64, account: ?[]const u8, strategy_id: ?[]const u8, security_id: ?[]const u8, id_source: ?NewOrderSingleIdSource, security_exchange: ?[]const u8, post_only: ?u8, reduce_only: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 1);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 38);
        try writeString(buf, cl_ord_id);
        try buf.append(NewOrderSingleSideToValue(side));
        try writeF64BE(buf, order_qty);
        try writeF64BE(buf, price orelse 0);
        try buf.append(if (price != null) 1 else 0);
        try writeF64BE(buf, stop_price orelse 0);
        try buf.append(if (stop_price != null) 1 else 0);
        try writeString(buf, symbol);
        try buf.append(NewOrderSingleOrderTypeToValue(order_type));
        try buf.append(NewOrderSingleTimeInForceToValue(time_in_force));
        try writeU64BE(buf, @as(u64, @bitCast(expire_time orelse 0)));
        try buf.append(if (expire_time != null) 1 else 0);
        if (account) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        if (strategy_id) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        if (security_id) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try buf.append(if (id_source) |e| NewOrderSingleIdSourceToValue(e) else 0);
        try buf.append(if (id_source != null) 1 else 0);
        if (security_exchange) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try buf.append(@intCast(post_only orelse 0));
        try buf.append(if (post_only != null) 1 else 0);
        try buf.append(@intCast(reduce_only orelse 0));
        try buf.append(if (reduce_only != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for NewOrderSingle: validates the header and reads the body.
pub const NewOrderSingleDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !NewOrderSingle {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 1) return error.InvalidTemplateId;
        var value: NewOrderSingle = undefined;
        var account: ?[]u8 = null;
        var strategy_id: ?[]u8 = null;
        var security_id: ?[]u8 = null;
        var security_exchange: ?[]u8 = null;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const side = try NewOrderSingleSideFromValue(readU8(buf, &pos));
        const order_qty = readF64BE(buf, &pos);
        const price_v = readF64BE(buf, &pos);
        var price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; price = price_v; } else { pos += 1; }
        const stop_price_v = readF64BE(buf, &pos);
        var stop_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; stop_price = stop_price_v; } else { pos += 1; }
        const symbol = try readString(buf, &pos, allocator);
        const order_type = try NewOrderSingleOrderTypeFromValue(readU8(buf, &pos));
        const time_in_force = try NewOrderSingleTimeInForceFromValue(readU8(buf, &pos));
        const expire_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var expire_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; expire_time = expire_time_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            account = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        if (buf[pos] == 1) {
            pos += 1;
            strategy_id = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        if (buf[pos] == 1) {
            pos += 1;
            security_id = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const id_source_raw = readU8(buf, &pos);
        var id_source: ?NewOrderSingleIdSource = null;
        if (buf[pos] == 1) {
            pos += 1;
            id_source = try NewOrderSingleIdSourceFromValue(id_source_raw);
        } else {
            pos += 1;
        }
        if (buf[pos] == 1) {
            pos += 1;
            security_exchange = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const post_only_v = readU8(buf, &pos);
        var post_only: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; post_only = post_only_v; } else { pos += 1; }
        const reduce_only_v = readU8(buf, &pos);
        var reduce_only: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; reduce_only = reduce_only_v; } else { pos += 1; }
        value.cl_ord_id = cl_ord_id;
        value.side = side;
        value.order_qty = order_qty;
        value.price = price;
        value.stop_price = stop_price;
        value.symbol = symbol;
        value.order_type = order_type;
        value.time_in_force = time_in_force;
        value.expire_time = expire_time;
        value.account = account;
        value.strategy_id = strategy_id;
        value.security_id = security_id;
        value.id_source = id_source;
        value.security_exchange = security_exchange;
        value.post_only = post_only;
        value.reduce_only = reduce_only;
        return value;
    }
};

/// SBE message CancelRequest
pub const CancelRequest = struct {
    cl_ord_id: []const u8,
    orig_cl_ord_id: []const u8,
    symbol: []const u8,
    side: CancelRequestSide,
    order_qty: ?f64,
};

/// SBE body encoder for CancelRequest (fields only, no header).
pub const CancelRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CancelRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.cl_ord_id);
        try writeString(buf, value.orig_cl_ord_id);
        try writeString(buf, value.symbol);
        try buf.append(CancelRequestSideToValue(value.side));
        try writeF64BE(buf, value.order_qty orelse 0);
        try buf.append(if (value.order_qty != null) 1 else 0);
    }
};

/// SBE body decoder for CancelRequest: reads fields at `pos`.
pub const CancelRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CancelRequest {
        var value: CancelRequest = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const side = try CancelRequestSideFromValue(readU8(buf, &pos));
        const order_qty_v = readF64BE(buf, &pos);
        var order_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; order_qty = order_qty_v; } else { pos += 1; }
        value.cl_ord_id = cl_ord_id;
        value.orig_cl_ord_id = orig_cl_ord_id;
        value.symbol = symbol;
        value.side = side;
        value.order_qty = order_qty;
        return value;
    }
};

/// SBE encoder for CancelRequest
pub const CancelRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id: []const u8, orig_cl_ord_id: []const u8, symbol: []const u8, side: CancelRequestSide, order_qty: ?f64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 2);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 9);
        try writeString(buf, cl_ord_id);
        try writeString(buf, orig_cl_ord_id);
        try writeString(buf, symbol);
        try buf.append(CancelRequestSideToValue(side));
        try writeF64BE(buf, order_qty orelse 0);
        try buf.append(if (order_qty != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CancelRequest: validates the header and reads the body.
pub const CancelRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CancelRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 2) return error.InvalidTemplateId;
        var value: CancelRequest = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const side = try CancelRequestSideFromValue(readU8(buf, &pos));
        const order_qty_v = readF64BE(buf, &pos);
        var order_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; order_qty = order_qty_v; } else { pos += 1; }
        value.cl_ord_id = cl_ord_id;
        value.orig_cl_ord_id = orig_cl_ord_id;
        value.symbol = symbol;
        value.side = side;
        value.order_qty = order_qty;
        return value;
    }
};

/// SBE message CancelReplaceRequest
pub const CancelReplaceRequest = struct {
    cl_ord_id: []const u8,
    orig_cl_ord_id: []const u8,
    symbol: []const u8,
    side: CancelReplaceRequestSide,
    order_qty: f64,
    price: ?f64,
};

/// SBE body encoder for CancelReplaceRequest (fields only, no header).
pub const CancelReplaceRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CancelReplaceRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.cl_ord_id);
        try writeString(buf, value.orig_cl_ord_id);
        try writeString(buf, value.symbol);
        try buf.append(CancelReplaceRequestSideToValue(value.side));
        try writeF64BE(buf, value.order_qty);
        try writeF64BE(buf, value.price orelse 0);
        try buf.append(if (value.price != null) 1 else 0);
    }
};

/// SBE body decoder for CancelReplaceRequest: reads fields at `pos`.
pub const CancelReplaceRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CancelReplaceRequest {
        var value: CancelReplaceRequest = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const side = try CancelReplaceRequestSideFromValue(readU8(buf, &pos));
        const order_qty = readF64BE(buf, &pos);
        const price_v = readF64BE(buf, &pos);
        var price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; price = price_v; } else { pos += 1; }
        value.cl_ord_id = cl_ord_id;
        value.orig_cl_ord_id = orig_cl_ord_id;
        value.symbol = symbol;
        value.side = side;
        value.order_qty = order_qty;
        value.price = price;
        return value;
    }
};

/// SBE encoder for CancelReplaceRequest
pub const CancelReplaceRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id: []const u8, orig_cl_ord_id: []const u8, symbol: []const u8, side: CancelReplaceRequestSide, order_qty: f64, price: ?f64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 3);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 17);
        try writeString(buf, cl_ord_id);
        try writeString(buf, orig_cl_ord_id);
        try writeString(buf, symbol);
        try buf.append(CancelReplaceRequestSideToValue(side));
        try writeF64BE(buf, order_qty);
        try writeF64BE(buf, price orelse 0);
        try buf.append(if (price != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CancelReplaceRequest: validates the header and reads the body.
pub const CancelReplaceRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CancelReplaceRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 3) return error.InvalidTemplateId;
        var value: CancelReplaceRequest = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const side = try CancelReplaceRequestSideFromValue(readU8(buf, &pos));
        const order_qty = readF64BE(buf, &pos);
        const price_v = readF64BE(buf, &pos);
        var price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; price = price_v; } else { pos += 1; }
        value.cl_ord_id = cl_ord_id;
        value.orig_cl_ord_id = orig_cl_ord_id;
        value.symbol = symbol;
        value.side = side;
        value.order_qty = order_qty;
        value.price = price;
        return value;
    }
};

/// SBE message ExecutionReport
pub const ExecutionReport = struct {
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
};

/// SBE body encoder for ExecutionReport (fields only, no header).
pub const ExecutionReportBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: ExecutionReport, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.cl_ord_id);
        try writeString(buf, value.order_id);
        try writeString(buf, value.exec_id);
        try buf.append(ExecutionReportExecTypeToValue(value.exec_type));
        try buf.append(ExecutionReportOrdStatusToValue(value.ord_status));
        try buf.append(ExecutionReportSideToValue(value.side));
        try writeF64BE(buf, value.last_qty orelse 0);
        try buf.append(if (value.last_qty != null) 1 else 0);
        try writeF64BE(buf, value.last_price orelse 0);
        try buf.append(if (value.last_price != null) 1 else 0);
        try writeF64BE(buf, value.leaves_qty);
        try writeF64BE(buf, value.cum_qty);
        try writeF64BE(buf, value.avg_price);
        try writeString(buf, value.symbol);
        try writeU64BE(buf, @as(u64, @bitCast(value.transact_time)));
    }
};

/// SBE body decoder for ExecutionReport: reads fields at `pos`.
pub const ExecutionReportBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !ExecutionReport {
        var value: ExecutionReport = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const order_id = try readString(buf, &pos, allocator);
        const exec_id = try readString(buf, &pos, allocator);
        const exec_type = try ExecutionReportExecTypeFromValue(readU8(buf, &pos));
        const ord_status = try ExecutionReportOrdStatusFromValue(readU8(buf, &pos));
        const side = try ExecutionReportSideFromValue(readU8(buf, &pos));
        const last_qty_v = readF64BE(buf, &pos);
        var last_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; last_qty = last_qty_v; } else { pos += 1; }
        const last_price_v = readF64BE(buf, &pos);
        var last_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; last_price = last_price_v; } else { pos += 1; }
        const leaves_qty = readF64BE(buf, &pos);
        const cum_qty = readF64BE(buf, &pos);
        const avg_price = readF64BE(buf, &pos);
        const symbol = try readString(buf, &pos, allocator);
        const transact_time = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.cl_ord_id = cl_ord_id;
        value.order_id = order_id;
        value.exec_id = exec_id;
        value.exec_type = exec_type;
        value.ord_status = ord_status;
        value.side = side;
        value.last_qty = last_qty;
        value.last_price = last_price;
        value.leaves_qty = leaves_qty;
        value.cum_qty = cum_qty;
        value.avg_price = avg_price;
        value.symbol = symbol;
        value.transact_time = transact_time;
        return value;
    }
};

/// SBE encoder for ExecutionReport
pub const ExecutionReportEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id: []const u8, order_id: []const u8, exec_id: []const u8, exec_type: ExecutionReportExecType, ord_status: ExecutionReportOrdStatus, side: ExecutionReportSide, last_qty: ?f64, last_price: ?f64, leaves_qty: f64, cum_qty: f64, avg_price: f64, symbol: []const u8, transact_time: i64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 4);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 51);
        try writeString(buf, cl_ord_id);
        try writeString(buf, order_id);
        try writeString(buf, exec_id);
        try buf.append(ExecutionReportExecTypeToValue(exec_type));
        try buf.append(ExecutionReportOrdStatusToValue(ord_status));
        try buf.append(ExecutionReportSideToValue(side));
        try writeF64BE(buf, last_qty orelse 0);
        try buf.append(if (last_qty != null) 1 else 0);
        try writeF64BE(buf, last_price orelse 0);
        try buf.append(if (last_price != null) 1 else 0);
        try writeF64BE(buf, leaves_qty);
        try writeF64BE(buf, cum_qty);
        try writeF64BE(buf, avg_price);
        try writeString(buf, symbol);
        try writeU64BE(buf, @as(u64, @bitCast(transact_time)));
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for ExecutionReport: validates the header and reads the body.
pub const ExecutionReportDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !ExecutionReport {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 4) return error.InvalidTemplateId;
        var value: ExecutionReport = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const order_id = try readString(buf, &pos, allocator);
        const exec_id = try readString(buf, &pos, allocator);
        const exec_type = try ExecutionReportExecTypeFromValue(readU8(buf, &pos));
        const ord_status = try ExecutionReportOrdStatusFromValue(readU8(buf, &pos));
        const side = try ExecutionReportSideFromValue(readU8(buf, &pos));
        const last_qty_v = readF64BE(buf, &pos);
        var last_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; last_qty = last_qty_v; } else { pos += 1; }
        const last_price_v = readF64BE(buf, &pos);
        var last_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; last_price = last_price_v; } else { pos += 1; }
        const leaves_qty = readF64BE(buf, &pos);
        const cum_qty = readF64BE(buf, &pos);
        const avg_price = readF64BE(buf, &pos);
        const symbol = try readString(buf, &pos, allocator);
        const transact_time = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.cl_ord_id = cl_ord_id;
        value.order_id = order_id;
        value.exec_id = exec_id;
        value.exec_type = exec_type;
        value.ord_status = ord_status;
        value.side = side;
        value.last_qty = last_qty;
        value.last_price = last_price;
        value.leaves_qty = leaves_qty;
        value.cum_qty = cum_qty;
        value.avg_price = avg_price;
        value.symbol = symbol;
        value.transact_time = transact_time;
        return value;
    }
};

/// SBE message CancelReject
pub const CancelReject = struct {
    cl_ord_id: []const u8,
    orig_cl_ord_id: []const u8,
    reject_reason: CancelRejectRejectReason,
    symbol: []const u8,
};

/// SBE body encoder for CancelReject (fields only, no header).
pub const CancelRejectBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CancelReject, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.cl_ord_id);
        try writeString(buf, value.orig_cl_ord_id);
        try buf.append(CancelRejectRejectReasonToValue(value.reject_reason));
        try writeString(buf, value.symbol);
    }
};

/// SBE body decoder for CancelReject: reads fields at `pos`.
pub const CancelRejectBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CancelReject {
        var value: CancelReject = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const reject_reason = try CancelRejectRejectReasonFromValue(readU8(buf, &pos));
        const symbol = try readString(buf, &pos, allocator);
        value.cl_ord_id = cl_ord_id;
        value.orig_cl_ord_id = orig_cl_ord_id;
        value.reject_reason = reject_reason;
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for CancelReject
pub const CancelRejectEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id: []const u8, orig_cl_ord_id: []const u8, reject_reason: CancelRejectRejectReason, symbol: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 5);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, cl_ord_id);
        try writeString(buf, orig_cl_ord_id);
        try buf.append(CancelRejectRejectReasonToValue(reject_reason));
        try writeString(buf, symbol);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CancelReject: validates the header and reads the body.
pub const CancelRejectDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CancelReject {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 5) return error.InvalidTemplateId;
        var value: CancelReject = undefined;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const orig_cl_ord_id = try readString(buf, &pos, allocator);
        const reject_reason = try CancelRejectRejectReasonFromValue(readU8(buf, &pos));
        const symbol = try readString(buf, &pos, allocator);
        value.cl_ord_id = cl_ord_id;
        value.orig_cl_ord_id = orig_cl_ord_id;
        value.reject_reason = reject_reason;
        value.symbol = symbol;
        return value;
    }
};

/// SBE message MarketDataSnapshot
pub const MarketDataSnapshot = struct {
    symbol: []const u8,
    exchange: []const u8,
    bids: []PriceLevel,
    asks: []PriceLevel,
    timestamp: i64,
    sequence: ?u64,
    is_snapshot: ?u8,
};

/// SBE body encoder for MarketDataSnapshot (fields only, no header).
pub const MarketDataSnapshotBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MarketDataSnapshot, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.symbol);
        try writeString(buf, value.exchange);
        try writeU32BE(buf, @intCast(value.bids.len));
        for (value.bids) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU32BE(buf, @intCast(value.asks.len));
        for (value.asks) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try writeU64BE(buf, @intCast(value.sequence orelse 0));
        try buf.append(if (value.sequence != null) 1 else 0);
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for MarketDataSnapshot: reads fields at `pos`.
pub const MarketDataSnapshotBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MarketDataSnapshot {
        var value: MarketDataSnapshot = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const exchange = try readString(buf, &pos, allocator);
        const bids_count = readU32BE(buf, &pos);
        var bids = try allocator.alloc(PriceLevel, bids_count);
        var bids_idx: usize = 0;
        while (bids_idx < bids_count) : (bids_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            bids[bids_idx] = item;
        }
        const asks_count = readU32BE(buf, &pos);
        var asks = try allocator.alloc(PriceLevel, asks_count);
        var asks_idx: usize = 0;
        while (asks_idx < asks_count) : (asks_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            asks[asks_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.exchange = exchange;
        value.bids = bids;
        value.asks = asks;
        value.timestamp = timestamp;
        value.sequence = sequence;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for MarketDataSnapshot
pub const MarketDataSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, exchange: []const u8, bids: []const PriceLevel, asks: []const PriceLevel, timestamp: i64, sequence: ?u64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 6);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 17);
        try writeString(buf, symbol);
        try writeString(buf, exchange);
        try writeU32BE(buf, @intCast(bids.len));
        for (bids) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU32BE(buf, @intCast(asks.len));
        for (asks) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try writeU64BE(buf, @intCast(sequence orelse 0));
        try buf.append(if (sequence != null) 1 else 0);
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarketDataSnapshot: validates the header and reads the body.
pub const MarketDataSnapshotDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MarketDataSnapshot {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 6) return error.InvalidTemplateId;
        var value: MarketDataSnapshot = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const exchange = try readString(buf, &pos, allocator);
        const bids_count = readU32BE(buf, &pos);
        var bids = try allocator.alloc(PriceLevel, bids_count);
        var bids_idx: usize = 0;
        while (bids_idx < bids_count) : (bids_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            bids[bids_idx] = item;
        }
        const asks_count = readU32BE(buf, &pos);
        var asks = try allocator.alloc(PriceLevel, asks_count);
        var asks_idx: usize = 0;
        while (asks_idx < asks_count) : (asks_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            asks[asks_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.exchange = exchange;
        value.bids = bids;
        value.asks = asks;
        value.timestamp = timestamp;
        value.sequence = sequence;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message MarketDataIncrementalRefresh
pub const MarketDataIncrementalRefresh = struct {
    symbol: []const u8,
    updates: []MarketDataUpdate,
    timestamp: i64,
    sequence: ?u64,
};

/// SBE body encoder for MarketDataIncrementalRefresh (fields only, no header).
pub const MarketDataIncrementalRefreshBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MarketDataIncrementalRefresh, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.symbol);
        try writeU32BE(buf, @intCast(value.updates.len));
        for (value.updates) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try MarketDataUpdate.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try writeU64BE(buf, @intCast(value.sequence orelse 0));
        try buf.append(if (value.sequence != null) 1 else 0);
    }
};

/// SBE body decoder for MarketDataIncrementalRefresh: reads fields at `pos`.
pub const MarketDataIncrementalRefreshBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MarketDataIncrementalRefresh {
        var value: MarketDataIncrementalRefresh = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const updates_count = readU32BE(buf, &pos);
        var updates = try allocator.alloc(MarketDataUpdate, updates_count);
        var updates_idx: usize = 0;
        while (updates_idx < updates_count) : (updates_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try MarketDataUpdate.decode(allocator, buf, &pos);
            pos = item_end;
            updates[updates_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        value.symbol = symbol;
        value.updates = updates;
        value.timestamp = timestamp;
        value.sequence = sequence;
        return value;
    }
};

/// SBE encoder for MarketDataIncrementalRefresh
pub const MarketDataIncrementalRefreshEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, updates: []const MarketDataUpdate, timestamp: i64, sequence: ?u64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 7);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 16);
        try writeString(buf, symbol);
        try writeU32BE(buf, @intCast(updates.len));
        for (updates) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try MarketDataUpdate.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try writeU64BE(buf, @intCast(sequence orelse 0));
        try buf.append(if (sequence != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarketDataIncrementalRefresh: validates the header and reads the body.
pub const MarketDataIncrementalRefreshDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MarketDataIncrementalRefresh {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 7) return error.InvalidTemplateId;
        var value: MarketDataIncrementalRefresh = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const updates_count = readU32BE(buf, &pos);
        var updates = try allocator.alloc(MarketDataUpdate, updates_count);
        var updates_idx: usize = 0;
        while (updates_idx < updates_count) : (updates_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try MarketDataUpdate.decode(allocator, buf, &pos);
            pos = item_end;
            updates[updates_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        value.symbol = symbol;
        value.updates = updates;
        value.timestamp = timestamp;
        value.sequence = sequence;
        return value;
    }
};

/// SBE message OrderBookRequest
pub const OrderBookRequest = struct {
    symbol: []const u8,
    depth: ?u32,
    at_time: ?i64,
};

/// SBE body encoder for OrderBookRequest (fields only, no header).
pub const OrderBookRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OrderBookRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeU32BE(buf, @intCast(value.depth orelse 0));
        try buf.append(if (value.depth != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.at_time orelse 0)));
        try buf.append(if (value.at_time != null) 1 else 0);
    }
};

/// SBE body decoder for OrderBookRequest: reads fields at `pos`.
pub const OrderBookRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OrderBookRequest {
        var value: OrderBookRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const depth_v = readU32BE(buf, &pos);
        var depth: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; depth = depth_v; } else { pos += 1; }
        const at_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var at_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; at_time = at_time_v; } else { pos += 1; }
        value.symbol = symbol;
        value.depth = depth;
        value.at_time = at_time;
        return value;
    }
};

/// SBE encoder for OrderBookRequest
pub const OrderBookRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, depth: ?u32, at_time: ?i64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 8);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 12);
        try writeString(buf, symbol);
        try writeU32BE(buf, @intCast(depth orelse 0));
        try buf.append(if (depth != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(at_time orelse 0)));
        try buf.append(if (at_time != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderBookRequest: validates the header and reads the body.
pub const OrderBookRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OrderBookRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 8) return error.InvalidTemplateId;
        var value: OrderBookRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const depth_v = readU32BE(buf, &pos);
        var depth: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; depth = depth_v; } else { pos += 1; }
        const at_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var at_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; at_time = at_time_v; } else { pos += 1; }
        value.symbol = symbol;
        value.depth = depth;
        value.at_time = at_time;
        return value;
    }
};

/// SBE message OrderBookSnapshot
pub const OrderBookSnapshot = struct {
    symbol: []const u8,
    exchange: []const u8,
    bids: []PriceLevel,
    asks: []PriceLevel,
    timestamp: i64,
    sequence: ?u64,
    is_snapshot: ?u8,
};

/// SBE body encoder for OrderBookSnapshot (fields only, no header).
pub const OrderBookSnapshotBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OrderBookSnapshot, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.symbol);
        try writeString(buf, value.exchange);
        try writeU32BE(buf, @intCast(value.bids.len));
        for (value.bids) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU32BE(buf, @intCast(value.asks.len));
        for (value.asks) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try writeU64BE(buf, @intCast(value.sequence orelse 0));
        try buf.append(if (value.sequence != null) 1 else 0);
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for OrderBookSnapshot: reads fields at `pos`.
pub const OrderBookSnapshotBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OrderBookSnapshot {
        var value: OrderBookSnapshot = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const exchange = try readString(buf, &pos, allocator);
        const bids_count = readU32BE(buf, &pos);
        var bids = try allocator.alloc(PriceLevel, bids_count);
        var bids_idx: usize = 0;
        while (bids_idx < bids_count) : (bids_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            bids[bids_idx] = item;
        }
        const asks_count = readU32BE(buf, &pos);
        var asks = try allocator.alloc(PriceLevel, asks_count);
        var asks_idx: usize = 0;
        while (asks_idx < asks_count) : (asks_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            asks[asks_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.exchange = exchange;
        value.bids = bids;
        value.asks = asks;
        value.timestamp = timestamp;
        value.sequence = sequence;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for OrderBookSnapshot
pub const OrderBookSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, exchange: []const u8, bids: []const PriceLevel, asks: []const PriceLevel, timestamp: i64, sequence: ?u64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 9);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 17);
        try writeString(buf, symbol);
        try writeString(buf, exchange);
        try writeU32BE(buf, @intCast(bids.len));
        for (bids) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU32BE(buf, @intCast(asks.len));
        for (asks) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PriceLevel.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try writeU64BE(buf, @intCast(sequence orelse 0));
        try buf.append(if (sequence != null) 1 else 0);
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderBookSnapshot: validates the header and reads the body.
pub const OrderBookSnapshotDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OrderBookSnapshot {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 9) return error.InvalidTemplateId;
        var value: OrderBookSnapshot = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const exchange = try readString(buf, &pos, allocator);
        const bids_count = readU32BE(buf, &pos);
        var bids = try allocator.alloc(PriceLevel, bids_count);
        var bids_idx: usize = 0;
        while (bids_idx < bids_count) : (bids_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            bids[bids_idx] = item;
        }
        const asks_count = readU32BE(buf, &pos);
        var asks = try allocator.alloc(PriceLevel, asks_count);
        var asks_idx: usize = 0;
        while (asks_idx < asks_count) : (asks_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PriceLevel.decode(allocator, buf, &pos);
            pos = item_end;
            asks[asks_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.exchange = exchange;
        value.bids = bids;
        value.asks = asks;
        value.timestamp = timestamp;
        value.sequence = sequence;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message OrderBookDelta
pub const OrderBookDelta = struct {
    symbol: []const u8,
    updates: []MarketDataUpdate,
    timestamp: i64,
    sequence: ?u64,
};

/// SBE body encoder for OrderBookDelta (fields only, no header).
pub const OrderBookDeltaBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OrderBookDelta, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.symbol);
        try writeU32BE(buf, @intCast(value.updates.len));
        for (value.updates) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try MarketDataUpdate.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try writeU64BE(buf, @intCast(value.sequence orelse 0));
        try buf.append(if (value.sequence != null) 1 else 0);
    }
};

/// SBE body decoder for OrderBookDelta: reads fields at `pos`.
pub const OrderBookDeltaBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OrderBookDelta {
        var value: OrderBookDelta = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const updates_count = readU32BE(buf, &pos);
        var updates = try allocator.alloc(MarketDataUpdate, updates_count);
        var updates_idx: usize = 0;
        while (updates_idx < updates_count) : (updates_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try MarketDataUpdate.decode(allocator, buf, &pos);
            pos = item_end;
            updates[updates_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        value.symbol = symbol;
        value.updates = updates;
        value.timestamp = timestamp;
        value.sequence = sequence;
        return value;
    }
};

/// SBE encoder for OrderBookDelta
pub const OrderBookDeltaEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, updates: []const MarketDataUpdate, timestamp: i64, sequence: ?u64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 10);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 16);
        try writeString(buf, symbol);
        try writeU32BE(buf, @intCast(updates.len));
        for (updates) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try MarketDataUpdate.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try writeU64BE(buf, @intCast(sequence orelse 0));
        try buf.append(if (sequence != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderBookDelta: validates the header and reads the body.
pub const OrderBookDeltaDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OrderBookDelta {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 10) return error.InvalidTemplateId;
        var value: OrderBookDelta = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const updates_count = readU32BE(buf, &pos);
        var updates = try allocator.alloc(MarketDataUpdate, updates_count);
        var updates_idx: usize = 0;
        while (updates_idx < updates_count) : (updates_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try MarketDataUpdate.decode(allocator, buf, &pos);
            pos = item_end;
            updates[updates_idx] = item;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const sequence_v = readU64BE(buf, &pos);
        var sequence: ?u64 = null;
        if (buf[pos] == 1) { pos += 1; sequence = sequence_v; } else { pos += 1; }
        value.symbol = symbol;
        value.updates = updates;
        value.timestamp = timestamp;
        value.sequence = sequence;
        return value;
    }
};

/// SBE message AggregateTradeEvent
pub const AggregateTradeEvent = struct {
    trade: AggregateTrade,
};

/// SBE body encoder for AggregateTradeEvent (fields only, no header).
pub const AggregateTradeEventBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: AggregateTradeEvent, buf: *std.ArrayList(u8)) !void {
        try AggregateTrade.encode(allocator, value.trade, buf);
    }
};

/// SBE body decoder for AggregateTradeEvent: reads fields at `pos`.
pub const AggregateTradeEventBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !AggregateTradeEvent {
        var value: AggregateTradeEvent = undefined;
        const trade = try AggregateTrade.decode(allocator, buf, &pos);
        value.trade = trade;
        return value;
    }
};

/// SBE encoder for AggregateTradeEvent
pub const AggregateTradeEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, trade: AggregateTrade) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 11);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try AggregateTrade.encode(allocator, trade, buf);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AggregateTradeEvent: validates the header and reads the body.
pub const AggregateTradeEventDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !AggregateTradeEvent {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 11) return error.InvalidTemplateId;
        var value: AggregateTradeEvent = undefined;
        const trade = try AggregateTrade.decode(allocator, buf, &pos);
        value.trade = trade;
        return value;
    }
};

/// SBE message AggregateTradeRequest
pub const AggregateTradeRequest = struct {
    symbol: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
};

/// SBE body encoder for AggregateTradeRequest (fields only, no header).
pub const AggregateTradeRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: AggregateTradeRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeU64BE(buf, @as(u64, @bitCast(value.start_time orelse 0)));
        try buf.append(if (value.start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.end_time orelse 0)));
        try buf.append(if (value.end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(value.limit orelse 0));
        try buf.append(if (value.limit != null) 1 else 0);
        if (value.cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for AggregateTradeRequest: reads fields at `pos`.
pub const AggregateTradeRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !AggregateTradeRequest {
        var value: AggregateTradeRequest = undefined;
        var cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE encoder for AggregateTradeRequest
pub const AggregateTradeRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, start_time: ?i64, end_time: ?i64, limit: ?u32, cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 12);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 20);
        try writeString(buf, symbol);
        try writeU64BE(buf, @as(u64, @bitCast(start_time orelse 0)));
        try buf.append(if (start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(end_time orelse 0)));
        try buf.append(if (end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(limit orelse 0));
        try buf.append(if (limit != null) 1 else 0);
        if (cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AggregateTradeRequest: validates the header and reads the body.
pub const AggregateTradeRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !AggregateTradeRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 12) return error.InvalidTemplateId;
        var value: AggregateTradeRequest = undefined;
        var cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE message AggregateTradeBatch
pub const AggregateTradeBatch = struct {
    symbol: []const u8,
    trades: []AggregateTrade,
    has_more: u8,
    next_cursor: ?[]const u8,
};

/// SBE body encoder for AggregateTradeBatch (fields only, no header).
pub const AggregateTradeBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: AggregateTradeBatch, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.symbol);
        try writeU32BE(buf, @intCast(value.trades.len));
        for (value.trades) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try AggregateTrade.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for AggregateTradeBatch: reads fields at `pos`.
pub const AggregateTradeBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !AggregateTradeBatch {
        var value: AggregateTradeBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const trades_count = readU32BE(buf, &pos);
        var trades = try allocator.alloc(AggregateTrade, trades_count);
        var trades_idx: usize = 0;
        while (trades_idx < trades_count) : (trades_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try AggregateTrade.decode(allocator, buf, &pos);
            pos = item_end;
            trades[trades_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.trades = trades;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE encoder for AggregateTradeBatch
pub const AggregateTradeBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, trades: []const AggregateTrade, has_more: u8, next_cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 13);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, symbol);
        try writeU32BE(buf, @intCast(trades.len));
        for (trades) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try AggregateTrade.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(has_more));
        if (next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AggregateTradeBatch: validates the header and reads the body.
pub const AggregateTradeBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !AggregateTradeBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 13) return error.InvalidTemplateId;
        var value: AggregateTradeBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const trades_count = readU32BE(buf, &pos);
        var trades = try allocator.alloc(AggregateTrade, trades_count);
        var trades_idx: usize = 0;
        while (trades_idx < trades_count) : (trades_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try AggregateTrade.decode(allocator, buf, &pos);
            pos = item_end;
            trades[trades_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.trades = trades;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE message MiniTicker
pub const MiniTicker = struct {
    symbol: []const u8,
    last_price: f64,
    volume: f64,
    timestamp: i64,
    is_snapshot: ?u8,
};

/// SBE body encoder for MiniTicker (fields only, no header).
pub const MiniTickerBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MiniTicker, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.last_price);
        try writeF64BE(buf, value.volume);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for MiniTicker: reads fields at `pos`.
pub const MiniTickerBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MiniTicker {
        var value: MiniTicker = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const last_price = readF64BE(buf, &pos);
        const volume = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.last_price = last_price;
        value.volume = volume;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for MiniTicker
pub const MiniTickerEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, last_price: f64, volume: f64, timestamp: i64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 14);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 25);
        try writeString(buf, symbol);
        try writeF64BE(buf, last_price);
        try writeF64BE(buf, volume);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MiniTicker: validates the header and reads the body.
pub const MiniTickerDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MiniTicker {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 14) return error.InvalidTemplateId;
        var value: MiniTicker = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const last_price = readF64BE(buf, &pos);
        const volume = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.last_price = last_price;
        value.volume = volume;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message AllMidsRequest
pub const AllMidsRequest = struct {
};

/// SBE body encoder for AllMidsRequest (fields only, no header).
pub const AllMidsRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: AllMidsRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        _ = value;
        _ = buf;
    }
};

/// SBE body decoder for AllMidsRequest: reads fields at `pos`.
pub const AllMidsRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !AllMidsRequest {
        _ = allocator;
        _ = buf;
        _ = pos;
        const value: AllMidsRequest = undefined;
        return value;
    }
};

/// SBE encoder for AllMidsRequest
pub const AllMidsRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, ) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 15);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AllMidsRequest: validates the header and reads the body.
pub const AllMidsRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !AllMidsRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 15) return error.InvalidTemplateId;
        _ = allocator;
        const value: AllMidsRequest = undefined;
        return value;
    }
};

/// SBE message AllMidsBatch
pub const AllMidsBatch = struct {
    tickers: []MiniTicker,
};

/// SBE body encoder for AllMidsBatch (fields only, no header).
pub const AllMidsBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: AllMidsBatch, buf: *std.ArrayList(u8)) !void {
        try writeU32BE(buf, @intCast(value.tickers.len));
        for (value.tickers) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try MiniTickerBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
    }
};

/// SBE body decoder for AllMidsBatch: reads fields at `pos`.
pub const AllMidsBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !AllMidsBatch {
        var value: AllMidsBatch = undefined;
        const tickers_count = readU32BE(buf, &pos);
        var tickers = try allocator.alloc(MiniTicker, tickers_count);
        var tickers_idx: usize = 0;
        while (tickers_idx < tickers_count) : (tickers_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try MiniTickerBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            tickers[tickers_idx] = item;
        }
        value.tickers = tickers;
        return value;
    }
};

/// SBE encoder for AllMidsBatch
pub const AllMidsBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, tickers: []const MiniTicker) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 16);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeU32BE(buf, @intCast(tickers.len));
        for (tickers) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try MiniTickerBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AllMidsBatch: validates the header and reads the body.
pub const AllMidsBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !AllMidsBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 16) return error.InvalidTemplateId;
        var value: AllMidsBatch = undefined;
        const tickers_count = readU32BE(buf, &pos);
        var tickers = try allocator.alloc(MiniTicker, tickers_count);
        var tickers_idx: usize = 0;
        while (tickers_idx < tickers_count) : (tickers_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try MiniTickerBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            tickers[tickers_idx] = item;
        }
        value.tickers = tickers;
        return value;
    }
};

/// SBE message MarkPriceUpdate
pub const MarkPriceUpdate = struct {
    symbol: []const u8,
    mark_price: f64,
    index_price: ?f64,
    funding_rate: ?f64,
    timestamp: i64,
    is_snapshot: ?u8,
};

/// SBE body encoder for MarkPriceUpdate (fields only, no header).
pub const MarkPriceUpdateBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MarkPriceUpdate, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.mark_price);
        try writeF64BE(buf, value.index_price orelse 0);
        try buf.append(if (value.index_price != null) 1 else 0);
        try writeF64BE(buf, value.funding_rate orelse 0);
        try buf.append(if (value.funding_rate != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for MarkPriceUpdate: reads fields at `pos`.
pub const MarkPriceUpdateBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MarkPriceUpdate {
        var value: MarkPriceUpdate = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const mark_price = readF64BE(buf, &pos);
        const index_price_v = readF64BE(buf, &pos);
        var index_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; index_price = index_price_v; } else { pos += 1; }
        const funding_rate_v = readF64BE(buf, &pos);
        var funding_rate: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; funding_rate = funding_rate_v; } else { pos += 1; }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.mark_price = mark_price;
        value.index_price = index_price;
        value.funding_rate = funding_rate;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for MarkPriceUpdate
pub const MarkPriceUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, mark_price: f64, index_price: ?f64, funding_rate: ?f64, timestamp: i64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 17);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 33);
        try writeString(buf, symbol);
        try writeF64BE(buf, mark_price);
        try writeF64BE(buf, index_price orelse 0);
        try buf.append(if (index_price != null) 1 else 0);
        try writeF64BE(buf, funding_rate orelse 0);
        try buf.append(if (funding_rate != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarkPriceUpdate: validates the header and reads the body.
pub const MarkPriceUpdateDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MarkPriceUpdate {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 17) return error.InvalidTemplateId;
        var value: MarkPriceUpdate = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const mark_price = readF64BE(buf, &pos);
        const index_price_v = readF64BE(buf, &pos);
        var index_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; index_price = index_price_v; } else { pos += 1; }
        const funding_rate_v = readF64BE(buf, &pos);
        var funding_rate: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; funding_rate = funding_rate_v; } else { pos += 1; }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.mark_price = mark_price;
        value.index_price = index_price;
        value.funding_rate = funding_rate;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message MarkPriceRequest
pub const MarkPriceRequest = struct {
    symbol: []const u8,
};

/// SBE body encoder for MarkPriceRequest (fields only, no header).
pub const MarkPriceRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MarkPriceRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
    }
};

/// SBE body decoder for MarkPriceRequest: reads fields at `pos`.
pub const MarkPriceRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MarkPriceRequest {
        var value: MarkPriceRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for MarkPriceRequest
pub const MarkPriceRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 18);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeString(buf, symbol);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarkPriceRequest: validates the header and reads the body.
pub const MarkPriceRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MarkPriceRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 18) return error.InvalidTemplateId;
        var value: MarkPriceRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE message LiquidationTradeEvent
pub const LiquidationTradeEvent = struct {
    trade: LiquidationTrade,
};

/// SBE body encoder for LiquidationTradeEvent (fields only, no header).
pub const LiquidationTradeEventBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: LiquidationTradeEvent, buf: *std.ArrayList(u8)) !void {
        try LiquidationTrade.encode(allocator, value.trade, buf);
    }
};

/// SBE body decoder for LiquidationTradeEvent: reads fields at `pos`.
pub const LiquidationTradeEventBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !LiquidationTradeEvent {
        var value: LiquidationTradeEvent = undefined;
        const trade = try LiquidationTrade.decode(allocator, buf, &pos);
        value.trade = trade;
        return value;
    }
};

/// SBE encoder for LiquidationTradeEvent
pub const LiquidationTradeEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, trade: LiquidationTrade) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 19);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try LiquidationTrade.encode(allocator, trade, buf);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LiquidationTradeEvent: validates the header and reads the body.
pub const LiquidationTradeEventDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !LiquidationTradeEvent {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 19) return error.InvalidTemplateId;
        var value: LiquidationTradeEvent = undefined;
        const trade = try LiquidationTrade.decode(allocator, buf, &pos);
        value.trade = trade;
        return value;
    }
};

/// SBE message CandleBarEvent
pub const CandleBarEvent = struct {
    bar: CandleBar,
};

/// SBE body encoder for CandleBarEvent (fields only, no header).
pub const CandleBarEventBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CandleBarEvent, buf: *std.ArrayList(u8)) !void {
        try CandleBar.encode(allocator, value.bar, buf);
    }
};

/// SBE body decoder for CandleBarEvent: reads fields at `pos`.
pub const CandleBarEventBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CandleBarEvent {
        var value: CandleBarEvent = undefined;
        const bar = try CandleBar.decode(allocator, buf, &pos);
        value.bar = bar;
        return value;
    }
};

/// SBE encoder for CandleBarEvent
pub const CandleBarEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, bar: CandleBar) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 20);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try CandleBar.encode(allocator, bar, buf);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CandleBarEvent: validates the header and reads the body.
pub const CandleBarEventDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CandleBarEvent {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 20) return error.InvalidTemplateId;
        var value: CandleBarEvent = undefined;
        const bar = try CandleBar.decode(allocator, buf, &pos);
        value.bar = bar;
        return value;
    }
};

/// SBE message CandleBarRequest
pub const CandleBarRequest = struct {
    symbol: []const u8,
    interval: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
};

/// SBE body encoder for CandleBarRequest (fields only, no header).
pub const CandleBarRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CandleBarRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeString(buf, value.interval);
        try writeU64BE(buf, @as(u64, @bitCast(value.start_time orelse 0)));
        try buf.append(if (value.start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.end_time orelse 0)));
        try buf.append(if (value.end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(value.limit orelse 0));
        try buf.append(if (value.limit != null) 1 else 0);
        if (value.cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for CandleBarRequest: reads fields at `pos`.
pub const CandleBarRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CandleBarRequest {
        var value: CandleBarRequest = undefined;
        var cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const interval = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.interval = interval;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE encoder for CandleBarRequest
pub const CandleBarRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, interval: []const u8, start_time: ?i64, end_time: ?i64, limit: ?u32, cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 21);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 20);
        try writeString(buf, symbol);
        try writeString(buf, interval);
        try writeU64BE(buf, @as(u64, @bitCast(start_time orelse 0)));
        try buf.append(if (start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(end_time orelse 0)));
        try buf.append(if (end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(limit orelse 0));
        try buf.append(if (limit != null) 1 else 0);
        if (cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CandleBarRequest: validates the header and reads the body.
pub const CandleBarRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CandleBarRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 21) return error.InvalidTemplateId;
        var value: CandleBarRequest = undefined;
        var cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const interval = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.interval = interval;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE message CandleBarBatch
pub const CandleBarBatch = struct {
    symbol: []const u8,
    interval: []const u8,
    bars: []CandleBar,
    has_more: u8,
    next_cursor: ?[]const u8,
};

/// SBE body encoder for CandleBarBatch (fields only, no header).
pub const CandleBarBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CandleBarBatch, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.symbol);
        try writeString(buf, value.interval);
        try writeU32BE(buf, @intCast(value.bars.len));
        for (value.bars) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try CandleBar.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for CandleBarBatch: reads fields at `pos`.
pub const CandleBarBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CandleBarBatch {
        var value: CandleBarBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const interval = try readString(buf, &pos, allocator);
        const bars_count = readU32BE(buf, &pos);
        var bars = try allocator.alloc(CandleBar, bars_count);
        var bars_idx: usize = 0;
        while (bars_idx < bars_count) : (bars_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try CandleBar.decode(allocator, buf, &pos);
            pos = item_end;
            bars[bars_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.interval = interval;
        value.bars = bars;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE encoder for CandleBarBatch
pub const CandleBarBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, interval: []const u8, bars: []const CandleBar, has_more: u8, next_cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 22);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, symbol);
        try writeString(buf, interval);
        try writeU32BE(buf, @intCast(bars.len));
        for (bars) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try CandleBar.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(has_more));
        if (next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CandleBarBatch: validates the header and reads the body.
pub const CandleBarBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CandleBarBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 22) return error.InvalidTemplateId;
        var value: CandleBarBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const interval = try readString(buf, &pos, allocator);
        const bars_count = readU32BE(buf, &pos);
        var bars = try allocator.alloc(CandleBar, bars_count);
        var bars_idx: usize = 0;
        while (bars_idx < bars_count) : (bars_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try CandleBar.decode(allocator, buf, &pos);
            pos = item_end;
            bars[bars_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.interval = interval;
        value.bars = bars;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE message PublicTradeEvent
pub const PublicTradeEvent = struct {
    trade: PublicTrade,
};

/// SBE body encoder for PublicTradeEvent (fields only, no header).
pub const PublicTradeEventBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: PublicTradeEvent, buf: *std.ArrayList(u8)) !void {
        try PublicTrade.encode(allocator, value.trade, buf);
    }
};

/// SBE body decoder for PublicTradeEvent: reads fields at `pos`.
pub const PublicTradeEventBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PublicTradeEvent {
        var value: PublicTradeEvent = undefined;
        const trade = try PublicTrade.decode(allocator, buf, &pos);
        value.trade = trade;
        return value;
    }
};

/// SBE encoder for PublicTradeEvent
pub const PublicTradeEventEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, trade: PublicTrade) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 23);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try PublicTrade.encode(allocator, trade, buf);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PublicTradeEvent: validates the header and reads the body.
pub const PublicTradeEventDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !PublicTradeEvent {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 23) return error.InvalidTemplateId;
        var value: PublicTradeEvent = undefined;
        const trade = try PublicTrade.decode(allocator, buf, &pos);
        value.trade = trade;
        return value;
    }
};

/// SBE message TradeHistoryRequest
pub const TradeHistoryRequest = struct {
    symbol: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
};

/// SBE body encoder for TradeHistoryRequest (fields only, no header).
pub const TradeHistoryRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: TradeHistoryRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeU64BE(buf, @as(u64, @bitCast(value.start_time orelse 0)));
        try buf.append(if (value.start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.end_time orelse 0)));
        try buf.append(if (value.end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(value.limit orelse 0));
        try buf.append(if (value.limit != null) 1 else 0);
        if (value.cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for TradeHistoryRequest: reads fields at `pos`.
pub const TradeHistoryRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !TradeHistoryRequest {
        var value: TradeHistoryRequest = undefined;
        var cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE encoder for TradeHistoryRequest
pub const TradeHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, start_time: ?i64, end_time: ?i64, limit: ?u32, cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 24);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 20);
        try writeString(buf, symbol);
        try writeU64BE(buf, @as(u64, @bitCast(start_time orelse 0)));
        try buf.append(if (start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(end_time orelse 0)));
        try buf.append(if (end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(limit orelse 0));
        try buf.append(if (limit != null) 1 else 0);
        if (cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for TradeHistoryRequest: validates the header and reads the body.
pub const TradeHistoryRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !TradeHistoryRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 24) return error.InvalidTemplateId;
        var value: TradeHistoryRequest = undefined;
        var cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE message PublicTradeBatch
pub const PublicTradeBatch = struct {
    symbol: []const u8,
    trades: []PublicTrade,
    has_more: u8,
    next_cursor: ?[]const u8,
};

/// SBE body encoder for PublicTradeBatch (fields only, no header).
pub const PublicTradeBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: PublicTradeBatch, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.symbol);
        try writeU32BE(buf, @intCast(value.trades.len));
        for (value.trades) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PublicTrade.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for PublicTradeBatch: reads fields at `pos`.
pub const PublicTradeBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PublicTradeBatch {
        var value: PublicTradeBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const trades_count = readU32BE(buf, &pos);
        var trades = try allocator.alloc(PublicTrade, trades_count);
        var trades_idx: usize = 0;
        while (trades_idx < trades_count) : (trades_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PublicTrade.decode(allocator, buf, &pos);
            pos = item_end;
            trades[trades_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.trades = trades;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE encoder for PublicTradeBatch
pub const PublicTradeBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, trades: []const PublicTrade, has_more: u8, next_cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 25);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, symbol);
        try writeU32BE(buf, @intCast(trades.len));
        for (trades) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PublicTrade.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(has_more));
        if (next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PublicTradeBatch: validates the header and reads the body.
pub const PublicTradeBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !PublicTradeBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 25) return error.InvalidTemplateId;
        var value: PublicTradeBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const symbol = try readString(buf, &pos, allocator);
        const trades_count = readU32BE(buf, &pos);
        var trades = try allocator.alloc(PublicTrade, trades_count);
        var trades_idx: usize = 0;
        while (trades_idx < trades_count) : (trades_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PublicTrade.decode(allocator, buf, &pos);
            pos = item_end;
            trades[trades_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        value.trades = trades;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE message BestBidOffer
pub const BestBidOffer = struct {
    symbol: []const u8,
    bid_price: ?f64,
    bid_qty: ?f64,
    ask_price: ?f64,
    ask_qty: ?f64,
    timestamp: i64,
    is_snapshot: ?u8,
};

/// SBE body encoder for BestBidOffer (fields only, no header).
pub const BestBidOfferBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: BestBidOffer, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.bid_price orelse 0);
        try buf.append(if (value.bid_price != null) 1 else 0);
        try writeF64BE(buf, value.bid_qty orelse 0);
        try buf.append(if (value.bid_qty != null) 1 else 0);
        try writeF64BE(buf, value.ask_price orelse 0);
        try buf.append(if (value.ask_price != null) 1 else 0);
        try writeF64BE(buf, value.ask_qty orelse 0);
        try buf.append(if (value.ask_qty != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for BestBidOffer: reads fields at `pos`.
pub const BestBidOfferBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !BestBidOffer {
        var value: BestBidOffer = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const bid_price_v = readF64BE(buf, &pos);
        var bid_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; bid_price = bid_price_v; } else { pos += 1; }
        const bid_qty_v = readF64BE(buf, &pos);
        var bid_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; bid_qty = bid_qty_v; } else { pos += 1; }
        const ask_price_v = readF64BE(buf, &pos);
        var ask_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; ask_price = ask_price_v; } else { pos += 1; }
        const ask_qty_v = readF64BE(buf, &pos);
        var ask_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; ask_qty = ask_qty_v; } else { pos += 1; }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.bid_price = bid_price;
        value.bid_qty = bid_qty;
        value.ask_price = ask_price;
        value.ask_qty = ask_qty;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for BestBidOffer
pub const BestBidOfferEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, bid_price: ?f64, bid_qty: ?f64, ask_price: ?f64, ask_qty: ?f64, timestamp: i64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 26);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 41);
        try writeString(buf, symbol);
        try writeF64BE(buf, bid_price orelse 0);
        try buf.append(if (bid_price != null) 1 else 0);
        try writeF64BE(buf, bid_qty orelse 0);
        try buf.append(if (bid_qty != null) 1 else 0);
        try writeF64BE(buf, ask_price orelse 0);
        try buf.append(if (ask_price != null) 1 else 0);
        try writeF64BE(buf, ask_qty orelse 0);
        try buf.append(if (ask_qty != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for BestBidOffer: validates the header and reads the body.
pub const BestBidOfferDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !BestBidOffer {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 26) return error.InvalidTemplateId;
        var value: BestBidOffer = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const bid_price_v = readF64BE(buf, &pos);
        var bid_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; bid_price = bid_price_v; } else { pos += 1; }
        const bid_qty_v = readF64BE(buf, &pos);
        var bid_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; bid_qty = bid_qty_v; } else { pos += 1; }
        const ask_price_v = readF64BE(buf, &pos);
        var ask_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; ask_price = ask_price_v; } else { pos += 1; }
        const ask_qty_v = readF64BE(buf, &pos);
        var ask_qty: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; ask_qty = ask_qty_v; } else { pos += 1; }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.bid_price = bid_price;
        value.bid_qty = bid_qty;
        value.ask_price = ask_price;
        value.ask_qty = ask_qty;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message SymbolTicker
pub const SymbolTicker = struct {
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
};

/// SBE body encoder for SymbolTicker (fields only, no header).
pub const SymbolTickerBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: SymbolTicker, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.last_price);
        try writeF64BE(buf, value.price_change);
        try writeF64BE(buf, value.price_change_pct);
        try writeF64BE(buf, value.volume);
        try writeF64BE(buf, value.high);
        try writeF64BE(buf, value.low);
        try writeF64BE(buf, value.open);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for SymbolTicker: reads fields at `pos`.
pub const SymbolTickerBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !SymbolTicker {
        var value: SymbolTicker = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const last_price = readF64BE(buf, &pos);
        const price_change = readF64BE(buf, &pos);
        const price_change_pct = readF64BE(buf, &pos);
        const volume = readF64BE(buf, &pos);
        const high = readF64BE(buf, &pos);
        const low = readF64BE(buf, &pos);
        const open = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.last_price = last_price;
        value.price_change = price_change;
        value.price_change_pct = price_change_pct;
        value.volume = volume;
        value.high = high;
        value.low = low;
        value.open = open;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for SymbolTicker
pub const SymbolTickerEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, last_price: f64, price_change: f64, price_change_pct: f64, volume: f64, high: f64, low: f64, open: f64, timestamp: i64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 27);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 65);
        try writeString(buf, symbol);
        try writeF64BE(buf, last_price);
        try writeF64BE(buf, price_change);
        try writeF64BE(buf, price_change_pct);
        try writeF64BE(buf, volume);
        try writeF64BE(buf, high);
        try writeF64BE(buf, low);
        try writeF64BE(buf, open);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for SymbolTicker: validates the header and reads the body.
pub const SymbolTickerDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !SymbolTicker {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 27) return error.InvalidTemplateId;
        var value: SymbolTicker = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const last_price = readF64BE(buf, &pos);
        const price_change = readF64BE(buf, &pos);
        const price_change_pct = readF64BE(buf, &pos);
        const volume = readF64BE(buf, &pos);
        const high = readF64BE(buf, &pos);
        const low = readF64BE(buf, &pos);
        const open = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.last_price = last_price;
        value.price_change = price_change;
        value.price_change_pct = price_change_pct;
        value.volume = volume;
        value.high = high;
        value.low = low;
        value.open = open;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message TickerRequest
pub const TickerRequest = struct {
    symbol: []const u8,
};

/// SBE body encoder for TickerRequest (fields only, no header).
pub const TickerRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: TickerRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
    }
};

/// SBE body decoder for TickerRequest: reads fields at `pos`.
pub const TickerRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !TickerRequest {
        var value: TickerRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for TickerRequest
pub const TickerRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 28);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeString(buf, symbol);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for TickerRequest: validates the header and reads the body.
pub const TickerRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !TickerRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 28) return error.InvalidTemplateId;
        var value: TickerRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE message InstrumentCatalogRequest
pub const InstrumentCatalogRequest = struct {
};

/// SBE body encoder for InstrumentCatalogRequest (fields only, no header).
pub const InstrumentCatalogRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: InstrumentCatalogRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        _ = value;
        _ = buf;
    }
};

/// SBE body decoder for InstrumentCatalogRequest: reads fields at `pos`.
pub const InstrumentCatalogRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !InstrumentCatalogRequest {
        _ = allocator;
        _ = buf;
        _ = pos;
        const value: InstrumentCatalogRequest = undefined;
        return value;
    }
};

/// SBE encoder for InstrumentCatalogRequest
pub const InstrumentCatalogRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, ) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 29);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for InstrumentCatalogRequest: validates the header and reads the body.
pub const InstrumentCatalogRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !InstrumentCatalogRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 29) return error.InvalidTemplateId;
        _ = allocator;
        const value: InstrumentCatalogRequest = undefined;
        return value;
    }
};

/// SBE message InstrumentCatalogResponse
pub const InstrumentCatalogResponse = struct {
    instruments: []InstrumentMetadata,
};

/// SBE body encoder for InstrumentCatalogResponse (fields only, no header).
pub const InstrumentCatalogResponseBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: InstrumentCatalogResponse, buf: *std.ArrayList(u8)) !void {
        try writeU32BE(buf, @intCast(value.instruments.len));
        for (value.instruments) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try InstrumentMetadata.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
    }
};

/// SBE body decoder for InstrumentCatalogResponse: reads fields at `pos`.
pub const InstrumentCatalogResponseBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !InstrumentCatalogResponse {
        var value: InstrumentCatalogResponse = undefined;
        const instruments_count = readU32BE(buf, &pos);
        var instruments = try allocator.alloc(InstrumentMetadata, instruments_count);
        var instruments_idx: usize = 0;
        while (instruments_idx < instruments_count) : (instruments_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try InstrumentMetadata.decode(allocator, buf, &pos);
            pos = item_end;
            instruments[instruments_idx] = item;
        }
        value.instruments = instruments;
        return value;
    }
};

/// SBE encoder for InstrumentCatalogResponse
pub const InstrumentCatalogResponseEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, instruments: []const InstrumentMetadata) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 30);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeU32BE(buf, @intCast(instruments.len));
        for (instruments) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try InstrumentMetadata.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for InstrumentCatalogResponse: validates the header and reads the body.
pub const InstrumentCatalogResponseDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !InstrumentCatalogResponse {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 30) return error.InvalidTemplateId;
        var value: InstrumentCatalogResponse = undefined;
        const instruments_count = readU32BE(buf, &pos);
        var instruments = try allocator.alloc(InstrumentMetadata, instruments_count);
        var instruments_idx: usize = 0;
        while (instruments_idx < instruments_count) : (instruments_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try InstrumentMetadata.decode(allocator, buf, &pos);
            pos = item_end;
            instruments[instruments_idx] = item;
        }
        value.instruments = instruments;
        return value;
    }
};

/// SBE message AccountSummary
pub const AccountSummary = struct {
    account: []const u8,
    balance: f64,
    buying_power: f64,
    currency: []const u8,
};

/// SBE body encoder for AccountSummary (fields only, no header).
pub const AccountSummaryBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: AccountSummary, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeF64BE(buf, value.balance);
        try writeF64BE(buf, value.buying_power);
        try writeString(buf, value.currency);
    }
};

/// SBE body decoder for AccountSummary: reads fields at `pos`.
pub const AccountSummaryBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !AccountSummary {
        var value: AccountSummary = undefined;
        const account = try readString(buf, &pos, allocator);
        const balance = readF64BE(buf, &pos);
        const buying_power = readF64BE(buf, &pos);
        const currency = try readString(buf, &pos, allocator);
        value.account = account;
        value.balance = balance;
        value.buying_power = buying_power;
        value.currency = currency;
        return value;
    }
};

/// SBE encoder for AccountSummary
pub const AccountSummaryEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, balance: f64, buying_power: f64, currency: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 31);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 16);
        try writeString(buf, account);
        try writeF64BE(buf, balance);
        try writeF64BE(buf, buying_power);
        try writeString(buf, currency);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for AccountSummary: validates the header and reads the body.
pub const AccountSummaryDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !AccountSummary {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 31) return error.InvalidTemplateId;
        var value: AccountSummary = undefined;
        const account = try readString(buf, &pos, allocator);
        const balance = readF64BE(buf, &pos);
        const buying_power = readF64BE(buf, &pos);
        const currency = try readString(buf, &pos, allocator);
        value.account = account;
        value.balance = balance;
        value.buying_power = buying_power;
        value.currency = currency;
        return value;
    }
};

/// SBE message MarginSummary
pub const MarginSummary = struct {
    account: []const u8,
    balance: f64,
    buying_power: f64,
    equity: f64,
    margin_used: f64,
    available: f64,
    currency: []const u8,
};

/// SBE body encoder for MarginSummary (fields only, no header).
pub const MarginSummaryBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MarginSummary, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeF64BE(buf, value.balance);
        try writeF64BE(buf, value.buying_power);
        try writeF64BE(buf, value.equity);
        try writeF64BE(buf, value.margin_used);
        try writeF64BE(buf, value.available);
        try writeString(buf, value.currency);
    }
};

/// SBE body decoder for MarginSummary: reads fields at `pos`.
pub const MarginSummaryBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MarginSummary {
        var value: MarginSummary = undefined;
        const account = try readString(buf, &pos, allocator);
        const balance = readF64BE(buf, &pos);
        const buying_power = readF64BE(buf, &pos);
        const equity = readF64BE(buf, &pos);
        const margin_used = readF64BE(buf, &pos);
        const available = readF64BE(buf, &pos);
        const currency = try readString(buf, &pos, allocator);
        value.account = account;
        value.balance = balance;
        value.buying_power = buying_power;
        value.equity = equity;
        value.margin_used = margin_used;
        value.available = available;
        value.currency = currency;
        return value;
    }
};

/// SBE encoder for MarginSummary
pub const MarginSummaryEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, balance: f64, buying_power: f64, equity: f64, margin_used: f64, available: f64, currency: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 32);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 40);
        try writeString(buf, account);
        try writeF64BE(buf, balance);
        try writeF64BE(buf, buying_power);
        try writeF64BE(buf, equity);
        try writeF64BE(buf, margin_used);
        try writeF64BE(buf, available);
        try writeString(buf, currency);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarginSummary: validates the header and reads the body.
pub const MarginSummaryDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MarginSummary {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 32) return error.InvalidTemplateId;
        var value: MarginSummary = undefined;
        const account = try readString(buf, &pos, allocator);
        const balance = readF64BE(buf, &pos);
        const buying_power = readF64BE(buf, &pos);
        const equity = readF64BE(buf, &pos);
        const margin_used = readF64BE(buf, &pos);
        const available = readF64BE(buf, &pos);
        const currency = try readString(buf, &pos, allocator);
        value.account = account;
        value.balance = balance;
        value.buying_power = buying_power;
        value.equity = equity;
        value.margin_used = margin_used;
        value.available = available;
        value.currency = currency;
        return value;
    }
};

/// SBE message BalanceSnapshot
pub const BalanceSnapshot = struct {
    account: []const u8,
    balances: []BalanceEntry,
    is_snapshot: ?u8,
};

/// SBE body encoder for BalanceSnapshot (fields only, no header).
pub const BalanceSnapshotBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: BalanceSnapshot, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try writeU32BE(buf, @intCast(value.balances.len));
        for (value.balances) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try BalanceEntry.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for BalanceSnapshot: reads fields at `pos`.
pub const BalanceSnapshotBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !BalanceSnapshot {
        var value: BalanceSnapshot = undefined;
        const account = try readString(buf, &pos, allocator);
        const balances_count = readU32BE(buf, &pos);
        var balances = try allocator.alloc(BalanceEntry, balances_count);
        var balances_idx: usize = 0;
        while (balances_idx < balances_count) : (balances_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try BalanceEntry.decode(allocator, buf, &pos);
            pos = item_end;
            balances[balances_idx] = item;
        }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.balances = balances;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for BalanceSnapshot
pub const BalanceSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, balances: []const BalanceEntry, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 33);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeU32BE(buf, @intCast(balances.len));
        for (balances) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try BalanceEntry.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for BalanceSnapshot: validates the header and reads the body.
pub const BalanceSnapshotDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !BalanceSnapshot {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 33) return error.InvalidTemplateId;
        var value: BalanceSnapshot = undefined;
        const account = try readString(buf, &pos, allocator);
        const balances_count = readU32BE(buf, &pos);
        var balances = try allocator.alloc(BalanceEntry, balances_count);
        var balances_idx: usize = 0;
        while (balances_idx < balances_count) : (balances_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try BalanceEntry.decode(allocator, buf, &pos);
            pos = item_end;
            balances[balances_idx] = item;
        }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.balances = balances;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message BalanceUpdate
pub const BalanceUpdate = struct {
    account: []const u8,
    asset: []const u8,
    delta: f64,
    total: f64,
    available: f64,
    reason: BalanceUpdateReason,
};

/// SBE body encoder for BalanceUpdate (fields only, no header).
pub const BalanceUpdateBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: BalanceUpdate, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeString(buf, value.asset);
        try writeF64BE(buf, value.delta);
        try writeF64BE(buf, value.total);
        try writeF64BE(buf, value.available);
        try buf.append(BalanceUpdateReasonToValue(value.reason));
    }
};

/// SBE body decoder for BalanceUpdate: reads fields at `pos`.
pub const BalanceUpdateBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !BalanceUpdate {
        var value: BalanceUpdate = undefined;
        const account = try readString(buf, &pos, allocator);
        const asset = try readString(buf, &pos, allocator);
        const delta = readF64BE(buf, &pos);
        const total = readF64BE(buf, &pos);
        const available = readF64BE(buf, &pos);
        const reason = try BalanceUpdateReasonFromValue(readU8(buf, &pos));
        value.account = account;
        value.asset = asset;
        value.delta = delta;
        value.total = total;
        value.available = available;
        value.reason = reason;
        return value;
    }
};

/// SBE encoder for BalanceUpdate
pub const BalanceUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, asset: []const u8, delta: f64, total: f64, available: f64, reason: BalanceUpdateReason) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 34);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 25);
        try writeString(buf, account);
        try writeString(buf, asset);
        try writeF64BE(buf, delta);
        try writeF64BE(buf, total);
        try writeF64BE(buf, available);
        try buf.append(BalanceUpdateReasonToValue(reason));
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for BalanceUpdate: validates the header and reads the body.
pub const BalanceUpdateDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !BalanceUpdate {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 34) return error.InvalidTemplateId;
        var value: BalanceUpdate = undefined;
        const account = try readString(buf, &pos, allocator);
        const asset = try readString(buf, &pos, allocator);
        const delta = readF64BE(buf, &pos);
        const total = readF64BE(buf, &pos);
        const available = readF64BE(buf, &pos);
        const reason = try BalanceUpdateReasonFromValue(readU8(buf, &pos));
        value.account = account;
        value.asset = asset;
        value.delta = delta;
        value.total = total;
        value.available = available;
        value.reason = reason;
        return value;
    }
};

/// SBE message PositionSnapshot
pub const PositionSnapshot = struct {
    account: []const u8,
    positions: []PositionEntry,
    is_snapshot: ?u8,
};

/// SBE body encoder for PositionSnapshot (fields only, no header).
pub const PositionSnapshotBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: PositionSnapshot, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try writeU32BE(buf, @intCast(value.positions.len));
        for (value.positions) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PositionEntry.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for PositionSnapshot: reads fields at `pos`.
pub const PositionSnapshotBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PositionSnapshot {
        var value: PositionSnapshot = undefined;
        const account = try readString(buf, &pos, allocator);
        const positions_count = readU32BE(buf, &pos);
        var positions = try allocator.alloc(PositionEntry, positions_count);
        var positions_idx: usize = 0;
        while (positions_idx < positions_count) : (positions_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PositionEntry.decode(allocator, buf, &pos);
            pos = item_end;
            positions[positions_idx] = item;
        }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.positions = positions;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for PositionSnapshot
pub const PositionSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, positions: []const PositionEntry, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 35);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeU32BE(buf, @intCast(positions.len));
        for (positions) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try PositionEntry.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PositionSnapshot: validates the header and reads the body.
pub const PositionSnapshotDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !PositionSnapshot {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 35) return error.InvalidTemplateId;
        var value: PositionSnapshot = undefined;
        const account = try readString(buf, &pos, allocator);
        const positions_count = readU32BE(buf, &pos);
        var positions = try allocator.alloc(PositionEntry, positions_count);
        var positions_idx: usize = 0;
        while (positions_idx < positions_count) : (positions_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try PositionEntry.decode(allocator, buf, &pos);
            pos = item_end;
            positions[positions_idx] = item;
        }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.positions = positions;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message PositionUpdate
pub const PositionUpdate = struct {
    account: []const u8,
    symbol: []const u8,
    qty: f64,
    entry_price: f64,
    unrealized_pnl: f64,
};

/// SBE body encoder for PositionUpdate (fields only, no header).
pub const PositionUpdateBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: PositionUpdate, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.qty);
        try writeF64BE(buf, value.entry_price);
        try writeF64BE(buf, value.unrealized_pnl);
    }
};

/// SBE body decoder for PositionUpdate: reads fields at `pos`.
pub const PositionUpdateBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PositionUpdate {
        var value: PositionUpdate = undefined;
        const account = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const qty = readF64BE(buf, &pos);
        const entry_price = readF64BE(buf, &pos);
        const unrealized_pnl = readF64BE(buf, &pos);
        value.account = account;
        value.symbol = symbol;
        value.qty = qty;
        value.entry_price = entry_price;
        value.unrealized_pnl = unrealized_pnl;
        return value;
    }
};

/// SBE encoder for PositionUpdate
pub const PositionUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, symbol: []const u8, qty: f64, entry_price: f64, unrealized_pnl: f64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 36);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 24);
        try writeString(buf, account);
        try writeString(buf, symbol);
        try writeF64BE(buf, qty);
        try writeF64BE(buf, entry_price);
        try writeF64BE(buf, unrealized_pnl);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PositionUpdate: validates the header and reads the body.
pub const PositionUpdateDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !PositionUpdate {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 36) return error.InvalidTemplateId;
        var value: PositionUpdate = undefined;
        const account = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const qty = readF64BE(buf, &pos);
        const entry_price = readF64BE(buf, &pos);
        const unrealized_pnl = readF64BE(buf, &pos);
        value.account = account;
        value.symbol = symbol;
        value.qty = qty;
        value.entry_price = entry_price;
        value.unrealized_pnl = unrealized_pnl;
        return value;
    }
};

/// SBE message MarginUpdate
pub const MarginUpdate = struct {
    account: []const u8,
    summary: MarginSummary,
    is_snapshot: ?u8,
};

/// SBE body encoder for MarginUpdate (fields only, no header).
pub const MarginUpdateBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MarginUpdate, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try MarginSummaryBodyEncoder.encode(allocator, value.summary, buf);
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for MarginUpdate: reads fields at `pos`.
pub const MarginUpdateBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MarginUpdate {
        var value: MarginUpdate = undefined;
        const account = try readString(buf, &pos, allocator);
        const summary = try MarginSummaryBodyDecoder.decode(allocator, buf, &pos);
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.summary = summary;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for MarginUpdate
pub const MarginUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, summary: MarginSummary, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 37);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try MarginSummaryBodyEncoder.encode(allocator, summary, buf);
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MarginUpdate: validates the header and reads the body.
pub const MarginUpdateDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MarginUpdate {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 37) return error.InvalidTemplateId;
        var value: MarginUpdate = undefined;
        const account = try readString(buf, &pos, allocator);
        const summary = try MarginSummaryBodyDecoder.decode(allocator, buf, &pos);
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.summary = summary;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message UserLiquidation
pub const UserLiquidation = struct {
    account: []const u8,
    symbol: []const u8,
    qty: f64,
    price: f64,
    timestamp: i64,
};

/// SBE body encoder for UserLiquidation (fields only, no header).
pub const UserLiquidationBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: UserLiquidation, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.qty);
        try writeF64BE(buf, value.price);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
    }
};

/// SBE body decoder for UserLiquidation: reads fields at `pos`.
pub const UserLiquidationBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !UserLiquidation {
        var value: UserLiquidation = undefined;
        const account = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const qty = readF64BE(buf, &pos);
        const price = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.account = account;
        value.symbol = symbol;
        value.qty = qty;
        value.price = price;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE encoder for UserLiquidation
pub const UserLiquidationEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, symbol: []const u8, qty: f64, price: f64, timestamp: i64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 38);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 24);
        try writeString(buf, account);
        try writeString(buf, symbol);
        try writeF64BE(buf, qty);
        try writeF64BE(buf, price);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for UserLiquidation: validates the header and reads the body.
pub const UserLiquidationDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !UserLiquidation {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 38) return error.InvalidTemplateId;
        var value: UserLiquidation = undefined;
        const account = try readString(buf, &pos, allocator);
        const symbol = try readString(buf, &pos, allocator);
        const qty = readF64BE(buf, &pos);
        const price = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.account = account;
        value.symbol = symbol;
        value.qty = qty;
        value.price = price;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE message OrderListStatus
pub const OrderListStatus = struct {
    account: []const u8,
    list_id: []const u8,
    status: OrderListStatusStatus,
    symbol: ?[]const u8,
};

/// SBE body encoder for OrderListStatus (fields only, no header).
pub const OrderListStatusBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OrderListStatus, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeString(buf, value.list_id);
        try buf.append(OrderListStatusStatusToValue(value.status));
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for OrderListStatus: reads fields at `pos`.
pub const OrderListStatusBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OrderListStatus {
        var value: OrderListStatus = undefined;
        var symbol: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const list_id = try readString(buf, &pos, allocator);
        const status = try OrderListStatusStatusFromValue(readU8(buf, &pos));
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.list_id = list_id;
        value.status = status;
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for OrderListStatus
pub const OrderListStatusEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, list_id: []const u8, status: OrderListStatusStatus, symbol: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 39);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeString(buf, list_id);
        try buf.append(OrderListStatusStatusToValue(status));
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderListStatus: validates the header and reads the body.
pub const OrderListStatusDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OrderListStatus {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 39) return error.InvalidTemplateId;
        var value: OrderListStatus = undefined;
        var symbol: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const list_id = try readString(buf, &pos, allocator);
        const status = try OrderListStatusStatusFromValue(readU8(buf, &pos));
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.list_id = list_id;
        value.status = status;
        value.symbol = symbol;
        return value;
    }
};

/// SBE message FillHistoryRequest
pub const FillHistoryRequest = struct {
    account: []const u8,
    symbol: ?[]const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
};

/// SBE body encoder for FillHistoryRequest (fields only, no header).
pub const FillHistoryRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: FillHistoryRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeU64BE(buf, @as(u64, @bitCast(value.start_time orelse 0)));
        try buf.append(if (value.start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.end_time orelse 0)));
        try buf.append(if (value.end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(value.limit orelse 0));
        try buf.append(if (value.limit != null) 1 else 0);
        if (value.cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for FillHistoryRequest: reads fields at `pos`.
pub const FillHistoryRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !FillHistoryRequest {
        var value: FillHistoryRequest = undefined;
        var symbol: ?[]u8 = null;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE encoder for FillHistoryRequest
pub const FillHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, symbol: ?[]const u8, start_time: ?i64, end_time: ?i64, limit: ?u32, cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 40);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 20);
        try writeString(buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeU64BE(buf, @as(u64, @bitCast(start_time orelse 0)));
        try buf.append(if (start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(end_time orelse 0)));
        try buf.append(if (end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(limit orelse 0));
        try buf.append(if (limit != null) 1 else 0);
        if (cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FillHistoryRequest: validates the header and reads the body.
pub const FillHistoryRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !FillHistoryRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 40) return error.InvalidTemplateId;
        var value: FillHistoryRequest = undefined;
        var symbol: ?[]u8 = null;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE message FillHistoryBatch
pub const FillHistoryBatch = struct {
    account: []const u8,
    fills: []ExecutionReport,
    has_more: u8,
    next_cursor: ?[]const u8,
};

/// SBE body encoder for FillHistoryBatch (fields only, no header).
pub const FillHistoryBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: FillHistoryBatch, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try writeU32BE(buf, @intCast(value.fills.len));
        for (value.fills) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try ExecutionReportBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for FillHistoryBatch: reads fields at `pos`.
pub const FillHistoryBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !FillHistoryBatch {
        var value: FillHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const fills_count = readU32BE(buf, &pos);
        var fills = try allocator.alloc(ExecutionReport, fills_count);
        var fills_idx: usize = 0;
        while (fills_idx < fills_count) : (fills_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try ExecutionReportBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            fills[fills_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.fills = fills;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE encoder for FillHistoryBatch
pub const FillHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, fills: []const ExecutionReport, has_more: u8, next_cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 41);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeU32BE(buf, @intCast(fills.len));
        for (fills) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try ExecutionReportBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(has_more));
        if (next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FillHistoryBatch: validates the header and reads the body.
pub const FillHistoryBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !FillHistoryBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 41) return error.InvalidTemplateId;
        var value: FillHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const fills_count = readU32BE(buf, &pos);
        var fills = try allocator.alloc(ExecutionReport, fills_count);
        var fills_idx: usize = 0;
        while (fills_idx < fills_count) : (fills_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try ExecutionReportBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            fills[fills_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.fills = fills;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE message FundingPayment
pub const FundingPayment = struct {
    account: []const u8,
    symbol: ?[]const u8,
    amount: f64,
    rate: f64,
    timestamp: i64,
};

/// SBE body encoder for FundingPayment (fields only, no header).
pub const FundingPaymentBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: FundingPayment, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeF64BE(buf, value.amount);
        try writeF64BE(buf, value.rate);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
    }
};

/// SBE body decoder for FundingPayment: reads fields at `pos`.
pub const FundingPaymentBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !FundingPayment {
        var value: FundingPayment = undefined;
        var symbol: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const amount = readF64BE(buf, &pos);
        const rate = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.account = account;
        value.symbol = symbol;
        value.amount = amount;
        value.rate = rate;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE encoder for FundingPayment
pub const FundingPaymentEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, symbol: ?[]const u8, amount: f64, rate: f64, timestamp: i64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 42);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 24);
        try writeString(buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeF64BE(buf, amount);
        try writeF64BE(buf, rate);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FundingPayment: validates the header and reads the body.
pub const FundingPaymentDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !FundingPayment {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 42) return error.InvalidTemplateId;
        var value: FundingPayment = undefined;
        var symbol: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const amount = readF64BE(buf, &pos);
        const rate = readF64BE(buf, &pos);
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.account = account;
        value.symbol = symbol;
        value.amount = amount;
        value.rate = rate;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE message FundingHistoryRequest
pub const FundingHistoryRequest = struct {
    account: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
};

/// SBE body encoder for FundingHistoryRequest (fields only, no header).
pub const FundingHistoryRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: FundingHistoryRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeU64BE(buf, @as(u64, @bitCast(value.start_time orelse 0)));
        try buf.append(if (value.start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.end_time orelse 0)));
        try buf.append(if (value.end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(value.limit orelse 0));
        try buf.append(if (value.limit != null) 1 else 0);
        if (value.cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for FundingHistoryRequest: reads fields at `pos`.
pub const FundingHistoryRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !FundingHistoryRequest {
        var value: FundingHistoryRequest = undefined;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE encoder for FundingHistoryRequest
pub const FundingHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, start_time: ?i64, end_time: ?i64, limit: ?u32, cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 43);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 20);
        try writeString(buf, account);
        try writeU64BE(buf, @as(u64, @bitCast(start_time orelse 0)));
        try buf.append(if (start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(end_time orelse 0)));
        try buf.append(if (end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(limit orelse 0));
        try buf.append(if (limit != null) 1 else 0);
        if (cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FundingHistoryRequest: validates the header and reads the body.
pub const FundingHistoryRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !FundingHistoryRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 43) return error.InvalidTemplateId;
        var value: FundingHistoryRequest = undefined;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE message FundingHistoryBatch
pub const FundingHistoryBatch = struct {
    account: []const u8,
    payments: []FundingPayment,
    has_more: u8,
    next_cursor: ?[]const u8,
};

/// SBE body encoder for FundingHistoryBatch (fields only, no header).
pub const FundingHistoryBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: FundingHistoryBatch, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try writeU32BE(buf, @intCast(value.payments.len));
        for (value.payments) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try FundingPaymentBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for FundingHistoryBatch: reads fields at `pos`.
pub const FundingHistoryBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !FundingHistoryBatch {
        var value: FundingHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const payments_count = readU32BE(buf, &pos);
        var payments = try allocator.alloc(FundingPayment, payments_count);
        var payments_idx: usize = 0;
        while (payments_idx < payments_count) : (payments_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try FundingPaymentBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            payments[payments_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.payments = payments;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE encoder for FundingHistoryBatch
pub const FundingHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, payments: []const FundingPayment, has_more: u8, next_cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 44);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeU32BE(buf, @intCast(payments.len));
        for (payments) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try FundingPaymentBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(has_more));
        if (next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for FundingHistoryBatch: validates the header and reads the body.
pub const FundingHistoryBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !FundingHistoryBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 44) return error.InvalidTemplateId;
        var value: FundingHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const payments_count = readU32BE(buf, &pos);
        var payments = try allocator.alloc(FundingPayment, payments_count);
        var payments_idx: usize = 0;
        while (payments_idx < payments_count) : (payments_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try FundingPaymentBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            payments[payments_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.payments = payments;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE message LedgerUpdate
pub const LedgerUpdate = struct {
    account: []const u8,
    asset: []const u8,
    delta: f64,
    kind: LedgerUpdateKind,
    timestamp: i64,
    reference_id: ?[]const u8,
};

/// SBE body encoder for LedgerUpdate (fields only, no header).
pub const LedgerUpdateBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: LedgerUpdate, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeString(buf, value.asset);
        try writeF64BE(buf, value.delta);
        try buf.append(LedgerUpdateKindToValue(value.kind));
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        if (value.reference_id) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for LedgerUpdate: reads fields at `pos`.
pub const LedgerUpdateBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !LedgerUpdate {
        var value: LedgerUpdate = undefined;
        var reference_id: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const asset = try readString(buf, &pos, allocator);
        const delta = readF64BE(buf, &pos);
        const kind = try LedgerUpdateKindFromValue(readU8(buf, &pos));
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        if (buf[pos] == 1) {
            pos += 1;
            reference_id = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.asset = asset;
        value.delta = delta;
        value.kind = kind;
        value.timestamp = timestamp;
        value.reference_id = reference_id;
        return value;
    }
};

/// SBE encoder for LedgerUpdate
pub const LedgerUpdateEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, asset: []const u8, delta: f64, kind: LedgerUpdateKind, timestamp: i64, reference_id: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 45);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 17);
        try writeString(buf, account);
        try writeString(buf, asset);
        try writeF64BE(buf, delta);
        try buf.append(LedgerUpdateKindToValue(kind));
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        if (reference_id) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LedgerUpdate: validates the header and reads the body.
pub const LedgerUpdateDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !LedgerUpdate {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 45) return error.InvalidTemplateId;
        var value: LedgerUpdate = undefined;
        var reference_id: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const asset = try readString(buf, &pos, allocator);
        const delta = readF64BE(buf, &pos);
        const kind = try LedgerUpdateKindFromValue(readU8(buf, &pos));
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        if (buf[pos] == 1) {
            pos += 1;
            reference_id = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.asset = asset;
        value.delta = delta;
        value.kind = kind;
        value.timestamp = timestamp;
        value.reference_id = reference_id;
        return value;
    }
};

/// SBE message LedgerHistoryRequest
pub const LedgerHistoryRequest = struct {
    account: []const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
};

/// SBE body encoder for LedgerHistoryRequest (fields only, no header).
pub const LedgerHistoryRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: LedgerHistoryRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        try writeU64BE(buf, @as(u64, @bitCast(value.start_time orelse 0)));
        try buf.append(if (value.start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.end_time orelse 0)));
        try buf.append(if (value.end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(value.limit orelse 0));
        try buf.append(if (value.limit != null) 1 else 0);
        if (value.cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for LedgerHistoryRequest: reads fields at `pos`.
pub const LedgerHistoryRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !LedgerHistoryRequest {
        var value: LedgerHistoryRequest = undefined;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE encoder for LedgerHistoryRequest
pub const LedgerHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, start_time: ?i64, end_time: ?i64, limit: ?u32, cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 46);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 20);
        try writeString(buf, account);
        try writeU64BE(buf, @as(u64, @bitCast(start_time orelse 0)));
        try buf.append(if (start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(end_time orelse 0)));
        try buf.append(if (end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(limit orelse 0));
        try buf.append(if (limit != null) 1 else 0);
        if (cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LedgerHistoryRequest: validates the header and reads the body.
pub const LedgerHistoryRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !LedgerHistoryRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 46) return error.InvalidTemplateId;
        var value: LedgerHistoryRequest = undefined;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE message LedgerHistoryBatch
pub const LedgerHistoryBatch = struct {
    account: []const u8,
    entries: []LedgerUpdate,
    has_more: u8,
    next_cursor: ?[]const u8,
};

/// SBE body encoder for LedgerHistoryBatch (fields only, no header).
pub const LedgerHistoryBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: LedgerHistoryBatch, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try writeU32BE(buf, @intCast(value.entries.len));
        for (value.entries) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try LedgerUpdateBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for LedgerHistoryBatch: reads fields at `pos`.
pub const LedgerHistoryBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !LedgerHistoryBatch {
        var value: LedgerHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const entries_count = readU32BE(buf, &pos);
        var entries = try allocator.alloc(LedgerUpdate, entries_count);
        var entries_idx: usize = 0;
        while (entries_idx < entries_count) : (entries_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try LedgerUpdateBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            entries[entries_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.entries = entries;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE encoder for LedgerHistoryBatch
pub const LedgerHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, entries: []const LedgerUpdate, has_more: u8, next_cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 47);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeU32BE(buf, @intCast(entries.len));
        for (entries) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try LedgerUpdateBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(has_more));
        if (next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for LedgerHistoryBatch: validates the header and reads the body.
pub const LedgerHistoryBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !LedgerHistoryBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 47) return error.InvalidTemplateId;
        var value: LedgerHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const entries_count = readU32BE(buf, &pos);
        var entries = try allocator.alloc(LedgerUpdate, entries_count);
        var entries_idx: usize = 0;
        while (entries_idx < entries_count) : (entries_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try LedgerUpdateBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            entries[entries_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.entries = entries;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE message OpenOrdersRequest
pub const OpenOrdersRequest = struct {
    account: []const u8,
    symbol: ?[]const u8,
};

/// SBE body encoder for OpenOrdersRequest (fields only, no header).
pub const OpenOrdersRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OpenOrdersRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for OpenOrdersRequest: reads fields at `pos`.
pub const OpenOrdersRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OpenOrdersRequest {
        var value: OpenOrdersRequest = undefined;
        var symbol: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for OpenOrdersRequest
pub const OpenOrdersRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, symbol: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 48);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeString(buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OpenOrdersRequest: validates the header and reads the body.
pub const OpenOrdersRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OpenOrdersRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 48) return error.InvalidTemplateId;
        var value: OpenOrdersRequest = undefined;
        var symbol: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.symbol = symbol;
        return value;
    }
};

/// SBE message OpenOrdersSnapshot
pub const OpenOrdersSnapshot = struct {
    account: []const u8,
    orders: []ExecutionReport,
    is_snapshot: ?u8,
};

/// SBE body encoder for OpenOrdersSnapshot (fields only, no header).
pub const OpenOrdersSnapshotBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OpenOrdersSnapshot, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try writeU32BE(buf, @intCast(value.orders.len));
        for (value.orders) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try ExecutionReportBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for OpenOrdersSnapshot: reads fields at `pos`.
pub const OpenOrdersSnapshotBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OpenOrdersSnapshot {
        var value: OpenOrdersSnapshot = undefined;
        const account = try readString(buf, &pos, allocator);
        const orders_count = readU32BE(buf, &pos);
        var orders = try allocator.alloc(ExecutionReport, orders_count);
        var orders_idx: usize = 0;
        while (orders_idx < orders_count) : (orders_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try ExecutionReportBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            orders[orders_idx] = item;
        }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.orders = orders;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for OpenOrdersSnapshot
pub const OpenOrdersSnapshotEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, orders: []const ExecutionReport, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 49);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeU32BE(buf, @intCast(orders.len));
        for (orders) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try ExecutionReportBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OpenOrdersSnapshot: validates the header and reads the body.
pub const OpenOrdersSnapshotDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OpenOrdersSnapshot {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 49) return error.InvalidTemplateId;
        var value: OpenOrdersSnapshot = undefined;
        const account = try readString(buf, &pos, allocator);
        const orders_count = readU32BE(buf, &pos);
        var orders = try allocator.alloc(ExecutionReport, orders_count);
        var orders_idx: usize = 0;
        while (orders_idx < orders_count) : (orders_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try ExecutionReportBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            orders[orders_idx] = item;
        }
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.account = account;
        value.orders = orders;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message OrderHistoryRequest
pub const OrderHistoryRequest = struct {
    account: []const u8,
    symbol: ?[]const u8,
    start_time: ?i64,
    end_time: ?i64,
    limit: ?u32,
    cursor: ?[]const u8,
};

/// SBE body encoder for OrderHistoryRequest (fields only, no header).
pub const OrderHistoryRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OrderHistoryRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeU64BE(buf, @as(u64, @bitCast(value.start_time orelse 0)));
        try buf.append(if (value.start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.end_time orelse 0)));
        try buf.append(if (value.end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(value.limit orelse 0));
        try buf.append(if (value.limit != null) 1 else 0);
        if (value.cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for OrderHistoryRequest: reads fields at `pos`.
pub const OrderHistoryRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OrderHistoryRequest {
        var value: OrderHistoryRequest = undefined;
        var symbol: ?[]u8 = null;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE encoder for OrderHistoryRequest
pub const OrderHistoryRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, symbol: ?[]const u8, start_time: ?i64, end_time: ?i64, limit: ?u32, cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 50);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 20);
        try writeString(buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeU64BE(buf, @as(u64, @bitCast(start_time orelse 0)));
        try buf.append(if (start_time != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(end_time orelse 0)));
        try buf.append(if (end_time != null) 1 else 0);
        try writeU32BE(buf, @intCast(limit orelse 0));
        try buf.append(if (limit != null) 1 else 0);
        if (cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderHistoryRequest: validates the header and reads the body.
pub const OrderHistoryRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OrderHistoryRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 50) return error.InvalidTemplateId;
        var value: OrderHistoryRequest = undefined;
        var symbol: ?[]u8 = null;
        var cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const start_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var start_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; start_time = start_time_v; } else { pos += 1; }
        const end_time_v = @as(i64, @bitCast(readU64BE(buf, &pos)));
        var end_time: ?i64 = null;
        if (buf[pos] == 1) { pos += 1; end_time = end_time_v; } else { pos += 1; }
        const limit_v = readU32BE(buf, &pos);
        var limit: ?u32 = null;
        if (buf[pos] == 1) { pos += 1; limit = limit_v; } else { pos += 1; }
        if (buf[pos] == 1) {
            pos += 1;
            cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.symbol = symbol;
        value.start_time = start_time;
        value.end_time = end_time;
        value.limit = limit;
        value.cursor = cursor;
        return value;
    }
};

/// SBE message OrderHistoryBatch
pub const OrderHistoryBatch = struct {
    account: []const u8,
    orders: []ExecutionReport,
    has_more: u8,
    next_cursor: ?[]const u8,
};

/// SBE body encoder for OrderHistoryBatch (fields only, no header).
pub const OrderHistoryBatchBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: OrderHistoryBatch, buf: *std.ArrayList(u8)) !void {
        try writeString(buf, value.account);
        try writeU32BE(buf, @intCast(value.orders.len));
        for (value.orders) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try ExecutionReportBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(value.has_more));
        if (value.next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for OrderHistoryBatch: reads fields at `pos`.
pub const OrderHistoryBatchBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !OrderHistoryBatch {
        var value: OrderHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const orders_count = readU32BE(buf, &pos);
        var orders = try allocator.alloc(ExecutionReport, orders_count);
        var orders_idx: usize = 0;
        while (orders_idx < orders_count) : (orders_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try ExecutionReportBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            orders[orders_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.orders = orders;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE encoder for OrderHistoryBatch
pub const OrderHistoryBatchEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8, orders: []const ExecutionReport, has_more: u8, next_cursor: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 51);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, account);
        try writeU32BE(buf, @intCast(orders.len));
        for (orders) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try ExecutionReportBodyEncoder.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try buf.append(@intCast(has_more));
        if (next_cursor) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for OrderHistoryBatch: validates the header and reads the body.
pub const OrderHistoryBatchDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !OrderHistoryBatch {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 51) return error.InvalidTemplateId;
        var value: OrderHistoryBatch = undefined;
        var next_cursor: ?[]u8 = null;
        const account = try readString(buf, &pos, allocator);
        const orders_count = readU32BE(buf, &pos);
        var orders = try allocator.alloc(ExecutionReport, orders_count);
        var orders_idx: usize = 0;
        while (orders_idx < orders_count) : (orders_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try ExecutionReportBodyDecoder.decode(allocator, buf, &pos);
            pos = item_end;
            orders[orders_idx] = item;
        }
        const has_more = readU8(buf, &pos);
        if (buf[pos] == 1) {
            pos += 1;
            next_cursor = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.account = account;
        value.orders = orders;
        value.has_more = has_more;
        value.next_cursor = next_cursor;
        return value;
    }
};

/// SBE message CapabilitiesRequest
pub const CapabilitiesRequest = struct {
};

/// SBE body encoder for CapabilitiesRequest (fields only, no header).
pub const CapabilitiesRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CapabilitiesRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        _ = value;
        _ = buf;
    }
};

/// SBE body decoder for CapabilitiesRequest: reads fields at `pos`.
pub const CapabilitiesRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CapabilitiesRequest {
        _ = allocator;
        _ = buf;
        _ = pos;
        const value: CapabilitiesRequest = undefined;
        return value;
    }
};

/// SBE encoder for CapabilitiesRequest
pub const CapabilitiesRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, ) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 52);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CapabilitiesRequest: validates the header and reads the body.
pub const CapabilitiesRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CapabilitiesRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 52) return error.InvalidTemplateId;
        _ = allocator;
        const value: CapabilitiesRequest = undefined;
        return value;
    }
};

/// SBE message CapabilitiesResponse
pub const CapabilitiesResponse = struct {
    schema_ids: []u8,
    paths: []CapabilityPath,
    symbols: [][]const u8,
    intervals: [][]const u8,
};

/// SBE body encoder for CapabilitiesResponse (fields only, no header).
pub const CapabilitiesResponseBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: CapabilitiesResponse, buf: *std.ArrayList(u8)) !void {
        try writeU32BE(buf, @intCast(value.schema_ids.len));
        for (value.schema_ids) |item| {
        try buf.append(@intCast(item));
        }
        try writeU32BE(buf, @intCast(value.paths.len));
        for (value.paths) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try CapabilityPath.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU32BE(buf, @intCast(value.symbols.len));
        for (value.symbols) |item| {
            try writeStringU32(buf, item);
        }
        try writeU32BE(buf, @intCast(value.intervals.len));
        for (value.intervals) |item| {
            try writeStringU32(buf, item);
        }
    }
};

/// SBE body decoder for CapabilitiesResponse: reads fields at `pos`.
pub const CapabilitiesResponseBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !CapabilitiesResponse {
        var value: CapabilitiesResponse = undefined;
        const schema_ids_count = readU32BE(buf, &pos);
        var schema_ids = try allocator.alloc(u8, schema_ids_count);
        var schema_ids_idx: usize = 0;
        while (schema_ids_idx < schema_ids_count) : (schema_ids_idx += 1) {
            schema_ids[schema_ids_idx] = readU8(buf, &pos);
        }
        const paths_count = readU32BE(buf, &pos);
        var paths = try allocator.alloc(CapabilityPath, paths_count);
        var paths_idx: usize = 0;
        while (paths_idx < paths_count) : (paths_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try CapabilityPath.decode(allocator, buf, &pos);
            pos = item_end;
            paths[paths_idx] = item;
        }
        const symbols_count = readU32BE(buf, &pos);
        var symbols = try allocator.alloc([]const u8, symbols_count);
        var symbols_idx: usize = 0;
        while (symbols_idx < symbols_count) : (symbols_idx += 1) {
            symbols[symbols_idx] = try readStringU32(buf, &pos, allocator);
        }
        const intervals_count = readU32BE(buf, &pos);
        var intervals = try allocator.alloc([]const u8, intervals_count);
        var intervals_idx: usize = 0;
        while (intervals_idx < intervals_count) : (intervals_idx += 1) {
            intervals[intervals_idx] = try readStringU32(buf, &pos, allocator);
        }
        value.schema_ids = schema_ids;
        value.paths = paths;
        value.symbols = symbols;
        value.intervals = intervals;
        return value;
    }
};

/// SBE encoder for CapabilitiesResponse
pub const CapabilitiesResponseEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, schema_ids: []const u8, paths: []const CapabilityPath, symbols: []const []const u8, intervals: []const []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 53);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeU32BE(buf, @intCast(schema_ids.len));
        for (schema_ids) |item| {
        try buf.append(@intCast(item));
        }
        try writeU32BE(buf, @intCast(paths.len));
        for (paths) |item| {
            var item_buf = std.ArrayList(u8).init(allocator);
            defer item_buf.deinit();
            try CapabilityPath.encode(allocator, item, &item_buf);
            try writeU32BE(buf, @intCast(item_buf.items.len));
            try buf.appendSlice(item_buf.items);
        }
        try writeU32BE(buf, @intCast(symbols.len));
        for (symbols) |item| {
            try writeStringU32(buf, item);
        }
        try writeU32BE(buf, @intCast(intervals.len));
        for (intervals) |item| {
            try writeStringU32(buf, item);
        }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for CapabilitiesResponse: validates the header and reads the body.
pub const CapabilitiesResponseDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !CapabilitiesResponse {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 53) return error.InvalidTemplateId;
        var value: CapabilitiesResponse = undefined;
        const schema_ids_count = readU32BE(buf, &pos);
        var schema_ids = try allocator.alloc(u8, schema_ids_count);
        var schema_ids_idx: usize = 0;
        while (schema_ids_idx < schema_ids_count) : (schema_ids_idx += 1) {
            schema_ids[schema_ids_idx] = readU8(buf, &pos);
        }
        const paths_count = readU32BE(buf, &pos);
        var paths = try allocator.alloc(CapabilityPath, paths_count);
        var paths_idx: usize = 0;
        while (paths_idx < paths_count) : (paths_idx += 1) {
            const item_len = readU32BE(buf, &pos);
            const item_end = pos + item_len;
            const item = try CapabilityPath.decode(allocator, buf, &pos);
            pos = item_end;
            paths[paths_idx] = item;
        }
        const symbols_count = readU32BE(buf, &pos);
        var symbols = try allocator.alloc([]const u8, symbols_count);
        var symbols_idx: usize = 0;
        while (symbols_idx < symbols_count) : (symbols_idx += 1) {
            symbols[symbols_idx] = try readStringU32(buf, &pos, allocator);
        }
        const intervals_count = readU32BE(buf, &pos);
        var intervals = try allocator.alloc([]const u8, intervals_count);
        var intervals_idx: usize = 0;
        while (intervals_idx < intervals_count) : (intervals_idx += 1) {
            intervals[intervals_idx] = try readStringU32(buf, &pos, allocator);
        }
        value.schema_ids = schema_ids;
        value.paths = paths;
        value.symbols = symbols;
        value.intervals = intervals;
        return value;
    }
};

/// SBE message PositionRequest
pub const PositionRequest = struct {
    account: []const u8,
};

/// SBE body encoder for PositionRequest (fields only, no header).
pub const PositionRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: PositionRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.account);
    }
};

/// SBE body decoder for PositionRequest: reads fields at `pos`.
pub const PositionRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PositionRequest {
        var value: PositionRequest = undefined;
        const account = try readString(buf, &pos, allocator);
        value.account = account;
        return value;
    }
};

/// SBE encoder for PositionRequest
pub const PositionRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, account: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 54);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeString(buf, account);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PositionRequest: validates the header and reads the body.
pub const PositionRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !PositionRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 54) return error.InvalidTemplateId;
        var value: PositionRequest = undefined;
        const account = try readString(buf, &pos, allocator);
        value.account = account;
        return value;
    }
};

/// SBE message TradingSessionStatusRequest
pub const TradingSessionStatusRequest = struct {
    symbol: ?[]const u8,
};

/// SBE body encoder for TradingSessionStatusRequest (fields only, no header).
pub const TradingSessionStatusRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: TradingSessionStatusRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
    }
};

/// SBE body decoder for TradingSessionStatusRequest: reads fields at `pos`.
pub const TradingSessionStatusRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !TradingSessionStatusRequest {
        var value: TradingSessionStatusRequest = undefined;
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for TradingSessionStatusRequest
pub const TradingSessionStatusRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: ?[]const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 55);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for TradingSessionStatusRequest: validates the header and reads the body.
pub const TradingSessionStatusRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !TradingSessionStatusRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 55) return error.InvalidTemplateId;
        var value: TradingSessionStatusRequest = undefined;
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        value.symbol = symbol;
        return value;
    }
};

/// SBE message TradingSessionStatus
pub const TradingSessionStatus = struct {
    symbol: ?[]const u8,
    trading_session_id: []const u8,
    status: TradingSessionStatusStatus,
    halt_reason: ?TradingSessionStatusHaltReason,
    timestamp: i64,
    is_snapshot: ?u8,
};

/// SBE body encoder for TradingSessionStatus (fields only, no header).
pub const TradingSessionStatusBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: TradingSessionStatus, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeString(buf, value.trading_session_id);
        try buf.append(TradingSessionStatusStatusToValue(value.status));
        try buf.append(if (value.halt_reason) |e| TradingSessionStatusHaltReasonToValue(e) else 0);
        try buf.append(if (value.halt_reason != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for TradingSessionStatus: reads fields at `pos`.
pub const TradingSessionStatusBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !TradingSessionStatus {
        var value: TradingSessionStatus = undefined;
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const trading_session_id = try readString(buf, &pos, allocator);
        const status = try TradingSessionStatusStatusFromValue(readU8(buf, &pos));
        const halt_reason_raw = readU8(buf, &pos);
        var halt_reason: ?TradingSessionStatusHaltReason = null;
        if (buf[pos] == 1) {
            pos += 1;
            halt_reason = try TradingSessionStatusHaltReasonFromValue(halt_reason_raw);
        } else {
            pos += 1;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.trading_session_id = trading_session_id;
        value.status = status;
        value.halt_reason = halt_reason;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for TradingSessionStatus
pub const TradingSessionStatusEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: ?[]const u8, trading_session_id: []const u8, status: TradingSessionStatusStatus, halt_reason: ?TradingSessionStatusHaltReason, timestamp: i64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 56);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 11);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeString(buf, trading_session_id);
        try buf.append(TradingSessionStatusStatusToValue(status));
        try buf.append(if (halt_reason) |e| TradingSessionStatusHaltReasonToValue(e) else 0);
        try buf.append(if (halt_reason != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for TradingSessionStatus: validates the header and reads the body.
pub const TradingSessionStatusDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !TradingSessionStatus {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 56) return error.InvalidTemplateId;
        var value: TradingSessionStatus = undefined;
        var symbol: ?[]u8 = null;
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const trading_session_id = try readString(buf, &pos, allocator);
        const status = try TradingSessionStatusStatusFromValue(readU8(buf, &pos));
        const halt_reason_raw = readU8(buf, &pos);
        var halt_reason: ?TradingSessionStatusHaltReason = null;
        if (buf[pos] == 1) {
            pos += 1;
            halt_reason = try TradingSessionStatusHaltReasonFromValue(halt_reason_raw);
        } else {
            pos += 1;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.trading_session_id = trading_session_id;
        value.status = status;
        value.halt_reason = halt_reason;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message SecurityStatusRequest
pub const SecurityStatusRequest = struct {
    symbol: []const u8,
};

/// SBE body encoder for SecurityStatusRequest (fields only, no header).
pub const SecurityStatusRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: SecurityStatusRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
    }
};

/// SBE body decoder for SecurityStatusRequest: reads fields at `pos`.
pub const SecurityStatusRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !SecurityStatusRequest {
        var value: SecurityStatusRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for SecurityStatusRequest
pub const SecurityStatusRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 57);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeString(buf, symbol);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for SecurityStatusRequest: validates the header and reads the body.
pub const SecurityStatusRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !SecurityStatusRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 57) return error.InvalidTemplateId;
        var value: SecurityStatusRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE message SecurityStatus
pub const SecurityStatus = struct {
    symbol: []const u8,
    status: SecurityStatusStatus,
    halt_reason: ?SecurityStatusHaltReason,
    timestamp: i64,
    is_snapshot: ?u8,
};

/// SBE body encoder for SecurityStatus (fields only, no header).
pub const SecurityStatusBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: SecurityStatus, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try buf.append(SecurityStatusStatusToValue(value.status));
        try buf.append(if (value.halt_reason) |e| SecurityStatusHaltReasonToValue(e) else 0);
        try buf.append(if (value.halt_reason != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for SecurityStatus: reads fields at `pos`.
pub const SecurityStatusBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !SecurityStatus {
        var value: SecurityStatus = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const status = try SecurityStatusStatusFromValue(readU8(buf, &pos));
        const halt_reason_raw = readU8(buf, &pos);
        var halt_reason: ?SecurityStatusHaltReason = null;
        if (buf[pos] == 1) {
            pos += 1;
            halt_reason = try SecurityStatusHaltReasonFromValue(halt_reason_raw);
        } else {
            pos += 1;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.status = status;
        value.halt_reason = halt_reason;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for SecurityStatus
pub const SecurityStatusEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, status: SecurityStatusStatus, halt_reason: ?SecurityStatusHaltReason, timestamp: i64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 58);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 11);
        try writeString(buf, symbol);
        try buf.append(SecurityStatusStatusToValue(status));
        try buf.append(if (halt_reason) |e| SecurityStatusHaltReasonToValue(e) else 0);
        try buf.append(if (halt_reason != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for SecurityStatus: validates the header and reads the body.
pub const SecurityStatusDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !SecurityStatus {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 58) return error.InvalidTemplateId;
        var value: SecurityStatus = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const status = try SecurityStatusStatusFromValue(readU8(buf, &pos));
        const halt_reason_raw = readU8(buf, &pos);
        var halt_reason: ?SecurityStatusHaltReason = null;
        if (buf[pos] == 1) {
            pos += 1;
            halt_reason = try SecurityStatusHaltReasonFromValue(halt_reason_raw);
        } else {
            pos += 1;
        }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.status = status;
        value.halt_reason = halt_reason;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE message MassCancelRequest
pub const MassCancelRequest = struct {
    cl_ord_id: []const u8,
    account: []const u8,
    symbol: ?[]const u8,
    side: ?MassCancelRequestSide,
};

/// SBE body encoder for MassCancelRequest (fields only, no header).
pub const MassCancelRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MassCancelRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.cl_ord_id);
        try writeString(buf, value.account);
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try buf.append(if (value.side) |e| MassCancelRequestSideToValue(e) else 0);
        try buf.append(if (value.side != null) 1 else 0);
    }
};

/// SBE body decoder for MassCancelRequest: reads fields at `pos`.
pub const MassCancelRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MassCancelRequest {
        var value: MassCancelRequest = undefined;
        var symbol: ?[]u8 = null;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const side_raw = readU8(buf, &pos);
        var side: ?MassCancelRequestSide = null;
        if (buf[pos] == 1) {
            pos += 1;
            side = try MassCancelRequestSideFromValue(side_raw);
        } else {
            pos += 1;
        }
        value.cl_ord_id = cl_ord_id;
        value.account = account;
        value.symbol = symbol;
        value.side = side;
        return value;
    }
};

/// SBE encoder for MassCancelRequest
pub const MassCancelRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id: []const u8, account: []const u8, symbol: ?[]const u8, side: ?MassCancelRequestSide) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 59);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 1);
        try writeString(buf, cl_ord_id);
        try writeString(buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try buf.append(if (side) |e| MassCancelRequestSideToValue(e) else 0);
        try buf.append(if (side != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MassCancelRequest: validates the header and reads the body.
pub const MassCancelRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MassCancelRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 59) return error.InvalidTemplateId;
        var value: MassCancelRequest = undefined;
        var symbol: ?[]u8 = null;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const side_raw = readU8(buf, &pos);
        var side: ?MassCancelRequestSide = null;
        if (buf[pos] == 1) {
            pos += 1;
            side = try MassCancelRequestSideFromValue(side_raw);
        } else {
            pos += 1;
        }
        value.cl_ord_id = cl_ord_id;
        value.account = account;
        value.symbol = symbol;
        value.side = side;
        return value;
    }
};

/// SBE message MassCancelReport
pub const MassCancelReport = struct {
    cl_ord_id: []const u8,
    account: []const u8,
    symbol: ?[]const u8,
    total_affected: u32,
    status: MassCancelReportStatus,
    timestamp: i64,
};

/// SBE body encoder for MassCancelReport (fields only, no header).
pub const MassCancelReportBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: MassCancelReport, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.cl_ord_id);
        try writeString(buf, value.account);
        if (value.symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeU32BE(buf, @intCast(value.total_affected));
        try buf.append(MassCancelReportStatusToValue(value.status));
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
    }
};

/// SBE body decoder for MassCancelReport: reads fields at `pos`.
pub const MassCancelReportBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !MassCancelReport {
        var value: MassCancelReport = undefined;
        var symbol: ?[]u8 = null;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const total_affected = readU32BE(buf, &pos);
        const status = try MassCancelReportStatusFromValue(readU8(buf, &pos));
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.cl_ord_id = cl_ord_id;
        value.account = account;
        value.symbol = symbol;
        value.total_affected = total_affected;
        value.status = status;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE encoder for MassCancelReport
pub const MassCancelReportEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, cl_ord_id: []const u8, account: []const u8, symbol: ?[]const u8, total_affected: u32, status: MassCancelReportStatus, timestamp: i64) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 60);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 13);
        try writeString(buf, cl_ord_id);
        try writeString(buf, account);
        if (symbol) |s| { try buf.append(1); try writeString(buf, s); } else { try buf.append(0); }
        try writeU32BE(buf, @intCast(total_affected));
        try buf.append(MassCancelReportStatusToValue(status));
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for MassCancelReport: validates the header and reads the body.
pub const MassCancelReportDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !MassCancelReport {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 60) return error.InvalidTemplateId;
        var value: MassCancelReport = undefined;
        var symbol: ?[]u8 = null;
        const cl_ord_id = try readString(buf, &pos, allocator);
        const account = try readString(buf, &pos, allocator);
        if (buf[pos] == 1) {
            pos += 1;
            symbol = try readString(buf, &pos, allocator);
        } else {
            pos += 1;
        }
        const total_affected = readU32BE(buf, &pos);
        const status = try MassCancelReportStatusFromValue(readU8(buf, &pos));
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        value.cl_ord_id = cl_ord_id;
        value.account = account;
        value.symbol = symbol;
        value.total_affected = total_affected;
        value.status = status;
        value.timestamp = timestamp;
        return value;
    }
};

/// SBE message PriceLimitsRequest
pub const PriceLimitsRequest = struct {
    symbol: []const u8,
};

/// SBE body encoder for PriceLimitsRequest (fields only, no header).
pub const PriceLimitsRequestBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: PriceLimitsRequest, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
    }
};

/// SBE body decoder for PriceLimitsRequest: reads fields at `pos`.
pub const PriceLimitsRequestBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PriceLimitsRequest {
        var value: PriceLimitsRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE encoder for PriceLimitsRequest
pub const PriceLimitsRequestEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 61);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 0);
        try writeString(buf, symbol);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PriceLimitsRequest: validates the header and reads the body.
pub const PriceLimitsRequestDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !PriceLimitsRequest {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 61) return error.InvalidTemplateId;
        var value: PriceLimitsRequest = undefined;
        const symbol = try readString(buf, &pos, allocator);
        value.symbol = symbol;
        return value;
    }
};

/// SBE message PriceLimits
pub const PriceLimits = struct {
    symbol: []const u8,
    lower_limit: ?f64,
    upper_limit: ?f64,
    reference_price: ?f64,
    timestamp: i64,
    is_snapshot: ?u8,
};

/// SBE body encoder for PriceLimits (fields only, no header).
pub const PriceLimitsBodyEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, value: PriceLimits, buf: *std.ArrayList(u8)) !void {
        _ = allocator;
        try writeString(buf, value.symbol);
        try writeF64BE(buf, value.lower_limit orelse 0);
        try buf.append(if (value.lower_limit != null) 1 else 0);
        try writeF64BE(buf, value.upper_limit orelse 0);
        try buf.append(if (value.upper_limit != null) 1 else 0);
        try writeF64BE(buf, value.reference_price orelse 0);
        try buf.append(if (value.reference_price != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(value.timestamp)));
        try buf.append(@intCast(value.is_snapshot orelse 0));
        try buf.append(if (value.is_snapshot != null) 1 else 0);
    }
};

/// SBE body decoder for PriceLimits: reads fields at `pos`.
pub const PriceLimitsBodyDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8, pos: *usize) !PriceLimits {
        var value: PriceLimits = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const lower_limit_v = readF64BE(buf, &pos);
        var lower_limit: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; lower_limit = lower_limit_v; } else { pos += 1; }
        const upper_limit_v = readF64BE(buf, &pos);
        var upper_limit: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; upper_limit = upper_limit_v; } else { pos += 1; }
        const reference_price_v = readF64BE(buf, &pos);
        var reference_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; reference_price = reference_price_v; } else { pos += 1; }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.lower_limit = lower_limit;
        value.upper_limit = upper_limit;
        value.reference_price = reference_price;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

/// SBE encoder for PriceLimits
pub const PriceLimitsEncoder = struct {
    pub fn encode(allocator: std.mem.Allocator, symbol: []const u8, lower_limit: ?f64, upper_limit: ?f64, reference_price: ?f64, timestamp: i64, is_snapshot: ?u8) ![]u8 {
        var list = std.ArrayList(u8).init(allocator);
        defer list.deinit();
        const buf = &list;
        try writeU16BE(buf, SCHEMA_ID);
        try writeU16BE(buf, 62);
        try writeU16BE(buf, 0);
        try writeU16BE(buf, 33);
        try writeString(buf, symbol);
        try writeF64BE(buf, lower_limit orelse 0);
        try buf.append(if (lower_limit != null) 1 else 0);
        try writeF64BE(buf, upper_limit orelse 0);
        try buf.append(if (upper_limit != null) 1 else 0);
        try writeF64BE(buf, reference_price orelse 0);
        try buf.append(if (reference_price != null) 1 else 0);
        try writeU64BE(buf, @as(u64, @bitCast(timestamp)));
        try buf.append(@intCast(is_snapshot orelse 0));
        try buf.append(if (is_snapshot != null) 1 else 0);
        return buf.toOwnedSlice();
    }
};

/// SBE decoder for PriceLimits: validates the header and reads the body.
pub const PriceLimitsDecoder = struct {
    pub fn decode(allocator: std.mem.Allocator, buf: []const u8) !PriceLimits {
        if (buf.len < 8) return error.BufferTooShort;
        var pos: usize = 0;
        const schema_id = readU16BE(buf, &pos);
        const tmpl_id = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        _ = readU16BE(buf, &pos);
        if (schema_id != SCHEMA_ID) return error.InvalidSchemaId;
        if (tmpl_id != 62) return error.InvalidTemplateId;
        var value: PriceLimits = undefined;
        const symbol = try readString(buf, &pos, allocator);
        const lower_limit_v = readF64BE(buf, &pos);
        var lower_limit: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; lower_limit = lower_limit_v; } else { pos += 1; }
        const upper_limit_v = readF64BE(buf, &pos);
        var upper_limit: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; upper_limit = upper_limit_v; } else { pos += 1; }
        const reference_price_v = readF64BE(buf, &pos);
        var reference_price: ?f64 = null;
        if (buf[pos] == 1) { pos += 1; reference_price = reference_price_v; } else { pos += 1; }
        const timestamp = @as(i64, @bitCast(readU64BE(buf, &pos)));
        const is_snapshot_v = readU8(buf, &pos);
        var is_snapshot: ?u8 = null;
        if (buf[pos] == 1) { pos += 1; is_snapshot = is_snapshot_v; } else { pos += 1; }
        value.symbol = symbol;
        value.lower_limit = lower_limit;
        value.upper_limit = upper_limit;
        value.reference_price = reference_price;
        value.timestamp = timestamp;
        value.is_snapshot = is_snapshot;
        return value;
    }
};

