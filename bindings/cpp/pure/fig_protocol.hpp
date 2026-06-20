#pragma once
// Auto-generated pure FIG protocol library (no Rust runtime).
// Regenerate: cargo xtask codegen  OR  ftlc compile --lang protocol-cpp

#include <cstdint>
#include <cstring>
#include <optional>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

namespace fig::protocol {

inline constexpr std::size_t kHeaderSize = 16;
inline constexpr std::uint16_t kControlChannel = 0;

enum class ExtensionTag : std::uint16_t {
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


enum class FrameType : std::uint8_t {
    Control = 0x00,
    Request = 0x01,
    Response = 0x02,
    StreamOpen = 0x03,
    StreamItem = 0x04,
    StreamClose = 0x05,
    StreamError = 0x06,
    OneWay = 0x07,
    Subscribe = 0x08,
    Unsubscribe = 0x09,
    AckRange = 0x0A,
    FlowControl = 0x0B,
    Redirect = 0x0C,
};

enum class ControlSubtype : std::uint8_t {
    Ping = 0x00,
    Pong = 0x01,
    Goaway = 0x02,
    Settings = 0x03,
    AuthRefresh = 0x04,
    SeqReset = 0x05,
    Resend = 0x06,
};

inline constexpr std::uint8_t kControlPing = static_cast<std::uint8_t>(ControlSubtype::Ping);
inline constexpr std::uint8_t kControlPong = static_cast<std::uint8_t>(ControlSubtype::Pong);

struct FrameHeader {
    std::uint32_t length = kHeaderSize;
    FrameType frame_type = FrameType::Control;
    std::uint8_t flags = 0;
    std::uint16_t channel_id = 0;
    std::uint32_t stream_seq = 0;
    std::uint8_t header_count = 0;
    std::uint8_t schema_id = 0;
};

struct TextExtension {
    ExtensionTag tag;
    std::string value;
};

inline std::uint64_t channel_stream_id(std::uint16_t channel_id, bool is_server) {
    const std::uint64_t offset = is_server ? 1 : 0;
    return static_cast<std::uint64_t>(channel_id) * 4 + offset;
}

inline void write_be16(std::vector<std::uint8_t>& out, std::uint16_t v) {
    out.push_back(static_cast<std::uint8_t>((v >> 8) & 0xFF));
    out.push_back(static_cast<std::uint8_t>(v & 0xFF));
}

inline void write_be32(std::vector<std::uint8_t>& out, std::uint32_t v) {
    out.push_back(static_cast<std::uint8_t>((v >> 24) & 0xFF));
    out.push_back(static_cast<std::uint8_t>((v >> 16) & 0xFF));
    out.push_back(static_cast<std::uint8_t>((v >> 8) & 0xFF));
    out.push_back(static_cast<std::uint8_t>(v & 0xFF));
}

inline std::uint16_t read_be16(const std::uint8_t* p) {
    return static_cast<std::uint16_t>((static_cast<std::uint16_t>(p[0]) << 8) | p[1]);
}

inline std::uint32_t read_be32(const std::uint8_t* p) {
    return (static_cast<std::uint32_t>(p[0]) << 24) | (static_cast<std::uint32_t>(p[1]) << 16) |
           (static_cast<std::uint32_t>(p[2]) << 8) | static_cast<std::uint32_t>(p[3]);
}

inline std::vector<std::uint8_t> encode_text_extensions(const std::vector<TextExtension>& extensions) {
    std::vector<std::uint8_t> out;
    for (const auto& ext : extensions) {
        write_be16(out, static_cast<std::uint16_t>(ext.tag));
        const auto len = static_cast<std::uint16_t>(ext.value.size());
        write_be16(out, len);
        out.insert(out.end(), ext.value.begin(), ext.value.end());
    }
    return out;
}

inline std::vector<TextExtension> decode_text_extensions(const std::uint8_t* data,
                                                         std::size_t len,
                                                         std::uint8_t count) {
    std::vector<TextExtension> out;
    std::size_t offset = 0;
    for (std::uint8_t i = 0; i < count; ++i) {
        if (offset + 4 > len) {
            throw std::runtime_error("extension block too short");
        }
        const auto tag = static_cast<ExtensionTag>(read_be16(data + offset));
        offset += 2;
        const auto value_len = read_be16(data + offset);
        offset += 2;
        if (offset + value_len > len) {
            throw std::runtime_error("extension value truncated");
        }
        out.push_back(TextExtension{tag, std::string(reinterpret_cast<const char*>(data + offset), value_len)});
        offset += value_len;
    }
    return out;
}

inline std::vector<std::uint8_t> encode_frame_header(const FrameHeader& hdr,
                                                       const std::vector<std::uint8_t>& ext_bytes,
                                                       std::size_t payload_len) {
    const std::uint32_t total = static_cast<std::uint32_t>(kHeaderSize + ext_bytes.size() + payload_len);
    std::vector<std::uint8_t> out;
    out.reserve(total);
    write_be32(out, total);
    out.push_back(static_cast<std::uint8_t>(hdr.frame_type));
    const std::uint8_t flags = hdr.header_count > 0 ? static_cast<std::uint8_t>(hdr.flags | 0x01) : hdr.flags;
    out.push_back(flags);
    write_be16(out, hdr.channel_id);
    write_be32(out, hdr.stream_seq);
    out.push_back(hdr.header_count);
    out.push_back(hdr.schema_id);
    write_be16(out, 0); // reserved
    out.insert(out.end(), ext_bytes.begin(), ext_bytes.end());
    return out;
}

inline FrameHeader decode_frame_header(const std::uint8_t* data, std::size_t len) {
    if (len < kHeaderSize) {
        throw std::runtime_error("frame header too short");
    }
    FrameHeader hdr{};
    hdr.length = read_be32(data);
    hdr.frame_type = static_cast<FrameType>(data[4]);
    hdr.flags = data[5];
    hdr.channel_id = read_be16(data + 6);
    hdr.stream_seq = read_be32(data + 8);
    hdr.header_count = data[12];
    hdr.schema_id = data[13];
    const auto reserved = read_be16(data + 14);
    if (reserved != 0) {
        throw std::runtime_error("non-zero reserved field");
    }
    if (hdr.length < kHeaderSize || len < hdr.length) {
        throw std::runtime_error("invalid frame length");
    }
    return hdr;
}

inline std::vector<std::uint8_t> encode_ping() {
    FrameHeader hdr{};
    hdr.length = kHeaderSize + 1;
    hdr.frame_type = FrameType::Control;
    hdr.channel_id = kControlChannel;
    std::vector<std::uint8_t> frame = encode_frame_header(hdr, {}, 1);
    frame.push_back(kControlPing);
    return frame;
}

inline std::vector<std::uint8_t> encode_pong() {
    FrameHeader hdr{};
    hdr.length = kHeaderSize + 1;
    hdr.frame_type = FrameType::Control;
    hdr.channel_id = kControlChannel;
    std::vector<std::uint8_t> frame = encode_frame_header(hdr, {}, 1);
    frame.push_back(kControlPong);
    return frame;
}

struct ChannelState {
    std::uint32_t last_sent_seq = 0;
    std::uint32_t last_recv_seq = 0;
};

class ChannelManager {
public:
    explicit ChannelManager(bool is_server) : is_server_(is_server), next_channel_id_(1) {}

    std::uint16_t open_channel() {
        while (channels_.count(next_channel_id_) != 0) {
            next_channel_id_ = next_channel_id_ == 0xFFFF ? 1 : static_cast<std::uint16_t>(next_channel_id_ + 1);
        }
        const std::uint16_t id = next_channel_id_;
        channels_[id] = ChannelState{};
        next_channel_id_ = id == 0xFFFF ? 1 : static_cast<std::uint16_t>(id + 1);
        return id;
    }

    std::uint32_t next_send_seq(std::uint16_t channel_id) {
        auto& ch = channels_.at(channel_id);
        ch.last_sent_seq += 1;
        return ch.last_sent_seq;
    }

    void record_recv_seq(std::uint16_t channel_id, std::uint32_t seq) {
        channels_.at(channel_id).last_recv_seq = seq;
    }

    std::uint64_t tree_stream_id(std::uint16_t channel_id) const {
        return channel_stream_id(channel_id, is_server_);
    }

private:
    bool is_server_;
    std::uint16_t next_channel_id_;
    std::unordered_map<std::uint16_t, ChannelState> channels_;
};

} // namespace fig::protocol
