#pragma once

#include "../../../../crates/fig-ffi/include/fig.h"

#include <cstdint>
#include <stdexcept>
#include <string>
#include <vector>

namespace fig {

inline void buffer_free(FigBuffer buf) { fig_buffer_free(buf); }

class Buffer {
public:
    Buffer() = default;
    explicit Buffer(FigBuffer raw) : data_(copy_raw(raw)) {
        fig_buffer_free(raw);
    }

    const std::vector<uint8_t>& bytes() const { return data_; }

private:
    static std::vector<uint8_t> copy_raw(FigBuffer raw) {
        if (raw.data == nullptr || raw.len == 0) {
            return {};
        }
        return std::vector<uint8_t>(raw.data, raw.data + raw.len);
    }

    std::vector<uint8_t> data_;
};

inline Buffer encode_request(uint16_t channel_id,
                             uint32_t stream_seq,
                             uint8_t schema_id,
                             const char* channel_path,
                             const char* method,
                             const uint8_t* payload,
                             std::size_t payload_len) {
    FigBuffer out{};
    if (fig_frame_encode_request_ex(channel_id, stream_seq, schema_id, channel_path, method,
                                    nullptr, payload, payload_len, &out) != 0) {
        throw std::runtime_error("fig_frame_encode_request_ex failed");
    }
    return Buffer(out);
}

inline Buffer encode_subscribe(uint16_t channel_id,
                               uint32_t stream_seq,
                               const char* routing_key,
                               const char* channel_path) {
    FigBuffer out{};
    if (fig_frame_encode_subscribe(channel_id, stream_seq, routing_key, channel_path, &out) != 0) {
        throw std::runtime_error("fig_frame_encode_subscribe failed");
    }
    return Buffer(out);
}

inline uint64_t channel_stream_id(uint16_t channel_id, bool is_server) {
    return fig_client_channel_stream_id(channel_id, is_server ? 1 : 0);
}

} // namespace fig
