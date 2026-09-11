#pragma once

#ifdef __cplusplus
extern "C" {
#endif
#include "../../../../crates/fig-ffi/include/fig.h"
#ifdef __cplusplus
}
#endif

#include <cstdint>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>
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

class Subscription {
public:
    explicit Subscription(FigSubHandle* raw) : handle_(raw) {}

    /// Next live frame. timeout_ms 0 waits forever. Empty Buffer on EOF.
    Buffer next(uint32_t timeout_ms = 0) {
        FigBuffer out{};
        int32_t rc = fig_client_sub_next(handle_.get(), timeout_ms, &out);
        if (rc == 1) {
            throw std::runtime_error("fig_client_sub_next timeout");
        }
        if (rc == 2) {
            return Buffer();
        }
        if (rc != 0) {
            throw std::runtime_error("fig_client_sub_next failed");
        }
        return Buffer(out);
    }

private:
    struct Deleter {
        void operator()(FigSubHandle* h) const {
            if (h != nullptr) {
                fig_client_sub_close(h);
            }
        }
    };

    std::unique_ptr<FigSubHandle, Deleter> handle_;
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

    /// SUBSCRIBE snapshot plus a live handle. Do not use request_and_recv for SUBSCRIBE.
    std::pair<std::vector<Buffer>, Subscription> subscribe(const uint8_t* frame, std::size_t len) {
        FigFrameList list{};
        FigSubHandle* raw = nullptr;
        if (fig_client_subscribe(handle_.get(), frame, len, &list, &raw) != 0) {
            throw std::runtime_error("fig_client_subscribe failed");
        }
        std::vector<Buffer> snapshot;
        if (list.frames != nullptr && list.count > 0) {
            for (std::size_t i = 0; i < list.count; ++i) {
                snapshot.emplace_back(list.frames[i]);
            }
        }
        fig_frame_list_free(list);
        return {std::move(snapshot), Subscription(raw)};
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

inline std::string jwt_encode(const char* sub, uint64_t exp, const char* secret) {
    FigBuffer out{};
    if (fig_jwt_encode(sub, exp, secret, &out) != 0) {
        throw std::runtime_error("fig_jwt_encode failed");
    }
    Buffer buf(out);
    const auto& bytes = buf.bytes();
    return std::string(reinterpret_cast<const char*>(bytes.data()), bytes.size());
}

inline std::string jwt_decode_sub(const char* token, const char* secret) {
    char* sub = nullptr;
    if (fig_jwt_decode_sub(token, secret, &sub) != 0) {
        throw std::runtime_error("fig_jwt_decode_sub failed");
    }
    std::string out(sub);
    fig_string_free(sub);
    return out;
}

inline void jwt_verify_bearer(const char* token, const char* secret) {
    if (fig_jwt_verify_bearer(token, secret) != 0) {
        throw std::runtime_error("fig_jwt_verify_bearer failed");
    }
}

inline Buffer sbe_encode_new_order_single(const char* cl_ord_id,
                                          const char* symbol,
                                          bool side_buy,
                                          double qty,
                                          double price) {
    FigBuffer out{};
    if (fig_sbe_encode_new_order_single(cl_ord_id, symbol, side_buy ? 1 : 0, qty, price, -1, -1, &out) !=
        0) {
        throw std::runtime_error("fig_sbe_encode_new_order_single failed");
    }
    return Buffer(out);
}

} // namespace fig
