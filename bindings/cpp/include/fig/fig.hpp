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
#include <optional>
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

struct MassCancelRequestFields {
    std::string cl_ord_id;
    std::string account;
    std::optional<std::string> symbol;
    uint8_t side = 0;
};

struct TradingSessionStatusFields {
    uint8_t status = 0;
    std::string session_id;
};

struct SecurityStatusFields {
    std::string symbol;
    uint8_t status = 0;
};

struct MassCancelReportFields {
    std::string cl_ord_id;
    uint32_t total_affected = 0;
    uint8_t status = 0;
};

inline std::string take_c_string(char* s) {
    if (s == nullptr) {
        return {};
    }
    std::string out(s);
    fig_string_free(s);
    return out;
}

inline std::optional<std::string> take_optional_c_string(char* s) {
    if (s == nullptr) {
        return std::nullopt;
    }
    std::string out(s);
    fig_string_free(s);
    return out;
}

/// `side` 0 omits the field. `symbol` nullptr omits the field.
inline Buffer cbor_encode_mass_cancel_request(const char* cl_ord_id,
                                              const char* account,
                                              const char* symbol,
                                              uint8_t side) {
    FigBuffer out{};
    if (fig_cbor_encode_mass_cancel_request(cl_ord_id, account, symbol, side, &out) != 0) {
        throw std::runtime_error("fig_cbor_encode_mass_cancel_request failed");
    }
    return Buffer(out);
}

inline MassCancelRequestFields cbor_decode_mass_cancel_request(const uint8_t* data, std::size_t len) {
    char* cl = nullptr;
    char* account = nullptr;
    char* symbol = nullptr;
    uint8_t side = 0;
    if (fig_cbor_decode_mass_cancel_request(data, len, &cl, &account, &symbol, &side) != 0) {
        throw std::runtime_error("fig_cbor_decode_mass_cancel_request failed");
    }
    return {take_c_string(cl), take_c_string(account), take_optional_c_string(symbol), side};
}

inline TradingSessionStatusFields cbor_decode_trading_session_status(const uint8_t* data,
                                                                     std::size_t len) {
    uint8_t status = 0;
    char* session_id = nullptr;
    if (fig_cbor_decode_trading_session_status(data, len, &status, &session_id) != 0) {
        throw std::runtime_error("fig_cbor_decode_trading_session_status failed");
    }
    return {status, take_c_string(session_id)};
}

inline SecurityStatusFields cbor_decode_security_status(const uint8_t* data, std::size_t len) {
    char* symbol = nullptr;
    uint8_t status = 0;
    if (fig_cbor_decode_security_status(data, len, &symbol, &status) != 0) {
        throw std::runtime_error("fig_cbor_decode_security_status failed");
    }
    return {take_c_string(symbol), status};
}

inline MassCancelReportFields cbor_decode_mass_cancel_report(const uint8_t* data, std::size_t len) {
    char* cl = nullptr;
    uint32_t total = 0;
    uint8_t status = 0;
    if (fig_cbor_decode_mass_cancel_report(data, len, &cl, &total, &status) != 0) {
        throw std::runtime_error("fig_cbor_decode_mass_cancel_report failed");
    }
    return {take_c_string(cl), total, status};
}

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

    /// GET marketdata/session.
    TradingSessionStatusFields request_session_status(uint16_t channel_id = 1) {
        uint8_t status = 0;
        char* session_id = nullptr;
        if (fig_client_request_session_status(handle_.get(), channel_id, &status, &session_id) != 0) {
            throw std::runtime_error("fig_client_request_session_status failed");
        }
        return {status, take_c_string(session_id)};
    }

    /// GET marketdata/{symbol}/status.
    SecurityStatusFields request_security_status(const char* symbol, uint16_t channel_id = 1) {
        char* out_symbol = nullptr;
        uint8_t status = 0;
        if (fig_client_request_security_status(handle_.get(), symbol, channel_id, &out_symbol, &status) !=
            0) {
            throw std::runtime_error("fig_client_request_security_status failed");
        }
        return {take_c_string(out_symbol), status};
    }

    /// POST trading/accounts/{account}/orders/masscancel. side 0 and symbol nullptr omit filters.
    MassCancelReportFields mass_cancel(const char* cl_ord_id,
                                       const char* account,
                                       const char* symbol,
                                       uint8_t side,
                                       uint16_t channel_id = 1) {
        char* out_cl = nullptr;
        uint32_t total = 0;
        uint8_t status = 0;
        if (fig_client_mass_cancel(handle_.get(), cl_ord_id, account, symbol, side, channel_id, &out_cl,
                                   &total, &status) != 0) {
            throw std::runtime_error("fig_client_mass_cancel failed");
        }
        return {take_c_string(out_cl), total, status};
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
