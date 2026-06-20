#pragma once

#include "../../../../crates/fig-ffi/include/fig.h"

#include <cstdint>
#include <memory>
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

class Client {
public:
    Client(const char* addr, const char* server_name = nullptr) {
        FigClientHandle* raw = nullptr;
        if (fig_client_connect(addr, server_name, &raw) != 0) {
            throw std::runtime_error("fig_client_connect failed");
        }
        handle_.reset(raw);
    }

    void ping() {
        if (fig_client_ping(handle_.get()) != 0) {
            throw std::runtime_error("fig_client_ping failed");
        }
    }

    std::vector<Buffer> request_and_recv(const uint8_t* frame, std::size_t len) {
        FigFrameList list{};
        if (fig_client_request_and_recv(handle_.get(), frame, len, &list) != 0) {
            throw std::runtime_error("fig_client_request_and_recv failed");
        }
        std::vector<Buffer> out;
        if (list.frames != nullptr && list.count > 0) {
            for (std::size_t i = 0; i < list.count; ++i) {
                out.emplace_back(list.frames[i]);
            }
        }
        fig_frame_list_free(list);
        return out;
    }

private:
    struct Deleter {
        void operator()(FigClientHandle* h) const {
            if (h != nullptr) {
                fig_client_close(h);
            }
        }
    };

    std::unique_ptr<FigClientHandle, Deleter> handle_;
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

inline Buffer encode_request_auth(uint16_t channel_id,
                                  uint32_t stream_seq,
                                  uint8_t schema_id,
                                  const char* channel_path,
                                  const char* method,
                                  const char* content_type,
                                  const char* auth_token,
                                  const uint8_t* payload,
                                  std::size_t payload_len) {
    FigBuffer out{};
    if (fig_frame_encode_request_auth(channel_id, stream_seq, schema_id, channel_path, method,
                                      content_type, auth_token, payload, payload_len, &out) != 0) {
        throw std::runtime_error("fig_frame_encode_request_auth failed");
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

inline Buffer encode_subscribe_auth(uint16_t channel_id,
                                    uint32_t stream_seq,
                                    const char* routing_key,
                                    const char* channel_path,
                                    const char* auth_token) {
    FigBuffer out{};
    if (fig_frame_encode_subscribe_auth(channel_id, stream_seq, routing_key, channel_path,
                                        auth_token, &out) != 0) {
        throw std::runtime_error("fig_frame_encode_subscribe_auth failed");
    }
    return Buffer(out);
}

inline uint64_t channel_stream_id(uint16_t channel_id, bool is_server) {
    return fig_client_channel_stream_id(channel_id, is_server ? 1 : 0);
}

} // namespace fig
