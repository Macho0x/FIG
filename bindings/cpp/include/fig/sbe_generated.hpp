// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' vv1.0.0
// Standard order entry and execution messages

#pragma once

#include <cstdint>
#include <cstring>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>

namespace fig::sbe {

constexpr uint16_t SCHEMA_ID = 0x01;

inline void write_u16_be(std::vector<uint8_t>& buf, uint16_t v) {
    buf.push_back(static_cast<uint8_t>(v >> 8));
    buf.push_back(static_cast<uint8_t>(v));
}
inline void write_u32_be(std::vector<uint8_t>& buf, uint32_t v) {
    buf.push_back(static_cast<uint8_t>(v >> 24));
    buf.push_back(static_cast<uint8_t>(v >> 16));
    buf.push_back(static_cast<uint8_t>(v >> 8));
    buf.push_back(static_cast<uint8_t>(v));
}
inline void write_u64_be(std::vector<uint8_t>& buf, uint64_t v) {
    for (int i = 7; i >= 0; --i) buf.push_back(static_cast<uint8_t>((v >> (i * 8)) & 0xFF));
}
inline void write_f64_be(std::vector<uint8_t>& buf, double v) {
    uint64_t bits; std::memcpy(&bits, &v, sizeof(bits)); write_u64_be(buf, bits);
}
inline void write_i64_be(std::vector<uint8_t>& buf, int64_t v) { write_u64_be(buf, static_cast<uint64_t>(v)); }
inline void write_string(std::vector<uint8_t>& buf, const std::string& s) {
    write_u16_be(buf, static_cast<uint16_t>(s.size()));
    buf.insert(buf.end(), s.begin(), s.end());
}
inline uint16_t read_u16_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint16_t v = (static_cast<uint16_t>(buf[pos]) << 8) | buf[pos + 1]; pos += 2; return v;
}
inline uint32_t read_u32_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint32_t v = (static_cast<uint32_t>(buf[pos]) << 24) | (static_cast<uint32_t>(buf[pos + 1]) << 16) |
                 (static_cast<uint32_t>(buf[pos + 2]) << 8) | buf[pos + 3]; pos += 4; return v;
}
inline uint64_t read_u64_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint64_t v = 0; for (int i = 0; i < 8; ++i) v = (v << 8) | buf[pos + i]; pos += 8; return v;
}
inline double read_f64_be(const std::vector<uint8_t>& buf, size_t& pos) {
    uint64_t bits = read_u64_be(buf, pos); double v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
inline int64_t read_i64_be(const std::vector<uint8_t>& buf, size_t& pos) { return static_cast<int64_t>(read_u64_be(buf, pos)); }
inline std::string read_string(const std::vector<uint8_t>& buf, size_t& pos) {
    uint16_t len = read_u16_be(buf, pos); std::string s(reinterpret_cast<const char*>(&buf[pos]), len); pos += len; return s;
}

enum class NewOrderSingleSide : uint8_t {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
    SellShortExempt = 4,
};

inline std::optional<NewOrderSingleSide> NewOrderSingleSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return NewOrderSingleSide::Buy;
    case 2: return NewOrderSingleSide::Sell;
    case 3: return NewOrderSingleSide::SellShort;
    case 4: return NewOrderSingleSide::SellShortExempt;
    default: return std::nullopt;
    }
}

inline uint8_t NewOrderSingleSideToValue(NewOrderSingleSide e) { return static_cast<uint8_t>(e); }

enum class NewOrderSingleOrderType : uint8_t {
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLimit = 4,
    MarketOnClose = 5,
    LimitOnClose = 6,
    Pegged = 7,
};

inline std::optional<NewOrderSingleOrderType> NewOrderSingleOrderTypeFromValue(uint8_t v) {
    switch (v) {
    case 1: return NewOrderSingleOrderType::Market;
    case 2: return NewOrderSingleOrderType::Limit;
    case 3: return NewOrderSingleOrderType::Stop;
    case 4: return NewOrderSingleOrderType::StopLimit;
    case 5: return NewOrderSingleOrderType::MarketOnClose;
    case 6: return NewOrderSingleOrderType::LimitOnClose;
    case 7: return NewOrderSingleOrderType::Pegged;
    default: return std::nullopt;
    }
}

inline uint8_t NewOrderSingleOrderTypeToValue(NewOrderSingleOrderType e) { return static_cast<uint8_t>(e); }

enum class NewOrderSingleTimeInForce : uint8_t {
    Day = 1,
    Gtc = 2,
    Ioc = 3,
    Fok = 4,
    Gtd = 5,
};

inline std::optional<NewOrderSingleTimeInForce> NewOrderSingleTimeInForceFromValue(uint8_t v) {
    switch (v) {
    case 1: return NewOrderSingleTimeInForce::Day;
    case 2: return NewOrderSingleTimeInForce::Gtc;
    case 3: return NewOrderSingleTimeInForce::Ioc;
    case 4: return NewOrderSingleTimeInForce::Fok;
    case 5: return NewOrderSingleTimeInForce::Gtd;
    default: return std::nullopt;
    }
}

inline uint8_t NewOrderSingleTimeInForceToValue(NewOrderSingleTimeInForce e) { return static_cast<uint8_t>(e); }

enum class NewOrderSingleIdSource : uint8_t {
    Cusip = 1,
    Sedol = 2,
    Isin = 3,
    Ric = 4,
    ExchangeSymbol = 5,
};

inline std::optional<NewOrderSingleIdSource> NewOrderSingleIdSourceFromValue(uint8_t v) {
    switch (v) {
    case 1: return NewOrderSingleIdSource::Cusip;
    case 2: return NewOrderSingleIdSource::Sedol;
    case 3: return NewOrderSingleIdSource::Isin;
    case 4: return NewOrderSingleIdSource::Ric;
    case 5: return NewOrderSingleIdSource::ExchangeSymbol;
    default: return std::nullopt;
    }
}

inline uint8_t NewOrderSingleIdSourceToValue(NewOrderSingleIdSource e) { return static_cast<uint8_t>(e); }

enum class CancelRequestSide : uint8_t {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
};

inline std::optional<CancelRequestSide> CancelRequestSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return CancelRequestSide::Buy;
    case 2: return CancelRequestSide::Sell;
    case 3: return CancelRequestSide::SellShort;
    default: return std::nullopt;
    }
}

inline uint8_t CancelRequestSideToValue(CancelRequestSide e) { return static_cast<uint8_t>(e); }

enum class CancelReplaceRequestSide : uint8_t {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
};

inline std::optional<CancelReplaceRequestSide> CancelReplaceRequestSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return CancelReplaceRequestSide::Buy;
    case 2: return CancelReplaceRequestSide::Sell;
    case 3: return CancelReplaceRequestSide::SellShort;
    default: return std::nullopt;
    }
}

inline uint8_t CancelReplaceRequestSideToValue(CancelReplaceRequestSide e) { return static_cast<uint8_t>(e); }

enum class ExecutionReportExecType : uint8_t {
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

inline std::optional<ExecutionReportExecType> ExecutionReportExecTypeFromValue(uint8_t v) {
    switch (v) {
    case 1: return ExecutionReportExecType::New;
    case 2: return ExecutionReportExecType::PartialFill;
    case 3: return ExecutionReportExecType::Fill;
    case 4: return ExecutionReportExecType::DoneForDay;
    case 5: return ExecutionReportExecType::Canceled;
    case 6: return ExecutionReportExecType::Replaced;
    case 7: return ExecutionReportExecType::PendingCancel;
    case 8: return ExecutionReportExecType::Stopped;
    case 9: return ExecutionReportExecType::Rejected;
    case 10: return ExecutionReportExecType::Suspended;
    case 11: return ExecutionReportExecType::PendingNew;
    case 12: return ExecutionReportExecType::Expired;
    default: return std::nullopt;
    }
}

inline uint8_t ExecutionReportExecTypeToValue(ExecutionReportExecType e) { return static_cast<uint8_t>(e); }

enum class ExecutionReportOrdStatus : uint8_t {
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

inline std::optional<ExecutionReportOrdStatus> ExecutionReportOrdStatusFromValue(uint8_t v) {
    switch (v) {
    case 1: return ExecutionReportOrdStatus::New;
    case 2: return ExecutionReportOrdStatus::PartiallyFilled;
    case 3: return ExecutionReportOrdStatus::Filled;
    case 4: return ExecutionReportOrdStatus::DoneForDay;
    case 5: return ExecutionReportOrdStatus::Canceled;
    case 6: return ExecutionReportOrdStatus::PendingCancel;
    case 7: return ExecutionReportOrdStatus::Stopped;
    case 8: return ExecutionReportOrdStatus::Rejected;
    case 9: return ExecutionReportOrdStatus::Suspended;
    case 10: return ExecutionReportOrdStatus::PendingNew;
    case 11: return ExecutionReportOrdStatus::Expired;
    case 12: return ExecutionReportOrdStatus::Replaced;
    default: return std::nullopt;
    }
}

inline uint8_t ExecutionReportOrdStatusToValue(ExecutionReportOrdStatus e) { return static_cast<uint8_t>(e); }

enum class ExecutionReportSide : uint8_t {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
};

inline std::optional<ExecutionReportSide> ExecutionReportSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return ExecutionReportSide::Buy;
    case 2: return ExecutionReportSide::Sell;
    case 3: return ExecutionReportSide::SellShort;
    default: return std::nullopt;
    }
}

inline uint8_t ExecutionReportSideToValue(ExecutionReportSide e) { return static_cast<uint8_t>(e); }

enum class CancelRejectRejectReason : uint8_t {
    OrderNotFound = 1,
    AlreadyCanceled = 2,
    AlreadyFilled = 3,
    TooLateToCancel = 4,
};

inline std::optional<CancelRejectRejectReason> CancelRejectRejectReasonFromValue(uint8_t v) {
    switch (v) {
    case 1: return CancelRejectRejectReason::OrderNotFound;
    case 2: return CancelRejectRejectReason::AlreadyCanceled;
    case 3: return CancelRejectRejectReason::AlreadyFilled;
    case 4: return CancelRejectRejectReason::TooLateToCancel;
    default: return std::nullopt;
    }
}

inline uint8_t CancelRejectRejectReasonToValue(CancelRejectRejectReason e) { return static_cast<uint8_t>(e); }

enum class BalanceUpdateReason : uint8_t {
    Trade = 1,
    Deposit = 2,
    Withdrawal = 3,
    Transfer = 4,
    Fee = 5,
};

inline std::optional<BalanceUpdateReason> BalanceUpdateReasonFromValue(uint8_t v) {
    switch (v) {
    case 1: return BalanceUpdateReason::Trade;
    case 2: return BalanceUpdateReason::Deposit;
    case 3: return BalanceUpdateReason::Withdrawal;
    case 4: return BalanceUpdateReason::Transfer;
    case 5: return BalanceUpdateReason::Fee;
    default: return std::nullopt;
    }
}

inline uint8_t BalanceUpdateReasonToValue(BalanceUpdateReason e) { return static_cast<uint8_t>(e); }

enum class OrderListStatusStatus : uint8_t {
    Executing = 1,
    AllDone = 2,
    Reject = 3,
};

inline std::optional<OrderListStatusStatus> OrderListStatusStatusFromValue(uint8_t v) {
    switch (v) {
    case 1: return OrderListStatusStatus::Executing;
    case 2: return OrderListStatusStatus::AllDone;
    case 3: return OrderListStatusStatus::Reject;
    default: return std::nullopt;
    }
}

inline uint8_t OrderListStatusStatusToValue(OrderListStatusStatus e) { return static_cast<uint8_t>(e); }

enum class LedgerUpdateKind : uint8_t {
    Deposit = 1,
    Withdrawal = 2,
    Transfer = 3,
    Fee = 4,
    Funding = 5,
};

inline std::optional<LedgerUpdateKind> LedgerUpdateKindFromValue(uint8_t v) {
    switch (v) {
    case 1: return LedgerUpdateKind::Deposit;
    case 2: return LedgerUpdateKind::Withdrawal;
    case 3: return LedgerUpdateKind::Transfer;
    case 4: return LedgerUpdateKind::Fee;
    case 5: return LedgerUpdateKind::Funding;
    default: return std::nullopt;
    }
}

inline uint8_t LedgerUpdateKindToValue(LedgerUpdateKind e) { return static_cast<uint8_t>(e); }

enum class AggregateTradeSide : uint8_t {
    Buy = 1,
    Sell = 2,
};

inline std::optional<AggregateTradeSide> AggregateTradeSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return AggregateTradeSide::Buy;
    case 2: return AggregateTradeSide::Sell;
    default: return std::nullopt;
    }
}

inline uint8_t AggregateTradeSideToValue(AggregateTradeSide e) { return static_cast<uint8_t>(e); }

enum class LiquidationTradeSide : uint8_t {
    Buy = 1,
    Sell = 2,
};

inline std::optional<LiquidationTradeSide> LiquidationTradeSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return LiquidationTradeSide::Buy;
    case 2: return LiquidationTradeSide::Sell;
    default: return std::nullopt;
    }
}

inline uint8_t LiquidationTradeSideToValue(LiquidationTradeSide e) { return static_cast<uint8_t>(e); }

enum class MarketDataUpdateSide : uint8_t {
    Buy = 1,
    Sell = 2,
};

inline std::optional<MarketDataUpdateSide> MarketDataUpdateSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return MarketDataUpdateSide::Buy;
    case 2: return MarketDataUpdateSide::Sell;
    default: return std::nullopt;
    }
}

inline uint8_t MarketDataUpdateSideToValue(MarketDataUpdateSide e) { return static_cast<uint8_t>(e); }

enum class MarketDataUpdateAction : uint8_t {
    New = 1,
    Change = 2,
    Delete = 3,
};

inline std::optional<MarketDataUpdateAction> MarketDataUpdateActionFromValue(uint8_t v) {
    switch (v) {
    case 1: return MarketDataUpdateAction::New;
    case 2: return MarketDataUpdateAction::Change;
    case 3: return MarketDataUpdateAction::Delete;
    default: return std::nullopt;
    }
}

inline uint8_t MarketDataUpdateActionToValue(MarketDataUpdateAction e) { return static_cast<uint8_t>(e); }

enum class PublicTradeSide : uint8_t {
    Buy = 1,
    Sell = 2,
};

inline std::optional<PublicTradeSide> PublicTradeSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return PublicTradeSide::Buy;
    case 2: return PublicTradeSide::Sell;
    default: return std::nullopt;
    }
}

inline uint8_t PublicTradeSideToValue(PublicTradeSide e) { return static_cast<uint8_t>(e); }

enum class CapabilityPathPattern : uint8_t {
    PubSub = 1,
    RequestResponse = 2,
    RequestStream = 3,
};

inline std::optional<CapabilityPathPattern> CapabilityPathPatternFromValue(uint8_t v) {
    switch (v) {
    case 1: return CapabilityPathPattern::PubSub;
    case 2: return CapabilityPathPattern::RequestResponse;
    case 3: return CapabilityPathPattern::RequestStream;
    default: return std::nullopt;
    }
}

inline uint8_t CapabilityPathPatternToValue(CapabilityPathPattern e) { return static_cast<uint8_t>(e); }

// SBE encoder for NewOrderSingle
struct NewOrderSingleEncoder {
    static std::vector<uint8_t> encode(cl_ord_id std::string, side NewOrderSingleSide, order_qty double, price std::optional<double>, stop_price std::optional<double>, symbol std::string, order_type NewOrderSingleOrderType, time_in_force NewOrderSingleTimeInForce, expire_time std::optional<int64_t>, account std::optional<std::string>, strategy_id std::optional<std::string>, security_id std::optional<std::string>, id_source std::optional<NewOrderSingleIdSource>, security_exchange std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 1);
        write_u16_be(buf, 0);
        write_u16_be(buf, 36);

        // Fixed fields
        write_string(buf, cl_ord_id);
        buf.push_back(NewOrderSingleSideToValue(side));
        write_f64_be(buf, order_qty);
        write_f64_be(buf, price.value_or(0.0));
        buf.push_back(price.has_value() ? 1 : 0);
        write_f64_be(buf, stop_price.value_or(0.0));
        buf.push_back(stop_price.has_value() ? 1 : 0);
        write_string(buf, symbol);
        buf.push_back(NewOrderSingleOrderTypeToValue(order_type));
        buf.push_back(NewOrderSingleTimeInForceToValue(time_in_force));
        if (expire_time.has_value()) { buf.push_back(1); write_i64_be(buf, *expire_time); } else { buf.push_back(0); }
        if (account.has_value()) { buf.push_back(1); write_string(buf, *account); } else { buf.push_back(0); }
        if (strategy_id.has_value()) { buf.push_back(1); write_string(buf, *strategy_id); } else { buf.push_back(0); }
        if (security_id.has_value()) { buf.push_back(1); write_string(buf, *security_id); } else { buf.push_back(0); }
        buf.push_back(id_source.has_value() ? NewOrderSingleIdSourceToValue(*id_source) : 0);
        if (security_exchange.has_value()) { buf.push_back(1); write_string(buf, *security_exchange); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for NewOrderSingle
struct NewOrderSingleDecoder {
    std::string cl_ord_id;
    NewOrderSingleSide side;
    double order_qty;
    std::optional<double> price;
    std::optional<double> stop_price;
    std::string symbol;
    NewOrderSingleOrderType order_type;
    NewOrderSingleTimeInForce time_in_force;
    std::optional<int64_t> expire_time;
    std::optional<std::string> account;
    std::optional<std::string> strategy_id;
    std::optional<std::string> security_id;
    std::optional<NewOrderSingleIdSource> id_source;
    std::optional<std::string> security_exchange;
    size_t encoded_len() const { return 0; }

    static NewOrderSingleDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 1) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        uint8_t side_raw = buf[pos++];
        auto side_opt = NewOrderSingleSideFromValue(side_raw);
        if (!side_opt) throw std::runtime_error("invalid NewOrderSingleSide");
        NewOrderSingleSide side = *side_opt;
        double order_qty = read_f64_be(buf, pos);
        double price_raw = read_f64_be(buf, pos);
        std::optional<double> price;
        if (buf[pos++] == 1) price = price_raw;
        double stop_price_raw = read_f64_be(buf, pos);
        std::optional<double> stop_price;
        if (buf[pos++] == 1) stop_price = stop_price_raw;
        std::string symbol = read_string(buf, pos);
        uint8_t order_type_raw = buf[pos++];
        auto order_type_opt = NewOrderSingleOrderTypeFromValue(order_type_raw);
        if (!order_type_opt) throw std::runtime_error("invalid NewOrderSingleOrderType");
        NewOrderSingleOrderType order_type = *order_type_opt;
        uint8_t time_in_force_raw = buf[pos++];
        auto time_in_force_opt = NewOrderSingleTimeInForceFromValue(time_in_force_raw);
        if (!time_in_force_opt) throw std::runtime_error("invalid NewOrderSingleTimeInForce");
        NewOrderSingleTimeInForce time_in_force = *time_in_force_opt;
        std::optional<int64_t> expire_time;
        if (buf[pos++] == 1) expire_time = read_i64_be(buf, pos);
        std::optional<std::string> account;
        if (buf[pos++] == 1) account = read_string(buf, pos);
        std::optional<std::string> strategy_id;
        if (buf[pos++] == 1) strategy_id = read_string(buf, pos);
        std::optional<std::string> security_id;
        if (buf[pos++] == 1) security_id = read_string(buf, pos);
        uint8_t id_source_raw = buf[pos++];
        auto id_source_opt = NewOrderSingleIdSourceFromValue(id_source_raw);
        if (!id_source_opt) throw std::runtime_error("invalid NewOrderSingleIdSource");
        NewOrderSingleIdSource id_source = *id_source_opt;
        std::optional<std::string> security_exchange;
        if (buf[pos++] == 1) security_exchange = read_string(buf, pos);
        NewOrderSingleDecoder out{};
        out.cl_ord_id = cl_ord_id;
        out.side = side;
        out.order_qty = order_qty;
        out.price = price;
        out.stop_price = stop_price;
        out.symbol = symbol;
        out.order_type = order_type;
        out.time_in_force = time_in_force;
        out.expire_time = expire_time;
        out.account = account;
        out.strategy_id = strategy_id;
        out.security_id = security_id;
        out.id_source = id_source;
        out.security_exchange = security_exchange;
        return out;
    }
};

// SBE encoder for CancelRequest
struct CancelRequestEncoder {
    static std::vector<uint8_t> encode(cl_ord_id std::string, orig_cl_ord_id std::string, symbol std::string, side CancelRequestSide, order_qty std::optional<double>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 2);
        write_u16_be(buf, 0);
        write_u16_be(buf, 9);

        // Fixed fields
        write_string(buf, cl_ord_id);
        write_string(buf, orig_cl_ord_id);
        write_string(buf, symbol);
        buf.push_back(CancelRequestSideToValue(side));
        write_f64_be(buf, order_qty.value_or(0.0));
        buf.push_back(order_qty.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for CancelRequest
struct CancelRequestDecoder {
    std::string cl_ord_id;
    std::string orig_cl_ord_id;
    std::string symbol;
    CancelRequestSide side;
    std::optional<double> order_qty;
    size_t encoded_len() const { return 0; }

    static CancelRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 2) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        uint8_t side_raw = buf[pos++];
        auto side_opt = CancelRequestSideFromValue(side_raw);
        if (!side_opt) throw std::runtime_error("invalid CancelRequestSide");
        CancelRequestSide side = *side_opt;
        double order_qty_raw = read_f64_be(buf, pos);
        std::optional<double> order_qty;
        if (buf[pos++] == 1) order_qty = order_qty_raw;
        CancelRequestDecoder out{};
        out.cl_ord_id = cl_ord_id;
        out.orig_cl_ord_id = orig_cl_ord_id;
        out.symbol = symbol;
        out.side = side;
        out.order_qty = order_qty;
        return out;
    }
};

// SBE encoder for CancelReplaceRequest
struct CancelReplaceRequestEncoder {
    static std::vector<uint8_t> encode(cl_ord_id std::string, orig_cl_ord_id std::string, symbol std::string, side CancelReplaceRequestSide, order_qty double, price std::optional<double>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 3);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);

        // Fixed fields
        write_string(buf, cl_ord_id);
        write_string(buf, orig_cl_ord_id);
        write_string(buf, symbol);
        buf.push_back(CancelReplaceRequestSideToValue(side));
        write_f64_be(buf, order_qty);
        write_f64_be(buf, price.value_or(0.0));
        buf.push_back(price.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for CancelReplaceRequest
struct CancelReplaceRequestDecoder {
    std::string cl_ord_id;
    std::string orig_cl_ord_id;
    std::string symbol;
    CancelReplaceRequestSide side;
    double order_qty;
    std::optional<double> price;
    size_t encoded_len() const { return 0; }

    static CancelReplaceRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 3) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        uint8_t side_raw = buf[pos++];
        auto side_opt = CancelReplaceRequestSideFromValue(side_raw);
        if (!side_opt) throw std::runtime_error("invalid CancelReplaceRequestSide");
        CancelReplaceRequestSide side = *side_opt;
        double order_qty = read_f64_be(buf, pos);
        double price_raw = read_f64_be(buf, pos);
        std::optional<double> price;
        if (buf[pos++] == 1) price = price_raw;
        CancelReplaceRequestDecoder out{};
        out.cl_ord_id = cl_ord_id;
        out.orig_cl_ord_id = orig_cl_ord_id;
        out.symbol = symbol;
        out.side = side;
        out.order_qty = order_qty;
        out.price = price;
        return out;
    }
};

// SBE encoder for ExecutionReport
struct ExecutionReportEncoder {
    static std::vector<uint8_t> encode(cl_ord_id std::string, order_id std::string, exec_id std::string, exec_type ExecutionReportExecType, ord_status ExecutionReportOrdStatus, side ExecutionReportSide, last_qty std::optional<double>, last_price std::optional<double>, leaves_qty double, cum_qty double, avg_price double, symbol std::string, transact_time int64_t) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 4);
        write_u16_be(buf, 0);
        write_u16_be(buf, 51);

        // Fixed fields
        write_string(buf, cl_ord_id);
        write_string(buf, order_id);
        write_string(buf, exec_id);
        buf.push_back(ExecutionReportExecTypeToValue(exec_type));
        buf.push_back(ExecutionReportOrdStatusToValue(ord_status));
        buf.push_back(ExecutionReportSideToValue(side));
        write_f64_be(buf, last_qty.value_or(0.0));
        buf.push_back(last_qty.has_value() ? 1 : 0);
        write_f64_be(buf, last_price.value_or(0.0));
        buf.push_back(last_price.has_value() ? 1 : 0);
        write_f64_be(buf, leaves_qty);
        write_f64_be(buf, cum_qty);
        write_f64_be(buf, avg_price);
        write_string(buf, symbol);
        write_i64_be(buf, transact_time);
        return buf;
    }
};

// SBE decoder for ExecutionReport
struct ExecutionReportDecoder {
    std::string cl_ord_id;
    std::string order_id;
    std::string exec_id;
    ExecutionReportExecType exec_type;
    ExecutionReportOrdStatus ord_status;
    ExecutionReportSide side;
    std::optional<double> last_qty;
    std::optional<double> last_price;
    double leaves_qty;
    double cum_qty;
    double avg_price;
    std::string symbol;
    int64_t transact_time;
    size_t encoded_len() const { return 0; }

    static ExecutionReportDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 4) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string order_id = read_string(buf, pos);
        std::string exec_id = read_string(buf, pos);
        uint8_t exec_type_raw = buf[pos++];
        auto exec_type_opt = ExecutionReportExecTypeFromValue(exec_type_raw);
        if (!exec_type_opt) throw std::runtime_error("invalid ExecutionReportExecType");
        ExecutionReportExecType exec_type = *exec_type_opt;
        uint8_t ord_status_raw = buf[pos++];
        auto ord_status_opt = ExecutionReportOrdStatusFromValue(ord_status_raw);
        if (!ord_status_opt) throw std::runtime_error("invalid ExecutionReportOrdStatus");
        ExecutionReportOrdStatus ord_status = *ord_status_opt;
        uint8_t side_raw = buf[pos++];
        auto side_opt = ExecutionReportSideFromValue(side_raw);
        if (!side_opt) throw std::runtime_error("invalid ExecutionReportSide");
        ExecutionReportSide side = *side_opt;
        double last_qty_raw = read_f64_be(buf, pos);
        std::optional<double> last_qty;
        if (buf[pos++] == 1) last_qty = last_qty_raw;
        double last_price_raw = read_f64_be(buf, pos);
        std::optional<double> last_price;
        if (buf[pos++] == 1) last_price = last_price_raw;
        double leaves_qty = read_f64_be(buf, pos);
        double cum_qty = read_f64_be(buf, pos);
        double avg_price = read_f64_be(buf, pos);
        std::string symbol = read_string(buf, pos);
        int64_t transact_time = read_i64_be(buf, pos);
        ExecutionReportDecoder out{};
        out.cl_ord_id = cl_ord_id;
        out.order_id = order_id;
        out.exec_id = exec_id;
        out.exec_type = exec_type;
        out.ord_status = ord_status;
        out.side = side;
        out.last_qty = last_qty;
        out.last_price = last_price;
        out.leaves_qty = leaves_qty;
        out.cum_qty = cum_qty;
        out.avg_price = avg_price;
        out.symbol = symbol;
        out.transact_time = transact_time;
        return out;
    }
};

// SBE encoder for CancelReject
struct CancelRejectEncoder {
    static std::vector<uint8_t> encode(cl_ord_id std::string, orig_cl_ord_id std::string, reject_reason CancelRejectRejectReason, symbol std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 5);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, cl_ord_id);
        write_string(buf, orig_cl_ord_id);
        buf.push_back(CancelRejectRejectReasonToValue(reject_reason));
        write_string(buf, symbol);
        return buf;
    }
};

// SBE decoder for CancelReject
struct CancelRejectDecoder {
    std::string cl_ord_id;
    std::string orig_cl_ord_id;
    CancelRejectRejectReason reject_reason;
    std::string symbol;
    size_t encoded_len() const { return 0; }

    static CancelRejectDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 5) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        uint8_t reject_reason_raw = buf[pos++];
        auto reject_reason_opt = CancelRejectRejectReasonFromValue(reject_reason_raw);
        if (!reject_reason_opt) throw std::runtime_error("invalid CancelRejectRejectReason");
        CancelRejectRejectReason reject_reason = *reject_reason_opt;
        std::string symbol = read_string(buf, pos);
        CancelRejectDecoder out{};
        out.cl_ord_id = cl_ord_id;
        out.orig_cl_ord_id = orig_cl_ord_id;
        out.reject_reason = reject_reason;
        out.symbol = symbol;
        return out;
    }
};

// SBE encoder for MarketDataSnapshot
struct MarketDataSnapshotEncoder {
    static std::vector<uint8_t> encode(symbol std::string, exchange std::string, bids std::vector<std::string>, asks std::vector<std::string>, timestamp int64_t, sequence std::optional<int64_t>, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 6);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);

        // Fixed fields
        write_string(buf, symbol);
        write_string(buf, exchange);
        write_u32_be(buf, static_cast<uint32_t>(bids.size()));
        for (const auto& item : bids) {
            PriceLevelEncoder::encode(item, buf);
        }
        write_u32_be(buf, static_cast<uint32_t>(asks.size()));
        for (const auto& item : asks) {
            PriceLevelEncoder::encode(item, buf);
        }
        write_i64_be(buf, timestamp);
        if (sequence.has_value()) { buf.push_back(1); write_i64_be(buf, *sequence); } else { buf.push_back(0); }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for MarketDataSnapshot
struct MarketDataSnapshotDecoder {
    std::string symbol;
    std::string exchange;
    std::vector<std::string> bids;
    std::vector<std::string> asks;
    int64_t timestamp;
    std::optional<int64_t> sequence;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static MarketDataSnapshotDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 6) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::string exchange = read_string(buf, pos);
        uint32_t bids_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> bids;
        bids.reserve(bids_count);
        for (uint32_t i = 0; i < bids_count; ++i) {
            auto item = PriceLevelDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            bids.push_back(item);
        }
        uint32_t asks_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> asks;
        asks.reserve(asks_count);
        for (uint32_t i = 0; i < asks_count; ++i) {
            auto item = PriceLevelDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            asks.push_back(item);
        }
        int64_t timestamp = read_i64_be(buf, pos);
        std::optional<int64_t> sequence;
        if (buf[pos++] == 1) sequence = read_i64_be(buf, pos);
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        MarketDataSnapshotDecoder out{};
        out.symbol = symbol;
        out.exchange = exchange;
        out.bids = bids;
        out.asks = asks;
        out.timestamp = timestamp;
        out.sequence = sequence;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for MarketDataIncrementalRefresh
struct MarketDataIncrementalRefreshEncoder {
    static std::vector<uint8_t> encode(symbol std::string, updates std::vector<std::string>, timestamp int64_t, sequence std::optional<int64_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 7);
        write_u16_be(buf, 0);
        write_u16_be(buf, 16);

        // Fixed fields
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(updates.size()));
        for (const auto& item : updates) {
            MarketDataUpdateEncoder::encode(item, buf);
        }
        write_i64_be(buf, timestamp);
        if (sequence.has_value()) { buf.push_back(1); write_i64_be(buf, *sequence); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for MarketDataIncrementalRefresh
struct MarketDataIncrementalRefreshDecoder {
    std::string symbol;
    std::vector<std::string> updates;
    int64_t timestamp;
    std::optional<int64_t> sequence;
    size_t encoded_len() const { return 0; }

    static MarketDataIncrementalRefreshDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 7) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        uint32_t updates_count = read_u32_be(buf, pos);
        std::vector<MarketDataUpdate> updates;
        updates.reserve(updates_count);
        for (uint32_t i = 0; i < updates_count; ++i) {
            auto item = MarketDataUpdateDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            updates.push_back(item);
        }
        int64_t timestamp = read_i64_be(buf, pos);
        std::optional<int64_t> sequence;
        if (buf[pos++] == 1) sequence = read_i64_be(buf, pos);
        MarketDataIncrementalRefreshDecoder out{};
        out.symbol = symbol;
        out.updates = updates;
        out.timestamp = timestamp;
        out.sequence = sequence;
        return out;
    }
};

// SBE encoder for OrderBookRequest
struct OrderBookRequestEncoder {
    static std::vector<uint8_t> encode(symbol std::string, depth std::optional<u32>, at_time std::optional<int64_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 8);
        write_u16_be(buf, 0);
        write_u16_be(buf, 12);

        // Fixed fields
        write_string(buf, symbol);
        // TODO: encode depth as u32
        if (at_time.has_value()) { buf.push_back(1); write_i64_be(buf, *at_time); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for OrderBookRequest
struct OrderBookRequestDecoder {
    std::string symbol;
    std::optional<u32> depth;
    std::optional<int64_t> at_time;
    size_t encoded_len() const { return 0; }

    static OrderBookRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 8) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        // TODO: decode depth
        std::optional<int64_t> at_time;
        if (buf[pos++] == 1) at_time = read_i64_be(buf, pos);
        OrderBookRequestDecoder out{};
        out.symbol = symbol;
        out.depth = depth;
        out.at_time = at_time;
        return out;
    }
};

// SBE encoder for OrderBookSnapshot
struct OrderBookSnapshotEncoder {
    static std::vector<uint8_t> encode(symbol std::string, exchange std::string, bids std::vector<std::string>, asks std::vector<std::string>, timestamp int64_t, sequence std::optional<int64_t>, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 9);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);

        // Fixed fields
        write_string(buf, symbol);
        write_string(buf, exchange);
        write_u32_be(buf, static_cast<uint32_t>(bids.size()));
        for (const auto& item : bids) {
            PriceLevelEncoder::encode(item, buf);
        }
        write_u32_be(buf, static_cast<uint32_t>(asks.size()));
        for (const auto& item : asks) {
            PriceLevelEncoder::encode(item, buf);
        }
        write_i64_be(buf, timestamp);
        if (sequence.has_value()) { buf.push_back(1); write_i64_be(buf, *sequence); } else { buf.push_back(0); }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for OrderBookSnapshot
struct OrderBookSnapshotDecoder {
    std::string symbol;
    std::string exchange;
    std::vector<std::string> bids;
    std::vector<std::string> asks;
    int64_t timestamp;
    std::optional<int64_t> sequence;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static OrderBookSnapshotDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 9) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::string exchange = read_string(buf, pos);
        uint32_t bids_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> bids;
        bids.reserve(bids_count);
        for (uint32_t i = 0; i < bids_count; ++i) {
            auto item = PriceLevelDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            bids.push_back(item);
        }
        uint32_t asks_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> asks;
        asks.reserve(asks_count);
        for (uint32_t i = 0; i < asks_count; ++i) {
            auto item = PriceLevelDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            asks.push_back(item);
        }
        int64_t timestamp = read_i64_be(buf, pos);
        std::optional<int64_t> sequence;
        if (buf[pos++] == 1) sequence = read_i64_be(buf, pos);
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        OrderBookSnapshotDecoder out{};
        out.symbol = symbol;
        out.exchange = exchange;
        out.bids = bids;
        out.asks = asks;
        out.timestamp = timestamp;
        out.sequence = sequence;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for OrderBookDelta
struct OrderBookDeltaEncoder {
    static std::vector<uint8_t> encode(symbol std::string, updates std::vector<std::string>, timestamp int64_t, sequence std::optional<int64_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 10);
        write_u16_be(buf, 0);
        write_u16_be(buf, 16);

        // Fixed fields
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(updates.size()));
        for (const auto& item : updates) {
            MarketDataUpdateEncoder::encode(item, buf);
        }
        write_i64_be(buf, timestamp);
        if (sequence.has_value()) { buf.push_back(1); write_i64_be(buf, *sequence); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for OrderBookDelta
struct OrderBookDeltaDecoder {
    std::string symbol;
    std::vector<std::string> updates;
    int64_t timestamp;
    std::optional<int64_t> sequence;
    size_t encoded_len() const { return 0; }

    static OrderBookDeltaDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 10) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        uint32_t updates_count = read_u32_be(buf, pos);
        std::vector<MarketDataUpdate> updates;
        updates.reserve(updates_count);
        for (uint32_t i = 0; i < updates_count; ++i) {
            auto item = MarketDataUpdateDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            updates.push_back(item);
        }
        int64_t timestamp = read_i64_be(buf, pos);
        std::optional<int64_t> sequence;
        if (buf[pos++] == 1) sequence = read_i64_be(buf, pos);
        OrderBookDeltaDecoder out{};
        out.symbol = symbol;
        out.updates = updates;
        out.timestamp = timestamp;
        out.sequence = sequence;
        return out;
    }
};

// SBE encoder for AggregateTradeEvent
struct AggregateTradeEventEncoder {
    static std::vector<uint8_t> encode(trade std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 11);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, trade);
        return buf;
    }
};

// SBE decoder for AggregateTradeEvent
struct AggregateTradeEventDecoder {
    std::string trade;
    size_t encoded_len() const { return 0; }

    static AggregateTradeEventDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 11) throw std::runtime_error("invalid template_id");
        std::string trade = read_string(buf, pos);
        AggregateTradeEventDecoder out{};
        out.trade = trade;
        return out;
    }
};

// SBE encoder for AggregateTradeRequest
struct AggregateTradeRequestEncoder {
    static std::vector<uint8_t> encode(symbol std::string, start_time std::optional<int64_t>, end_time std::optional<int64_t>, limit std::optional<u32>, cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 12);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);

        // Fixed fields
        write_string(buf, symbol);
        if (start_time.has_value()) { buf.push_back(1); write_i64_be(buf, *start_time); } else { buf.push_back(0); }
        if (end_time.has_value()) { buf.push_back(1); write_i64_be(buf, *end_time); } else { buf.push_back(0); }
        // TODO: encode limit as u32
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for AggregateTradeRequest
struct AggregateTradeRequestDecoder {
    std::string symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<u32> limit;
    std::optional<std::string> cursor;
    size_t encoded_len() const { return 0; }

    static AggregateTradeRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 12) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = read_i64_be(buf, pos);
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = read_i64_be(buf, pos);
        // TODO: decode limit
        std::optional<std::string> cursor;
        if (buf[pos++] == 1) cursor = read_string(buf, pos);
        AggregateTradeRequestDecoder out{};
        out.symbol = symbol;
        out.start_time = start_time;
        out.end_time = end_time;
        out.limit = limit;
        out.cursor = cursor;
        return out;
    }
};

// SBE encoder for AggregateTradeBatch
struct AggregateTradeBatchEncoder {
    static std::vector<uint8_t> encode(symbol std::string, trades std::vector<std::string>, has_more uint8_t, next_cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 13);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(trades.size()));
        for (const auto& item : trades) {
            AggregateTradeEncoder::encode(item, buf);
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for AggregateTradeBatch
struct AggregateTradeBatchDecoder {
    std::string symbol;
    std::vector<std::string> trades;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
    size_t encoded_len() const { return 0; }

    static AggregateTradeBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 13) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        uint32_t trades_count = read_u32_be(buf, pos);
        std::vector<AggregateTrade> trades;
        trades.reserve(trades_count);
        for (uint32_t i = 0; i < trades_count; ++i) {
            auto item = AggregateTradeDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            trades.push_back(item);
        }
        uint8_t has_more = buf[pos++];
        std::optional<std::string> next_cursor;
        if (buf[pos++] == 1) next_cursor = read_string(buf, pos);
        AggregateTradeBatchDecoder out{};
        out.symbol = symbol;
        out.trades = trades;
        out.has_more = has_more;
        out.next_cursor = next_cursor;
        return out;
    }
};

// SBE encoder for MiniTicker
struct MiniTickerEncoder {
    static std::vector<uint8_t> encode(symbol std::string, last_price double, volume double, timestamp int64_t, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 14);
        write_u16_be(buf, 0);
        write_u16_be(buf, 25);

        // Fixed fields
        write_string(buf, symbol);
        write_f64_be(buf, last_price);
        write_f64_be(buf, volume);
        write_i64_be(buf, timestamp);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for MiniTicker
struct MiniTickerDecoder {
    std::string symbol;
    double last_price;
    double volume;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static MiniTickerDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 14) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        double last_price = read_f64_be(buf, pos);
        double volume = read_f64_be(buf, pos);
        int64_t timestamp = read_i64_be(buf, pos);
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        MiniTickerDecoder out{};
        out.symbol = symbol;
        out.last_price = last_price;
        out.volume = volume;
        out.timestamp = timestamp;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for AllMidsRequest
struct AllMidsRequestEncoder {
    static std::vector<uint8_t> encode() {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 15);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        return buf;
    }
};

// SBE decoder for AllMidsRequest
struct AllMidsRequestDecoder {
    size_t encoded_len() const { return 0; }

    static AllMidsRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 15) throw std::runtime_error("invalid template_id");
        AllMidsRequestDecoder out{};

        return out;
    }
};

// SBE encoder for AllMidsBatch
struct AllMidsBatchEncoder {
    static std::vector<uint8_t> encode(tickers std::vector<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 16);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_u32_be(buf, static_cast<uint32_t>(tickers.size()));
        for (const auto& item : tickers) {
            MiniTickerEncoder::encode(item, buf);
        }
        return buf;
    }
};

// SBE decoder for AllMidsBatch
struct AllMidsBatchDecoder {
    std::vector<std::string> tickers;
    size_t encoded_len() const { return 0; }

    static AllMidsBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 16) throw std::runtime_error("invalid template_id");
        uint32_t tickers_count = read_u32_be(buf, pos);
        std::vector<MiniTicker> tickers;
        tickers.reserve(tickers_count);
        for (uint32_t i = 0; i < tickers_count; ++i) {
            auto item = MiniTickerDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            tickers.push_back(item);
        }
        AllMidsBatchDecoder out{};
        out.tickers = tickers;
        return out;
    }
};

// SBE encoder for MarkPriceUpdate
struct MarkPriceUpdateEncoder {
    static std::vector<uint8_t> encode(symbol std::string, mark_price double, index_price std::optional<double>, funding_rate std::optional<double>, timestamp int64_t, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 17);
        write_u16_be(buf, 0);
        write_u16_be(buf, 33);

        // Fixed fields
        write_string(buf, symbol);
        write_f64_be(buf, mark_price);
        write_f64_be(buf, index_price.value_or(0.0));
        buf.push_back(index_price.has_value() ? 1 : 0);
        write_f64_be(buf, funding_rate.value_or(0.0));
        buf.push_back(funding_rate.has_value() ? 1 : 0);
        write_i64_be(buf, timestamp);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for MarkPriceUpdate
struct MarkPriceUpdateDecoder {
    std::string symbol;
    double mark_price;
    std::optional<double> index_price;
    std::optional<double> funding_rate;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static MarkPriceUpdateDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 17) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        double mark_price = read_f64_be(buf, pos);
        double index_price_raw = read_f64_be(buf, pos);
        std::optional<double> index_price;
        if (buf[pos++] == 1) index_price = index_price_raw;
        double funding_rate_raw = read_f64_be(buf, pos);
        std::optional<double> funding_rate;
        if (buf[pos++] == 1) funding_rate = funding_rate_raw;
        int64_t timestamp = read_i64_be(buf, pos);
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        MarkPriceUpdateDecoder out{};
        out.symbol = symbol;
        out.mark_price = mark_price;
        out.index_price = index_price;
        out.funding_rate = funding_rate;
        out.timestamp = timestamp;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for MarkPriceRequest
struct MarkPriceRequestEncoder {
    static std::vector<uint8_t> encode(symbol std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 18);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, symbol);
        return buf;
    }
};

// SBE decoder for MarkPriceRequest
struct MarkPriceRequestDecoder {
    std::string symbol;
    size_t encoded_len() const { return 0; }

    static MarkPriceRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 18) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        MarkPriceRequestDecoder out{};
        out.symbol = symbol;
        return out;
    }
};

// SBE encoder for LiquidationTradeEvent
struct LiquidationTradeEventEncoder {
    static std::vector<uint8_t> encode(trade std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 19);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, trade);
        return buf;
    }
};

// SBE decoder for LiquidationTradeEvent
struct LiquidationTradeEventDecoder {
    std::string trade;
    size_t encoded_len() const { return 0; }

    static LiquidationTradeEventDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 19) throw std::runtime_error("invalid template_id");
        std::string trade = read_string(buf, pos);
        LiquidationTradeEventDecoder out{};
        out.trade = trade;
        return out;
    }
};

// SBE encoder for CandleBarEvent
struct CandleBarEventEncoder {
    static std::vector<uint8_t> encode(bar std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 20);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, bar);
        return buf;
    }
};

// SBE decoder for CandleBarEvent
struct CandleBarEventDecoder {
    std::string bar;
    size_t encoded_len() const { return 0; }

    static CandleBarEventDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 20) throw std::runtime_error("invalid template_id");
        std::string bar = read_string(buf, pos);
        CandleBarEventDecoder out{};
        out.bar = bar;
        return out;
    }
};

// SBE encoder for CandleBarRequest
struct CandleBarRequestEncoder {
    static std::vector<uint8_t> encode(symbol std::string, interval std::string, start_time std::optional<int64_t>, end_time std::optional<int64_t>, limit std::optional<u32>, cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 21);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);

        // Fixed fields
        write_string(buf, symbol);
        write_string(buf, interval);
        if (start_time.has_value()) { buf.push_back(1); write_i64_be(buf, *start_time); } else { buf.push_back(0); }
        if (end_time.has_value()) { buf.push_back(1); write_i64_be(buf, *end_time); } else { buf.push_back(0); }
        // TODO: encode limit as u32
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for CandleBarRequest
struct CandleBarRequestDecoder {
    std::string symbol;
    std::string interval;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<u32> limit;
    std::optional<std::string> cursor;
    size_t encoded_len() const { return 0; }

    static CandleBarRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 21) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::string interval = read_string(buf, pos);
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = read_i64_be(buf, pos);
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = read_i64_be(buf, pos);
        // TODO: decode limit
        std::optional<std::string> cursor;
        if (buf[pos++] == 1) cursor = read_string(buf, pos);
        CandleBarRequestDecoder out{};
        out.symbol = symbol;
        out.interval = interval;
        out.start_time = start_time;
        out.end_time = end_time;
        out.limit = limit;
        out.cursor = cursor;
        return out;
    }
};

// SBE encoder for CandleBarBatch
struct CandleBarBatchEncoder {
    static std::vector<uint8_t> encode(symbol std::string, interval std::string, bars std::vector<std::string>, has_more uint8_t, next_cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 22);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, symbol);
        write_string(buf, interval);
        write_u32_be(buf, static_cast<uint32_t>(bars.size()));
        for (const auto& item : bars) {
            CandleBarEncoder::encode(item, buf);
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for CandleBarBatch
struct CandleBarBatchDecoder {
    std::string symbol;
    std::string interval;
    std::vector<std::string> bars;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
    size_t encoded_len() const { return 0; }

    static CandleBarBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 22) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::string interval = read_string(buf, pos);
        uint32_t bars_count = read_u32_be(buf, pos);
        std::vector<CandleBar> bars;
        bars.reserve(bars_count);
        for (uint32_t i = 0; i < bars_count; ++i) {
            auto item = CandleBarDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            bars.push_back(item);
        }
        uint8_t has_more = buf[pos++];
        std::optional<std::string> next_cursor;
        if (buf[pos++] == 1) next_cursor = read_string(buf, pos);
        CandleBarBatchDecoder out{};
        out.symbol = symbol;
        out.interval = interval;
        out.bars = bars;
        out.has_more = has_more;
        out.next_cursor = next_cursor;
        return out;
    }
};

// SBE encoder for PublicTradeEvent
struct PublicTradeEventEncoder {
    static std::vector<uint8_t> encode(trade std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 23);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, trade);
        return buf;
    }
};

// SBE decoder for PublicTradeEvent
struct PublicTradeEventDecoder {
    std::string trade;
    size_t encoded_len() const { return 0; }

    static PublicTradeEventDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 23) throw std::runtime_error("invalid template_id");
        std::string trade = read_string(buf, pos);
        PublicTradeEventDecoder out{};
        out.trade = trade;
        return out;
    }
};

// SBE encoder for TradeHistoryRequest
struct TradeHistoryRequestEncoder {
    static std::vector<uint8_t> encode(symbol std::string, start_time std::optional<int64_t>, end_time std::optional<int64_t>, limit std::optional<u32>, cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 24);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);

        // Fixed fields
        write_string(buf, symbol);
        if (start_time.has_value()) { buf.push_back(1); write_i64_be(buf, *start_time); } else { buf.push_back(0); }
        if (end_time.has_value()) { buf.push_back(1); write_i64_be(buf, *end_time); } else { buf.push_back(0); }
        // TODO: encode limit as u32
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for TradeHistoryRequest
struct TradeHistoryRequestDecoder {
    std::string symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<u32> limit;
    std::optional<std::string> cursor;
    size_t encoded_len() const { return 0; }

    static TradeHistoryRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 24) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = read_i64_be(buf, pos);
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = read_i64_be(buf, pos);
        // TODO: decode limit
        std::optional<std::string> cursor;
        if (buf[pos++] == 1) cursor = read_string(buf, pos);
        TradeHistoryRequestDecoder out{};
        out.symbol = symbol;
        out.start_time = start_time;
        out.end_time = end_time;
        out.limit = limit;
        out.cursor = cursor;
        return out;
    }
};

// SBE encoder for PublicTradeBatch
struct PublicTradeBatchEncoder {
    static std::vector<uint8_t> encode(symbol std::string, trades std::vector<std::string>, has_more uint8_t, next_cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 25);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(trades.size()));
        for (const auto& item : trades) {
            PublicTradeEncoder::encode(item, buf);
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for PublicTradeBatch
struct PublicTradeBatchDecoder {
    std::string symbol;
    std::vector<std::string> trades;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
    size_t encoded_len() const { return 0; }

    static PublicTradeBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 25) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        uint32_t trades_count = read_u32_be(buf, pos);
        std::vector<PublicTrade> trades;
        trades.reserve(trades_count);
        for (uint32_t i = 0; i < trades_count; ++i) {
            auto item = PublicTradeDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            trades.push_back(item);
        }
        uint8_t has_more = buf[pos++];
        std::optional<std::string> next_cursor;
        if (buf[pos++] == 1) next_cursor = read_string(buf, pos);
        PublicTradeBatchDecoder out{};
        out.symbol = symbol;
        out.trades = trades;
        out.has_more = has_more;
        out.next_cursor = next_cursor;
        return out;
    }
};

// SBE encoder for BestBidOffer
struct BestBidOfferEncoder {
    static std::vector<uint8_t> encode(symbol std::string, bid_price std::optional<double>, bid_qty std::optional<double>, ask_price std::optional<double>, ask_qty std::optional<double>, timestamp int64_t, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 26);
        write_u16_be(buf, 0);
        write_u16_be(buf, 41);

        // Fixed fields
        write_string(buf, symbol);
        write_f64_be(buf, bid_price.value_or(0.0));
        buf.push_back(bid_price.has_value() ? 1 : 0);
        write_f64_be(buf, bid_qty.value_or(0.0));
        buf.push_back(bid_qty.has_value() ? 1 : 0);
        write_f64_be(buf, ask_price.value_or(0.0));
        buf.push_back(ask_price.has_value() ? 1 : 0);
        write_f64_be(buf, ask_qty.value_or(0.0));
        buf.push_back(ask_qty.has_value() ? 1 : 0);
        write_i64_be(buf, timestamp);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for BestBidOffer
struct BestBidOfferDecoder {
    std::string symbol;
    std::optional<double> bid_price;
    std::optional<double> bid_qty;
    std::optional<double> ask_price;
    std::optional<double> ask_qty;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static BestBidOfferDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 26) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        double bid_price_raw = read_f64_be(buf, pos);
        std::optional<double> bid_price;
        if (buf[pos++] == 1) bid_price = bid_price_raw;
        double bid_qty_raw = read_f64_be(buf, pos);
        std::optional<double> bid_qty;
        if (buf[pos++] == 1) bid_qty = bid_qty_raw;
        double ask_price_raw = read_f64_be(buf, pos);
        std::optional<double> ask_price;
        if (buf[pos++] == 1) ask_price = ask_price_raw;
        double ask_qty_raw = read_f64_be(buf, pos);
        std::optional<double> ask_qty;
        if (buf[pos++] == 1) ask_qty = ask_qty_raw;
        int64_t timestamp = read_i64_be(buf, pos);
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        BestBidOfferDecoder out{};
        out.symbol = symbol;
        out.bid_price = bid_price;
        out.bid_qty = bid_qty;
        out.ask_price = ask_price;
        out.ask_qty = ask_qty;
        out.timestamp = timestamp;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for SymbolTicker
struct SymbolTickerEncoder {
    static std::vector<uint8_t> encode(symbol std::string, last_price double, price_change double, price_change_pct double, volume double, high double, low double, open double, timestamp int64_t, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 27);
        write_u16_be(buf, 0);
        write_u16_be(buf, 65);

        // Fixed fields
        write_string(buf, symbol);
        write_f64_be(buf, last_price);
        write_f64_be(buf, price_change);
        write_f64_be(buf, price_change_pct);
        write_f64_be(buf, volume);
        write_f64_be(buf, high);
        write_f64_be(buf, low);
        write_f64_be(buf, open);
        write_i64_be(buf, timestamp);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for SymbolTicker
struct SymbolTickerDecoder {
    std::string symbol;
    double last_price;
    double price_change;
    double price_change_pct;
    double volume;
    double high;
    double low;
    double open;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static SymbolTickerDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 27) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        double last_price = read_f64_be(buf, pos);
        double price_change = read_f64_be(buf, pos);
        double price_change_pct = read_f64_be(buf, pos);
        double volume = read_f64_be(buf, pos);
        double high = read_f64_be(buf, pos);
        double low = read_f64_be(buf, pos);
        double open = read_f64_be(buf, pos);
        int64_t timestamp = read_i64_be(buf, pos);
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        SymbolTickerDecoder out{};
        out.symbol = symbol;
        out.last_price = last_price;
        out.price_change = price_change;
        out.price_change_pct = price_change_pct;
        out.volume = volume;
        out.high = high;
        out.low = low;
        out.open = open;
        out.timestamp = timestamp;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for TickerRequest
struct TickerRequestEncoder {
    static std::vector<uint8_t> encode(symbol std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 28);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, symbol);
        return buf;
    }
};

// SBE decoder for TickerRequest
struct TickerRequestDecoder {
    std::string symbol;
    size_t encoded_len() const { return 0; }

    static TickerRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 28) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        TickerRequestDecoder out{};
        out.symbol = symbol;
        return out;
    }
};

// SBE encoder for AccountSummary
struct AccountSummaryEncoder {
    static std::vector<uint8_t> encode(account std::string, balance double, buying_power double, currency std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 29);
        write_u16_be(buf, 0);
        write_u16_be(buf, 16);

        // Fixed fields
        write_string(buf, account);
        write_f64_be(buf, balance);
        write_f64_be(buf, buying_power);
        write_string(buf, currency);
        return buf;
    }
};

// SBE decoder for AccountSummary
struct AccountSummaryDecoder {
    std::string account;
    double balance;
    double buying_power;
    std::string currency;
    size_t encoded_len() const { return 0; }

    static AccountSummaryDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 29) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        double balance = read_f64_be(buf, pos);
        double buying_power = read_f64_be(buf, pos);
        std::string currency = read_string(buf, pos);
        AccountSummaryDecoder out{};
        out.account = account;
        out.balance = balance;
        out.buying_power = buying_power;
        out.currency = currency;
        return out;
    }
};

// SBE encoder for MarginSummary
struct MarginSummaryEncoder {
    static std::vector<uint8_t> encode(account std::string, balance double, buying_power double, equity double, margin_used double, available double, currency std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 30);
        write_u16_be(buf, 0);
        write_u16_be(buf, 40);

        // Fixed fields
        write_string(buf, account);
        write_f64_be(buf, balance);
        write_f64_be(buf, buying_power);
        write_f64_be(buf, equity);
        write_f64_be(buf, margin_used);
        write_f64_be(buf, available);
        write_string(buf, currency);
        return buf;
    }
};

// SBE decoder for MarginSummary
struct MarginSummaryDecoder {
    std::string account;
    double balance;
    double buying_power;
    double equity;
    double margin_used;
    double available;
    std::string currency;
    size_t encoded_len() const { return 0; }

    static MarginSummaryDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 30) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        double balance = read_f64_be(buf, pos);
        double buying_power = read_f64_be(buf, pos);
        double equity = read_f64_be(buf, pos);
        double margin_used = read_f64_be(buf, pos);
        double available = read_f64_be(buf, pos);
        std::string currency = read_string(buf, pos);
        MarginSummaryDecoder out{};
        out.account = account;
        out.balance = balance;
        out.buying_power = buying_power;
        out.equity = equity;
        out.margin_used = margin_used;
        out.available = available;
        out.currency = currency;
        return out;
    }
};

// SBE encoder for BalanceSnapshot
struct BalanceSnapshotEncoder {
    static std::vector<uint8_t> encode(account std::string, balances std::vector<std::string>, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 31);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(balances.size()));
        for (const auto& item : balances) {
            BalanceEntryEncoder::encode(item, buf);
        }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for BalanceSnapshot
struct BalanceSnapshotDecoder {
    std::string account;
    std::vector<std::string> balances;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static BalanceSnapshotDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 31) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t balances_count = read_u32_be(buf, pos);
        std::vector<BalanceEntry> balances;
        balances.reserve(balances_count);
        for (uint32_t i = 0; i < balances_count; ++i) {
            auto item = BalanceEntryDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            balances.push_back(item);
        }
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        BalanceSnapshotDecoder out{};
        out.account = account;
        out.balances = balances;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for BalanceUpdate
struct BalanceUpdateEncoder {
    static std::vector<uint8_t> encode(account std::string, asset std::string, delta double, total double, available double, reason BalanceUpdateReason) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 32);
        write_u16_be(buf, 0);
        write_u16_be(buf, 25);

        // Fixed fields
        write_string(buf, account);
        write_string(buf, asset);
        write_f64_be(buf, delta);
        write_f64_be(buf, total);
        write_f64_be(buf, available);
        buf.push_back(BalanceUpdateReasonToValue(reason));
        return buf;
    }
};

// SBE decoder for BalanceUpdate
struct BalanceUpdateDecoder {
    std::string account;
    std::string asset;
    double delta;
    double total;
    double available;
    BalanceUpdateReason reason;
    size_t encoded_len() const { return 0; }

    static BalanceUpdateDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 32) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string asset = read_string(buf, pos);
        double delta = read_f64_be(buf, pos);
        double total = read_f64_be(buf, pos);
        double available = read_f64_be(buf, pos);
        uint8_t reason_raw = buf[pos++];
        auto reason_opt = BalanceUpdateReasonFromValue(reason_raw);
        if (!reason_opt) throw std::runtime_error("invalid BalanceUpdateReason");
        BalanceUpdateReason reason = *reason_opt;
        BalanceUpdateDecoder out{};
        out.account = account;
        out.asset = asset;
        out.delta = delta;
        out.total = total;
        out.available = available;
        out.reason = reason;
        return out;
    }
};

// SBE encoder for PositionSnapshot
struct PositionSnapshotEncoder {
    static std::vector<uint8_t> encode(account std::string, positions std::vector<std::string>, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 33);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(positions.size()));
        for (const auto& item : positions) {
            PositionEntryEncoder::encode(item, buf);
        }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for PositionSnapshot
struct PositionSnapshotDecoder {
    std::string account;
    std::vector<std::string> positions;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static PositionSnapshotDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 33) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t positions_count = read_u32_be(buf, pos);
        std::vector<PositionEntry> positions;
        positions.reserve(positions_count);
        for (uint32_t i = 0; i < positions_count; ++i) {
            auto item = PositionEntryDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            positions.push_back(item);
        }
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        PositionSnapshotDecoder out{};
        out.account = account;
        out.positions = positions;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for PositionUpdate
struct PositionUpdateEncoder {
    static std::vector<uint8_t> encode(account std::string, symbol std::string, qty double, entry_price double, unrealized_pnl double) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 34);
        write_u16_be(buf, 0);
        write_u16_be(buf, 24);

        // Fixed fields
        write_string(buf, account);
        write_string(buf, symbol);
        write_f64_be(buf, qty);
        write_f64_be(buf, entry_price);
        write_f64_be(buf, unrealized_pnl);
        return buf;
    }
};

// SBE decoder for PositionUpdate
struct PositionUpdateDecoder {
    std::string account;
    std::string symbol;
    double qty;
    double entry_price;
    double unrealized_pnl;
    size_t encoded_len() const { return 0; }

    static PositionUpdateDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 34) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        double qty = read_f64_be(buf, pos);
        double entry_price = read_f64_be(buf, pos);
        double unrealized_pnl = read_f64_be(buf, pos);
        PositionUpdateDecoder out{};
        out.account = account;
        out.symbol = symbol;
        out.qty = qty;
        out.entry_price = entry_price;
        out.unrealized_pnl = unrealized_pnl;
        return out;
    }
};

// SBE encoder for MarginUpdate
struct MarginUpdateEncoder {
    static std::vector<uint8_t> encode(account std::string, summary std::string, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 35);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_string(buf, summary);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for MarginUpdate
struct MarginUpdateDecoder {
    std::string account;
    std::string summary;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static MarginUpdateDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 35) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string summary = read_string(buf, pos);
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        MarginUpdateDecoder out{};
        out.account = account;
        out.summary = summary;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for UserLiquidation
struct UserLiquidationEncoder {
    static std::vector<uint8_t> encode(account std::string, symbol std::string, qty double, price double, timestamp int64_t) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 36);
        write_u16_be(buf, 0);
        write_u16_be(buf, 24);

        // Fixed fields
        write_string(buf, account);
        write_string(buf, symbol);
        write_f64_be(buf, qty);
        write_f64_be(buf, price);
        write_i64_be(buf, timestamp);
        return buf;
    }
};

// SBE decoder for UserLiquidation
struct UserLiquidationDecoder {
    std::string account;
    std::string symbol;
    double qty;
    double price;
    int64_t timestamp;
    size_t encoded_len() const { return 0; }

    static UserLiquidationDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 36) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        double qty = read_f64_be(buf, pos);
        double price = read_f64_be(buf, pos);
        int64_t timestamp = read_i64_be(buf, pos);
        UserLiquidationDecoder out{};
        out.account = account;
        out.symbol = symbol;
        out.qty = qty;
        out.price = price;
        out.timestamp = timestamp;
        return out;
    }
};

// SBE encoder for OrderListStatus
struct OrderListStatusEncoder {
    static std::vector<uint8_t> encode(account std::string, list_id std::string, status OrderListStatusStatus, symbol std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 37);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_string(buf, list_id);
        buf.push_back(OrderListStatusStatusToValue(status));
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for OrderListStatus
struct OrderListStatusDecoder {
    std::string account;
    std::string list_id;
    OrderListStatusStatus status;
    std::optional<std::string> symbol;
    size_t encoded_len() const { return 0; }

    static OrderListStatusDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 37) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string list_id = read_string(buf, pos);
        uint8_t status_raw = buf[pos++];
        auto status_opt = OrderListStatusStatusFromValue(status_raw);
        if (!status_opt) throw std::runtime_error("invalid OrderListStatusStatus");
        OrderListStatusStatus status = *status_opt;
        std::optional<std::string> symbol;
        if (buf[pos++] == 1) symbol = read_string(buf, pos);
        OrderListStatusDecoder out{};
        out.account = account;
        out.list_id = list_id;
        out.status = status;
        out.symbol = symbol;
        return out;
    }
};

// SBE encoder for FillHistoryRequest
struct FillHistoryRequestEncoder {
    static std::vector<uint8_t> encode(account std::string, symbol std::optional<std::string>, start_time std::optional<int64_t>, end_time std::optional<int64_t>, limit std::optional<u32>, cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 38);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);

        // Fixed fields
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        if (start_time.has_value()) { buf.push_back(1); write_i64_be(buf, *start_time); } else { buf.push_back(0); }
        if (end_time.has_value()) { buf.push_back(1); write_i64_be(buf, *end_time); } else { buf.push_back(0); }
        // TODO: encode limit as u32
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for FillHistoryRequest
struct FillHistoryRequestDecoder {
    std::string account;
    std::optional<std::string> symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<u32> limit;
    std::optional<std::string> cursor;
    size_t encoded_len() const { return 0; }

    static FillHistoryRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 38) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::optional<std::string> symbol;
        if (buf[pos++] == 1) symbol = read_string(buf, pos);
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = read_i64_be(buf, pos);
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = read_i64_be(buf, pos);
        // TODO: decode limit
        std::optional<std::string> cursor;
        if (buf[pos++] == 1) cursor = read_string(buf, pos);
        FillHistoryRequestDecoder out{};
        out.account = account;
        out.symbol = symbol;
        out.start_time = start_time;
        out.end_time = end_time;
        out.limit = limit;
        out.cursor = cursor;
        return out;
    }
};

// SBE encoder for FillHistoryBatch
struct FillHistoryBatchEncoder {
    static std::vector<uint8_t> encode(account std::string, fills std::vector<std::string>, has_more uint8_t, next_cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 39);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(fills.size()));
        for (const auto& item : fills) {
            ExecutionReportEncoder::encode(item, buf);
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for FillHistoryBatch
struct FillHistoryBatchDecoder {
    std::string account;
    std::vector<std::string> fills;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
    size_t encoded_len() const { return 0; }

    static FillHistoryBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 39) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t fills_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> fills;
        fills.reserve(fills_count);
        for (uint32_t i = 0; i < fills_count; ++i) {
            auto item = ExecutionReportDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            fills.push_back(item);
        }
        uint8_t has_more = buf[pos++];
        std::optional<std::string> next_cursor;
        if (buf[pos++] == 1) next_cursor = read_string(buf, pos);
        FillHistoryBatchDecoder out{};
        out.account = account;
        out.fills = fills;
        out.has_more = has_more;
        out.next_cursor = next_cursor;
        return out;
    }
};

// SBE encoder for FundingPayment
struct FundingPaymentEncoder {
    static std::vector<uint8_t> encode(account std::string, symbol std::optional<std::string>, amount double, rate double, timestamp int64_t) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 40);
        write_u16_be(buf, 0);
        write_u16_be(buf, 24);

        // Fixed fields
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        write_f64_be(buf, amount);
        write_f64_be(buf, rate);
        write_i64_be(buf, timestamp);
        return buf;
    }
};

// SBE decoder for FundingPayment
struct FundingPaymentDecoder {
    std::string account;
    std::optional<std::string> symbol;
    double amount;
    double rate;
    int64_t timestamp;
    size_t encoded_len() const { return 0; }

    static FundingPaymentDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 40) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::optional<std::string> symbol;
        if (buf[pos++] == 1) symbol = read_string(buf, pos);
        double amount = read_f64_be(buf, pos);
        double rate = read_f64_be(buf, pos);
        int64_t timestamp = read_i64_be(buf, pos);
        FundingPaymentDecoder out{};
        out.account = account;
        out.symbol = symbol;
        out.amount = amount;
        out.rate = rate;
        out.timestamp = timestamp;
        return out;
    }
};

// SBE encoder for FundingHistoryRequest
struct FundingHistoryRequestEncoder {
    static std::vector<uint8_t> encode(account std::string, start_time std::optional<int64_t>, end_time std::optional<int64_t>, limit std::optional<u32>, cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 41);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);

        // Fixed fields
        write_string(buf, account);
        if (start_time.has_value()) { buf.push_back(1); write_i64_be(buf, *start_time); } else { buf.push_back(0); }
        if (end_time.has_value()) { buf.push_back(1); write_i64_be(buf, *end_time); } else { buf.push_back(0); }
        // TODO: encode limit as u32
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for FundingHistoryRequest
struct FundingHistoryRequestDecoder {
    std::string account;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<u32> limit;
    std::optional<std::string> cursor;
    size_t encoded_len() const { return 0; }

    static FundingHistoryRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 41) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = read_i64_be(buf, pos);
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = read_i64_be(buf, pos);
        // TODO: decode limit
        std::optional<std::string> cursor;
        if (buf[pos++] == 1) cursor = read_string(buf, pos);
        FundingHistoryRequestDecoder out{};
        out.account = account;
        out.start_time = start_time;
        out.end_time = end_time;
        out.limit = limit;
        out.cursor = cursor;
        return out;
    }
};

// SBE encoder for FundingHistoryBatch
struct FundingHistoryBatchEncoder {
    static std::vector<uint8_t> encode(account std::string, payments std::vector<std::string>, has_more uint8_t, next_cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 42);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(payments.size()));
        for (const auto& item : payments) {
            FundingPaymentEncoder::encode(item, buf);
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for FundingHistoryBatch
struct FundingHistoryBatchDecoder {
    std::string account;
    std::vector<std::string> payments;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
    size_t encoded_len() const { return 0; }

    static FundingHistoryBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 42) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t payments_count = read_u32_be(buf, pos);
        std::vector<FundingPayment> payments;
        payments.reserve(payments_count);
        for (uint32_t i = 0; i < payments_count; ++i) {
            auto item = FundingPaymentDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            payments.push_back(item);
        }
        uint8_t has_more = buf[pos++];
        std::optional<std::string> next_cursor;
        if (buf[pos++] == 1) next_cursor = read_string(buf, pos);
        FundingHistoryBatchDecoder out{};
        out.account = account;
        out.payments = payments;
        out.has_more = has_more;
        out.next_cursor = next_cursor;
        return out;
    }
};

// SBE encoder for LedgerUpdate
struct LedgerUpdateEncoder {
    static std::vector<uint8_t> encode(account std::string, asset std::string, delta double, kind LedgerUpdateKind, timestamp int64_t, reference_id std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 43);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);

        // Fixed fields
        write_string(buf, account);
        write_string(buf, asset);
        write_f64_be(buf, delta);
        buf.push_back(LedgerUpdateKindToValue(kind));
        write_i64_be(buf, timestamp);
        if (reference_id.has_value()) { buf.push_back(1); write_string(buf, *reference_id); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for LedgerUpdate
struct LedgerUpdateDecoder {
    std::string account;
    std::string asset;
    double delta;
    LedgerUpdateKind kind;
    int64_t timestamp;
    std::optional<std::string> reference_id;
    size_t encoded_len() const { return 0; }

    static LedgerUpdateDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 43) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string asset = read_string(buf, pos);
        double delta = read_f64_be(buf, pos);
        uint8_t kind_raw = buf[pos++];
        auto kind_opt = LedgerUpdateKindFromValue(kind_raw);
        if (!kind_opt) throw std::runtime_error("invalid LedgerUpdateKind");
        LedgerUpdateKind kind = *kind_opt;
        int64_t timestamp = read_i64_be(buf, pos);
        std::optional<std::string> reference_id;
        if (buf[pos++] == 1) reference_id = read_string(buf, pos);
        LedgerUpdateDecoder out{};
        out.account = account;
        out.asset = asset;
        out.delta = delta;
        out.kind = kind;
        out.timestamp = timestamp;
        out.reference_id = reference_id;
        return out;
    }
};

// SBE encoder for LedgerHistoryRequest
struct LedgerHistoryRequestEncoder {
    static std::vector<uint8_t> encode(account std::string, start_time std::optional<int64_t>, end_time std::optional<int64_t>, limit std::optional<u32>, cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 44);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);

        // Fixed fields
        write_string(buf, account);
        if (start_time.has_value()) { buf.push_back(1); write_i64_be(buf, *start_time); } else { buf.push_back(0); }
        if (end_time.has_value()) { buf.push_back(1); write_i64_be(buf, *end_time); } else { buf.push_back(0); }
        // TODO: encode limit as u32
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for LedgerHistoryRequest
struct LedgerHistoryRequestDecoder {
    std::string account;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<u32> limit;
    std::optional<std::string> cursor;
    size_t encoded_len() const { return 0; }

    static LedgerHistoryRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 44) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = read_i64_be(buf, pos);
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = read_i64_be(buf, pos);
        // TODO: decode limit
        std::optional<std::string> cursor;
        if (buf[pos++] == 1) cursor = read_string(buf, pos);
        LedgerHistoryRequestDecoder out{};
        out.account = account;
        out.start_time = start_time;
        out.end_time = end_time;
        out.limit = limit;
        out.cursor = cursor;
        return out;
    }
};

// SBE encoder for LedgerHistoryBatch
struct LedgerHistoryBatchEncoder {
    static std::vector<uint8_t> encode(account std::string, entries std::vector<std::string>, has_more uint8_t, next_cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 45);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(entries.size()));
        for (const auto& item : entries) {
            LedgerUpdateEncoder::encode(item, buf);
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for LedgerHistoryBatch
struct LedgerHistoryBatchDecoder {
    std::string account;
    std::vector<std::string> entries;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
    size_t encoded_len() const { return 0; }

    static LedgerHistoryBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 45) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t entries_count = read_u32_be(buf, pos);
        std::vector<LedgerUpdate> entries;
        entries.reserve(entries_count);
        for (uint32_t i = 0; i < entries_count; ++i) {
            auto item = LedgerUpdateDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            entries.push_back(item);
        }
        uint8_t has_more = buf[pos++];
        std::optional<std::string> next_cursor;
        if (buf[pos++] == 1) next_cursor = read_string(buf, pos);
        LedgerHistoryBatchDecoder out{};
        out.account = account;
        out.entries = entries;
        out.has_more = has_more;
        out.next_cursor = next_cursor;
        return out;
    }
};

// SBE encoder for OpenOrdersRequest
struct OpenOrdersRequestEncoder {
    static std::vector<uint8_t> encode(account std::string, symbol std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 46);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for OpenOrdersRequest
struct OpenOrdersRequestDecoder {
    std::string account;
    std::optional<std::string> symbol;
    size_t encoded_len() const { return 0; }

    static OpenOrdersRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 46) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::optional<std::string> symbol;
        if (buf[pos++] == 1) symbol = read_string(buf, pos);
        OpenOrdersRequestDecoder out{};
        out.account = account;
        out.symbol = symbol;
        return out;
    }
};

// SBE encoder for OpenOrdersSnapshot
struct OpenOrdersSnapshotEncoder {
    static std::vector<uint8_t> encode(account std::string, orders std::vector<std::string>, is_snapshot std::optional<uint8_t>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 47);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(orders.size()));
        for (const auto& item : orders) {
            ExecutionReportEncoder::encode(item, buf);
        }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
    }
};

// SBE decoder for OpenOrdersSnapshot
struct OpenOrdersSnapshotDecoder {
    std::string account;
    std::vector<std::string> orders;
    std::optional<uint8_t> is_snapshot;
    size_t encoded_len() const { return 0; }

    static OpenOrdersSnapshotDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 47) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t orders_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> orders;
        orders.reserve(orders_count);
        for (uint32_t i = 0; i < orders_count; ++i) {
            auto item = ExecutionReportDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            orders.push_back(item);
        }
        uint8_t v = buf[pos++];
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = v;
        OpenOrdersSnapshotDecoder out{};
        out.account = account;
        out.orders = orders;
        out.is_snapshot = is_snapshot;
        return out;
    }
};

// SBE encoder for OrderHistoryRequest
struct OrderHistoryRequestEncoder {
    static std::vector<uint8_t> encode(account std::string, symbol std::optional<std::string>, start_time std::optional<int64_t>, end_time std::optional<int64_t>, limit std::optional<u32>, cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 48);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);

        // Fixed fields
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        if (start_time.has_value()) { buf.push_back(1); write_i64_be(buf, *start_time); } else { buf.push_back(0); }
        if (end_time.has_value()) { buf.push_back(1); write_i64_be(buf, *end_time); } else { buf.push_back(0); }
        // TODO: encode limit as u32
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for OrderHistoryRequest
struct OrderHistoryRequestDecoder {
    std::string account;
    std::optional<std::string> symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<u32> limit;
    std::optional<std::string> cursor;
    size_t encoded_len() const { return 0; }

    static OrderHistoryRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 48) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::optional<std::string> symbol;
        if (buf[pos++] == 1) symbol = read_string(buf, pos);
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = read_i64_be(buf, pos);
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = read_i64_be(buf, pos);
        // TODO: decode limit
        std::optional<std::string> cursor;
        if (buf[pos++] == 1) cursor = read_string(buf, pos);
        OrderHistoryRequestDecoder out{};
        out.account = account;
        out.symbol = symbol;
        out.start_time = start_time;
        out.end_time = end_time;
        out.limit = limit;
        out.cursor = cursor;
        return out;
    }
};

// SBE encoder for OrderHistoryBatch
struct OrderHistoryBatchEncoder {
    static std::vector<uint8_t> encode(account std::string, orders std::vector<std::string>, has_more uint8_t, next_cursor std::optional<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 49);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);

        // Fixed fields
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(orders.size()));
        for (const auto& item : orders) {
            ExecutionReportEncoder::encode(item, buf);
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
    }
};

// SBE decoder for OrderHistoryBatch
struct OrderHistoryBatchDecoder {
    std::string account;
    std::vector<std::string> orders;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
    size_t encoded_len() const { return 0; }

    static OrderHistoryBatchDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 49) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t orders_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> orders;
        orders.reserve(orders_count);
        for (uint32_t i = 0; i < orders_count; ++i) {
            auto item = ExecutionReportDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            orders.push_back(item);
        }
        uint8_t has_more = buf[pos++];
        std::optional<std::string> next_cursor;
        if (buf[pos++] == 1) next_cursor = read_string(buf, pos);
        OrderHistoryBatchDecoder out{};
        out.account = account;
        out.orders = orders;
        out.has_more = has_more;
        out.next_cursor = next_cursor;
        return out;
    }
};

// SBE encoder for CapabilitiesRequest
struct CapabilitiesRequestEncoder {
    static std::vector<uint8_t> encode() {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 50);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        return buf;
    }
};

// SBE decoder for CapabilitiesRequest
struct CapabilitiesRequestDecoder {
    size_t encoded_len() const { return 0; }

    static CapabilitiesRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 50) throw std::runtime_error("invalid template_id");
        CapabilitiesRequestDecoder out{};

        return out;
    }
};

// SBE encoder for CapabilitiesResponse
struct CapabilitiesResponseEncoder {
    static std::vector<uint8_t> encode(schema_ids std::vector<std::string>, paths std::vector<std::string>, symbols std::vector<std::string>, intervals std::vector<std::string>) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 51);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_u32_be(buf, static_cast<uint32_t>(schema_ids.size()));
        for (const auto& item : schema_ids) {
            u8Encoder::encode(item, buf);
        }
        write_u32_be(buf, static_cast<uint32_t>(paths.size()));
        for (const auto& item : paths) {
            CapabilityPathEncoder::encode(item, buf);
        }
        write_u32_be(buf, static_cast<uint32_t>(symbols.size()));
        for (const auto& item : symbols) {
            SymbolEncoder::encode(item, buf);
        }
        write_u32_be(buf, static_cast<uint32_t>(intervals.size()));
        for (const auto& item : intervals) {
            CandleIntervalEncoder::encode(item, buf);
        }
        return buf;
    }
};

// SBE decoder for CapabilitiesResponse
struct CapabilitiesResponseDecoder {
    std::vector<std::string> schema_ids;
    std::vector<std::string> paths;
    std::vector<std::string> symbols;
    std::vector<std::string> intervals;
    size_t encoded_len() const { return 0; }

    static CapabilitiesResponseDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 51) throw std::runtime_error("invalid template_id");
        uint32_t schema_ids_count = read_u32_be(buf, pos);
        std::vector<u8> schema_ids;
        schema_ids.reserve(schema_ids_count);
        for (uint32_t i = 0; i < schema_ids_count; ++i) {
            auto item = u8Decoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            schema_ids.push_back(item);
        }
        uint32_t paths_count = read_u32_be(buf, pos);
        std::vector<CapabilityPath> paths;
        paths.reserve(paths_count);
        for (uint32_t i = 0; i < paths_count; ++i) {
            auto item = CapabilityPathDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            paths.push_back(item);
        }
        uint32_t symbols_count = read_u32_be(buf, pos);
        std::vector<Symbol> symbols;
        symbols.reserve(symbols_count);
        for (uint32_t i = 0; i < symbols_count; ++i) {
            auto item = SymbolDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            symbols.push_back(item);
        }
        uint32_t intervals_count = read_u32_be(buf, pos);
        std::vector<CandleInterval> intervals;
        intervals.reserve(intervals_count);
        for (uint32_t i = 0; i < intervals_count; ++i) {
            auto item = CandleIntervalDecoder::decode(std::vector<uint8_t>(buf.begin() + pos, buf.end()));
            pos += item.encoded_len();
            intervals.push_back(item);
        }
        CapabilitiesResponseDecoder out{};
        out.schema_ids = schema_ids;
        out.paths = paths;
        out.symbols = symbols;
        out.intervals = intervals;
        return out;
    }
};

// SBE encoder for PositionRequest
struct PositionRequestEncoder {
    static std::vector<uint8_t> encode(account std::string) {
        std::vector<uint8_t> buf;
        // SBE Message Header (8 bytes)
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 52);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);

        // Fixed fields
        write_string(buf, account);
        return buf;
    }
};

// SBE decoder for PositionRequest
struct PositionRequestDecoder {
    std::string account;
    size_t encoded_len() const { return 0; }

    static PositionRequestDecoder decode(const std::vector<uint8_t>& buf) {
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        read_u16_be(buf, pos); read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 52) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        PositionRequestDecoder out{};
        out.account = account;
        return out;
    }
};

}
