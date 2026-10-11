// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' v1.0.0
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

inline uint8_t take_u8(const std::vector<uint8_t>& buf, std::size_t& pos) { return buf[pos++]; }
inline float f32_from_bits(uint32_t bits) { float v; std::memcpy(&v, &bits, 4); return v; }
inline uint32_t f32_bits(float v) { uint32_t bits; std::memcpy(&bits, &v, 4); return bits; }

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

inline void write_string_u32(std::vector<uint8_t>& buf, const std::string& s) { write_u32_be(buf, static_cast<uint32_t>(s.size())); buf.insert(buf.end(), s.begin(), s.end()); }
inline std::string read_string_u32(const std::vector<uint8_t>& buf, std::size_t& pos) { uint32_t n = read_u32_be(buf, pos); std::string s(reinterpret_cast<const char*>(buf.data() + pos), n); pos += n; return s; }

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

enum class TradingSessionStatusStatus : uint8_t {
    PreOpen = 1,
    Open = 2,
    Break = 3,
    Close = 4,
    Halt = 5,
    Auction = 6,
};

inline std::optional<TradingSessionStatusStatus> TradingSessionStatusStatusFromValue(uint8_t v) {
    switch (v) {
    case 1: return TradingSessionStatusStatus::PreOpen;
    case 2: return TradingSessionStatusStatus::Open;
    case 3: return TradingSessionStatusStatus::Break;
    case 4: return TradingSessionStatusStatus::Close;
    case 5: return TradingSessionStatusStatus::Halt;
    case 6: return TradingSessionStatusStatus::Auction;
    default: return std::nullopt;
    }
}

inline uint8_t TradingSessionStatusStatusToValue(TradingSessionStatusStatus e) { return static_cast<uint8_t>(e); }

enum class TradingSessionStatusHaltReason : uint8_t {
    None = 1,
    News = 2,
    MatchingEngine = 3,
    Regulatory = 4,
    Auction = 5,
    Other = 6,
};

inline std::optional<TradingSessionStatusHaltReason> TradingSessionStatusHaltReasonFromValue(uint8_t v) {
    switch (v) {
    case 1: return TradingSessionStatusHaltReason::None;
    case 2: return TradingSessionStatusHaltReason::News;
    case 3: return TradingSessionStatusHaltReason::MatchingEngine;
    case 4: return TradingSessionStatusHaltReason::Regulatory;
    case 5: return TradingSessionStatusHaltReason::Auction;
    case 6: return TradingSessionStatusHaltReason::Other;
    default: return std::nullopt;
    }
}

inline uint8_t TradingSessionStatusHaltReasonToValue(TradingSessionStatusHaltReason e) { return static_cast<uint8_t>(e); }

enum class SecurityStatusStatus : uint8_t {
    Trading = 1,
    Halt = 2,
    Auction = 3,
    Suspended = 4,
};

inline std::optional<SecurityStatusStatus> SecurityStatusStatusFromValue(uint8_t v) {
    switch (v) {
    case 1: return SecurityStatusStatus::Trading;
    case 2: return SecurityStatusStatus::Halt;
    case 3: return SecurityStatusStatus::Auction;
    case 4: return SecurityStatusStatus::Suspended;
    default: return std::nullopt;
    }
}

inline uint8_t SecurityStatusStatusToValue(SecurityStatusStatus e) { return static_cast<uint8_t>(e); }

enum class SecurityStatusHaltReason : uint8_t {
    None = 1,
    News = 2,
    MatchingEngine = 3,
    Regulatory = 4,
    Auction = 5,
    Other = 6,
};

inline std::optional<SecurityStatusHaltReason> SecurityStatusHaltReasonFromValue(uint8_t v) {
    switch (v) {
    case 1: return SecurityStatusHaltReason::None;
    case 2: return SecurityStatusHaltReason::News;
    case 3: return SecurityStatusHaltReason::MatchingEngine;
    case 4: return SecurityStatusHaltReason::Regulatory;
    case 5: return SecurityStatusHaltReason::Auction;
    case 6: return SecurityStatusHaltReason::Other;
    default: return std::nullopt;
    }
}

inline uint8_t SecurityStatusHaltReasonToValue(SecurityStatusHaltReason e) { return static_cast<uint8_t>(e); }

enum class MassCancelRequestSide : uint8_t {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
    SellShortExempt = 4,
};

inline std::optional<MassCancelRequestSide> MassCancelRequestSideFromValue(uint8_t v) {
    switch (v) {
    case 1: return MassCancelRequestSide::Buy;
    case 2: return MassCancelRequestSide::Sell;
    case 3: return MassCancelRequestSide::SellShort;
    case 4: return MassCancelRequestSide::SellShortExempt;
    default: return std::nullopt;
    }
}

inline uint8_t MassCancelRequestSideToValue(MassCancelRequestSide e) { return static_cast<uint8_t>(e); }

enum class MassCancelReportStatus : uint8_t {
    Accepted = 1,
    Completed = 2,
    Rejected = 3,
};

inline std::optional<MassCancelReportStatus> MassCancelReportStatusFromValue(uint8_t v) {
    switch (v) {
    case 1: return MassCancelReportStatus::Accepted;
    case 2: return MassCancelReportStatus::Completed;
    case 3: return MassCancelReportStatus::Rejected;
    default: return std::nullopt;
    }
}

inline uint8_t MassCancelReportStatusToValue(MassCancelReportStatus e) { return static_cast<uint8_t>(e); }

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

// SBE struct AggregateTrade
struct AggregateTrade {
    std::string symbol;
    std::string agg_trade_id;
    double price;
    double qty;
    AggregateTradeSide side;
    std::string first_trade_id;
    std::string last_trade_id;
    int64_t timestamp;
};

struct AggregateTradeEncoder {
    static void encode(const AggregateTrade& value, std::vector<uint8_t>& buf);
};

struct AggregateTradeDecoder {
    static std::pair<AggregateTrade, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void AggregateTradeEncoder::encode(const AggregateTrade& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_string(buf, value.agg_trade_id);
        write_f64_be(buf, value.price);
        write_f64_be(buf, value.qty);
        buf.push_back(AggregateTradeSideToValue(value.side));
        write_string(buf, value.first_trade_id);
        write_string(buf, value.last_trade_id);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
}

inline std::pair<AggregateTrade, std::size_t> AggregateTradeDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    AggregateTrade value{};
        std::string symbol = read_string(buf, pos);
        std::string agg_trade_id = read_string(buf, pos);
        double price = read_f64_be(buf, pos);
        double qty = read_f64_be(buf, pos);
        auto side_v = AggregateTradeSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid AggregateTradeSide");
        AggregateTradeSide side = *side_v;
        std::string first_trade_id = read_string(buf, pos);
        std::string last_trade_id = read_string(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.symbol = symbol;
    value.agg_trade_id = agg_trade_id;
    value.price = price;
    value.qty = qty;
    value.side = side;
    value.first_trade_id = first_trade_id;
    value.last_trade_id = last_trade_id;
    value.timestamp = timestamp;
    return {value, pos};
}

// SBE struct LiquidationTrade
struct LiquidationTrade {
    std::string symbol;
    LiquidationTradeSide side;
    double price;
    double qty;
    int64_t timestamp;
};

struct LiquidationTradeEncoder {
    static void encode(const LiquidationTrade& value, std::vector<uint8_t>& buf);
};

struct LiquidationTradeDecoder {
    static std::pair<LiquidationTrade, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void LiquidationTradeEncoder::encode(const LiquidationTrade& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        buf.push_back(LiquidationTradeSideToValue(value.side));
        write_f64_be(buf, value.price);
        write_f64_be(buf, value.qty);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
}

inline std::pair<LiquidationTrade, std::size_t> LiquidationTradeDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    LiquidationTrade value{};
        std::string symbol = read_string(buf, pos);
        auto side_v = LiquidationTradeSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid LiquidationTradeSide");
        LiquidationTradeSide side = *side_v;
        double price = read_f64_be(buf, pos);
        double qty = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.symbol = symbol;
    value.side = side;
    value.price = price;
    value.qty = qty;
    value.timestamp = timestamp;
    return {value, pos};
}

// SBE struct PriceLevel
struct PriceLevel {
    double price;
    double qty;
    std::optional<uint32_t> order_count;
};

struct PriceLevelEncoder {
    static void encode(const PriceLevel& value, std::vector<uint8_t>& buf);
};

struct PriceLevelDecoder {
    static std::pair<PriceLevel, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void PriceLevelEncoder::encode(const PriceLevel& value, std::vector<uint8_t>& buf) {
        write_f64_be(buf, value.price);
        write_f64_be(buf, value.qty);
        write_u32_be(buf, value.order_count.value_or(0));
        buf.push_back(value.order_count.has_value() ? 1 : 0);
}

inline std::pair<PriceLevel, std::size_t> PriceLevelDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PriceLevel value{};
        double price = read_f64_be(buf, pos);
        double qty = read_f64_be(buf, pos);
        uint32_t order_count_v = read_u32_be(buf, pos);
        std::optional<uint32_t> order_count;
        if (buf[pos++] == 1) order_count = order_count_v;
    value.price = price;
    value.qty = qty;
    value.order_count = order_count;
    return {value, pos};
}

// SBE struct MarketDataUpdate
struct MarketDataUpdate {
    MarketDataUpdateSide side;
    MarketDataUpdateAction action;
    double price;
    double qty;
};

struct MarketDataUpdateEncoder {
    static void encode(const MarketDataUpdate& value, std::vector<uint8_t>& buf);
};

struct MarketDataUpdateDecoder {
    static std::pair<MarketDataUpdate, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void MarketDataUpdateEncoder::encode(const MarketDataUpdate& value, std::vector<uint8_t>& buf) {
        buf.push_back(MarketDataUpdateSideToValue(value.side));
        buf.push_back(MarketDataUpdateActionToValue(value.action));
        write_f64_be(buf, value.price);
        write_f64_be(buf, value.qty);
}

inline std::pair<MarketDataUpdate, std::size_t> MarketDataUpdateDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MarketDataUpdate value{};
        auto side_v = MarketDataUpdateSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid MarketDataUpdateSide");
        MarketDataUpdateSide side = *side_v;
        auto action_v = MarketDataUpdateActionFromValue(take_u8(buf, pos));
        if (!action_v) throw std::runtime_error("invalid MarketDataUpdateAction");
        MarketDataUpdateAction action = *action_v;
        double price = read_f64_be(buf, pos);
        double qty = read_f64_be(buf, pos);
    value.side = side;
    value.action = action;
    value.price = price;
    value.qty = qty;
    return {value, pos};
}

// SBE struct CandleBar
struct CandleBar {
    std::string symbol;
    std::string interval;
    double open;
    double high;
    double low;
    double close;
    double volume;
    int64_t bar_start;
    int64_t bar_end;
    uint8_t is_final;
    std::optional<uint8_t> is_snapshot;
};

struct CandleBarEncoder {
    static void encode(const CandleBar& value, std::vector<uint8_t>& buf);
};

struct CandleBarDecoder {
    static std::pair<CandleBar, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void CandleBarEncoder::encode(const CandleBar& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_string(buf, value.interval);
        write_f64_be(buf, value.open);
        write_f64_be(buf, value.high);
        write_f64_be(buf, value.low);
        write_f64_be(buf, value.close);
        write_f64_be(buf, value.volume);
        write_u64_be(buf, static_cast<uint64_t>(value.bar_start));
        write_u64_be(buf, static_cast<uint64_t>(value.bar_end));
        buf.push_back(value.is_final);
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

inline std::pair<CandleBar, std::size_t> CandleBarDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CandleBar value{};
        std::string symbol = read_string(buf, pos);
        std::string interval = read_string(buf, pos);
        double open = read_f64_be(buf, pos);
        double high = read_f64_be(buf, pos);
        double low = read_f64_be(buf, pos);
        double close = read_f64_be(buf, pos);
        double volume = read_f64_be(buf, pos);
        int64_t bar_start = static_cast<int64_t>(read_u64_be(buf, pos));
        int64_t bar_end = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_final = take_u8(buf, pos);
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
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
    return {value, pos};
}

// SBE struct PublicTrade
struct PublicTrade {
    std::string symbol;
    std::string trade_id;
    double price;
    double qty;
    PublicTradeSide side;
    int64_t timestamp;
};

struct PublicTradeEncoder {
    static void encode(const PublicTrade& value, std::vector<uint8_t>& buf);
};

struct PublicTradeDecoder {
    static std::pair<PublicTrade, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void PublicTradeEncoder::encode(const PublicTrade& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_string(buf, value.trade_id);
        write_f64_be(buf, value.price);
        write_f64_be(buf, value.qty);
        buf.push_back(PublicTradeSideToValue(value.side));
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
}

inline std::pair<PublicTrade, std::size_t> PublicTradeDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PublicTrade value{};
        std::string symbol = read_string(buf, pos);
        std::string trade_id = read_string(buf, pos);
        double price = read_f64_be(buf, pos);
        double qty = read_f64_be(buf, pos);
        auto side_v = PublicTradeSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid PublicTradeSide");
        PublicTradeSide side = *side_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.symbol = symbol;
    value.trade_id = trade_id;
    value.price = price;
    value.qty = qty;
    value.side = side;
    value.timestamp = timestamp;
    return {value, pos};
}

// SBE struct InstrumentMetadata
struct InstrumentMetadata {
    std::string instrument_id;
    std::string symbol;
    std::string product_kind;
    std::optional<std::string> margin_asset;
    std::optional<std::string> display_name;
    std::optional<double> tick_size;
    std::optional<double> lot_size;
};

struct InstrumentMetadataEncoder {
    static void encode(const InstrumentMetadata& value, std::vector<uint8_t>& buf);
};

struct InstrumentMetadataDecoder {
    static std::pair<InstrumentMetadata, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void InstrumentMetadataEncoder::encode(const InstrumentMetadata& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.instrument_id);
        write_string(buf, value.symbol);
        write_string(buf, value.product_kind);
        if (value.margin_asset.has_value()) { buf.push_back(1); write_string(buf, *value.margin_asset); } else { buf.push_back(0); }
        if (value.display_name.has_value()) { buf.push_back(1); write_string(buf, *value.display_name); } else { buf.push_back(0); }
        write_f64_be(buf, value.tick_size.value_or(0.0));
        buf.push_back(value.tick_size.has_value() ? 1 : 0);
        write_f64_be(buf, value.lot_size.value_or(0.0));
        buf.push_back(value.lot_size.has_value() ? 1 : 0);
}

inline std::pair<InstrumentMetadata, std::size_t> InstrumentMetadataDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    InstrumentMetadata value{};
        std::optional<std::string> margin_asset;
        std::optional<std::string> display_name;
        std::string instrument_id = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        std::string product_kind = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) margin_asset = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) display_name = read_string(buf, pos);
        double tick_size_v = read_f64_be(buf, pos);
        std::optional<double> tick_size;
        if (buf[pos++] == 1) tick_size = tick_size_v;
        double lot_size_v = read_f64_be(buf, pos);
        std::optional<double> lot_size;
        if (buf[pos++] == 1) lot_size = lot_size_v;
    value.instrument_id = instrument_id;
    value.symbol = symbol;
    value.product_kind = product_kind;
    value.margin_asset = margin_asset;
    value.display_name = display_name;
    value.tick_size = tick_size;
    value.lot_size = lot_size;
    return {value, pos};
}

// SBE struct BalanceEntry
struct BalanceEntry {
    std::string asset;
    double total;
    double available;
    double hold;
};

struct BalanceEntryEncoder {
    static void encode(const BalanceEntry& value, std::vector<uint8_t>& buf);
};

struct BalanceEntryDecoder {
    static std::pair<BalanceEntry, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void BalanceEntryEncoder::encode(const BalanceEntry& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.asset);
        write_f64_be(buf, value.total);
        write_f64_be(buf, value.available);
        write_f64_be(buf, value.hold);
}

inline std::pair<BalanceEntry, std::size_t> BalanceEntryDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    BalanceEntry value{};
        std::string asset = read_string(buf, pos);
        double total = read_f64_be(buf, pos);
        double available = read_f64_be(buf, pos);
        double hold = read_f64_be(buf, pos);
    value.asset = asset;
    value.total = total;
    value.available = available;
    value.hold = hold;
    return {value, pos};
}

// SBE struct PositionEntry
struct PositionEntry {
    std::string symbol;
    double qty;
    double entry_price;
    double unrealized_pnl;
};

struct PositionEntryEncoder {
    static void encode(const PositionEntry& value, std::vector<uint8_t>& buf);
};

struct PositionEntryDecoder {
    static std::pair<PositionEntry, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void PositionEntryEncoder::encode(const PositionEntry& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_f64_be(buf, value.qty);
        write_f64_be(buf, value.entry_price);
        write_f64_be(buf, value.unrealized_pnl);
}

inline std::pair<PositionEntry, std::size_t> PositionEntryDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PositionEntry value{};
        std::string symbol = read_string(buf, pos);
        double qty = read_f64_be(buf, pos);
        double entry_price = read_f64_be(buf, pos);
        double unrealized_pnl = read_f64_be(buf, pos);
    value.symbol = symbol;
    value.qty = qty;
    value.entry_price = entry_price;
    value.unrealized_pnl = unrealized_pnl;
    return {value, pos};
}

// SBE struct PageInfo
struct PageInfo {
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

struct PageInfoEncoder {
    static void encode(const PageInfo& value, std::vector<uint8_t>& buf);
};

struct PageInfoDecoder {
    static std::pair<PageInfo, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void PageInfoEncoder::encode(const PageInfo& value, std::vector<uint8_t>& buf) {
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

inline std::pair<PageInfo, std::size_t> PageInfoDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PageInfo value{};
        std::optional<std::string> next_cursor;
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE struct CapabilityPath
struct CapabilityPath {
    std::string path;
    CapabilityPathPattern pattern;
    uint8_t auth_required;
};

struct CapabilityPathEncoder {
    static void encode(const CapabilityPath& value, std::vector<uint8_t>& buf);
};

struct CapabilityPathDecoder {
    static std::pair<CapabilityPath, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline void CapabilityPathEncoder::encode(const CapabilityPath& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.path);
        buf.push_back(CapabilityPathPatternToValue(value.pattern));
        buf.push_back(value.auth_required);
}

inline std::pair<CapabilityPath, std::size_t> CapabilityPathDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CapabilityPath value{};
        std::string path = read_string(buf, pos);
        auto pattern_v = CapabilityPathPatternFromValue(take_u8(buf, pos));
        if (!pattern_v) throw std::runtime_error("invalid CapabilityPathPattern");
        CapabilityPathPattern pattern = *pattern_v;
        uint8_t auth_required = take_u8(buf, pos);
    value.path = path;
    value.pattern = pattern;
    value.auth_required = auth_required;
    return {value, pos};
}

// SBE message NewOrderSingle
struct NewOrderSingle {
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
    std::optional<uint8_t> post_only;
    std::optional<uint8_t> reduce_only;
};

// SBE body encoder for NewOrderSingle (fields only, no header).
struct NewOrderSingleBodyEncoder {
    static void encode(const NewOrderSingle& value, std::vector<uint8_t>& buf);
};

inline void NewOrderSingleBodyEncoder::encode(const NewOrderSingle& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.cl_ord_id);
        buf.push_back(NewOrderSingleSideToValue(value.side));
        write_f64_be(buf, value.order_qty);
        write_f64_be(buf, value.price.value_or(0.0));
        buf.push_back(value.price.has_value() ? 1 : 0);
        write_f64_be(buf, value.stop_price.value_or(0.0));
        buf.push_back(value.stop_price.has_value() ? 1 : 0);
        write_string(buf, value.symbol);
        buf.push_back(NewOrderSingleOrderTypeToValue(value.order_type));
        buf.push_back(NewOrderSingleTimeInForceToValue(value.time_in_force));
        write_u64_be(buf, static_cast<uint64_t>(value.expire_time.value_or(0)));
        buf.push_back(value.expire_time.has_value() ? 1 : 0);
        if (value.account.has_value()) { buf.push_back(1); write_string(buf, *value.account); } else { buf.push_back(0); }
        if (value.strategy_id.has_value()) { buf.push_back(1); write_string(buf, *value.strategy_id); } else { buf.push_back(0); }
        if (value.security_id.has_value()) { buf.push_back(1); write_string(buf, *value.security_id); } else { buf.push_back(0); }
        buf.push_back(value.id_source.has_value() ? NewOrderSingleIdSourceToValue(*value.id_source) : 0);
        buf.push_back(value.id_source.has_value() ? 1 : 0);
        if (value.security_exchange.has_value()) { buf.push_back(1); write_string(buf, *value.security_exchange); } else { buf.push_back(0); }
        buf.push_back(value.post_only.value_or(0));
        buf.push_back(value.post_only.has_value() ? 1 : 0);
        buf.push_back(value.reduce_only.value_or(0));
        buf.push_back(value.reduce_only.has_value() ? 1 : 0);
}

// SBE body decoder for NewOrderSingle: reads fields at pos.
struct NewOrderSingleBodyDecoder {
    static std::pair<NewOrderSingle, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<NewOrderSingle, std::size_t> NewOrderSingleBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    NewOrderSingle value{};
        std::optional<std::string> account;
        std::optional<std::string> strategy_id;
        std::optional<std::string> security_id;
        std::optional<std::string> security_exchange;
        std::string cl_ord_id = read_string(buf, pos);
        auto side_v = NewOrderSingleSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid NewOrderSingleSide");
        NewOrderSingleSide side = *side_v;
        double order_qty = read_f64_be(buf, pos);
        double price_v = read_f64_be(buf, pos);
        std::optional<double> price;
        if (buf[pos++] == 1) price = price_v;
        double stop_price_v = read_f64_be(buf, pos);
        std::optional<double> stop_price;
        if (buf[pos++] == 1) stop_price = stop_price_v;
        std::string symbol = read_string(buf, pos);
        auto order_type_v = NewOrderSingleOrderTypeFromValue(take_u8(buf, pos));
        if (!order_type_v) throw std::runtime_error("invalid NewOrderSingleOrderType");
        NewOrderSingleOrderType order_type = *order_type_v;
        auto time_in_force_v = NewOrderSingleTimeInForceFromValue(take_u8(buf, pos));
        if (!time_in_force_v) throw std::runtime_error("invalid NewOrderSingleTimeInForce");
        NewOrderSingleTimeInForce time_in_force = *time_in_force_v;
        int64_t expire_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> expire_time;
        if (buf[pos++] == 1) expire_time = expire_time_v;
        if (take_u8(buf, pos) == 1) account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) strategy_id = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) security_id = read_string(buf, pos);
        auto id_source_raw = take_u8(buf, pos);
        std::optional<NewOrderSingleIdSource> id_source;
        if (buf[pos++] == 1) {
            auto v = NewOrderSingleIdSourceFromValue(id_source_raw);
            if (!v) throw std::runtime_error("invalid NewOrderSingleIdSource");
            id_source = *v;
        }
        if (take_u8(buf, pos) == 1) security_exchange = read_string(buf, pos);
        uint8_t post_only_v = take_u8(buf, pos);
        std::optional<uint8_t> post_only;
        if (buf[pos++] == 1) post_only = post_only_v;
        uint8_t reduce_only_v = take_u8(buf, pos);
        std::optional<uint8_t> reduce_only;
        if (buf[pos++] == 1) reduce_only = reduce_only_v;
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
    return {value, pos};
}

// SBE encoder for NewOrderSingle
struct NewOrderSingleEncoder {
    static std::vector<uint8_t> encode(std::string cl_ord_id, NewOrderSingleSide side, double order_qty, std::optional<double> price, std::optional<double> stop_price, std::string symbol, NewOrderSingleOrderType order_type, NewOrderSingleTimeInForce time_in_force, std::optional<int64_t> expire_time, std::optional<std::string> account, std::optional<std::string> strategy_id, std::optional<std::string> security_id, std::optional<NewOrderSingleIdSource> id_source, std::optional<std::string> security_exchange, std::optional<uint8_t> post_only, std::optional<uint8_t> reduce_only);
};

inline std::vector<uint8_t> NewOrderSingleEncoder::encode(std::string cl_ord_id, NewOrderSingleSide side, double order_qty, std::optional<double> price, std::optional<double> stop_price, std::string symbol, NewOrderSingleOrderType order_type, NewOrderSingleTimeInForce time_in_force, std::optional<int64_t> expire_time, std::optional<std::string> account, std::optional<std::string> strategy_id, std::optional<std::string> security_id, std::optional<NewOrderSingleIdSource> id_source, std::optional<std::string> security_exchange, std::optional<uint8_t> post_only, std::optional<uint8_t> reduce_only) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 1);
        write_u16_be(buf, 0);
        write_u16_be(buf, 38);
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
        write_u64_be(buf, static_cast<uint64_t>(expire_time.value_or(0)));
        buf.push_back(expire_time.has_value() ? 1 : 0);
        if (account.has_value()) { buf.push_back(1); write_string(buf, *account); } else { buf.push_back(0); }
        if (strategy_id.has_value()) { buf.push_back(1); write_string(buf, *strategy_id); } else { buf.push_back(0); }
        if (security_id.has_value()) { buf.push_back(1); write_string(buf, *security_id); } else { buf.push_back(0); }
        buf.push_back(id_source.has_value() ? NewOrderSingleIdSourceToValue(*id_source) : 0);
        buf.push_back(id_source.has_value() ? 1 : 0);
        if (security_exchange.has_value()) { buf.push_back(1); write_string(buf, *security_exchange); } else { buf.push_back(0); }
        buf.push_back(post_only.value_or(0));
        buf.push_back(post_only.has_value() ? 1 : 0);
        buf.push_back(reduce_only.value_or(0));
        buf.push_back(reduce_only.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for NewOrderSingle: validates the header and reads the body.
struct NewOrderSingleDecoder {
    static NewOrderSingle decode(const std::vector<uint8_t>& buf);
};

inline NewOrderSingle NewOrderSingleDecoder::decode(const std::vector<uint8_t>& buf) {
    NewOrderSingle value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 1) throw std::runtime_error("invalid template_id");
        std::optional<std::string> account;
        std::optional<std::string> strategy_id;
        std::optional<std::string> security_id;
        std::optional<std::string> security_exchange;
        std::string cl_ord_id = read_string(buf, pos);
        auto side_v = NewOrderSingleSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid NewOrderSingleSide");
        NewOrderSingleSide side = *side_v;
        double order_qty = read_f64_be(buf, pos);
        double price_v = read_f64_be(buf, pos);
        std::optional<double> price;
        if (buf[pos++] == 1) price = price_v;
        double stop_price_v = read_f64_be(buf, pos);
        std::optional<double> stop_price;
        if (buf[pos++] == 1) stop_price = stop_price_v;
        std::string symbol = read_string(buf, pos);
        auto order_type_v = NewOrderSingleOrderTypeFromValue(take_u8(buf, pos));
        if (!order_type_v) throw std::runtime_error("invalid NewOrderSingleOrderType");
        NewOrderSingleOrderType order_type = *order_type_v;
        auto time_in_force_v = NewOrderSingleTimeInForceFromValue(take_u8(buf, pos));
        if (!time_in_force_v) throw std::runtime_error("invalid NewOrderSingleTimeInForce");
        NewOrderSingleTimeInForce time_in_force = *time_in_force_v;
        int64_t expire_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> expire_time;
        if (buf[pos++] == 1) expire_time = expire_time_v;
        if (take_u8(buf, pos) == 1) account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) strategy_id = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) security_id = read_string(buf, pos);
        auto id_source_raw = take_u8(buf, pos);
        std::optional<NewOrderSingleIdSource> id_source;
        if (buf[pos++] == 1) {
            auto v = NewOrderSingleIdSourceFromValue(id_source_raw);
            if (!v) throw std::runtime_error("invalid NewOrderSingleIdSource");
            id_source = *v;
        }
        if (take_u8(buf, pos) == 1) security_exchange = read_string(buf, pos);
        uint8_t post_only_v = take_u8(buf, pos);
        std::optional<uint8_t> post_only;
        if (buf[pos++] == 1) post_only = post_only_v;
        uint8_t reduce_only_v = take_u8(buf, pos);
        std::optional<uint8_t> reduce_only;
        if (buf[pos++] == 1) reduce_only = reduce_only_v;
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
    (void)value;
    return value;
}

// SBE message CancelRequest
struct CancelRequest {
    std::string cl_ord_id;
    std::string orig_cl_ord_id;
    std::string symbol;
    CancelRequestSide side;
    std::optional<double> order_qty;
};

// SBE body encoder for CancelRequest (fields only, no header).
struct CancelRequestBodyEncoder {
    static void encode(const CancelRequest& value, std::vector<uint8_t>& buf);
};

inline void CancelRequestBodyEncoder::encode(const CancelRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.cl_ord_id);
        write_string(buf, value.orig_cl_ord_id);
        write_string(buf, value.symbol);
        buf.push_back(CancelRequestSideToValue(value.side));
        write_f64_be(buf, value.order_qty.value_or(0.0));
        buf.push_back(value.order_qty.has_value() ? 1 : 0);
}

// SBE body decoder for CancelRequest: reads fields at pos.
struct CancelRequestBodyDecoder {
    static std::pair<CancelRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CancelRequest, std::size_t> CancelRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CancelRequest value{};
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        auto side_v = CancelRequestSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid CancelRequestSide");
        CancelRequestSide side = *side_v;
        double order_qty_v = read_f64_be(buf, pos);
        std::optional<double> order_qty;
        if (buf[pos++] == 1) order_qty = order_qty_v;
    value.cl_ord_id = cl_ord_id;
    value.orig_cl_ord_id = orig_cl_ord_id;
    value.symbol = symbol;
    value.side = side;
    value.order_qty = order_qty;
    return {value, pos};
}

// SBE encoder for CancelRequest
struct CancelRequestEncoder {
    static std::vector<uint8_t> encode(std::string cl_ord_id, std::string orig_cl_ord_id, std::string symbol, CancelRequestSide side, std::optional<double> order_qty);
};

inline std::vector<uint8_t> CancelRequestEncoder::encode(std::string cl_ord_id, std::string orig_cl_ord_id, std::string symbol, CancelRequestSide side, std::optional<double> order_qty) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 2);
        write_u16_be(buf, 0);
        write_u16_be(buf, 9);
        write_string(buf, cl_ord_id);
        write_string(buf, orig_cl_ord_id);
        write_string(buf, symbol);
        buf.push_back(CancelRequestSideToValue(side));
        write_f64_be(buf, order_qty.value_or(0.0));
        buf.push_back(order_qty.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for CancelRequest: validates the header and reads the body.
struct CancelRequestDecoder {
    static CancelRequest decode(const std::vector<uint8_t>& buf);
};

inline CancelRequest CancelRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    CancelRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 2) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        auto side_v = CancelRequestSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid CancelRequestSide");
        CancelRequestSide side = *side_v;
        double order_qty_v = read_f64_be(buf, pos);
        std::optional<double> order_qty;
        if (buf[pos++] == 1) order_qty = order_qty_v;
    value.cl_ord_id = cl_ord_id;
    value.orig_cl_ord_id = orig_cl_ord_id;
    value.symbol = symbol;
    value.side = side;
    value.order_qty = order_qty;
    (void)value;
    return value;
}

// SBE message CancelReplaceRequest
struct CancelReplaceRequest {
    std::string cl_ord_id;
    std::string orig_cl_ord_id;
    std::string symbol;
    CancelReplaceRequestSide side;
    double order_qty;
    std::optional<double> price;
};

// SBE body encoder for CancelReplaceRequest (fields only, no header).
struct CancelReplaceRequestBodyEncoder {
    static void encode(const CancelReplaceRequest& value, std::vector<uint8_t>& buf);
};

inline void CancelReplaceRequestBodyEncoder::encode(const CancelReplaceRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.cl_ord_id);
        write_string(buf, value.orig_cl_ord_id);
        write_string(buf, value.symbol);
        buf.push_back(CancelReplaceRequestSideToValue(value.side));
        write_f64_be(buf, value.order_qty);
        write_f64_be(buf, value.price.value_or(0.0));
        buf.push_back(value.price.has_value() ? 1 : 0);
}

// SBE body decoder for CancelReplaceRequest: reads fields at pos.
struct CancelReplaceRequestBodyDecoder {
    static std::pair<CancelReplaceRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CancelReplaceRequest, std::size_t> CancelReplaceRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CancelReplaceRequest value{};
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        auto side_v = CancelReplaceRequestSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid CancelReplaceRequestSide");
        CancelReplaceRequestSide side = *side_v;
        double order_qty = read_f64_be(buf, pos);
        double price_v = read_f64_be(buf, pos);
        std::optional<double> price;
        if (buf[pos++] == 1) price = price_v;
    value.cl_ord_id = cl_ord_id;
    value.orig_cl_ord_id = orig_cl_ord_id;
    value.symbol = symbol;
    value.side = side;
    value.order_qty = order_qty;
    value.price = price;
    return {value, pos};
}

// SBE encoder for CancelReplaceRequest
struct CancelReplaceRequestEncoder {
    static std::vector<uint8_t> encode(std::string cl_ord_id, std::string orig_cl_ord_id, std::string symbol, CancelReplaceRequestSide side, double order_qty, std::optional<double> price);
};

inline std::vector<uint8_t> CancelReplaceRequestEncoder::encode(std::string cl_ord_id, std::string orig_cl_ord_id, std::string symbol, CancelReplaceRequestSide side, double order_qty, std::optional<double> price) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 3);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);
        write_string(buf, cl_ord_id);
        write_string(buf, orig_cl_ord_id);
        write_string(buf, symbol);
        buf.push_back(CancelReplaceRequestSideToValue(side));
        write_f64_be(buf, order_qty);
        write_f64_be(buf, price.value_or(0.0));
        buf.push_back(price.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for CancelReplaceRequest: validates the header and reads the body.
struct CancelReplaceRequestDecoder {
    static CancelReplaceRequest decode(const std::vector<uint8_t>& buf);
};

inline CancelReplaceRequest CancelReplaceRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    CancelReplaceRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 3) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        auto side_v = CancelReplaceRequestSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid CancelReplaceRequestSide");
        CancelReplaceRequestSide side = *side_v;
        double order_qty = read_f64_be(buf, pos);
        double price_v = read_f64_be(buf, pos);
        std::optional<double> price;
        if (buf[pos++] == 1) price = price_v;
    value.cl_ord_id = cl_ord_id;
    value.orig_cl_ord_id = orig_cl_ord_id;
    value.symbol = symbol;
    value.side = side;
    value.order_qty = order_qty;
    value.price = price;
    (void)value;
    return value;
}

// SBE message ExecutionReport
struct ExecutionReport {
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
};

// SBE body encoder for ExecutionReport (fields only, no header).
struct ExecutionReportBodyEncoder {
    static void encode(const ExecutionReport& value, std::vector<uint8_t>& buf);
};

inline void ExecutionReportBodyEncoder::encode(const ExecutionReport& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.cl_ord_id);
        write_string(buf, value.order_id);
        write_string(buf, value.exec_id);
        buf.push_back(ExecutionReportExecTypeToValue(value.exec_type));
        buf.push_back(ExecutionReportOrdStatusToValue(value.ord_status));
        buf.push_back(ExecutionReportSideToValue(value.side));
        write_f64_be(buf, value.last_qty.value_or(0.0));
        buf.push_back(value.last_qty.has_value() ? 1 : 0);
        write_f64_be(buf, value.last_price.value_or(0.0));
        buf.push_back(value.last_price.has_value() ? 1 : 0);
        write_f64_be(buf, value.leaves_qty);
        write_f64_be(buf, value.cum_qty);
        write_f64_be(buf, value.avg_price);
        write_string(buf, value.symbol);
        write_u64_be(buf, static_cast<uint64_t>(value.transact_time));
}

// SBE body decoder for ExecutionReport: reads fields at pos.
struct ExecutionReportBodyDecoder {
    static std::pair<ExecutionReport, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<ExecutionReport, std::size_t> ExecutionReportBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    ExecutionReport value{};
        std::string cl_ord_id = read_string(buf, pos);
        std::string order_id = read_string(buf, pos);
        std::string exec_id = read_string(buf, pos);
        auto exec_type_v = ExecutionReportExecTypeFromValue(take_u8(buf, pos));
        if (!exec_type_v) throw std::runtime_error("invalid ExecutionReportExecType");
        ExecutionReportExecType exec_type = *exec_type_v;
        auto ord_status_v = ExecutionReportOrdStatusFromValue(take_u8(buf, pos));
        if (!ord_status_v) throw std::runtime_error("invalid ExecutionReportOrdStatus");
        ExecutionReportOrdStatus ord_status = *ord_status_v;
        auto side_v = ExecutionReportSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid ExecutionReportSide");
        ExecutionReportSide side = *side_v;
        double last_qty_v = read_f64_be(buf, pos);
        std::optional<double> last_qty;
        if (buf[pos++] == 1) last_qty = last_qty_v;
        double last_price_v = read_f64_be(buf, pos);
        std::optional<double> last_price;
        if (buf[pos++] == 1) last_price = last_price_v;
        double leaves_qty = read_f64_be(buf, pos);
        double cum_qty = read_f64_be(buf, pos);
        double avg_price = read_f64_be(buf, pos);
        std::string symbol = read_string(buf, pos);
        int64_t transact_time = static_cast<int64_t>(read_u64_be(buf, pos));
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
    return {value, pos};
}

// SBE encoder for ExecutionReport
struct ExecutionReportEncoder {
    static std::vector<uint8_t> encode(std::string cl_ord_id, std::string order_id, std::string exec_id, ExecutionReportExecType exec_type, ExecutionReportOrdStatus ord_status, ExecutionReportSide side, std::optional<double> last_qty, std::optional<double> last_price, double leaves_qty, double cum_qty, double avg_price, std::string symbol, int64_t transact_time);
};

inline std::vector<uint8_t> ExecutionReportEncoder::encode(std::string cl_ord_id, std::string order_id, std::string exec_id, ExecutionReportExecType exec_type, ExecutionReportOrdStatus ord_status, ExecutionReportSide side, std::optional<double> last_qty, std::optional<double> last_price, double leaves_qty, double cum_qty, double avg_price, std::string symbol, int64_t transact_time) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 4);
        write_u16_be(buf, 0);
        write_u16_be(buf, 51);
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
        write_u64_be(buf, static_cast<uint64_t>(transact_time));
        return buf;
}

// SBE decoder for ExecutionReport: validates the header and reads the body.
struct ExecutionReportDecoder {
    static ExecutionReport decode(const std::vector<uint8_t>& buf);
};

inline ExecutionReport ExecutionReportDecoder::decode(const std::vector<uint8_t>& buf) {
    ExecutionReport value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 4) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string order_id = read_string(buf, pos);
        std::string exec_id = read_string(buf, pos);
        auto exec_type_v = ExecutionReportExecTypeFromValue(take_u8(buf, pos));
        if (!exec_type_v) throw std::runtime_error("invalid ExecutionReportExecType");
        ExecutionReportExecType exec_type = *exec_type_v;
        auto ord_status_v = ExecutionReportOrdStatusFromValue(take_u8(buf, pos));
        if (!ord_status_v) throw std::runtime_error("invalid ExecutionReportOrdStatus");
        ExecutionReportOrdStatus ord_status = *ord_status_v;
        auto side_v = ExecutionReportSideFromValue(take_u8(buf, pos));
        if (!side_v) throw std::runtime_error("invalid ExecutionReportSide");
        ExecutionReportSide side = *side_v;
        double last_qty_v = read_f64_be(buf, pos);
        std::optional<double> last_qty;
        if (buf[pos++] == 1) last_qty = last_qty_v;
        double last_price_v = read_f64_be(buf, pos);
        std::optional<double> last_price;
        if (buf[pos++] == 1) last_price = last_price_v;
        double leaves_qty = read_f64_be(buf, pos);
        double cum_qty = read_f64_be(buf, pos);
        double avg_price = read_f64_be(buf, pos);
        std::string symbol = read_string(buf, pos);
        int64_t transact_time = static_cast<int64_t>(read_u64_be(buf, pos));
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
    (void)value;
    return value;
}

// SBE message CancelReject
struct CancelReject {
    std::string cl_ord_id;
    std::string orig_cl_ord_id;
    CancelRejectRejectReason reject_reason;
    std::string symbol;
};

// SBE body encoder for CancelReject (fields only, no header).
struct CancelRejectBodyEncoder {
    static void encode(const CancelReject& value, std::vector<uint8_t>& buf);
};

inline void CancelRejectBodyEncoder::encode(const CancelReject& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.cl_ord_id);
        write_string(buf, value.orig_cl_ord_id);
        buf.push_back(CancelRejectRejectReasonToValue(value.reject_reason));
        write_string(buf, value.symbol);
}

// SBE body decoder for CancelReject: reads fields at pos.
struct CancelRejectBodyDecoder {
    static std::pair<CancelReject, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CancelReject, std::size_t> CancelRejectBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CancelReject value{};
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        auto reject_reason_v = CancelRejectRejectReasonFromValue(take_u8(buf, pos));
        if (!reject_reason_v) throw std::runtime_error("invalid CancelRejectRejectReason");
        CancelRejectRejectReason reject_reason = *reject_reason_v;
        std::string symbol = read_string(buf, pos);
    value.cl_ord_id = cl_ord_id;
    value.orig_cl_ord_id = orig_cl_ord_id;
    value.reject_reason = reject_reason;
    value.symbol = symbol;
    return {value, pos};
}

// SBE encoder for CancelReject
struct CancelRejectEncoder {
    static std::vector<uint8_t> encode(std::string cl_ord_id, std::string orig_cl_ord_id, CancelRejectRejectReason reject_reason, std::string symbol);
};

inline std::vector<uint8_t> CancelRejectEncoder::encode(std::string cl_ord_id, std::string orig_cl_ord_id, CancelRejectRejectReason reject_reason, std::string symbol) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 5);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, cl_ord_id);
        write_string(buf, orig_cl_ord_id);
        buf.push_back(CancelRejectRejectReasonToValue(reject_reason));
        write_string(buf, symbol);
        return buf;
}

// SBE decoder for CancelReject: validates the header and reads the body.
struct CancelRejectDecoder {
    static CancelReject decode(const std::vector<uint8_t>& buf);
};

inline CancelReject CancelRejectDecoder::decode(const std::vector<uint8_t>& buf) {
    CancelReject value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 5) throw std::runtime_error("invalid template_id");
        std::string cl_ord_id = read_string(buf, pos);
        std::string orig_cl_ord_id = read_string(buf, pos);
        auto reject_reason_v = CancelRejectRejectReasonFromValue(take_u8(buf, pos));
        if (!reject_reason_v) throw std::runtime_error("invalid CancelRejectRejectReason");
        CancelRejectRejectReason reject_reason = *reject_reason_v;
        std::string symbol = read_string(buf, pos);
    value.cl_ord_id = cl_ord_id;
    value.orig_cl_ord_id = orig_cl_ord_id;
    value.reject_reason = reject_reason;
    value.symbol = symbol;
    (void)value;
    return value;
}

// SBE message MarketDataSnapshot
struct MarketDataSnapshot {
    std::string symbol;
    std::string exchange;
    std::vector<PriceLevel> bids;
    std::vector<PriceLevel> asks;
    int64_t timestamp;
    std::optional<uint64_t> sequence;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for MarketDataSnapshot (fields only, no header).
struct MarketDataSnapshotBodyEncoder {
    static void encode(const MarketDataSnapshot& value, std::vector<uint8_t>& buf);
};

inline void MarketDataSnapshotBodyEncoder::encode(const MarketDataSnapshot& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_string(buf, value.exchange);
        write_u32_be(buf, static_cast<uint32_t>(value.bids.size()));
        for (const auto& item : value.bids) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u32_be(buf, static_cast<uint32_t>(value.asks.size()));
        for (const auto& item : value.asks) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        write_u64_be(buf, value.sequence.value_or(0));
        buf.push_back(value.sequence.has_value() ? 1 : 0);
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for MarketDataSnapshot: reads fields at pos.
struct MarketDataSnapshotBodyDecoder {
    static std::pair<MarketDataSnapshot, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MarketDataSnapshot, std::size_t> MarketDataSnapshotBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MarketDataSnapshot value{};
        std::string symbol = read_string(buf, pos);
        std::string exchange = read_string(buf, pos);
        uint32_t bids_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> bids;
        bids.reserve(bids_count);
        for (uint32_t i = 0; i < bids_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            bids.push_back(item);
        }
        uint32_t asks_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> asks;
        asks.reserve(asks_count);
        for (uint32_t i = 0; i < asks_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            asks.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.exchange = exchange;
    value.bids = bids;
    value.asks = asks;
    value.timestamp = timestamp;
    value.sequence = sequence;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for MarketDataSnapshot
struct MarketDataSnapshotEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::string exchange, std::vector<PriceLevel> bids, std::vector<PriceLevel> asks, int64_t timestamp, std::optional<uint64_t> sequence, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> MarketDataSnapshotEncoder::encode(std::string symbol, std::string exchange, std::vector<PriceLevel> bids, std::vector<PriceLevel> asks, int64_t timestamp, std::optional<uint64_t> sequence, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 6);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);
        write_string(buf, symbol);
        write_string(buf, exchange);
        write_u32_be(buf, static_cast<uint32_t>(bids.size()));
        for (const auto& item : bids) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u32_be(buf, static_cast<uint32_t>(asks.size()));
        for (const auto& item : asks) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        write_u64_be(buf, sequence.value_or(0));
        buf.push_back(sequence.has_value() ? 1 : 0);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for MarketDataSnapshot: validates the header and reads the body.
struct MarketDataSnapshotDecoder {
    static MarketDataSnapshot decode(const std::vector<uint8_t>& buf);
};

inline MarketDataSnapshot MarketDataSnapshotDecoder::decode(const std::vector<uint8_t>& buf) {
    MarketDataSnapshot value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 6) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::string exchange = read_string(buf, pos);
        uint32_t bids_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> bids;
        bids.reserve(bids_count);
        for (uint32_t i = 0; i < bids_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            bids.push_back(item);
        }
        uint32_t asks_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> asks;
        asks.reserve(asks_count);
        for (uint32_t i = 0; i < asks_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            asks.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.exchange = exchange;
    value.bids = bids;
    value.asks = asks;
    value.timestamp = timestamp;
    value.sequence = sequence;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message MarketDataIncrementalRefresh
struct MarketDataIncrementalRefresh {
    std::string symbol;
    std::vector<MarketDataUpdate> updates;
    int64_t timestamp;
    std::optional<uint64_t> sequence;
};

// SBE body encoder for MarketDataIncrementalRefresh (fields only, no header).
struct MarketDataIncrementalRefreshBodyEncoder {
    static void encode(const MarketDataIncrementalRefresh& value, std::vector<uint8_t>& buf);
};

inline void MarketDataIncrementalRefreshBodyEncoder::encode(const MarketDataIncrementalRefresh& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_u32_be(buf, static_cast<uint32_t>(value.updates.size()));
        for (const auto& item : value.updates) {
            std::vector<uint8_t> item_buf;
            MarketDataUpdateEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        write_u64_be(buf, value.sequence.value_or(0));
        buf.push_back(value.sequence.has_value() ? 1 : 0);
}

// SBE body decoder for MarketDataIncrementalRefresh: reads fields at pos.
struct MarketDataIncrementalRefreshBodyDecoder {
    static std::pair<MarketDataIncrementalRefresh, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MarketDataIncrementalRefresh, std::size_t> MarketDataIncrementalRefreshBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MarketDataIncrementalRefresh value{};
        std::string symbol = read_string(buf, pos);
        uint32_t updates_count = read_u32_be(buf, pos);
        std::vector<MarketDataUpdate> updates;
        updates.reserve(updates_count);
        for (uint32_t i = 0; i < updates_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = MarketDataUpdateDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            updates.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
    value.symbol = symbol;
    value.updates = updates;
    value.timestamp = timestamp;
    value.sequence = sequence;
    return {value, pos};
}

// SBE encoder for MarketDataIncrementalRefresh
struct MarketDataIncrementalRefreshEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::vector<MarketDataUpdate> updates, int64_t timestamp, std::optional<uint64_t> sequence);
};

inline std::vector<uint8_t> MarketDataIncrementalRefreshEncoder::encode(std::string symbol, std::vector<MarketDataUpdate> updates, int64_t timestamp, std::optional<uint64_t> sequence) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 7);
        write_u16_be(buf, 0);
        write_u16_be(buf, 16);
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(updates.size()));
        for (const auto& item : updates) {
            std::vector<uint8_t> item_buf;
            MarketDataUpdateEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        write_u64_be(buf, sequence.value_or(0));
        buf.push_back(sequence.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for MarketDataIncrementalRefresh: validates the header and reads the body.
struct MarketDataIncrementalRefreshDecoder {
    static MarketDataIncrementalRefresh decode(const std::vector<uint8_t>& buf);
};

inline MarketDataIncrementalRefresh MarketDataIncrementalRefreshDecoder::decode(const std::vector<uint8_t>& buf) {
    MarketDataIncrementalRefresh value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 7) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        uint32_t updates_count = read_u32_be(buf, pos);
        std::vector<MarketDataUpdate> updates;
        updates.reserve(updates_count);
        for (uint32_t i = 0; i < updates_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = MarketDataUpdateDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            updates.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
    value.symbol = symbol;
    value.updates = updates;
    value.timestamp = timestamp;
    value.sequence = sequence;
    (void)value;
    return value;
}

// SBE message OrderBookRequest
struct OrderBookRequest {
    std::string symbol;
    std::optional<uint32_t> depth;
    std::optional<int64_t> at_time;
};

// SBE body encoder for OrderBookRequest (fields only, no header).
struct OrderBookRequestBodyEncoder {
    static void encode(const OrderBookRequest& value, std::vector<uint8_t>& buf);
};

inline void OrderBookRequestBodyEncoder::encode(const OrderBookRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_u32_be(buf, value.depth.value_or(0));
        buf.push_back(value.depth.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.at_time.value_or(0)));
        buf.push_back(value.at_time.has_value() ? 1 : 0);
}

// SBE body decoder for OrderBookRequest: reads fields at pos.
struct OrderBookRequestBodyDecoder {
    static std::pair<OrderBookRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OrderBookRequest, std::size_t> OrderBookRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OrderBookRequest value{};
        std::string symbol = read_string(buf, pos);
        uint32_t depth_v = read_u32_be(buf, pos);
        std::optional<uint32_t> depth;
        if (buf[pos++] == 1) depth = depth_v;
        int64_t at_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> at_time;
        if (buf[pos++] == 1) at_time = at_time_v;
    value.symbol = symbol;
    value.depth = depth;
    value.at_time = at_time;
    return {value, pos};
}

// SBE encoder for OrderBookRequest
struct OrderBookRequestEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::optional<uint32_t> depth, std::optional<int64_t> at_time);
};

inline std::vector<uint8_t> OrderBookRequestEncoder::encode(std::string symbol, std::optional<uint32_t> depth, std::optional<int64_t> at_time) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 8);
        write_u16_be(buf, 0);
        write_u16_be(buf, 12);
        write_string(buf, symbol);
        write_u32_be(buf, depth.value_or(0));
        buf.push_back(depth.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(at_time.value_or(0)));
        buf.push_back(at_time.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for OrderBookRequest: validates the header and reads the body.
struct OrderBookRequestDecoder {
    static OrderBookRequest decode(const std::vector<uint8_t>& buf);
};

inline OrderBookRequest OrderBookRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    OrderBookRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 8) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        uint32_t depth_v = read_u32_be(buf, pos);
        std::optional<uint32_t> depth;
        if (buf[pos++] == 1) depth = depth_v;
        int64_t at_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> at_time;
        if (buf[pos++] == 1) at_time = at_time_v;
    value.symbol = symbol;
    value.depth = depth;
    value.at_time = at_time;
    (void)value;
    return value;
}

// SBE message OrderBookSnapshot
struct OrderBookSnapshot {
    std::string symbol;
    std::string exchange;
    std::vector<PriceLevel> bids;
    std::vector<PriceLevel> asks;
    int64_t timestamp;
    std::optional<uint64_t> sequence;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for OrderBookSnapshot (fields only, no header).
struct OrderBookSnapshotBodyEncoder {
    static void encode(const OrderBookSnapshot& value, std::vector<uint8_t>& buf);
};

inline void OrderBookSnapshotBodyEncoder::encode(const OrderBookSnapshot& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_string(buf, value.exchange);
        write_u32_be(buf, static_cast<uint32_t>(value.bids.size()));
        for (const auto& item : value.bids) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u32_be(buf, static_cast<uint32_t>(value.asks.size()));
        for (const auto& item : value.asks) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        write_u64_be(buf, value.sequence.value_or(0));
        buf.push_back(value.sequence.has_value() ? 1 : 0);
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for OrderBookSnapshot: reads fields at pos.
struct OrderBookSnapshotBodyDecoder {
    static std::pair<OrderBookSnapshot, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OrderBookSnapshot, std::size_t> OrderBookSnapshotBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OrderBookSnapshot value{};
        std::string symbol = read_string(buf, pos);
        std::string exchange = read_string(buf, pos);
        uint32_t bids_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> bids;
        bids.reserve(bids_count);
        for (uint32_t i = 0; i < bids_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            bids.push_back(item);
        }
        uint32_t asks_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> asks;
        asks.reserve(asks_count);
        for (uint32_t i = 0; i < asks_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            asks.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.exchange = exchange;
    value.bids = bids;
    value.asks = asks;
    value.timestamp = timestamp;
    value.sequence = sequence;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for OrderBookSnapshot
struct OrderBookSnapshotEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::string exchange, std::vector<PriceLevel> bids, std::vector<PriceLevel> asks, int64_t timestamp, std::optional<uint64_t> sequence, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> OrderBookSnapshotEncoder::encode(std::string symbol, std::string exchange, std::vector<PriceLevel> bids, std::vector<PriceLevel> asks, int64_t timestamp, std::optional<uint64_t> sequence, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 9);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);
        write_string(buf, symbol);
        write_string(buf, exchange);
        write_u32_be(buf, static_cast<uint32_t>(bids.size()));
        for (const auto& item : bids) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u32_be(buf, static_cast<uint32_t>(asks.size()));
        for (const auto& item : asks) {
            std::vector<uint8_t> item_buf;
            PriceLevelEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        write_u64_be(buf, sequence.value_or(0));
        buf.push_back(sequence.has_value() ? 1 : 0);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for OrderBookSnapshot: validates the header and reads the body.
struct OrderBookSnapshotDecoder {
    static OrderBookSnapshot decode(const std::vector<uint8_t>& buf);
};

inline OrderBookSnapshot OrderBookSnapshotDecoder::decode(const std::vector<uint8_t>& buf) {
    OrderBookSnapshot value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 9) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        std::string exchange = read_string(buf, pos);
        uint32_t bids_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> bids;
        bids.reserve(bids_count);
        for (uint32_t i = 0; i < bids_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            bids.push_back(item);
        }
        uint32_t asks_count = read_u32_be(buf, pos);
        std::vector<PriceLevel> asks;
        asks.reserve(asks_count);
        for (uint32_t i = 0; i < asks_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PriceLevelDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            asks.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.exchange = exchange;
    value.bids = bids;
    value.asks = asks;
    value.timestamp = timestamp;
    value.sequence = sequence;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message OrderBookDelta
struct OrderBookDelta {
    std::string symbol;
    std::vector<MarketDataUpdate> updates;
    int64_t timestamp;
    std::optional<uint64_t> sequence;
};

// SBE body encoder for OrderBookDelta (fields only, no header).
struct OrderBookDeltaBodyEncoder {
    static void encode(const OrderBookDelta& value, std::vector<uint8_t>& buf);
};

inline void OrderBookDeltaBodyEncoder::encode(const OrderBookDelta& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_u32_be(buf, static_cast<uint32_t>(value.updates.size()));
        for (const auto& item : value.updates) {
            std::vector<uint8_t> item_buf;
            MarketDataUpdateEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        write_u64_be(buf, value.sequence.value_or(0));
        buf.push_back(value.sequence.has_value() ? 1 : 0);
}

// SBE body decoder for OrderBookDelta: reads fields at pos.
struct OrderBookDeltaBodyDecoder {
    static std::pair<OrderBookDelta, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OrderBookDelta, std::size_t> OrderBookDeltaBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OrderBookDelta value{};
        std::string symbol = read_string(buf, pos);
        uint32_t updates_count = read_u32_be(buf, pos);
        std::vector<MarketDataUpdate> updates;
        updates.reserve(updates_count);
        for (uint32_t i = 0; i < updates_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = MarketDataUpdateDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            updates.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
    value.symbol = symbol;
    value.updates = updates;
    value.timestamp = timestamp;
    value.sequence = sequence;
    return {value, pos};
}

// SBE encoder for OrderBookDelta
struct OrderBookDeltaEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::vector<MarketDataUpdate> updates, int64_t timestamp, std::optional<uint64_t> sequence);
};

inline std::vector<uint8_t> OrderBookDeltaEncoder::encode(std::string symbol, std::vector<MarketDataUpdate> updates, int64_t timestamp, std::optional<uint64_t> sequence) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 10);
        write_u16_be(buf, 0);
        write_u16_be(buf, 16);
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(updates.size()));
        for (const auto& item : updates) {
            std::vector<uint8_t> item_buf;
            MarketDataUpdateEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        write_u64_be(buf, sequence.value_or(0));
        buf.push_back(sequence.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for OrderBookDelta: validates the header and reads the body.
struct OrderBookDeltaDecoder {
    static OrderBookDelta decode(const std::vector<uint8_t>& buf);
};

inline OrderBookDelta OrderBookDeltaDecoder::decode(const std::vector<uint8_t>& buf) {
    OrderBookDelta value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 10) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        uint32_t updates_count = read_u32_be(buf, pos);
        std::vector<MarketDataUpdate> updates;
        updates.reserve(updates_count);
        for (uint32_t i = 0; i < updates_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = MarketDataUpdateDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            updates.push_back(item);
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint64_t sequence_v = read_u64_be(buf, pos);
        std::optional<uint64_t> sequence;
        if (buf[pos++] == 1) sequence = sequence_v;
    value.symbol = symbol;
    value.updates = updates;
    value.timestamp = timestamp;
    value.sequence = sequence;
    (void)value;
    return value;
}

// SBE message AggregateTradeEvent
struct AggregateTradeEvent {
    AggregateTrade trade;
};

// SBE body encoder for AggregateTradeEvent (fields only, no header).
struct AggregateTradeEventBodyEncoder {
    static void encode(const AggregateTradeEvent& value, std::vector<uint8_t>& buf);
};

inline void AggregateTradeEventBodyEncoder::encode(const AggregateTradeEvent& value, std::vector<uint8_t>& buf) {
        AggregateTradeEncoder::encode(value.trade, buf);
}

// SBE body decoder for AggregateTradeEvent: reads fields at pos.
struct AggregateTradeEventBodyDecoder {
    static std::pair<AggregateTradeEvent, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<AggregateTradeEvent, std::size_t> AggregateTradeEventBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    AggregateTradeEvent value{};
        auto [trade, trade_n] = AggregateTradeDecoder::decode(buf, pos);
        pos += trade_n;
    value.trade = trade;
    return {value, pos};
}

// SBE encoder for AggregateTradeEvent
struct AggregateTradeEventEncoder {
    static std::vector<uint8_t> encode(AggregateTrade trade);
};

inline std::vector<uint8_t> AggregateTradeEventEncoder::encode(AggregateTrade trade) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 11);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        AggregateTradeEncoder::encode(trade, buf);
        return buf;
}

// SBE decoder for AggregateTradeEvent: validates the header and reads the body.
struct AggregateTradeEventDecoder {
    static AggregateTradeEvent decode(const std::vector<uint8_t>& buf);
};

inline AggregateTradeEvent AggregateTradeEventDecoder::decode(const std::vector<uint8_t>& buf) {
    AggregateTradeEvent value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 11) throw std::runtime_error("invalid template_id");
        auto [trade, trade_n] = AggregateTradeDecoder::decode(buf, pos);
        pos += trade_n;
    value.trade = trade;
    (void)value;
    return value;
}

// SBE message AggregateTradeRequest
struct AggregateTradeRequest {
    std::string symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<uint32_t> limit;
    std::optional<std::string> cursor;
};

// SBE body encoder for AggregateTradeRequest (fields only, no header).
struct AggregateTradeRequestBodyEncoder {
    static void encode(const AggregateTradeRequest& value, std::vector<uint8_t>& buf);
};

inline void AggregateTradeRequestBodyEncoder::encode(const AggregateTradeRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_u64_be(buf, static_cast<uint64_t>(value.start_time.value_or(0)));
        buf.push_back(value.start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.end_time.value_or(0)));
        buf.push_back(value.end_time.has_value() ? 1 : 0);
        write_u32_be(buf, value.limit.value_or(0));
        buf.push_back(value.limit.has_value() ? 1 : 0);
        if (value.cursor.has_value()) { buf.push_back(1); write_string(buf, *value.cursor); } else { buf.push_back(0); }
}

// SBE body decoder for AggregateTradeRequest: reads fields at pos.
struct AggregateTradeRequestBodyDecoder {
    static std::pair<AggregateTradeRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<AggregateTradeRequest, std::size_t> AggregateTradeRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    AggregateTradeRequest value{};
        std::optional<std::string> cursor;
        std::string symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    return {value, pos};
}

// SBE encoder for AggregateTradeRequest
struct AggregateTradeRequestEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor);
};

inline std::vector<uint8_t> AggregateTradeRequestEncoder::encode(std::string symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 12);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);
        write_string(buf, symbol);
        write_u64_be(buf, static_cast<uint64_t>(start_time.value_or(0)));
        buf.push_back(start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(end_time.value_or(0)));
        buf.push_back(end_time.has_value() ? 1 : 0);
        write_u32_be(buf, limit.value_or(0));
        buf.push_back(limit.has_value() ? 1 : 0);
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for AggregateTradeRequest: validates the header and reads the body.
struct AggregateTradeRequestDecoder {
    static AggregateTradeRequest decode(const std::vector<uint8_t>& buf);
};

inline AggregateTradeRequest AggregateTradeRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    AggregateTradeRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 12) throw std::runtime_error("invalid template_id");
        std::optional<std::string> cursor;
        std::string symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    (void)value;
    return value;
}

// SBE message AggregateTradeBatch
struct AggregateTradeBatch {
    std::string symbol;
    std::vector<AggregateTrade> trades;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

// SBE body encoder for AggregateTradeBatch (fields only, no header).
struct AggregateTradeBatchBodyEncoder {
    static void encode(const AggregateTradeBatch& value, std::vector<uint8_t>& buf);
};

inline void AggregateTradeBatchBodyEncoder::encode(const AggregateTradeBatch& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_u32_be(buf, static_cast<uint32_t>(value.trades.size()));
        for (const auto& item : value.trades) {
            std::vector<uint8_t> item_buf;
            AggregateTradeEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

// SBE body decoder for AggregateTradeBatch: reads fields at pos.
struct AggregateTradeBatchBodyDecoder {
    static std::pair<AggregateTradeBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<AggregateTradeBatch, std::size_t> AggregateTradeBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    AggregateTradeBatch value{};
        std::optional<std::string> next_cursor;
        std::string symbol = read_string(buf, pos);
        uint32_t trades_count = read_u32_be(buf, pos);
        std::vector<AggregateTrade> trades;
        trades.reserve(trades_count);
        for (uint32_t i = 0; i < trades_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = AggregateTradeDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            trades.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.trades = trades;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE encoder for AggregateTradeBatch
struct AggregateTradeBatchEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::vector<AggregateTrade> trades, uint8_t has_more, std::optional<std::string> next_cursor);
};

inline std::vector<uint8_t> AggregateTradeBatchEncoder::encode(std::string symbol, std::vector<AggregateTrade> trades, uint8_t has_more, std::optional<std::string> next_cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 13);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(trades.size()));
        for (const auto& item : trades) {
            std::vector<uint8_t> item_buf;
            AggregateTradeEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for AggregateTradeBatch: validates the header and reads the body.
struct AggregateTradeBatchDecoder {
    static AggregateTradeBatch decode(const std::vector<uint8_t>& buf);
};

inline AggregateTradeBatch AggregateTradeBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    AggregateTradeBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 13) throw std::runtime_error("invalid template_id");
        std::optional<std::string> next_cursor;
        std::string symbol = read_string(buf, pos);
        uint32_t trades_count = read_u32_be(buf, pos);
        std::vector<AggregateTrade> trades;
        trades.reserve(trades_count);
        for (uint32_t i = 0; i < trades_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = AggregateTradeDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            trades.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.trades = trades;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    (void)value;
    return value;
}

// SBE message MiniTicker
struct MiniTicker {
    std::string symbol;
    double last_price;
    double volume;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for MiniTicker (fields only, no header).
struct MiniTickerBodyEncoder {
    static void encode(const MiniTicker& value, std::vector<uint8_t>& buf);
};

inline void MiniTickerBodyEncoder::encode(const MiniTicker& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_f64_be(buf, value.last_price);
        write_f64_be(buf, value.volume);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for MiniTicker: reads fields at pos.
struct MiniTickerBodyDecoder {
    static std::pair<MiniTicker, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MiniTicker, std::size_t> MiniTickerBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MiniTicker value{};
        std::string symbol = read_string(buf, pos);
        double last_price = read_f64_be(buf, pos);
        double volume = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.last_price = last_price;
    value.volume = volume;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for MiniTicker
struct MiniTickerEncoder {
    static std::vector<uint8_t> encode(std::string symbol, double last_price, double volume, int64_t timestamp, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> MiniTickerEncoder::encode(std::string symbol, double last_price, double volume, int64_t timestamp, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 14);
        write_u16_be(buf, 0);
        write_u16_be(buf, 25);
        write_string(buf, symbol);
        write_f64_be(buf, last_price);
        write_f64_be(buf, volume);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for MiniTicker: validates the header and reads the body.
struct MiniTickerDecoder {
    static MiniTicker decode(const std::vector<uint8_t>& buf);
};

inline MiniTicker MiniTickerDecoder::decode(const std::vector<uint8_t>& buf) {
    MiniTicker value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 14) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        double last_price = read_f64_be(buf, pos);
        double volume = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.last_price = last_price;
    value.volume = volume;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message AllMidsRequest
struct AllMidsRequest {
};

// SBE body encoder for AllMidsRequest (fields only, no header).
struct AllMidsRequestBodyEncoder {
    static void encode(const AllMidsRequest& value, std::vector<uint8_t>& buf);
};

inline void AllMidsRequestBodyEncoder::encode(const AllMidsRequest& value, std::vector<uint8_t>& buf) {
}

// SBE body decoder for AllMidsRequest: reads fields at pos.
struct AllMidsRequestBodyDecoder {
    static std::pair<AllMidsRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<AllMidsRequest, std::size_t> AllMidsRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    AllMidsRequest value{};
    return {value, pos};
}

// SBE encoder for AllMidsRequest
struct AllMidsRequestEncoder {
    static std::vector<uint8_t> encode();
};

inline std::vector<uint8_t> AllMidsRequestEncoder::encode() {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 15);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        return buf;
}

// SBE decoder for AllMidsRequest: validates the header and reads the body.
struct AllMidsRequestDecoder {
    static AllMidsRequest decode(const std::vector<uint8_t>& buf);
};

inline AllMidsRequest AllMidsRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    AllMidsRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 15) throw std::runtime_error("invalid template_id");
    (void)value;
    return value;
}

// SBE message AllMidsBatch
struct AllMidsBatch {
    std::vector<MiniTicker> tickers;
};

// SBE body encoder for AllMidsBatch (fields only, no header).
struct AllMidsBatchBodyEncoder {
    static void encode(const AllMidsBatch& value, std::vector<uint8_t>& buf);
};

inline void AllMidsBatchBodyEncoder::encode(const AllMidsBatch& value, std::vector<uint8_t>& buf) {
        write_u32_be(buf, static_cast<uint32_t>(value.tickers.size()));
        for (const auto& item : value.tickers) {
            std::vector<uint8_t> item_buf;
            MiniTickerBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
}

// SBE body decoder for AllMidsBatch: reads fields at pos.
struct AllMidsBatchBodyDecoder {
    static std::pair<AllMidsBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<AllMidsBatch, std::size_t> AllMidsBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    AllMidsBatch value{};
        uint32_t tickers_count = read_u32_be(buf, pos);
        std::vector<MiniTicker> tickers;
        tickers.reserve(tickers_count);
        for (uint32_t i = 0; i < tickers_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = MiniTickerBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            tickers.push_back(item);
        }
    value.tickers = tickers;
    return {value, pos};
}

// SBE encoder for AllMidsBatch
struct AllMidsBatchEncoder {
    static std::vector<uint8_t> encode(std::vector<MiniTicker> tickers);
};

inline std::vector<uint8_t> AllMidsBatchEncoder::encode(std::vector<MiniTicker> tickers) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 16);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_u32_be(buf, static_cast<uint32_t>(tickers.size()));
        for (const auto& item : tickers) {
            std::vector<uint8_t> item_buf;
            MiniTickerBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        return buf;
}

// SBE decoder for AllMidsBatch: validates the header and reads the body.
struct AllMidsBatchDecoder {
    static AllMidsBatch decode(const std::vector<uint8_t>& buf);
};

inline AllMidsBatch AllMidsBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    AllMidsBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 16) throw std::runtime_error("invalid template_id");
        uint32_t tickers_count = read_u32_be(buf, pos);
        std::vector<MiniTicker> tickers;
        tickers.reserve(tickers_count);
        for (uint32_t i = 0; i < tickers_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = MiniTickerBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            tickers.push_back(item);
        }
    value.tickers = tickers;
    (void)value;
    return value;
}

// SBE message MarkPriceUpdate
struct MarkPriceUpdate {
    std::string symbol;
    double mark_price;
    std::optional<double> index_price;
    std::optional<double> funding_rate;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for MarkPriceUpdate (fields only, no header).
struct MarkPriceUpdateBodyEncoder {
    static void encode(const MarkPriceUpdate& value, std::vector<uint8_t>& buf);
};

inline void MarkPriceUpdateBodyEncoder::encode(const MarkPriceUpdate& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_f64_be(buf, value.mark_price);
        write_f64_be(buf, value.index_price.value_or(0.0));
        buf.push_back(value.index_price.has_value() ? 1 : 0);
        write_f64_be(buf, value.funding_rate.value_or(0.0));
        buf.push_back(value.funding_rate.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for MarkPriceUpdate: reads fields at pos.
struct MarkPriceUpdateBodyDecoder {
    static std::pair<MarkPriceUpdate, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MarkPriceUpdate, std::size_t> MarkPriceUpdateBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MarkPriceUpdate value{};
        std::string symbol = read_string(buf, pos);
        double mark_price = read_f64_be(buf, pos);
        double index_price_v = read_f64_be(buf, pos);
        std::optional<double> index_price;
        if (buf[pos++] == 1) index_price = index_price_v;
        double funding_rate_v = read_f64_be(buf, pos);
        std::optional<double> funding_rate;
        if (buf[pos++] == 1) funding_rate = funding_rate_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.mark_price = mark_price;
    value.index_price = index_price;
    value.funding_rate = funding_rate;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for MarkPriceUpdate
struct MarkPriceUpdateEncoder {
    static std::vector<uint8_t> encode(std::string symbol, double mark_price, std::optional<double> index_price, std::optional<double> funding_rate, int64_t timestamp, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> MarkPriceUpdateEncoder::encode(std::string symbol, double mark_price, std::optional<double> index_price, std::optional<double> funding_rate, int64_t timestamp, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 17);
        write_u16_be(buf, 0);
        write_u16_be(buf, 33);
        write_string(buf, symbol);
        write_f64_be(buf, mark_price);
        write_f64_be(buf, index_price.value_or(0.0));
        buf.push_back(index_price.has_value() ? 1 : 0);
        write_f64_be(buf, funding_rate.value_or(0.0));
        buf.push_back(funding_rate.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for MarkPriceUpdate: validates the header and reads the body.
struct MarkPriceUpdateDecoder {
    static MarkPriceUpdate decode(const std::vector<uint8_t>& buf);
};

inline MarkPriceUpdate MarkPriceUpdateDecoder::decode(const std::vector<uint8_t>& buf) {
    MarkPriceUpdate value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 17) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        double mark_price = read_f64_be(buf, pos);
        double index_price_v = read_f64_be(buf, pos);
        std::optional<double> index_price;
        if (buf[pos++] == 1) index_price = index_price_v;
        double funding_rate_v = read_f64_be(buf, pos);
        std::optional<double> funding_rate;
        if (buf[pos++] == 1) funding_rate = funding_rate_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.mark_price = mark_price;
    value.index_price = index_price;
    value.funding_rate = funding_rate;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message MarkPriceRequest
struct MarkPriceRequest {
    std::string symbol;
};

// SBE body encoder for MarkPriceRequest (fields only, no header).
struct MarkPriceRequestBodyEncoder {
    static void encode(const MarkPriceRequest& value, std::vector<uint8_t>& buf);
};

inline void MarkPriceRequestBodyEncoder::encode(const MarkPriceRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
}

// SBE body decoder for MarkPriceRequest: reads fields at pos.
struct MarkPriceRequestBodyDecoder {
    static std::pair<MarkPriceRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MarkPriceRequest, std::size_t> MarkPriceRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MarkPriceRequest value{};
        std::string symbol = read_string(buf, pos);
    value.symbol = symbol;
    return {value, pos};
}

// SBE encoder for MarkPriceRequest
struct MarkPriceRequestEncoder {
    static std::vector<uint8_t> encode(std::string symbol);
};

inline std::vector<uint8_t> MarkPriceRequestEncoder::encode(std::string symbol) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 18);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_string(buf, symbol);
        return buf;
}

// SBE decoder for MarkPriceRequest: validates the header and reads the body.
struct MarkPriceRequestDecoder {
    static MarkPriceRequest decode(const std::vector<uint8_t>& buf);
};

inline MarkPriceRequest MarkPriceRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    MarkPriceRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 18) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
    value.symbol = symbol;
    (void)value;
    return value;
}

// SBE message LiquidationTradeEvent
struct LiquidationTradeEvent {
    LiquidationTrade trade;
};

// SBE body encoder for LiquidationTradeEvent (fields only, no header).
struct LiquidationTradeEventBodyEncoder {
    static void encode(const LiquidationTradeEvent& value, std::vector<uint8_t>& buf);
};

inline void LiquidationTradeEventBodyEncoder::encode(const LiquidationTradeEvent& value, std::vector<uint8_t>& buf) {
        LiquidationTradeEncoder::encode(value.trade, buf);
}

// SBE body decoder for LiquidationTradeEvent: reads fields at pos.
struct LiquidationTradeEventBodyDecoder {
    static std::pair<LiquidationTradeEvent, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<LiquidationTradeEvent, std::size_t> LiquidationTradeEventBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    LiquidationTradeEvent value{};
        auto [trade, trade_n] = LiquidationTradeDecoder::decode(buf, pos);
        pos += trade_n;
    value.trade = trade;
    return {value, pos};
}

// SBE encoder for LiquidationTradeEvent
struct LiquidationTradeEventEncoder {
    static std::vector<uint8_t> encode(LiquidationTrade trade);
};

inline std::vector<uint8_t> LiquidationTradeEventEncoder::encode(LiquidationTrade trade) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 19);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        LiquidationTradeEncoder::encode(trade, buf);
        return buf;
}

// SBE decoder for LiquidationTradeEvent: validates the header and reads the body.
struct LiquidationTradeEventDecoder {
    static LiquidationTradeEvent decode(const std::vector<uint8_t>& buf);
};

inline LiquidationTradeEvent LiquidationTradeEventDecoder::decode(const std::vector<uint8_t>& buf) {
    LiquidationTradeEvent value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 19) throw std::runtime_error("invalid template_id");
        auto [trade, trade_n] = LiquidationTradeDecoder::decode(buf, pos);
        pos += trade_n;
    value.trade = trade;
    (void)value;
    return value;
}

// SBE message CandleBarEvent
struct CandleBarEvent {
    CandleBar bar;
};

// SBE body encoder for CandleBarEvent (fields only, no header).
struct CandleBarEventBodyEncoder {
    static void encode(const CandleBarEvent& value, std::vector<uint8_t>& buf);
};

inline void CandleBarEventBodyEncoder::encode(const CandleBarEvent& value, std::vector<uint8_t>& buf) {
        CandleBarEncoder::encode(value.bar, buf);
}

// SBE body decoder for CandleBarEvent: reads fields at pos.
struct CandleBarEventBodyDecoder {
    static std::pair<CandleBarEvent, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CandleBarEvent, std::size_t> CandleBarEventBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CandleBarEvent value{};
        auto [bar, bar_n] = CandleBarDecoder::decode(buf, pos);
        pos += bar_n;
    value.bar = bar;
    return {value, pos};
}

// SBE encoder for CandleBarEvent
struct CandleBarEventEncoder {
    static std::vector<uint8_t> encode(CandleBar bar);
};

inline std::vector<uint8_t> CandleBarEventEncoder::encode(CandleBar bar) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 20);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        CandleBarEncoder::encode(bar, buf);
        return buf;
}

// SBE decoder for CandleBarEvent: validates the header and reads the body.
struct CandleBarEventDecoder {
    static CandleBarEvent decode(const std::vector<uint8_t>& buf);
};

inline CandleBarEvent CandleBarEventDecoder::decode(const std::vector<uint8_t>& buf) {
    CandleBarEvent value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 20) throw std::runtime_error("invalid template_id");
        auto [bar, bar_n] = CandleBarDecoder::decode(buf, pos);
        pos += bar_n;
    value.bar = bar;
    (void)value;
    return value;
}

// SBE message CandleBarRequest
struct CandleBarRequest {
    std::string symbol;
    std::string interval;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<uint32_t> limit;
    std::optional<std::string> cursor;
};

// SBE body encoder for CandleBarRequest (fields only, no header).
struct CandleBarRequestBodyEncoder {
    static void encode(const CandleBarRequest& value, std::vector<uint8_t>& buf);
};

inline void CandleBarRequestBodyEncoder::encode(const CandleBarRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_string(buf, value.interval);
        write_u64_be(buf, static_cast<uint64_t>(value.start_time.value_or(0)));
        buf.push_back(value.start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.end_time.value_or(0)));
        buf.push_back(value.end_time.has_value() ? 1 : 0);
        write_u32_be(buf, value.limit.value_or(0));
        buf.push_back(value.limit.has_value() ? 1 : 0);
        if (value.cursor.has_value()) { buf.push_back(1); write_string(buf, *value.cursor); } else { buf.push_back(0); }
}

// SBE body decoder for CandleBarRequest: reads fields at pos.
struct CandleBarRequestBodyDecoder {
    static std::pair<CandleBarRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CandleBarRequest, std::size_t> CandleBarRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CandleBarRequest value{};
        std::optional<std::string> cursor;
        std::string symbol = read_string(buf, pos);
        std::string interval = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.interval = interval;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    return {value, pos};
}

// SBE encoder for CandleBarRequest
struct CandleBarRequestEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::string interval, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor);
};

inline std::vector<uint8_t> CandleBarRequestEncoder::encode(std::string symbol, std::string interval, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 21);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);
        write_string(buf, symbol);
        write_string(buf, interval);
        write_u64_be(buf, static_cast<uint64_t>(start_time.value_or(0)));
        buf.push_back(start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(end_time.value_or(0)));
        buf.push_back(end_time.has_value() ? 1 : 0);
        write_u32_be(buf, limit.value_or(0));
        buf.push_back(limit.has_value() ? 1 : 0);
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for CandleBarRequest: validates the header and reads the body.
struct CandleBarRequestDecoder {
    static CandleBarRequest decode(const std::vector<uint8_t>& buf);
};

inline CandleBarRequest CandleBarRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    CandleBarRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 21) throw std::runtime_error("invalid template_id");
        std::optional<std::string> cursor;
        std::string symbol = read_string(buf, pos);
        std::string interval = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.interval = interval;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    (void)value;
    return value;
}

// SBE message CandleBarBatch
struct CandleBarBatch {
    std::string symbol;
    std::string interval;
    std::vector<CandleBar> bars;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

// SBE body encoder for CandleBarBatch (fields only, no header).
struct CandleBarBatchBodyEncoder {
    static void encode(const CandleBarBatch& value, std::vector<uint8_t>& buf);
};

inline void CandleBarBatchBodyEncoder::encode(const CandleBarBatch& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_string(buf, value.interval);
        write_u32_be(buf, static_cast<uint32_t>(value.bars.size()));
        for (const auto& item : value.bars) {
            std::vector<uint8_t> item_buf;
            CandleBarEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

// SBE body decoder for CandleBarBatch: reads fields at pos.
struct CandleBarBatchBodyDecoder {
    static std::pair<CandleBarBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CandleBarBatch, std::size_t> CandleBarBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CandleBarBatch value{};
        std::optional<std::string> next_cursor;
        std::string symbol = read_string(buf, pos);
        std::string interval = read_string(buf, pos);
        uint32_t bars_count = read_u32_be(buf, pos);
        std::vector<CandleBar> bars;
        bars.reserve(bars_count);
        for (uint32_t i = 0; i < bars_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = CandleBarDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            bars.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.interval = interval;
    value.bars = bars;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE encoder for CandleBarBatch
struct CandleBarBatchEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::string interval, std::vector<CandleBar> bars, uint8_t has_more, std::optional<std::string> next_cursor);
};

inline std::vector<uint8_t> CandleBarBatchEncoder::encode(std::string symbol, std::string interval, std::vector<CandleBar> bars, uint8_t has_more, std::optional<std::string> next_cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 22);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, symbol);
        write_string(buf, interval);
        write_u32_be(buf, static_cast<uint32_t>(bars.size()));
        for (const auto& item : bars) {
            std::vector<uint8_t> item_buf;
            CandleBarEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for CandleBarBatch: validates the header and reads the body.
struct CandleBarBatchDecoder {
    static CandleBarBatch decode(const std::vector<uint8_t>& buf);
};

inline CandleBarBatch CandleBarBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    CandleBarBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 22) throw std::runtime_error("invalid template_id");
        std::optional<std::string> next_cursor;
        std::string symbol = read_string(buf, pos);
        std::string interval = read_string(buf, pos);
        uint32_t bars_count = read_u32_be(buf, pos);
        std::vector<CandleBar> bars;
        bars.reserve(bars_count);
        for (uint32_t i = 0; i < bars_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = CandleBarDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            bars.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.interval = interval;
    value.bars = bars;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    (void)value;
    return value;
}

// SBE message PublicTradeEvent
struct PublicTradeEvent {
    PublicTrade trade;
};

// SBE body encoder for PublicTradeEvent (fields only, no header).
struct PublicTradeEventBodyEncoder {
    static void encode(const PublicTradeEvent& value, std::vector<uint8_t>& buf);
};

inline void PublicTradeEventBodyEncoder::encode(const PublicTradeEvent& value, std::vector<uint8_t>& buf) {
        PublicTradeEncoder::encode(value.trade, buf);
}

// SBE body decoder for PublicTradeEvent: reads fields at pos.
struct PublicTradeEventBodyDecoder {
    static std::pair<PublicTradeEvent, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<PublicTradeEvent, std::size_t> PublicTradeEventBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PublicTradeEvent value{};
        auto [trade, trade_n] = PublicTradeDecoder::decode(buf, pos);
        pos += trade_n;
    value.trade = trade;
    return {value, pos};
}

// SBE encoder for PublicTradeEvent
struct PublicTradeEventEncoder {
    static std::vector<uint8_t> encode(PublicTrade trade);
};

inline std::vector<uint8_t> PublicTradeEventEncoder::encode(PublicTrade trade) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 23);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        PublicTradeEncoder::encode(trade, buf);
        return buf;
}

// SBE decoder for PublicTradeEvent: validates the header and reads the body.
struct PublicTradeEventDecoder {
    static PublicTradeEvent decode(const std::vector<uint8_t>& buf);
};

inline PublicTradeEvent PublicTradeEventDecoder::decode(const std::vector<uint8_t>& buf) {
    PublicTradeEvent value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 23) throw std::runtime_error("invalid template_id");
        auto [trade, trade_n] = PublicTradeDecoder::decode(buf, pos);
        pos += trade_n;
    value.trade = trade;
    (void)value;
    return value;
}

// SBE message TradeHistoryRequest
struct TradeHistoryRequest {
    std::string symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<uint32_t> limit;
    std::optional<std::string> cursor;
};

// SBE body encoder for TradeHistoryRequest (fields only, no header).
struct TradeHistoryRequestBodyEncoder {
    static void encode(const TradeHistoryRequest& value, std::vector<uint8_t>& buf);
};

inline void TradeHistoryRequestBodyEncoder::encode(const TradeHistoryRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_u64_be(buf, static_cast<uint64_t>(value.start_time.value_or(0)));
        buf.push_back(value.start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.end_time.value_or(0)));
        buf.push_back(value.end_time.has_value() ? 1 : 0);
        write_u32_be(buf, value.limit.value_or(0));
        buf.push_back(value.limit.has_value() ? 1 : 0);
        if (value.cursor.has_value()) { buf.push_back(1); write_string(buf, *value.cursor); } else { buf.push_back(0); }
}

// SBE body decoder for TradeHistoryRequest: reads fields at pos.
struct TradeHistoryRequestBodyDecoder {
    static std::pair<TradeHistoryRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<TradeHistoryRequest, std::size_t> TradeHistoryRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    TradeHistoryRequest value{};
        std::optional<std::string> cursor;
        std::string symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    return {value, pos};
}

// SBE encoder for TradeHistoryRequest
struct TradeHistoryRequestEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor);
};

inline std::vector<uint8_t> TradeHistoryRequestEncoder::encode(std::string symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 24);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);
        write_string(buf, symbol);
        write_u64_be(buf, static_cast<uint64_t>(start_time.value_or(0)));
        buf.push_back(start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(end_time.value_or(0)));
        buf.push_back(end_time.has_value() ? 1 : 0);
        write_u32_be(buf, limit.value_or(0));
        buf.push_back(limit.has_value() ? 1 : 0);
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for TradeHistoryRequest: validates the header and reads the body.
struct TradeHistoryRequestDecoder {
    static TradeHistoryRequest decode(const std::vector<uint8_t>& buf);
};

inline TradeHistoryRequest TradeHistoryRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    TradeHistoryRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 24) throw std::runtime_error("invalid template_id");
        std::optional<std::string> cursor;
        std::string symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    (void)value;
    return value;
}

// SBE message PublicTradeBatch
struct PublicTradeBatch {
    std::string symbol;
    std::vector<PublicTrade> trades;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

// SBE body encoder for PublicTradeBatch (fields only, no header).
struct PublicTradeBatchBodyEncoder {
    static void encode(const PublicTradeBatch& value, std::vector<uint8_t>& buf);
};

inline void PublicTradeBatchBodyEncoder::encode(const PublicTradeBatch& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_u32_be(buf, static_cast<uint32_t>(value.trades.size()));
        for (const auto& item : value.trades) {
            std::vector<uint8_t> item_buf;
            PublicTradeEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

// SBE body decoder for PublicTradeBatch: reads fields at pos.
struct PublicTradeBatchBodyDecoder {
    static std::pair<PublicTradeBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<PublicTradeBatch, std::size_t> PublicTradeBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PublicTradeBatch value{};
        std::optional<std::string> next_cursor;
        std::string symbol = read_string(buf, pos);
        uint32_t trades_count = read_u32_be(buf, pos);
        std::vector<PublicTrade> trades;
        trades.reserve(trades_count);
        for (uint32_t i = 0; i < trades_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PublicTradeDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            trades.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.trades = trades;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE encoder for PublicTradeBatch
struct PublicTradeBatchEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::vector<PublicTrade> trades, uint8_t has_more, std::optional<std::string> next_cursor);
};

inline std::vector<uint8_t> PublicTradeBatchEncoder::encode(std::string symbol, std::vector<PublicTrade> trades, uint8_t has_more, std::optional<std::string> next_cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 25);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, symbol);
        write_u32_be(buf, static_cast<uint32_t>(trades.size()));
        for (const auto& item : trades) {
            std::vector<uint8_t> item_buf;
            PublicTradeEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for PublicTradeBatch: validates the header and reads the body.
struct PublicTradeBatchDecoder {
    static PublicTradeBatch decode(const std::vector<uint8_t>& buf);
};

inline PublicTradeBatch PublicTradeBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    PublicTradeBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 25) throw std::runtime_error("invalid template_id");
        std::optional<std::string> next_cursor;
        std::string symbol = read_string(buf, pos);
        uint32_t trades_count = read_u32_be(buf, pos);
        std::vector<PublicTrade> trades;
        trades.reserve(trades_count);
        for (uint32_t i = 0; i < trades_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PublicTradeDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            trades.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.symbol = symbol;
    value.trades = trades;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    (void)value;
    return value;
}

// SBE message BestBidOffer
struct BestBidOffer {
    std::string symbol;
    std::optional<double> bid_price;
    std::optional<double> bid_qty;
    std::optional<double> ask_price;
    std::optional<double> ask_qty;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for BestBidOffer (fields only, no header).
struct BestBidOfferBodyEncoder {
    static void encode(const BestBidOffer& value, std::vector<uint8_t>& buf);
};

inline void BestBidOfferBodyEncoder::encode(const BestBidOffer& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_f64_be(buf, value.bid_price.value_or(0.0));
        buf.push_back(value.bid_price.has_value() ? 1 : 0);
        write_f64_be(buf, value.bid_qty.value_or(0.0));
        buf.push_back(value.bid_qty.has_value() ? 1 : 0);
        write_f64_be(buf, value.ask_price.value_or(0.0));
        buf.push_back(value.ask_price.has_value() ? 1 : 0);
        write_f64_be(buf, value.ask_qty.value_or(0.0));
        buf.push_back(value.ask_qty.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for BestBidOffer: reads fields at pos.
struct BestBidOfferBodyDecoder {
    static std::pair<BestBidOffer, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<BestBidOffer, std::size_t> BestBidOfferBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    BestBidOffer value{};
        std::string symbol = read_string(buf, pos);
        double bid_price_v = read_f64_be(buf, pos);
        std::optional<double> bid_price;
        if (buf[pos++] == 1) bid_price = bid_price_v;
        double bid_qty_v = read_f64_be(buf, pos);
        std::optional<double> bid_qty;
        if (buf[pos++] == 1) bid_qty = bid_qty_v;
        double ask_price_v = read_f64_be(buf, pos);
        std::optional<double> ask_price;
        if (buf[pos++] == 1) ask_price = ask_price_v;
        double ask_qty_v = read_f64_be(buf, pos);
        std::optional<double> ask_qty;
        if (buf[pos++] == 1) ask_qty = ask_qty_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.bid_price = bid_price;
    value.bid_qty = bid_qty;
    value.ask_price = ask_price;
    value.ask_qty = ask_qty;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for BestBidOffer
struct BestBidOfferEncoder {
    static std::vector<uint8_t> encode(std::string symbol, std::optional<double> bid_price, std::optional<double> bid_qty, std::optional<double> ask_price, std::optional<double> ask_qty, int64_t timestamp, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> BestBidOfferEncoder::encode(std::string symbol, std::optional<double> bid_price, std::optional<double> bid_qty, std::optional<double> ask_price, std::optional<double> ask_qty, int64_t timestamp, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 26);
        write_u16_be(buf, 0);
        write_u16_be(buf, 41);
        write_string(buf, symbol);
        write_f64_be(buf, bid_price.value_or(0.0));
        buf.push_back(bid_price.has_value() ? 1 : 0);
        write_f64_be(buf, bid_qty.value_or(0.0));
        buf.push_back(bid_qty.has_value() ? 1 : 0);
        write_f64_be(buf, ask_price.value_or(0.0));
        buf.push_back(ask_price.has_value() ? 1 : 0);
        write_f64_be(buf, ask_qty.value_or(0.0));
        buf.push_back(ask_qty.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for BestBidOffer: validates the header and reads the body.
struct BestBidOfferDecoder {
    static BestBidOffer decode(const std::vector<uint8_t>& buf);
};

inline BestBidOffer BestBidOfferDecoder::decode(const std::vector<uint8_t>& buf) {
    BestBidOffer value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 26) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        double bid_price_v = read_f64_be(buf, pos);
        std::optional<double> bid_price;
        if (buf[pos++] == 1) bid_price = bid_price_v;
        double bid_qty_v = read_f64_be(buf, pos);
        std::optional<double> bid_qty;
        if (buf[pos++] == 1) bid_qty = bid_qty_v;
        double ask_price_v = read_f64_be(buf, pos);
        std::optional<double> ask_price;
        if (buf[pos++] == 1) ask_price = ask_price_v;
        double ask_qty_v = read_f64_be(buf, pos);
        std::optional<double> ask_qty;
        if (buf[pos++] == 1) ask_qty = ask_qty_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.bid_price = bid_price;
    value.bid_qty = bid_qty;
    value.ask_price = ask_price;
    value.ask_qty = ask_qty;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message SymbolTicker
struct SymbolTicker {
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
};

// SBE body encoder for SymbolTicker (fields only, no header).
struct SymbolTickerBodyEncoder {
    static void encode(const SymbolTicker& value, std::vector<uint8_t>& buf);
};

inline void SymbolTickerBodyEncoder::encode(const SymbolTicker& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        write_f64_be(buf, value.last_price);
        write_f64_be(buf, value.price_change);
        write_f64_be(buf, value.price_change_pct);
        write_f64_be(buf, value.volume);
        write_f64_be(buf, value.high);
        write_f64_be(buf, value.low);
        write_f64_be(buf, value.open);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for SymbolTicker: reads fields at pos.
struct SymbolTickerBodyDecoder {
    static std::pair<SymbolTicker, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<SymbolTicker, std::size_t> SymbolTickerBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    SymbolTicker value{};
        std::string symbol = read_string(buf, pos);
        double last_price = read_f64_be(buf, pos);
        double price_change = read_f64_be(buf, pos);
        double price_change_pct = read_f64_be(buf, pos);
        double volume = read_f64_be(buf, pos);
        double high = read_f64_be(buf, pos);
        double low = read_f64_be(buf, pos);
        double open = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
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
    return {value, pos};
}

// SBE encoder for SymbolTicker
struct SymbolTickerEncoder {
    static std::vector<uint8_t> encode(std::string symbol, double last_price, double price_change, double price_change_pct, double volume, double high, double low, double open, int64_t timestamp, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> SymbolTickerEncoder::encode(std::string symbol, double last_price, double price_change, double price_change_pct, double volume, double high, double low, double open, int64_t timestamp, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 27);
        write_u16_be(buf, 0);
        write_u16_be(buf, 65);
        write_string(buf, symbol);
        write_f64_be(buf, last_price);
        write_f64_be(buf, price_change);
        write_f64_be(buf, price_change_pct);
        write_f64_be(buf, volume);
        write_f64_be(buf, high);
        write_f64_be(buf, low);
        write_f64_be(buf, open);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for SymbolTicker: validates the header and reads the body.
struct SymbolTickerDecoder {
    static SymbolTicker decode(const std::vector<uint8_t>& buf);
};

inline SymbolTicker SymbolTickerDecoder::decode(const std::vector<uint8_t>& buf) {
    SymbolTicker value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
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
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
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
    (void)value;
    return value;
}

// SBE message TickerRequest
struct TickerRequest {
    std::string symbol;
};

// SBE body encoder for TickerRequest (fields only, no header).
struct TickerRequestBodyEncoder {
    static void encode(const TickerRequest& value, std::vector<uint8_t>& buf);
};

inline void TickerRequestBodyEncoder::encode(const TickerRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
}

// SBE body decoder for TickerRequest: reads fields at pos.
struct TickerRequestBodyDecoder {
    static std::pair<TickerRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<TickerRequest, std::size_t> TickerRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    TickerRequest value{};
        std::string symbol = read_string(buf, pos);
    value.symbol = symbol;
    return {value, pos};
}

// SBE encoder for TickerRequest
struct TickerRequestEncoder {
    static std::vector<uint8_t> encode(std::string symbol);
};

inline std::vector<uint8_t> TickerRequestEncoder::encode(std::string symbol) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 28);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_string(buf, symbol);
        return buf;
}

// SBE decoder for TickerRequest: validates the header and reads the body.
struct TickerRequestDecoder {
    static TickerRequest decode(const std::vector<uint8_t>& buf);
};

inline TickerRequest TickerRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    TickerRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 28) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
    value.symbol = symbol;
    (void)value;
    return value;
}

// SBE message InstrumentCatalogRequest
struct InstrumentCatalogRequest {
};

// SBE body encoder for InstrumentCatalogRequest (fields only, no header).
struct InstrumentCatalogRequestBodyEncoder {
    static void encode(const InstrumentCatalogRequest& value, std::vector<uint8_t>& buf);
};

inline void InstrumentCatalogRequestBodyEncoder::encode(const InstrumentCatalogRequest& value, std::vector<uint8_t>& buf) {
}

// SBE body decoder for InstrumentCatalogRequest: reads fields at pos.
struct InstrumentCatalogRequestBodyDecoder {
    static std::pair<InstrumentCatalogRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<InstrumentCatalogRequest, std::size_t> InstrumentCatalogRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    InstrumentCatalogRequest value{};
    return {value, pos};
}

// SBE encoder for InstrumentCatalogRequest
struct InstrumentCatalogRequestEncoder {
    static std::vector<uint8_t> encode();
};

inline std::vector<uint8_t> InstrumentCatalogRequestEncoder::encode() {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 29);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        return buf;
}

// SBE decoder for InstrumentCatalogRequest: validates the header and reads the body.
struct InstrumentCatalogRequestDecoder {
    static InstrumentCatalogRequest decode(const std::vector<uint8_t>& buf);
};

inline InstrumentCatalogRequest InstrumentCatalogRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    InstrumentCatalogRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 29) throw std::runtime_error("invalid template_id");
    (void)value;
    return value;
}

// SBE message InstrumentCatalogResponse
struct InstrumentCatalogResponse {
    std::vector<InstrumentMetadata> instruments;
};

// SBE body encoder for InstrumentCatalogResponse (fields only, no header).
struct InstrumentCatalogResponseBodyEncoder {
    static void encode(const InstrumentCatalogResponse& value, std::vector<uint8_t>& buf);
};

inline void InstrumentCatalogResponseBodyEncoder::encode(const InstrumentCatalogResponse& value, std::vector<uint8_t>& buf) {
        write_u32_be(buf, static_cast<uint32_t>(value.instruments.size()));
        for (const auto& item : value.instruments) {
            std::vector<uint8_t> item_buf;
            InstrumentMetadataEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
}

// SBE body decoder for InstrumentCatalogResponse: reads fields at pos.
struct InstrumentCatalogResponseBodyDecoder {
    static std::pair<InstrumentCatalogResponse, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<InstrumentCatalogResponse, std::size_t> InstrumentCatalogResponseBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    InstrumentCatalogResponse value{};
        uint32_t instruments_count = read_u32_be(buf, pos);
        std::vector<InstrumentMetadata> instruments;
        instruments.reserve(instruments_count);
        for (uint32_t i = 0; i < instruments_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = InstrumentMetadataDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            instruments.push_back(item);
        }
    value.instruments = instruments;
    return {value, pos};
}

// SBE encoder for InstrumentCatalogResponse
struct InstrumentCatalogResponseEncoder {
    static std::vector<uint8_t> encode(std::vector<InstrumentMetadata> instruments);
};

inline std::vector<uint8_t> InstrumentCatalogResponseEncoder::encode(std::vector<InstrumentMetadata> instruments) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 30);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_u32_be(buf, static_cast<uint32_t>(instruments.size()));
        for (const auto& item : instruments) {
            std::vector<uint8_t> item_buf;
            InstrumentMetadataEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        return buf;
}

// SBE decoder for InstrumentCatalogResponse: validates the header and reads the body.
struct InstrumentCatalogResponseDecoder {
    static InstrumentCatalogResponse decode(const std::vector<uint8_t>& buf);
};

inline InstrumentCatalogResponse InstrumentCatalogResponseDecoder::decode(const std::vector<uint8_t>& buf) {
    InstrumentCatalogResponse value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 30) throw std::runtime_error("invalid template_id");
        uint32_t instruments_count = read_u32_be(buf, pos);
        std::vector<InstrumentMetadata> instruments;
        instruments.reserve(instruments_count);
        for (uint32_t i = 0; i < instruments_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = InstrumentMetadataDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            instruments.push_back(item);
        }
    value.instruments = instruments;
    (void)value;
    return value;
}

// SBE message AccountSummary
struct AccountSummary {
    std::string account;
    double balance;
    double buying_power;
    std::string currency;
};

// SBE body encoder for AccountSummary (fields only, no header).
struct AccountSummaryBodyEncoder {
    static void encode(const AccountSummary& value, std::vector<uint8_t>& buf);
};

inline void AccountSummaryBodyEncoder::encode(const AccountSummary& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_f64_be(buf, value.balance);
        write_f64_be(buf, value.buying_power);
        write_string(buf, value.currency);
}

// SBE body decoder for AccountSummary: reads fields at pos.
struct AccountSummaryBodyDecoder {
    static std::pair<AccountSummary, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<AccountSummary, std::size_t> AccountSummaryBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    AccountSummary value{};
        std::string account = read_string(buf, pos);
        double balance = read_f64_be(buf, pos);
        double buying_power = read_f64_be(buf, pos);
        std::string currency = read_string(buf, pos);
    value.account = account;
    value.balance = balance;
    value.buying_power = buying_power;
    value.currency = currency;
    return {value, pos};
}

// SBE encoder for AccountSummary
struct AccountSummaryEncoder {
    static std::vector<uint8_t> encode(std::string account, double balance, double buying_power, std::string currency);
};

inline std::vector<uint8_t> AccountSummaryEncoder::encode(std::string account, double balance, double buying_power, std::string currency) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 31);
        write_u16_be(buf, 0);
        write_u16_be(buf, 16);
        write_string(buf, account);
        write_f64_be(buf, balance);
        write_f64_be(buf, buying_power);
        write_string(buf, currency);
        return buf;
}

// SBE decoder for AccountSummary: validates the header and reads the body.
struct AccountSummaryDecoder {
    static AccountSummary decode(const std::vector<uint8_t>& buf);
};

inline AccountSummary AccountSummaryDecoder::decode(const std::vector<uint8_t>& buf) {
    AccountSummary value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 31) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        double balance = read_f64_be(buf, pos);
        double buying_power = read_f64_be(buf, pos);
        std::string currency = read_string(buf, pos);
    value.account = account;
    value.balance = balance;
    value.buying_power = buying_power;
    value.currency = currency;
    (void)value;
    return value;
}

// SBE message MarginSummary
struct MarginSummary {
    std::string account;
    double balance;
    double buying_power;
    double equity;
    double margin_used;
    double available;
    std::string currency;
};

// SBE body encoder for MarginSummary (fields only, no header).
struct MarginSummaryBodyEncoder {
    static void encode(const MarginSummary& value, std::vector<uint8_t>& buf);
};

inline void MarginSummaryBodyEncoder::encode(const MarginSummary& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_f64_be(buf, value.balance);
        write_f64_be(buf, value.buying_power);
        write_f64_be(buf, value.equity);
        write_f64_be(buf, value.margin_used);
        write_f64_be(buf, value.available);
        write_string(buf, value.currency);
}

// SBE body decoder for MarginSummary: reads fields at pos.
struct MarginSummaryBodyDecoder {
    static std::pair<MarginSummary, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MarginSummary, std::size_t> MarginSummaryBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MarginSummary value{};
        std::string account = read_string(buf, pos);
        double balance = read_f64_be(buf, pos);
        double buying_power = read_f64_be(buf, pos);
        double equity = read_f64_be(buf, pos);
        double margin_used = read_f64_be(buf, pos);
        double available = read_f64_be(buf, pos);
        std::string currency = read_string(buf, pos);
    value.account = account;
    value.balance = balance;
    value.buying_power = buying_power;
    value.equity = equity;
    value.margin_used = margin_used;
    value.available = available;
    value.currency = currency;
    return {value, pos};
}

// SBE encoder for MarginSummary
struct MarginSummaryEncoder {
    static std::vector<uint8_t> encode(std::string account, double balance, double buying_power, double equity, double margin_used, double available, std::string currency);
};

inline std::vector<uint8_t> MarginSummaryEncoder::encode(std::string account, double balance, double buying_power, double equity, double margin_used, double available, std::string currency) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 32);
        write_u16_be(buf, 0);
        write_u16_be(buf, 40);
        write_string(buf, account);
        write_f64_be(buf, balance);
        write_f64_be(buf, buying_power);
        write_f64_be(buf, equity);
        write_f64_be(buf, margin_used);
        write_f64_be(buf, available);
        write_string(buf, currency);
        return buf;
}

// SBE decoder for MarginSummary: validates the header and reads the body.
struct MarginSummaryDecoder {
    static MarginSummary decode(const std::vector<uint8_t>& buf);
};

inline MarginSummary MarginSummaryDecoder::decode(const std::vector<uint8_t>& buf) {
    MarginSummary value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 32) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        double balance = read_f64_be(buf, pos);
        double buying_power = read_f64_be(buf, pos);
        double equity = read_f64_be(buf, pos);
        double margin_used = read_f64_be(buf, pos);
        double available = read_f64_be(buf, pos);
        std::string currency = read_string(buf, pos);
    value.account = account;
    value.balance = balance;
    value.buying_power = buying_power;
    value.equity = equity;
    value.margin_used = margin_used;
    value.available = available;
    value.currency = currency;
    (void)value;
    return value;
}

// SBE message BalanceSnapshot
struct BalanceSnapshot {
    std::string account;
    std::vector<BalanceEntry> balances;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for BalanceSnapshot (fields only, no header).
struct BalanceSnapshotBodyEncoder {
    static void encode(const BalanceSnapshot& value, std::vector<uint8_t>& buf);
};

inline void BalanceSnapshotBodyEncoder::encode(const BalanceSnapshot& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u32_be(buf, static_cast<uint32_t>(value.balances.size()));
        for (const auto& item : value.balances) {
            std::vector<uint8_t> item_buf;
            BalanceEntryEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for BalanceSnapshot: reads fields at pos.
struct BalanceSnapshotBodyDecoder {
    static std::pair<BalanceSnapshot, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<BalanceSnapshot, std::size_t> BalanceSnapshotBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    BalanceSnapshot value{};
        std::string account = read_string(buf, pos);
        uint32_t balances_count = read_u32_be(buf, pos);
        std::vector<BalanceEntry> balances;
        balances.reserve(balances_count);
        for (uint32_t i = 0; i < balances_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = BalanceEntryDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            balances.push_back(item);
        }
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.balances = balances;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for BalanceSnapshot
struct BalanceSnapshotEncoder {
    static std::vector<uint8_t> encode(std::string account, std::vector<BalanceEntry> balances, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> BalanceSnapshotEncoder::encode(std::string account, std::vector<BalanceEntry> balances, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 33);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(balances.size()));
        for (const auto& item : balances) {
            std::vector<uint8_t> item_buf;
            BalanceEntryEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for BalanceSnapshot: validates the header and reads the body.
struct BalanceSnapshotDecoder {
    static BalanceSnapshot decode(const std::vector<uint8_t>& buf);
};

inline BalanceSnapshot BalanceSnapshotDecoder::decode(const std::vector<uint8_t>& buf) {
    BalanceSnapshot value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 33) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t balances_count = read_u32_be(buf, pos);
        std::vector<BalanceEntry> balances;
        balances.reserve(balances_count);
        for (uint32_t i = 0; i < balances_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = BalanceEntryDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            balances.push_back(item);
        }
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.balances = balances;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message BalanceUpdate
struct BalanceUpdate {
    std::string account;
    std::string asset;
    double delta;
    double total;
    double available;
    BalanceUpdateReason reason;
};

// SBE body encoder for BalanceUpdate (fields only, no header).
struct BalanceUpdateBodyEncoder {
    static void encode(const BalanceUpdate& value, std::vector<uint8_t>& buf);
};

inline void BalanceUpdateBodyEncoder::encode(const BalanceUpdate& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_string(buf, value.asset);
        write_f64_be(buf, value.delta);
        write_f64_be(buf, value.total);
        write_f64_be(buf, value.available);
        buf.push_back(BalanceUpdateReasonToValue(value.reason));
}

// SBE body decoder for BalanceUpdate: reads fields at pos.
struct BalanceUpdateBodyDecoder {
    static std::pair<BalanceUpdate, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<BalanceUpdate, std::size_t> BalanceUpdateBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    BalanceUpdate value{};
        std::string account = read_string(buf, pos);
        std::string asset = read_string(buf, pos);
        double delta = read_f64_be(buf, pos);
        double total = read_f64_be(buf, pos);
        double available = read_f64_be(buf, pos);
        auto reason_v = BalanceUpdateReasonFromValue(take_u8(buf, pos));
        if (!reason_v) throw std::runtime_error("invalid BalanceUpdateReason");
        BalanceUpdateReason reason = *reason_v;
    value.account = account;
    value.asset = asset;
    value.delta = delta;
    value.total = total;
    value.available = available;
    value.reason = reason;
    return {value, pos};
}

// SBE encoder for BalanceUpdate
struct BalanceUpdateEncoder {
    static std::vector<uint8_t> encode(std::string account, std::string asset, double delta, double total, double available, BalanceUpdateReason reason);
};

inline std::vector<uint8_t> BalanceUpdateEncoder::encode(std::string account, std::string asset, double delta, double total, double available, BalanceUpdateReason reason) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 34);
        write_u16_be(buf, 0);
        write_u16_be(buf, 25);
        write_string(buf, account);
        write_string(buf, asset);
        write_f64_be(buf, delta);
        write_f64_be(buf, total);
        write_f64_be(buf, available);
        buf.push_back(BalanceUpdateReasonToValue(reason));
        return buf;
}

// SBE decoder for BalanceUpdate: validates the header and reads the body.
struct BalanceUpdateDecoder {
    static BalanceUpdate decode(const std::vector<uint8_t>& buf);
};

inline BalanceUpdate BalanceUpdateDecoder::decode(const std::vector<uint8_t>& buf) {
    BalanceUpdate value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 34) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string asset = read_string(buf, pos);
        double delta = read_f64_be(buf, pos);
        double total = read_f64_be(buf, pos);
        double available = read_f64_be(buf, pos);
        auto reason_v = BalanceUpdateReasonFromValue(take_u8(buf, pos));
        if (!reason_v) throw std::runtime_error("invalid BalanceUpdateReason");
        BalanceUpdateReason reason = *reason_v;
    value.account = account;
    value.asset = asset;
    value.delta = delta;
    value.total = total;
    value.available = available;
    value.reason = reason;
    (void)value;
    return value;
}

// SBE message PositionSnapshot
struct PositionSnapshot {
    std::string account;
    std::vector<PositionEntry> positions;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for PositionSnapshot (fields only, no header).
struct PositionSnapshotBodyEncoder {
    static void encode(const PositionSnapshot& value, std::vector<uint8_t>& buf);
};

inline void PositionSnapshotBodyEncoder::encode(const PositionSnapshot& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u32_be(buf, static_cast<uint32_t>(value.positions.size()));
        for (const auto& item : value.positions) {
            std::vector<uint8_t> item_buf;
            PositionEntryEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for PositionSnapshot: reads fields at pos.
struct PositionSnapshotBodyDecoder {
    static std::pair<PositionSnapshot, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<PositionSnapshot, std::size_t> PositionSnapshotBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PositionSnapshot value{};
        std::string account = read_string(buf, pos);
        uint32_t positions_count = read_u32_be(buf, pos);
        std::vector<PositionEntry> positions;
        positions.reserve(positions_count);
        for (uint32_t i = 0; i < positions_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PositionEntryDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            positions.push_back(item);
        }
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.positions = positions;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for PositionSnapshot
struct PositionSnapshotEncoder {
    static std::vector<uint8_t> encode(std::string account, std::vector<PositionEntry> positions, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> PositionSnapshotEncoder::encode(std::string account, std::vector<PositionEntry> positions, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 35);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(positions.size()));
        for (const auto& item : positions) {
            std::vector<uint8_t> item_buf;
            PositionEntryEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for PositionSnapshot: validates the header and reads the body.
struct PositionSnapshotDecoder {
    static PositionSnapshot decode(const std::vector<uint8_t>& buf);
};

inline PositionSnapshot PositionSnapshotDecoder::decode(const std::vector<uint8_t>& buf) {
    PositionSnapshot value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 35) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t positions_count = read_u32_be(buf, pos);
        std::vector<PositionEntry> positions;
        positions.reserve(positions_count);
        for (uint32_t i = 0; i < positions_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = PositionEntryDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            positions.push_back(item);
        }
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.positions = positions;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message PositionUpdate
struct PositionUpdate {
    std::string account;
    std::string symbol;
    double qty;
    double entry_price;
    double unrealized_pnl;
};

// SBE body encoder for PositionUpdate (fields only, no header).
struct PositionUpdateBodyEncoder {
    static void encode(const PositionUpdate& value, std::vector<uint8_t>& buf);
};

inline void PositionUpdateBodyEncoder::encode(const PositionUpdate& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_string(buf, value.symbol);
        write_f64_be(buf, value.qty);
        write_f64_be(buf, value.entry_price);
        write_f64_be(buf, value.unrealized_pnl);
}

// SBE body decoder for PositionUpdate: reads fields at pos.
struct PositionUpdateBodyDecoder {
    static std::pair<PositionUpdate, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<PositionUpdate, std::size_t> PositionUpdateBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PositionUpdate value{};
        std::string account = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        double qty = read_f64_be(buf, pos);
        double entry_price = read_f64_be(buf, pos);
        double unrealized_pnl = read_f64_be(buf, pos);
    value.account = account;
    value.symbol = symbol;
    value.qty = qty;
    value.entry_price = entry_price;
    value.unrealized_pnl = unrealized_pnl;
    return {value, pos};
}

// SBE encoder for PositionUpdate
struct PositionUpdateEncoder {
    static std::vector<uint8_t> encode(std::string account, std::string symbol, double qty, double entry_price, double unrealized_pnl);
};

inline std::vector<uint8_t> PositionUpdateEncoder::encode(std::string account, std::string symbol, double qty, double entry_price, double unrealized_pnl) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 36);
        write_u16_be(buf, 0);
        write_u16_be(buf, 24);
        write_string(buf, account);
        write_string(buf, symbol);
        write_f64_be(buf, qty);
        write_f64_be(buf, entry_price);
        write_f64_be(buf, unrealized_pnl);
        return buf;
}

// SBE decoder for PositionUpdate: validates the header and reads the body.
struct PositionUpdateDecoder {
    static PositionUpdate decode(const std::vector<uint8_t>& buf);
};

inline PositionUpdate PositionUpdateDecoder::decode(const std::vector<uint8_t>& buf) {
    PositionUpdate value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 36) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        double qty = read_f64_be(buf, pos);
        double entry_price = read_f64_be(buf, pos);
        double unrealized_pnl = read_f64_be(buf, pos);
    value.account = account;
    value.symbol = symbol;
    value.qty = qty;
    value.entry_price = entry_price;
    value.unrealized_pnl = unrealized_pnl;
    (void)value;
    return value;
}

// SBE message MarginUpdate
struct MarginUpdate {
    std::string account;
    MarginSummary summary;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for MarginUpdate (fields only, no header).
struct MarginUpdateBodyEncoder {
    static void encode(const MarginUpdate& value, std::vector<uint8_t>& buf);
};

inline void MarginUpdateBodyEncoder::encode(const MarginUpdate& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        MarginSummaryBodyEncoder::encode(value.summary, buf);
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for MarginUpdate: reads fields at pos.
struct MarginUpdateBodyDecoder {
    static std::pair<MarginUpdate, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MarginUpdate, std::size_t> MarginUpdateBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MarginUpdate value{};
        std::string account = read_string(buf, pos);
        auto [summary, summary_n] = MarginSummaryBodyDecoder::decode(buf, pos);
        pos += summary_n;
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.summary = summary;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for MarginUpdate
struct MarginUpdateEncoder {
    static std::vector<uint8_t> encode(std::string account, MarginSummary summary, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> MarginUpdateEncoder::encode(std::string account, MarginSummary summary, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 37);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        MarginSummaryBodyEncoder::encode(summary, buf);
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for MarginUpdate: validates the header and reads the body.
struct MarginUpdateDecoder {
    static MarginUpdate decode(const std::vector<uint8_t>& buf);
};

inline MarginUpdate MarginUpdateDecoder::decode(const std::vector<uint8_t>& buf) {
    MarginUpdate value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 37) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        auto [summary, summary_n] = MarginSummaryBodyDecoder::decode(buf, pos);
        pos += summary_n;
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.summary = summary;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message UserLiquidation
struct UserLiquidation {
    std::string account;
    std::string symbol;
    double qty;
    double price;
    int64_t timestamp;
};

// SBE body encoder for UserLiquidation (fields only, no header).
struct UserLiquidationBodyEncoder {
    static void encode(const UserLiquidation& value, std::vector<uint8_t>& buf);
};

inline void UserLiquidationBodyEncoder::encode(const UserLiquidation& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_string(buf, value.symbol);
        write_f64_be(buf, value.qty);
        write_f64_be(buf, value.price);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
}

// SBE body decoder for UserLiquidation: reads fields at pos.
struct UserLiquidationBodyDecoder {
    static std::pair<UserLiquidation, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<UserLiquidation, std::size_t> UserLiquidationBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    UserLiquidation value{};
        std::string account = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        double qty = read_f64_be(buf, pos);
        double price = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.account = account;
    value.symbol = symbol;
    value.qty = qty;
    value.price = price;
    value.timestamp = timestamp;
    return {value, pos};
}

// SBE encoder for UserLiquidation
struct UserLiquidationEncoder {
    static std::vector<uint8_t> encode(std::string account, std::string symbol, double qty, double price, int64_t timestamp);
};

inline std::vector<uint8_t> UserLiquidationEncoder::encode(std::string account, std::string symbol, double qty, double price, int64_t timestamp) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 38);
        write_u16_be(buf, 0);
        write_u16_be(buf, 24);
        write_string(buf, account);
        write_string(buf, symbol);
        write_f64_be(buf, qty);
        write_f64_be(buf, price);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        return buf;
}

// SBE decoder for UserLiquidation: validates the header and reads the body.
struct UserLiquidationDecoder {
    static UserLiquidation decode(const std::vector<uint8_t>& buf);
};

inline UserLiquidation UserLiquidationDecoder::decode(const std::vector<uint8_t>& buf) {
    UserLiquidation value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 38) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        std::string symbol = read_string(buf, pos);
        double qty = read_f64_be(buf, pos);
        double price = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.account = account;
    value.symbol = symbol;
    value.qty = qty;
    value.price = price;
    value.timestamp = timestamp;
    (void)value;
    return value;
}

// SBE message OrderListStatus
struct OrderListStatus {
    std::string account;
    std::string list_id;
    OrderListStatusStatus status;
    std::optional<std::string> symbol;
};

// SBE body encoder for OrderListStatus (fields only, no header).
struct OrderListStatusBodyEncoder {
    static void encode(const OrderListStatus& value, std::vector<uint8_t>& buf);
};

inline void OrderListStatusBodyEncoder::encode(const OrderListStatus& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_string(buf, value.list_id);
        buf.push_back(OrderListStatusStatusToValue(value.status));
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
}

// SBE body decoder for OrderListStatus: reads fields at pos.
struct OrderListStatusBodyDecoder {
    static std::pair<OrderListStatus, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OrderListStatus, std::size_t> OrderListStatusBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OrderListStatus value{};
        std::optional<std::string> symbol;
        std::string account = read_string(buf, pos);
        std::string list_id = read_string(buf, pos);
        auto status_v = OrderListStatusStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid OrderListStatusStatus");
        OrderListStatusStatus status = *status_v;
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
    value.account = account;
    value.list_id = list_id;
    value.status = status;
    value.symbol = symbol;
    return {value, pos};
}

// SBE encoder for OrderListStatus
struct OrderListStatusEncoder {
    static std::vector<uint8_t> encode(std::string account, std::string list_id, OrderListStatusStatus status, std::optional<std::string> symbol);
};

inline std::vector<uint8_t> OrderListStatusEncoder::encode(std::string account, std::string list_id, OrderListStatusStatus status, std::optional<std::string> symbol) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 39);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_string(buf, list_id);
        buf.push_back(OrderListStatusStatusToValue(status));
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for OrderListStatus: validates the header and reads the body.
struct OrderListStatusDecoder {
    static OrderListStatus decode(const std::vector<uint8_t>& buf);
};

inline OrderListStatus OrderListStatusDecoder::decode(const std::vector<uint8_t>& buf) {
    OrderListStatus value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 39) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        std::string account = read_string(buf, pos);
        std::string list_id = read_string(buf, pos);
        auto status_v = OrderListStatusStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid OrderListStatusStatus");
        OrderListStatusStatus status = *status_v;
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
    value.account = account;
    value.list_id = list_id;
    value.status = status;
    value.symbol = symbol;
    (void)value;
    return value;
}

// SBE message FillHistoryRequest
struct FillHistoryRequest {
    std::string account;
    std::optional<std::string> symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<uint32_t> limit;
    std::optional<std::string> cursor;
};

// SBE body encoder for FillHistoryRequest (fields only, no header).
struct FillHistoryRequestBodyEncoder {
    static void encode(const FillHistoryRequest& value, std::vector<uint8_t>& buf);
};

inline void FillHistoryRequestBodyEncoder::encode(const FillHistoryRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
        write_u64_be(buf, static_cast<uint64_t>(value.start_time.value_or(0)));
        buf.push_back(value.start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.end_time.value_or(0)));
        buf.push_back(value.end_time.has_value() ? 1 : 0);
        write_u32_be(buf, value.limit.value_or(0));
        buf.push_back(value.limit.has_value() ? 1 : 0);
        if (value.cursor.has_value()) { buf.push_back(1); write_string(buf, *value.cursor); } else { buf.push_back(0); }
}

// SBE body decoder for FillHistoryRequest: reads fields at pos.
struct FillHistoryRequestBodyDecoder {
    static std::pair<FillHistoryRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<FillHistoryRequest, std::size_t> FillHistoryRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    FillHistoryRequest value{};
        std::optional<std::string> symbol;
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    return {value, pos};
}

// SBE encoder for FillHistoryRequest
struct FillHistoryRequestEncoder {
    static std::vector<uint8_t> encode(std::string account, std::optional<std::string> symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor);
};

inline std::vector<uint8_t> FillHistoryRequestEncoder::encode(std::string account, std::optional<std::string> symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 40);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        write_u64_be(buf, static_cast<uint64_t>(start_time.value_or(0)));
        buf.push_back(start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(end_time.value_or(0)));
        buf.push_back(end_time.has_value() ? 1 : 0);
        write_u32_be(buf, limit.value_or(0));
        buf.push_back(limit.has_value() ? 1 : 0);
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for FillHistoryRequest: validates the header and reads the body.
struct FillHistoryRequestDecoder {
    static FillHistoryRequest decode(const std::vector<uint8_t>& buf);
};

inline FillHistoryRequest FillHistoryRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    FillHistoryRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 40) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    (void)value;
    return value;
}

// SBE message FillHistoryBatch
struct FillHistoryBatch {
    std::string account;
    std::vector<ExecutionReport> fills;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

// SBE body encoder for FillHistoryBatch (fields only, no header).
struct FillHistoryBatchBodyEncoder {
    static void encode(const FillHistoryBatch& value, std::vector<uint8_t>& buf);
};

inline void FillHistoryBatchBodyEncoder::encode(const FillHistoryBatch& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u32_be(buf, static_cast<uint32_t>(value.fills.size()));
        for (const auto& item : value.fills) {
            std::vector<uint8_t> item_buf;
            ExecutionReportBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

// SBE body decoder for FillHistoryBatch: reads fields at pos.
struct FillHistoryBatchBodyDecoder {
    static std::pair<FillHistoryBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<FillHistoryBatch, std::size_t> FillHistoryBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    FillHistoryBatch value{};
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t fills_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> fills;
        fills.reserve(fills_count);
        for (uint32_t i = 0; i < fills_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = ExecutionReportBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            fills.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.fills = fills;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE encoder for FillHistoryBatch
struct FillHistoryBatchEncoder {
    static std::vector<uint8_t> encode(std::string account, std::vector<ExecutionReport> fills, uint8_t has_more, std::optional<std::string> next_cursor);
};

inline std::vector<uint8_t> FillHistoryBatchEncoder::encode(std::string account, std::vector<ExecutionReport> fills, uint8_t has_more, std::optional<std::string> next_cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 41);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(fills.size()));
        for (const auto& item : fills) {
            std::vector<uint8_t> item_buf;
            ExecutionReportBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for FillHistoryBatch: validates the header and reads the body.
struct FillHistoryBatchDecoder {
    static FillHistoryBatch decode(const std::vector<uint8_t>& buf);
};

inline FillHistoryBatch FillHistoryBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    FillHistoryBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 41) throw std::runtime_error("invalid template_id");
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t fills_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> fills;
        fills.reserve(fills_count);
        for (uint32_t i = 0; i < fills_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = ExecutionReportBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            fills.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.fills = fills;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    (void)value;
    return value;
}

// SBE message FundingPayment
struct FundingPayment {
    std::string account;
    std::optional<std::string> symbol;
    double amount;
    double rate;
    int64_t timestamp;
};

// SBE body encoder for FundingPayment (fields only, no header).
struct FundingPaymentBodyEncoder {
    static void encode(const FundingPayment& value, std::vector<uint8_t>& buf);
};

inline void FundingPaymentBodyEncoder::encode(const FundingPayment& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
        write_f64_be(buf, value.amount);
        write_f64_be(buf, value.rate);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
}

// SBE body decoder for FundingPayment: reads fields at pos.
struct FundingPaymentBodyDecoder {
    static std::pair<FundingPayment, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<FundingPayment, std::size_t> FundingPaymentBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    FundingPayment value{};
        std::optional<std::string> symbol;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        double amount = read_f64_be(buf, pos);
        double rate = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.account = account;
    value.symbol = symbol;
    value.amount = amount;
    value.rate = rate;
    value.timestamp = timestamp;
    return {value, pos};
}

// SBE encoder for FundingPayment
struct FundingPaymentEncoder {
    static std::vector<uint8_t> encode(std::string account, std::optional<std::string> symbol, double amount, double rate, int64_t timestamp);
};

inline std::vector<uint8_t> FundingPaymentEncoder::encode(std::string account, std::optional<std::string> symbol, double amount, double rate, int64_t timestamp) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 42);
        write_u16_be(buf, 0);
        write_u16_be(buf, 24);
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        write_f64_be(buf, amount);
        write_f64_be(buf, rate);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        return buf;
}

// SBE decoder for FundingPayment: validates the header and reads the body.
struct FundingPaymentDecoder {
    static FundingPayment decode(const std::vector<uint8_t>& buf);
};

inline FundingPayment FundingPaymentDecoder::decode(const std::vector<uint8_t>& buf) {
    FundingPayment value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 42) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        double amount = read_f64_be(buf, pos);
        double rate = read_f64_be(buf, pos);
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.account = account;
    value.symbol = symbol;
    value.amount = amount;
    value.rate = rate;
    value.timestamp = timestamp;
    (void)value;
    return value;
}

// SBE message FundingHistoryRequest
struct FundingHistoryRequest {
    std::string account;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<uint32_t> limit;
    std::optional<std::string> cursor;
};

// SBE body encoder for FundingHistoryRequest (fields only, no header).
struct FundingHistoryRequestBodyEncoder {
    static void encode(const FundingHistoryRequest& value, std::vector<uint8_t>& buf);
};

inline void FundingHistoryRequestBodyEncoder::encode(const FundingHistoryRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u64_be(buf, static_cast<uint64_t>(value.start_time.value_or(0)));
        buf.push_back(value.start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.end_time.value_or(0)));
        buf.push_back(value.end_time.has_value() ? 1 : 0);
        write_u32_be(buf, value.limit.value_or(0));
        buf.push_back(value.limit.has_value() ? 1 : 0);
        if (value.cursor.has_value()) { buf.push_back(1); write_string(buf, *value.cursor); } else { buf.push_back(0); }
}

// SBE body decoder for FundingHistoryRequest: reads fields at pos.
struct FundingHistoryRequestBodyDecoder {
    static std::pair<FundingHistoryRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<FundingHistoryRequest, std::size_t> FundingHistoryRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    FundingHistoryRequest value{};
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    return {value, pos};
}

// SBE encoder for FundingHistoryRequest
struct FundingHistoryRequestEncoder {
    static std::vector<uint8_t> encode(std::string account, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor);
};

inline std::vector<uint8_t> FundingHistoryRequestEncoder::encode(std::string account, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 43);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);
        write_string(buf, account);
        write_u64_be(buf, static_cast<uint64_t>(start_time.value_or(0)));
        buf.push_back(start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(end_time.value_or(0)));
        buf.push_back(end_time.has_value() ? 1 : 0);
        write_u32_be(buf, limit.value_or(0));
        buf.push_back(limit.has_value() ? 1 : 0);
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for FundingHistoryRequest: validates the header and reads the body.
struct FundingHistoryRequestDecoder {
    static FundingHistoryRequest decode(const std::vector<uint8_t>& buf);
};

inline FundingHistoryRequest FundingHistoryRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    FundingHistoryRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 43) throw std::runtime_error("invalid template_id");
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    (void)value;
    return value;
}

// SBE message FundingHistoryBatch
struct FundingHistoryBatch {
    std::string account;
    std::vector<FundingPayment> payments;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

// SBE body encoder for FundingHistoryBatch (fields only, no header).
struct FundingHistoryBatchBodyEncoder {
    static void encode(const FundingHistoryBatch& value, std::vector<uint8_t>& buf);
};

inline void FundingHistoryBatchBodyEncoder::encode(const FundingHistoryBatch& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u32_be(buf, static_cast<uint32_t>(value.payments.size()));
        for (const auto& item : value.payments) {
            std::vector<uint8_t> item_buf;
            FundingPaymentBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

// SBE body decoder for FundingHistoryBatch: reads fields at pos.
struct FundingHistoryBatchBodyDecoder {
    static std::pair<FundingHistoryBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<FundingHistoryBatch, std::size_t> FundingHistoryBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    FundingHistoryBatch value{};
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t payments_count = read_u32_be(buf, pos);
        std::vector<FundingPayment> payments;
        payments.reserve(payments_count);
        for (uint32_t i = 0; i < payments_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = FundingPaymentBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            payments.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.payments = payments;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE encoder for FundingHistoryBatch
struct FundingHistoryBatchEncoder {
    static std::vector<uint8_t> encode(std::string account, std::vector<FundingPayment> payments, uint8_t has_more, std::optional<std::string> next_cursor);
};

inline std::vector<uint8_t> FundingHistoryBatchEncoder::encode(std::string account, std::vector<FundingPayment> payments, uint8_t has_more, std::optional<std::string> next_cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 44);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(payments.size()));
        for (const auto& item : payments) {
            std::vector<uint8_t> item_buf;
            FundingPaymentBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for FundingHistoryBatch: validates the header and reads the body.
struct FundingHistoryBatchDecoder {
    static FundingHistoryBatch decode(const std::vector<uint8_t>& buf);
};

inline FundingHistoryBatch FundingHistoryBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    FundingHistoryBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 44) throw std::runtime_error("invalid template_id");
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t payments_count = read_u32_be(buf, pos);
        std::vector<FundingPayment> payments;
        payments.reserve(payments_count);
        for (uint32_t i = 0; i < payments_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = FundingPaymentBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            payments.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.payments = payments;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    (void)value;
    return value;
}

// SBE message LedgerUpdate
struct LedgerUpdate {
    std::string account;
    std::string asset;
    double delta;
    LedgerUpdateKind kind;
    int64_t timestamp;
    std::optional<std::string> reference_id;
};

// SBE body encoder for LedgerUpdate (fields only, no header).
struct LedgerUpdateBodyEncoder {
    static void encode(const LedgerUpdate& value, std::vector<uint8_t>& buf);
};

inline void LedgerUpdateBodyEncoder::encode(const LedgerUpdate& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_string(buf, value.asset);
        write_f64_be(buf, value.delta);
        buf.push_back(LedgerUpdateKindToValue(value.kind));
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        if (value.reference_id.has_value()) { buf.push_back(1); write_string(buf, *value.reference_id); } else { buf.push_back(0); }
}

// SBE body decoder for LedgerUpdate: reads fields at pos.
struct LedgerUpdateBodyDecoder {
    static std::pair<LedgerUpdate, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<LedgerUpdate, std::size_t> LedgerUpdateBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    LedgerUpdate value{};
        std::optional<std::string> reference_id;
        std::string account = read_string(buf, pos);
        std::string asset = read_string(buf, pos);
        double delta = read_f64_be(buf, pos);
        auto kind_v = LedgerUpdateKindFromValue(take_u8(buf, pos));
        if (!kind_v) throw std::runtime_error("invalid LedgerUpdateKind");
        LedgerUpdateKind kind = *kind_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        if (take_u8(buf, pos) == 1) reference_id = read_string(buf, pos);
    value.account = account;
    value.asset = asset;
    value.delta = delta;
    value.kind = kind;
    value.timestamp = timestamp;
    value.reference_id = reference_id;
    return {value, pos};
}

// SBE encoder for LedgerUpdate
struct LedgerUpdateEncoder {
    static std::vector<uint8_t> encode(std::string account, std::string asset, double delta, LedgerUpdateKind kind, int64_t timestamp, std::optional<std::string> reference_id);
};

inline std::vector<uint8_t> LedgerUpdateEncoder::encode(std::string account, std::string asset, double delta, LedgerUpdateKind kind, int64_t timestamp, std::optional<std::string> reference_id) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 45);
        write_u16_be(buf, 0);
        write_u16_be(buf, 17);
        write_string(buf, account);
        write_string(buf, asset);
        write_f64_be(buf, delta);
        buf.push_back(LedgerUpdateKindToValue(kind));
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        if (reference_id.has_value()) { buf.push_back(1); write_string(buf, *reference_id); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for LedgerUpdate: validates the header and reads the body.
struct LedgerUpdateDecoder {
    static LedgerUpdate decode(const std::vector<uint8_t>& buf);
};

inline LedgerUpdate LedgerUpdateDecoder::decode(const std::vector<uint8_t>& buf) {
    LedgerUpdate value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 45) throw std::runtime_error("invalid template_id");
        std::optional<std::string> reference_id;
        std::string account = read_string(buf, pos);
        std::string asset = read_string(buf, pos);
        double delta = read_f64_be(buf, pos);
        auto kind_v = LedgerUpdateKindFromValue(take_u8(buf, pos));
        if (!kind_v) throw std::runtime_error("invalid LedgerUpdateKind");
        LedgerUpdateKind kind = *kind_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        if (take_u8(buf, pos) == 1) reference_id = read_string(buf, pos);
    value.account = account;
    value.asset = asset;
    value.delta = delta;
    value.kind = kind;
    value.timestamp = timestamp;
    value.reference_id = reference_id;
    (void)value;
    return value;
}

// SBE message LedgerHistoryRequest
struct LedgerHistoryRequest {
    std::string account;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<uint32_t> limit;
    std::optional<std::string> cursor;
};

// SBE body encoder for LedgerHistoryRequest (fields only, no header).
struct LedgerHistoryRequestBodyEncoder {
    static void encode(const LedgerHistoryRequest& value, std::vector<uint8_t>& buf);
};

inline void LedgerHistoryRequestBodyEncoder::encode(const LedgerHistoryRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u64_be(buf, static_cast<uint64_t>(value.start_time.value_or(0)));
        buf.push_back(value.start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.end_time.value_or(0)));
        buf.push_back(value.end_time.has_value() ? 1 : 0);
        write_u32_be(buf, value.limit.value_or(0));
        buf.push_back(value.limit.has_value() ? 1 : 0);
        if (value.cursor.has_value()) { buf.push_back(1); write_string(buf, *value.cursor); } else { buf.push_back(0); }
}

// SBE body decoder for LedgerHistoryRequest: reads fields at pos.
struct LedgerHistoryRequestBodyDecoder {
    static std::pair<LedgerHistoryRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<LedgerHistoryRequest, std::size_t> LedgerHistoryRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    LedgerHistoryRequest value{};
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    return {value, pos};
}

// SBE encoder for LedgerHistoryRequest
struct LedgerHistoryRequestEncoder {
    static std::vector<uint8_t> encode(std::string account, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor);
};

inline std::vector<uint8_t> LedgerHistoryRequestEncoder::encode(std::string account, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 46);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);
        write_string(buf, account);
        write_u64_be(buf, static_cast<uint64_t>(start_time.value_or(0)));
        buf.push_back(start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(end_time.value_or(0)));
        buf.push_back(end_time.has_value() ? 1 : 0);
        write_u32_be(buf, limit.value_or(0));
        buf.push_back(limit.has_value() ? 1 : 0);
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for LedgerHistoryRequest: validates the header and reads the body.
struct LedgerHistoryRequestDecoder {
    static LedgerHistoryRequest decode(const std::vector<uint8_t>& buf);
};

inline LedgerHistoryRequest LedgerHistoryRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    LedgerHistoryRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 46) throw std::runtime_error("invalid template_id");
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    (void)value;
    return value;
}

// SBE message LedgerHistoryBatch
struct LedgerHistoryBatch {
    std::string account;
    std::vector<LedgerUpdate> entries;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

// SBE body encoder for LedgerHistoryBatch (fields only, no header).
struct LedgerHistoryBatchBodyEncoder {
    static void encode(const LedgerHistoryBatch& value, std::vector<uint8_t>& buf);
};

inline void LedgerHistoryBatchBodyEncoder::encode(const LedgerHistoryBatch& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u32_be(buf, static_cast<uint32_t>(value.entries.size()));
        for (const auto& item : value.entries) {
            std::vector<uint8_t> item_buf;
            LedgerUpdateBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

// SBE body decoder for LedgerHistoryBatch: reads fields at pos.
struct LedgerHistoryBatchBodyDecoder {
    static std::pair<LedgerHistoryBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<LedgerHistoryBatch, std::size_t> LedgerHistoryBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    LedgerHistoryBatch value{};
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t entries_count = read_u32_be(buf, pos);
        std::vector<LedgerUpdate> entries;
        entries.reserve(entries_count);
        for (uint32_t i = 0; i < entries_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = LedgerUpdateBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            entries.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.entries = entries;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE encoder for LedgerHistoryBatch
struct LedgerHistoryBatchEncoder {
    static std::vector<uint8_t> encode(std::string account, std::vector<LedgerUpdate> entries, uint8_t has_more, std::optional<std::string> next_cursor);
};

inline std::vector<uint8_t> LedgerHistoryBatchEncoder::encode(std::string account, std::vector<LedgerUpdate> entries, uint8_t has_more, std::optional<std::string> next_cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 47);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(entries.size()));
        for (const auto& item : entries) {
            std::vector<uint8_t> item_buf;
            LedgerUpdateBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for LedgerHistoryBatch: validates the header and reads the body.
struct LedgerHistoryBatchDecoder {
    static LedgerHistoryBatch decode(const std::vector<uint8_t>& buf);
};

inline LedgerHistoryBatch LedgerHistoryBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    LedgerHistoryBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 47) throw std::runtime_error("invalid template_id");
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t entries_count = read_u32_be(buf, pos);
        std::vector<LedgerUpdate> entries;
        entries.reserve(entries_count);
        for (uint32_t i = 0; i < entries_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = LedgerUpdateBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            entries.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.entries = entries;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    (void)value;
    return value;
}

// SBE message OpenOrdersRequest
struct OpenOrdersRequest {
    std::string account;
    std::optional<std::string> symbol;
};

// SBE body encoder for OpenOrdersRequest (fields only, no header).
struct OpenOrdersRequestBodyEncoder {
    static void encode(const OpenOrdersRequest& value, std::vector<uint8_t>& buf);
};

inline void OpenOrdersRequestBodyEncoder::encode(const OpenOrdersRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
}

// SBE body decoder for OpenOrdersRequest: reads fields at pos.
struct OpenOrdersRequestBodyDecoder {
    static std::pair<OpenOrdersRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OpenOrdersRequest, std::size_t> OpenOrdersRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OpenOrdersRequest value{};
        std::optional<std::string> symbol;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
    value.account = account;
    value.symbol = symbol;
    return {value, pos};
}

// SBE encoder for OpenOrdersRequest
struct OpenOrdersRequestEncoder {
    static std::vector<uint8_t> encode(std::string account, std::optional<std::string> symbol);
};

inline std::vector<uint8_t> OpenOrdersRequestEncoder::encode(std::string account, std::optional<std::string> symbol) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 48);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for OpenOrdersRequest: validates the header and reads the body.
struct OpenOrdersRequestDecoder {
    static OpenOrdersRequest decode(const std::vector<uint8_t>& buf);
};

inline OpenOrdersRequest OpenOrdersRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    OpenOrdersRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 48) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
    value.account = account;
    value.symbol = symbol;
    (void)value;
    return value;
}

// SBE message OpenOrdersSnapshot
struct OpenOrdersSnapshot {
    std::string account;
    std::vector<ExecutionReport> orders;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for OpenOrdersSnapshot (fields only, no header).
struct OpenOrdersSnapshotBodyEncoder {
    static void encode(const OpenOrdersSnapshot& value, std::vector<uint8_t>& buf);
};

inline void OpenOrdersSnapshotBodyEncoder::encode(const OpenOrdersSnapshot& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u32_be(buf, static_cast<uint32_t>(value.orders.size()));
        for (const auto& item : value.orders) {
            std::vector<uint8_t> item_buf;
            ExecutionReportBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for OpenOrdersSnapshot: reads fields at pos.
struct OpenOrdersSnapshotBodyDecoder {
    static std::pair<OpenOrdersSnapshot, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OpenOrdersSnapshot, std::size_t> OpenOrdersSnapshotBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OpenOrdersSnapshot value{};
        std::string account = read_string(buf, pos);
        uint32_t orders_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> orders;
        orders.reserve(orders_count);
        for (uint32_t i = 0; i < orders_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = ExecutionReportBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            orders.push_back(item);
        }
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.orders = orders;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for OpenOrdersSnapshot
struct OpenOrdersSnapshotEncoder {
    static std::vector<uint8_t> encode(std::string account, std::vector<ExecutionReport> orders, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> OpenOrdersSnapshotEncoder::encode(std::string account, std::vector<ExecutionReport> orders, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 49);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(orders.size()));
        for (const auto& item : orders) {
            std::vector<uint8_t> item_buf;
            ExecutionReportBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for OpenOrdersSnapshot: validates the header and reads the body.
struct OpenOrdersSnapshotDecoder {
    static OpenOrdersSnapshot decode(const std::vector<uint8_t>& buf);
};

inline OpenOrdersSnapshot OpenOrdersSnapshotDecoder::decode(const std::vector<uint8_t>& buf) {
    OpenOrdersSnapshot value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 49) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
        uint32_t orders_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> orders;
        orders.reserve(orders_count);
        for (uint32_t i = 0; i < orders_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = ExecutionReportBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            orders.push_back(item);
        }
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.account = account;
    value.orders = orders;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message OrderHistoryRequest
struct OrderHistoryRequest {
    std::string account;
    std::optional<std::string> symbol;
    std::optional<int64_t> start_time;
    std::optional<int64_t> end_time;
    std::optional<uint32_t> limit;
    std::optional<std::string> cursor;
};

// SBE body encoder for OrderHistoryRequest (fields only, no header).
struct OrderHistoryRequestBodyEncoder {
    static void encode(const OrderHistoryRequest& value, std::vector<uint8_t>& buf);
};

inline void OrderHistoryRequestBodyEncoder::encode(const OrderHistoryRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
        write_u64_be(buf, static_cast<uint64_t>(value.start_time.value_or(0)));
        buf.push_back(value.start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.end_time.value_or(0)));
        buf.push_back(value.end_time.has_value() ? 1 : 0);
        write_u32_be(buf, value.limit.value_or(0));
        buf.push_back(value.limit.has_value() ? 1 : 0);
        if (value.cursor.has_value()) { buf.push_back(1); write_string(buf, *value.cursor); } else { buf.push_back(0); }
}

// SBE body decoder for OrderHistoryRequest: reads fields at pos.
struct OrderHistoryRequestBodyDecoder {
    static std::pair<OrderHistoryRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OrderHistoryRequest, std::size_t> OrderHistoryRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OrderHistoryRequest value{};
        std::optional<std::string> symbol;
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    return {value, pos};
}

// SBE encoder for OrderHistoryRequest
struct OrderHistoryRequestEncoder {
    static std::vector<uint8_t> encode(std::string account, std::optional<std::string> symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor);
};

inline std::vector<uint8_t> OrderHistoryRequestEncoder::encode(std::string account, std::optional<std::string> symbol, std::optional<int64_t> start_time, std::optional<int64_t> end_time, std::optional<uint32_t> limit, std::optional<std::string> cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 50);
        write_u16_be(buf, 0);
        write_u16_be(buf, 20);
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        write_u64_be(buf, static_cast<uint64_t>(start_time.value_or(0)));
        buf.push_back(start_time.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(end_time.value_or(0)));
        buf.push_back(end_time.has_value() ? 1 : 0);
        write_u32_be(buf, limit.value_or(0));
        buf.push_back(limit.has_value() ? 1 : 0);
        if (cursor.has_value()) { buf.push_back(1); write_string(buf, *cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for OrderHistoryRequest: validates the header and reads the body.
struct OrderHistoryRequestDecoder {
    static OrderHistoryRequest decode(const std::vector<uint8_t>& buf);
};

inline OrderHistoryRequest OrderHistoryRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    OrderHistoryRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 50) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        std::optional<std::string> cursor;
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        int64_t start_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> start_time;
        if (buf[pos++] == 1) start_time = start_time_v;
        int64_t end_time_v = static_cast<int64_t>(read_u64_be(buf, pos));
        std::optional<int64_t> end_time;
        if (buf[pos++] == 1) end_time = end_time_v;
        uint32_t limit_v = read_u32_be(buf, pos);
        std::optional<uint32_t> limit;
        if (buf[pos++] == 1) limit = limit_v;
        if (take_u8(buf, pos) == 1) cursor = read_string(buf, pos);
    value.account = account;
    value.symbol = symbol;
    value.start_time = start_time;
    value.end_time = end_time;
    value.limit = limit;
    value.cursor = cursor;
    (void)value;
    return value;
}

// SBE message OrderHistoryBatch
struct OrderHistoryBatch {
    std::string account;
    std::vector<ExecutionReport> orders;
    uint8_t has_more;
    std::optional<std::string> next_cursor;
};

// SBE body encoder for OrderHistoryBatch (fields only, no header).
struct OrderHistoryBatchBodyEncoder {
    static void encode(const OrderHistoryBatch& value, std::vector<uint8_t>& buf);
};

inline void OrderHistoryBatchBodyEncoder::encode(const OrderHistoryBatch& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
        write_u32_be(buf, static_cast<uint32_t>(value.orders.size()));
        for (const auto& item : value.orders) {
            std::vector<uint8_t> item_buf;
            ExecutionReportBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(value.has_more);
        if (value.next_cursor.has_value()) { buf.push_back(1); write_string(buf, *value.next_cursor); } else { buf.push_back(0); }
}

// SBE body decoder for OrderHistoryBatch: reads fields at pos.
struct OrderHistoryBatchBodyDecoder {
    static std::pair<OrderHistoryBatch, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<OrderHistoryBatch, std::size_t> OrderHistoryBatchBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    OrderHistoryBatch value{};
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t orders_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> orders;
        orders.reserve(orders_count);
        for (uint32_t i = 0; i < orders_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = ExecutionReportBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            orders.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.orders = orders;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    return {value, pos};
}

// SBE encoder for OrderHistoryBatch
struct OrderHistoryBatchEncoder {
    static std::vector<uint8_t> encode(std::string account, std::vector<ExecutionReport> orders, uint8_t has_more, std::optional<std::string> next_cursor);
};

inline std::vector<uint8_t> OrderHistoryBatchEncoder::encode(std::string account, std::vector<ExecutionReport> orders, uint8_t has_more, std::optional<std::string> next_cursor) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 51);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, account);
        write_u32_be(buf, static_cast<uint32_t>(orders.size()));
        for (const auto& item : orders) {
            std::vector<uint8_t> item_buf;
            ExecutionReportBodyEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        buf.push_back(has_more);
        if (next_cursor.has_value()) { buf.push_back(1); write_string(buf, *next_cursor); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for OrderHistoryBatch: validates the header and reads the body.
struct OrderHistoryBatchDecoder {
    static OrderHistoryBatch decode(const std::vector<uint8_t>& buf);
};

inline OrderHistoryBatch OrderHistoryBatchDecoder::decode(const std::vector<uint8_t>& buf) {
    OrderHistoryBatch value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 51) throw std::runtime_error("invalid template_id");
        std::optional<std::string> next_cursor;
        std::string account = read_string(buf, pos);
        uint32_t orders_count = read_u32_be(buf, pos);
        std::vector<ExecutionReport> orders;
        orders.reserve(orders_count);
        for (uint32_t i = 0; i < orders_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = ExecutionReportBodyDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            orders.push_back(item);
        }
        uint8_t has_more = take_u8(buf, pos);
        if (take_u8(buf, pos) == 1) next_cursor = read_string(buf, pos);
    value.account = account;
    value.orders = orders;
    value.has_more = has_more;
    value.next_cursor = next_cursor;
    (void)value;
    return value;
}

// SBE message CapabilitiesRequest
struct CapabilitiesRequest {
};

// SBE body encoder for CapabilitiesRequest (fields only, no header).
struct CapabilitiesRequestBodyEncoder {
    static void encode(const CapabilitiesRequest& value, std::vector<uint8_t>& buf);
};

inline void CapabilitiesRequestBodyEncoder::encode(const CapabilitiesRequest& value, std::vector<uint8_t>& buf) {
}

// SBE body decoder for CapabilitiesRequest: reads fields at pos.
struct CapabilitiesRequestBodyDecoder {
    static std::pair<CapabilitiesRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CapabilitiesRequest, std::size_t> CapabilitiesRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CapabilitiesRequest value{};
    return {value, pos};
}

// SBE encoder for CapabilitiesRequest
struct CapabilitiesRequestEncoder {
    static std::vector<uint8_t> encode();
};

inline std::vector<uint8_t> CapabilitiesRequestEncoder::encode() {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 52);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        return buf;
}

// SBE decoder for CapabilitiesRequest: validates the header and reads the body.
struct CapabilitiesRequestDecoder {
    static CapabilitiesRequest decode(const std::vector<uint8_t>& buf);
};

inline CapabilitiesRequest CapabilitiesRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    CapabilitiesRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 52) throw std::runtime_error("invalid template_id");
    (void)value;
    return value;
}

// SBE message CapabilitiesResponse
struct CapabilitiesResponse {
    std::vector<uint8_t> schema_ids;
    std::vector<CapabilityPath> paths;
    std::vector<std::string> symbols;
    std::vector<std::string> intervals;
};

// SBE body encoder for CapabilitiesResponse (fields only, no header).
struct CapabilitiesResponseBodyEncoder {
    static void encode(const CapabilitiesResponse& value, std::vector<uint8_t>& buf);
};

inline void CapabilitiesResponseBodyEncoder::encode(const CapabilitiesResponse& value, std::vector<uint8_t>& buf) {
        write_u32_be(buf, static_cast<uint32_t>(value.schema_ids.size()));
        for (const auto& item : value.schema_ids) {
        buf.push_back(item);
        }
        write_u32_be(buf, static_cast<uint32_t>(value.paths.size()));
        for (const auto& item : value.paths) {
            std::vector<uint8_t> item_buf;
            CapabilityPathEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u32_be(buf, static_cast<uint32_t>(value.symbols.size()));
        for (const auto& item : value.symbols) {
            write_string_u32(buf, item);
        }
        write_u32_be(buf, static_cast<uint32_t>(value.intervals.size()));
        for (const auto& item : value.intervals) {
            write_string_u32(buf, item);
        }
}

// SBE body decoder for CapabilitiesResponse: reads fields at pos.
struct CapabilitiesResponseBodyDecoder {
    static std::pair<CapabilitiesResponse, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<CapabilitiesResponse, std::size_t> CapabilitiesResponseBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    CapabilitiesResponse value{};
        uint32_t schema_ids_count = read_u32_be(buf, pos);
        std::vector<uint8_t> schema_ids;
        schema_ids.reserve(schema_ids_count);
        for (uint32_t i = 0; i < schema_ids_count; ++i) {
            schema_ids.push_back(take_u8(buf, pos));
        }
        uint32_t paths_count = read_u32_be(buf, pos);
        std::vector<CapabilityPath> paths;
        paths.reserve(paths_count);
        for (uint32_t i = 0; i < paths_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = CapabilityPathDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            paths.push_back(item);
        }
        uint32_t symbols_count = read_u32_be(buf, pos);
        std::vector<std::string> symbols;
        symbols.reserve(symbols_count);
        for (uint32_t i = 0; i < symbols_count; ++i) {
            symbols.push_back(read_string_u32(buf, pos));
        }
        uint32_t intervals_count = read_u32_be(buf, pos);
        std::vector<std::string> intervals;
        intervals.reserve(intervals_count);
        for (uint32_t i = 0; i < intervals_count; ++i) {
            intervals.push_back(read_string_u32(buf, pos));
        }
    value.schema_ids = schema_ids;
    value.paths = paths;
    value.symbols = symbols;
    value.intervals = intervals;
    return {value, pos};
}

// SBE encoder for CapabilitiesResponse
struct CapabilitiesResponseEncoder {
    static std::vector<uint8_t> encode(std::vector<uint8_t> schema_ids, std::vector<CapabilityPath> paths, std::vector<std::string> symbols, std::vector<std::string> intervals);
};

inline std::vector<uint8_t> CapabilitiesResponseEncoder::encode(std::vector<uint8_t> schema_ids, std::vector<CapabilityPath> paths, std::vector<std::string> symbols, std::vector<std::string> intervals) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 53);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_u32_be(buf, static_cast<uint32_t>(schema_ids.size()));
        for (const auto& item : schema_ids) {
        buf.push_back(item);
        }
        write_u32_be(buf, static_cast<uint32_t>(paths.size()));
        for (const auto& item : paths) {
            std::vector<uint8_t> item_buf;
            CapabilityPathEncoder::encode(item, item_buf);
            write_u32_be(buf, static_cast<uint32_t>(item_buf.size()));
            buf.insert(buf.end(), item_buf.begin(), item_buf.end());
        }
        write_u32_be(buf, static_cast<uint32_t>(symbols.size()));
        for (const auto& item : symbols) {
            write_string_u32(buf, item);
        }
        write_u32_be(buf, static_cast<uint32_t>(intervals.size()));
        for (const auto& item : intervals) {
            write_string_u32(buf, item);
        }
        return buf;
}

// SBE decoder for CapabilitiesResponse: validates the header and reads the body.
struct CapabilitiesResponseDecoder {
    static CapabilitiesResponse decode(const std::vector<uint8_t>& buf);
};

inline CapabilitiesResponse CapabilitiesResponseDecoder::decode(const std::vector<uint8_t>& buf) {
    CapabilitiesResponse value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 53) throw std::runtime_error("invalid template_id");
        uint32_t schema_ids_count = read_u32_be(buf, pos);
        std::vector<uint8_t> schema_ids;
        schema_ids.reserve(schema_ids_count);
        for (uint32_t i = 0; i < schema_ids_count; ++i) {
            schema_ids.push_back(take_u8(buf, pos));
        }
        uint32_t paths_count = read_u32_be(buf, pos);
        std::vector<CapabilityPath> paths;
        paths.reserve(paths_count);
        for (uint32_t i = 0; i < paths_count; ++i) {
            uint32_t item_len = read_u32_be(buf, pos);
            std::size_t item_end = pos + item_len;
            auto [item, consumed] = CapabilityPathDecoder::decode(buf, pos);
            (void)consumed;
            pos = item_end;
            paths.push_back(item);
        }
        uint32_t symbols_count = read_u32_be(buf, pos);
        std::vector<std::string> symbols;
        symbols.reserve(symbols_count);
        for (uint32_t i = 0; i < symbols_count; ++i) {
            symbols.push_back(read_string_u32(buf, pos));
        }
        uint32_t intervals_count = read_u32_be(buf, pos);
        std::vector<std::string> intervals;
        intervals.reserve(intervals_count);
        for (uint32_t i = 0; i < intervals_count; ++i) {
            intervals.push_back(read_string_u32(buf, pos));
        }
    value.schema_ids = schema_ids;
    value.paths = paths;
    value.symbols = symbols;
    value.intervals = intervals;
    (void)value;
    return value;
}

// SBE message PositionRequest
struct PositionRequest {
    std::string account;
};

// SBE body encoder for PositionRequest (fields only, no header).
struct PositionRequestBodyEncoder {
    static void encode(const PositionRequest& value, std::vector<uint8_t>& buf);
};

inline void PositionRequestBodyEncoder::encode(const PositionRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.account);
}

// SBE body decoder for PositionRequest: reads fields at pos.
struct PositionRequestBodyDecoder {
    static std::pair<PositionRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<PositionRequest, std::size_t> PositionRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    PositionRequest value{};
        std::string account = read_string(buf, pos);
    value.account = account;
    return {value, pos};
}

// SBE encoder for PositionRequest
struct PositionRequestEncoder {
    static std::vector<uint8_t> encode(std::string account);
};

inline std::vector<uint8_t> PositionRequestEncoder::encode(std::string account) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 54);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_string(buf, account);
        return buf;
}

// SBE decoder for PositionRequest: validates the header and reads the body.
struct PositionRequestDecoder {
    static PositionRequest decode(const std::vector<uint8_t>& buf);
};

inline PositionRequest PositionRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    PositionRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 54) throw std::runtime_error("invalid template_id");
        std::string account = read_string(buf, pos);
    value.account = account;
    (void)value;
    return value;
}

// SBE message TradingSessionStatusRequest
struct TradingSessionStatusRequest {
    std::optional<std::string> symbol;
};

// SBE body encoder for TradingSessionStatusRequest (fields only, no header).
struct TradingSessionStatusRequestBodyEncoder {
    static void encode(const TradingSessionStatusRequest& value, std::vector<uint8_t>& buf);
};

inline void TradingSessionStatusRequestBodyEncoder::encode(const TradingSessionStatusRequest& value, std::vector<uint8_t>& buf) {
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
}

// SBE body decoder for TradingSessionStatusRequest: reads fields at pos.
struct TradingSessionStatusRequestBodyDecoder {
    static std::pair<TradingSessionStatusRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<TradingSessionStatusRequest, std::size_t> TradingSessionStatusRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    TradingSessionStatusRequest value{};
        std::optional<std::string> symbol;
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
    value.symbol = symbol;
    return {value, pos};
}

// SBE encoder for TradingSessionStatusRequest
struct TradingSessionStatusRequestEncoder {
    static std::vector<uint8_t> encode(std::optional<std::string> symbol);
};

inline std::vector<uint8_t> TradingSessionStatusRequestEncoder::encode(std::optional<std::string> symbol) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 55);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        return buf;
}

// SBE decoder for TradingSessionStatusRequest: validates the header and reads the body.
struct TradingSessionStatusRequestDecoder {
    static TradingSessionStatusRequest decode(const std::vector<uint8_t>& buf);
};

inline TradingSessionStatusRequest TradingSessionStatusRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    TradingSessionStatusRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 55) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
    value.symbol = symbol;
    (void)value;
    return value;
}

// SBE message TradingSessionStatus
struct TradingSessionStatus {
    std::optional<std::string> symbol;
    std::string trading_session_id;
    TradingSessionStatusStatus status;
    std::optional<TradingSessionStatusHaltReason> halt_reason;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for TradingSessionStatus (fields only, no header).
struct TradingSessionStatusBodyEncoder {
    static void encode(const TradingSessionStatus& value, std::vector<uint8_t>& buf);
};

inline void TradingSessionStatusBodyEncoder::encode(const TradingSessionStatus& value, std::vector<uint8_t>& buf) {
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
        write_string(buf, value.trading_session_id);
        buf.push_back(TradingSessionStatusStatusToValue(value.status));
        buf.push_back(value.halt_reason.has_value() ? TradingSessionStatusHaltReasonToValue(*value.halt_reason) : 0);
        buf.push_back(value.halt_reason.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for TradingSessionStatus: reads fields at pos.
struct TradingSessionStatusBodyDecoder {
    static std::pair<TradingSessionStatus, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<TradingSessionStatus, std::size_t> TradingSessionStatusBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    TradingSessionStatus value{};
        std::optional<std::string> symbol;
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        std::string trading_session_id = read_string(buf, pos);
        auto status_v = TradingSessionStatusStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid TradingSessionStatusStatus");
        TradingSessionStatusStatus status = *status_v;
        auto halt_reason_raw = take_u8(buf, pos);
        std::optional<TradingSessionStatusHaltReason> halt_reason;
        if (buf[pos++] == 1) {
            auto v = TradingSessionStatusHaltReasonFromValue(halt_reason_raw);
            if (!v) throw std::runtime_error("invalid TradingSessionStatusHaltReason");
            halt_reason = *v;
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.trading_session_id = trading_session_id;
    value.status = status;
    value.halt_reason = halt_reason;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for TradingSessionStatus
struct TradingSessionStatusEncoder {
    static std::vector<uint8_t> encode(std::optional<std::string> symbol, std::string trading_session_id, TradingSessionStatusStatus status, std::optional<TradingSessionStatusHaltReason> halt_reason, int64_t timestamp, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> TradingSessionStatusEncoder::encode(std::optional<std::string> symbol, std::string trading_session_id, TradingSessionStatusStatus status, std::optional<TradingSessionStatusHaltReason> halt_reason, int64_t timestamp, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 56);
        write_u16_be(buf, 0);
        write_u16_be(buf, 11);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        write_string(buf, trading_session_id);
        buf.push_back(TradingSessionStatusStatusToValue(status));
        buf.push_back(halt_reason.has_value() ? TradingSessionStatusHaltReasonToValue(*halt_reason) : 0);
        buf.push_back(halt_reason.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for TradingSessionStatus: validates the header and reads the body.
struct TradingSessionStatusDecoder {
    static TradingSessionStatus decode(const std::vector<uint8_t>& buf);
};

inline TradingSessionStatus TradingSessionStatusDecoder::decode(const std::vector<uint8_t>& buf) {
    TradingSessionStatus value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 56) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        std::string trading_session_id = read_string(buf, pos);
        auto status_v = TradingSessionStatusStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid TradingSessionStatusStatus");
        TradingSessionStatusStatus status = *status_v;
        auto halt_reason_raw = take_u8(buf, pos);
        std::optional<TradingSessionStatusHaltReason> halt_reason;
        if (buf[pos++] == 1) {
            auto v = TradingSessionStatusHaltReasonFromValue(halt_reason_raw);
            if (!v) throw std::runtime_error("invalid TradingSessionStatusHaltReason");
            halt_reason = *v;
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.trading_session_id = trading_session_id;
    value.status = status;
    value.halt_reason = halt_reason;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message SecurityStatusRequest
struct SecurityStatusRequest {
    std::string symbol;
};

// SBE body encoder for SecurityStatusRequest (fields only, no header).
struct SecurityStatusRequestBodyEncoder {
    static void encode(const SecurityStatusRequest& value, std::vector<uint8_t>& buf);
};

inline void SecurityStatusRequestBodyEncoder::encode(const SecurityStatusRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
}

// SBE body decoder for SecurityStatusRequest: reads fields at pos.
struct SecurityStatusRequestBodyDecoder {
    static std::pair<SecurityStatusRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<SecurityStatusRequest, std::size_t> SecurityStatusRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    SecurityStatusRequest value{};
        std::string symbol = read_string(buf, pos);
    value.symbol = symbol;
    return {value, pos};
}

// SBE encoder for SecurityStatusRequest
struct SecurityStatusRequestEncoder {
    static std::vector<uint8_t> encode(std::string symbol);
};

inline std::vector<uint8_t> SecurityStatusRequestEncoder::encode(std::string symbol) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 57);
        write_u16_be(buf, 0);
        write_u16_be(buf, 0);
        write_string(buf, symbol);
        return buf;
}

// SBE decoder for SecurityStatusRequest: validates the header and reads the body.
struct SecurityStatusRequestDecoder {
    static SecurityStatusRequest decode(const std::vector<uint8_t>& buf);
};

inline SecurityStatusRequest SecurityStatusRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    SecurityStatusRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 57) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
    value.symbol = symbol;
    (void)value;
    return value;
}

// SBE message SecurityStatus
struct SecurityStatus {
    std::string symbol;
    SecurityStatusStatus status;
    std::optional<SecurityStatusHaltReason> halt_reason;
    int64_t timestamp;
    std::optional<uint8_t> is_snapshot;
};

// SBE body encoder for SecurityStatus (fields only, no header).
struct SecurityStatusBodyEncoder {
    static void encode(const SecurityStatus& value, std::vector<uint8_t>& buf);
};

inline void SecurityStatusBodyEncoder::encode(const SecurityStatus& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.symbol);
        buf.push_back(SecurityStatusStatusToValue(value.status));
        buf.push_back(value.halt_reason.has_value() ? SecurityStatusHaltReasonToValue(*value.halt_reason) : 0);
        buf.push_back(value.halt_reason.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
        buf.push_back(value.is_snapshot.value_or(0));
        buf.push_back(value.is_snapshot.has_value() ? 1 : 0);
}

// SBE body decoder for SecurityStatus: reads fields at pos.
struct SecurityStatusBodyDecoder {
    static std::pair<SecurityStatus, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<SecurityStatus, std::size_t> SecurityStatusBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    SecurityStatus value{};
        std::string symbol = read_string(buf, pos);
        auto status_v = SecurityStatusStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid SecurityStatusStatus");
        SecurityStatusStatus status = *status_v;
        auto halt_reason_raw = take_u8(buf, pos);
        std::optional<SecurityStatusHaltReason> halt_reason;
        if (buf[pos++] == 1) {
            auto v = SecurityStatusHaltReasonFromValue(halt_reason_raw);
            if (!v) throw std::runtime_error("invalid SecurityStatusHaltReason");
            halt_reason = *v;
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.status = status;
    value.halt_reason = halt_reason;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    return {value, pos};
}

// SBE encoder for SecurityStatus
struct SecurityStatusEncoder {
    static std::vector<uint8_t> encode(std::string symbol, SecurityStatusStatus status, std::optional<SecurityStatusHaltReason> halt_reason, int64_t timestamp, std::optional<uint8_t> is_snapshot);
};

inline std::vector<uint8_t> SecurityStatusEncoder::encode(std::string symbol, SecurityStatusStatus status, std::optional<SecurityStatusHaltReason> halt_reason, int64_t timestamp, std::optional<uint8_t> is_snapshot) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 58);
        write_u16_be(buf, 0);
        write_u16_be(buf, 11);
        write_string(buf, symbol);
        buf.push_back(SecurityStatusStatusToValue(status));
        buf.push_back(halt_reason.has_value() ? SecurityStatusHaltReasonToValue(*halt_reason) : 0);
        buf.push_back(halt_reason.has_value() ? 1 : 0);
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        buf.push_back(is_snapshot.value_or(0));
        buf.push_back(is_snapshot.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for SecurityStatus: validates the header and reads the body.
struct SecurityStatusDecoder {
    static SecurityStatus decode(const std::vector<uint8_t>& buf);
};

inline SecurityStatus SecurityStatusDecoder::decode(const std::vector<uint8_t>& buf) {
    SecurityStatus value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 58) throw std::runtime_error("invalid template_id");
        std::string symbol = read_string(buf, pos);
        auto status_v = SecurityStatusStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid SecurityStatusStatus");
        SecurityStatusStatus status = *status_v;
        auto halt_reason_raw = take_u8(buf, pos);
        std::optional<SecurityStatusHaltReason> halt_reason;
        if (buf[pos++] == 1) {
            auto v = SecurityStatusHaltReasonFromValue(halt_reason_raw);
            if (!v) throw std::runtime_error("invalid SecurityStatusHaltReason");
            halt_reason = *v;
        }
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
        uint8_t is_snapshot_v = take_u8(buf, pos);
        std::optional<uint8_t> is_snapshot;
        if (buf[pos++] == 1) is_snapshot = is_snapshot_v;
    value.symbol = symbol;
    value.status = status;
    value.halt_reason = halt_reason;
    value.timestamp = timestamp;
    value.is_snapshot = is_snapshot;
    (void)value;
    return value;
}

// SBE message MassCancelRequest
struct MassCancelRequest {
    std::string cl_ord_id;
    std::string account;
    std::optional<std::string> symbol;
    std::optional<MassCancelRequestSide> side;
};

// SBE body encoder for MassCancelRequest (fields only, no header).
struct MassCancelRequestBodyEncoder {
    static void encode(const MassCancelRequest& value, std::vector<uint8_t>& buf);
};

inline void MassCancelRequestBodyEncoder::encode(const MassCancelRequest& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.cl_ord_id);
        write_string(buf, value.account);
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
        buf.push_back(value.side.has_value() ? MassCancelRequestSideToValue(*value.side) : 0);
        buf.push_back(value.side.has_value() ? 1 : 0);
}

// SBE body decoder for MassCancelRequest: reads fields at pos.
struct MassCancelRequestBodyDecoder {
    static std::pair<MassCancelRequest, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MassCancelRequest, std::size_t> MassCancelRequestBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MassCancelRequest value{};
        std::optional<std::string> symbol;
        std::string cl_ord_id = read_string(buf, pos);
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        auto side_raw = take_u8(buf, pos);
        std::optional<MassCancelRequestSide> side;
        if (buf[pos++] == 1) {
            auto v = MassCancelRequestSideFromValue(side_raw);
            if (!v) throw std::runtime_error("invalid MassCancelRequestSide");
            side = *v;
        }
    value.cl_ord_id = cl_ord_id;
    value.account = account;
    value.symbol = symbol;
    value.side = side;
    return {value, pos};
}

// SBE encoder for MassCancelRequest
struct MassCancelRequestEncoder {
    static std::vector<uint8_t> encode(std::string cl_ord_id, std::string account, std::optional<std::string> symbol, std::optional<MassCancelRequestSide> side);
};

inline std::vector<uint8_t> MassCancelRequestEncoder::encode(std::string cl_ord_id, std::string account, std::optional<std::string> symbol, std::optional<MassCancelRequestSide> side) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 59);
        write_u16_be(buf, 0);
        write_u16_be(buf, 1);
        write_string(buf, cl_ord_id);
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        buf.push_back(side.has_value() ? MassCancelRequestSideToValue(*side) : 0);
        buf.push_back(side.has_value() ? 1 : 0);
        return buf;
}

// SBE decoder for MassCancelRequest: validates the header and reads the body.
struct MassCancelRequestDecoder {
    static MassCancelRequest decode(const std::vector<uint8_t>& buf);
};

inline MassCancelRequest MassCancelRequestDecoder::decode(const std::vector<uint8_t>& buf) {
    MassCancelRequest value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 59) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        std::string cl_ord_id = read_string(buf, pos);
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        auto side_raw = take_u8(buf, pos);
        std::optional<MassCancelRequestSide> side;
        if (buf[pos++] == 1) {
            auto v = MassCancelRequestSideFromValue(side_raw);
            if (!v) throw std::runtime_error("invalid MassCancelRequestSide");
            side = *v;
        }
    value.cl_ord_id = cl_ord_id;
    value.account = account;
    value.symbol = symbol;
    value.side = side;
    (void)value;
    return value;
}

// SBE message MassCancelReport
struct MassCancelReport {
    std::string cl_ord_id;
    std::string account;
    std::optional<std::string> symbol;
    uint32_t total_affected;
    MassCancelReportStatus status;
    int64_t timestamp;
};

// SBE body encoder for MassCancelReport (fields only, no header).
struct MassCancelReportBodyEncoder {
    static void encode(const MassCancelReport& value, std::vector<uint8_t>& buf);
};

inline void MassCancelReportBodyEncoder::encode(const MassCancelReport& value, std::vector<uint8_t>& buf) {
        write_string(buf, value.cl_ord_id);
        write_string(buf, value.account);
        if (value.symbol.has_value()) { buf.push_back(1); write_string(buf, *value.symbol); } else { buf.push_back(0); }
        write_u32_be(buf, value.total_affected);
        buf.push_back(MassCancelReportStatusToValue(value.status));
        write_u64_be(buf, static_cast<uint64_t>(value.timestamp));
}

// SBE body decoder for MassCancelReport: reads fields at pos.
struct MassCancelReportBodyDecoder {
    static std::pair<MassCancelReport, std::size_t> decode(const std::vector<uint8_t>& buf, std::size_t pos);
};

inline std::pair<MassCancelReport, std::size_t> MassCancelReportBodyDecoder::decode(const std::vector<uint8_t>& buf, std::size_t pos) {
    MassCancelReport value{};
        std::optional<std::string> symbol;
        std::string cl_ord_id = read_string(buf, pos);
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        uint32_t total_affected = read_u32_be(buf, pos);
        auto status_v = MassCancelReportStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid MassCancelReportStatus");
        MassCancelReportStatus status = *status_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.cl_ord_id = cl_ord_id;
    value.account = account;
    value.symbol = symbol;
    value.total_affected = total_affected;
    value.status = status;
    value.timestamp = timestamp;
    return {value, pos};
}

// SBE encoder for MassCancelReport
struct MassCancelReportEncoder {
    static std::vector<uint8_t> encode(std::string cl_ord_id, std::string account, std::optional<std::string> symbol, uint32_t total_affected, MassCancelReportStatus status, int64_t timestamp);
};

inline std::vector<uint8_t> MassCancelReportEncoder::encode(std::string cl_ord_id, std::string account, std::optional<std::string> symbol, uint32_t total_affected, MassCancelReportStatus status, int64_t timestamp) {
        std::vector<uint8_t> buf;
        write_u16_be(buf, SCHEMA_ID);
        write_u16_be(buf, 60);
        write_u16_be(buf, 0);
        write_u16_be(buf, 13);
        write_string(buf, cl_ord_id);
        write_string(buf, account);
        if (symbol.has_value()) { buf.push_back(1); write_string(buf, *symbol); } else { buf.push_back(0); }
        write_u32_be(buf, total_affected);
        buf.push_back(MassCancelReportStatusToValue(status));
        write_u64_be(buf, static_cast<uint64_t>(timestamp));
        return buf;
}

// SBE decoder for MassCancelReport: validates the header and reads the body.
struct MassCancelReportDecoder {
    static MassCancelReport decode(const std::vector<uint8_t>& buf);
};

inline MassCancelReport MassCancelReportDecoder::decode(const std::vector<uint8_t>& buf) {
    MassCancelReport value{};
        if (buf.size() < 8) throw std::runtime_error("buffer too short for SBE header");
        std::size_t pos = 0;
        uint16_t schema_id = read_u16_be(buf, pos);
        uint16_t tmpl_id = read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        (void)read_u16_be(buf, pos);
        if (schema_id != SCHEMA_ID) throw std::runtime_error("invalid schema_id");
        if (tmpl_id != 60) throw std::runtime_error("invalid template_id");
        std::optional<std::string> symbol;
        std::string cl_ord_id = read_string(buf, pos);
        std::string account = read_string(buf, pos);
        if (take_u8(buf, pos) == 1) symbol = read_string(buf, pos);
        uint32_t total_affected = read_u32_be(buf, pos);
        auto status_v = MassCancelReportStatusFromValue(take_u8(buf, pos));
        if (!status_v) throw std::runtime_error("invalid MassCancelReportStatus");
        MassCancelReportStatus status = *status_v;
        int64_t timestamp = static_cast<int64_t>(read_u64_be(buf, pos));
    value.cl_ord_id = cl_ord_id;
    value.account = account;
    value.symbol = symbol;
    value.total_affected = total_affected;
    value.status = status;
    value.timestamp = timestamp;
    (void)value;
    return value;
}

}
