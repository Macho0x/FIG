// Auto-generated pure FIG protocol library (no Rust runtime).
// Regenerate: cargo xtask codegen  OR  ftlc compile --lang protocol-zig

const std = @import("std");

pub const HEADER_SIZE: usize = 16;
pub const CONTROL_CHANNEL: u16 = 0;

pub const ExtensionTag = enum(u16) {
    RequestUri = 0x0001,
    ResponseUri = 0x0002,
    ContentType = 0x0003,
    StatusCode = 0x0004,
    CorrelationId = 0x0005,
    SequenceNum = 0x0006,
    SessionId = 0x0007,
    Timestamp = 0x0008,
    TtlMillis = 0x0009,
    RoutingKey = 0x000A,
    SchemaFingerprint = 0x000B,
    ContentEncoding = 0x000C,
    Method = 0x000D,
    UserAgent = 0x000E,
    AuthToken = 0x000F,
    Accept = 0x0010,
    CacheControl = 0x0011,
    IdempotencyKey = 0x0012,
    TraceId = 0x0013,
    ErrorCode = 0x0014,
    ErrorMessage = 0x0015,
    ChannelMode = 0x0016,
    ChannelPath = 0x0017,
    RedirectTarget = 0x0018,
    RedirectToken = 0x0019,
    AckRangeStart = 0x001A,
    AckRangeEnd = 0x001B,
    FlowControlCredit = 0x001C,
    Scope = 0x001D,
    AuthMethod = 0x001E,
};


pub const FrameType = enum(u8) {
    control = 0x00,
    request = 0x01,
    response = 0x02,
    stream_open = 0x03,
    stream_item = 0x04,
    stream_close = 0x05,
    stream_error = 0x06,
    one_way = 0x07,
    subscribe = 0x08,
    unsubscribe = 0x09,
    ack_range = 0x0A,
    flow_control = 0x0B,
    redirect = 0x0C,
};

pub const ControlSubtype = enum(u8) {
    ping = 0x00,
    pong = 0x01,
    goaway = 0x02,
    settings = 0x03,
    auth_refresh = 0x04,
    seq_reset = 0x05,
    resend = 0x06,
};

pub const CONTROL_PING: u8 = @intFromEnum(ControlSubtype.ping);
pub const CONTROL_PONG: u8 = @intFromEnum(ControlSubtype.pong);

pub const FrameHeader = struct {
    length: u32 = HEADER_SIZE,
    frame_type: FrameType = .control,
    flags: u8 = 0,
    channel_id: u16 = 0,
    stream_seq: u32 = 0,
    header_count: u8 = 0,
    schema_id: u8 = 0,
};

pub const TextExtension = struct {
    tag: ExtensionTag,
    value: []const u8,
};

pub fn channelStreamId(channel_id: u16, is_server: bool) u64 {
    const offset: u64 = if (is_server) 1 else 0;
    return @as(u64, channel_id) * 4 + offset;
}

fn writeBe16(out: *std.ArrayList(u8), v: u16) !void {
    try out.append(@intCast((v >> 8) & 0xFF));
    try out.append(@intCast(v & 0xFF));
}

fn writeBe32(out: *std.ArrayList(u8), v: u32) !void {
    try out.append(@intCast((v >> 24) & 0xFF));
    try out.append(@intCast((v >> 16) & 0xFF));
    try out.append(@intCast((v >> 8) & 0xFF));
    try out.append(@intCast(v & 0xFF));
}

fn readBe16(data: []const u8) u16 {
    return (@as(u16, data[0]) << 8) | data[1];
}

fn readBe32(data: []const u8) u32 {
    return (@as(u32, data[0]) << 24) | (@as(u32, data[1]) << 16) | (@as(u32, data[2]) << 8) | data[3];
}

pub fn encodeTextExtensions(allocator: std.mem.Allocator, extensions: []const TextExtension) ![]u8 {
    var out = std.ArrayList(u8).init(allocator);
    errdefer out.deinit();
    for (extensions) |ext| {
        try writeBe16(&out, @intFromEnum(ext.tag));
        try writeBe16(&out, @intCast(ext.value.len));
        try out.appendSlice(ext.value);
    }
    return out.toOwnedSlice();
}

pub fn decodeTextExtensions(allocator: std.mem.Allocator, data: []const u8, count: u8) ![]TextExtension {
    var out = std.ArrayList(TextExtension).init(allocator);
    errdefer out.deinit();
    var offset: usize = 0;
    var i: u8 = 0;
    while (i < count) : (i += 1) {
        if (offset + 4 > data.len) return error.ExtensionBlockTooShort;
        const tag: ExtensionTag = @enumFromInt(readBe16(data[offset .. offset + 2]));
        offset += 2;
        const value_len = readBe16(data[offset .. offset + 2]);
        offset += 2;
        if (offset + value_len > data.len) return error.ExtensionValueTruncated;
        const value = try allocator.dupe(u8, data[offset .. offset + value_len]);
        try out.append(.{ .tag = tag, .value = value });
        offset += value_len;
    }
    return out.toOwnedSlice();
}

pub fn encodeFrameHeader(allocator: std.mem.Allocator, hdr: FrameHeader, ext_bytes: []const u8, payload_len: usize) ![]u8 {
    const total: u32 = @intCast(HEADER_SIZE + ext_bytes.len + payload_len);
    var out = std.ArrayList(u8).init(allocator);
    errdefer out.deinit();
    try writeBe32(&out, total);
    try out.append(@intFromEnum(hdr.frame_type));
    const flags: u8 = if (hdr.header_count > 0) hdr.flags | 0x01 else hdr.flags;
    try out.append(flags);
    try writeBe16(&out, hdr.channel_id);
    try writeBe32(&out, hdr.stream_seq);
    try out.append(hdr.header_count);
    try out.append(hdr.schema_id);
    try writeBe16(&out, 0);
    try out.appendSlice(ext_bytes);
    return out.toOwnedSlice();
}

pub fn decodeFrameHeader(data: []const u8) !FrameHeader {
    if (data.len < HEADER_SIZE) return error.HeaderTooShort;
    const length = readBe32(data[0..4]);
    if (length < HEADER_SIZE or data.len < length) return error.InvalidFrameLength;
    const reserved = readBe16(data[14..16]);
    if (reserved != 0) return error.NonZeroReserved;
    return .{
        .length = length,
        .frame_type = @enumFromInt(data[4]),
        .flags = data[5],
        .channel_id = readBe16(data[6..8]),
        .stream_seq = readBe32(data[8..12]),
        .header_count = data[12],
        .schema_id = data[13],
    };
}

pub fn encodePing(allocator: std.mem.Allocator) ![]u8 {
    var out = std.ArrayList(u8).init(allocator);
    errdefer out.deinit();
    const header = try encodeFrameHeader(
        allocator,
        .{ .length = HEADER_SIZE + 1, .frame_type = .control, .channel_id = CONTROL_CHANNEL },
        &[_]u8{},
        1,
    );
    defer allocator.free(header);
    try out.appendSlice(header);
    try out.append(CONTROL_PING);
    return out.toOwnedSlice();
}

pub fn encodePong(allocator: std.mem.Allocator) ![]u8 {
    var out = std.ArrayList(u8).init(allocator);
    errdefer out.deinit();
    const header = try encodeFrameHeader(
        allocator,
        .{ .length = HEADER_SIZE + 1, .frame_type = .control, .channel_id = CONTROL_CHANNEL },
        &[_]u8{},
        1,
    );
    defer allocator.free(header);
    try out.appendSlice(header);
    try out.append(CONTROL_PONG);
    return out.toOwnedSlice();
}

pub const ChannelState = struct {
    last_sent_seq: u32 = 0,
    last_recv_seq: u32 = 0,
};

pub const ChannelManager = struct {
    is_server: bool,
    next_channel_id: u16,
    channels: std.AutoHashMap(u16, ChannelState),

    pub fn init(allocator: std.mem.Allocator, is_server: bool) ChannelManager {
        return .{
            .is_server = is_server,
            .next_channel_id = 1,
            .channels = std.AutoHashMap(u16, ChannelState).init(allocator),
        };
    }

    pub fn deinit(self: *ChannelManager) void {
        self.channels.deinit();
    }

    pub fn openChannel(self: *ChannelManager) !u16 {
        while (self.channels.contains(self.next_channel_id)) {
            self.next_channel_id = if (self.next_channel_id == 0xFFFF) 1 else self.next_channel_id + 1;
        }
        const id = self.next_channel_id;
        try self.channels.put(id, .{});
        self.next_channel_id = if (id == 0xFFFF) 1 else id + 1;
        return id;
    }

    pub fn nextSendSeq(self: *ChannelManager, channel_id: u16) !u32 {
        const entry = self.channels.getPtr(channel_id) orelse return error.ChannelNotFound;
        entry.last_sent_seq +%= 1;
        return entry.last_sent_seq;
    }

    pub fn recordRecvSeq(self: *ChannelManager, channel_id: u16, seq: u32) !void {
        const entry = self.channels.getPtr(channel_id) orelse return error.ChannelNotFound;
        entry.last_recv_seq = seq;
    }

    pub fn treeStreamId(self: *const ChannelManager, channel_id: u16) u64 {
        return channelStreamId(channel_id, self.is_server);
    }
};
