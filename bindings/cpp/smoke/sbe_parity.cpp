// Per-language SBE parity: encode with the *generated* C++ code and compare the
// bytes with the shared conformance vectors (produced by the Rust/FFI encoder).
#include "fig/sbe_generated.hpp"

#include <cstdio>
#include <string>
#include <vector>

static std::string hex(const std::vector<uint8_t>& bytes) {
    static const char* digits = "0123456789abcdef";
    std::string out;
    out.reserve(bytes.size() * 2);
    for (uint8_t b : bytes) {
        out.push_back(digits[b >> 4]);
        out.push_back(digits[b & 0x0F]);
    }
    return out;
}

static int check(const char* label, const std::vector<uint8_t>& got, const std::string& want) {
    if (hex(got) == want) {
        std::printf("fig-cpp sbe parity %s OK\n", label);
        return 0;
    }
    std::printf("fig-cpp sbe parity %s FAILED\n  got  %s\n  want %s\n", label,
                hex(got).c_str(), want.c_str());
    return 1;
}

int main() {
    int bad = 0;

    // sbe.new_order_single.limit_buy
    bad += check(
        "new_order_single",
        fig::sbe::NewOrderSingleEncoder::encode(
            "CONF-001", fig::sbe::NewOrderSingleSide::Buy, 100.0, 50.25, std::nullopt, "AAPL",
            fig::sbe::NewOrderSingleOrderType::Limit,
            fig::sbe::NewOrderSingleTimeInForce::Day, std::nullopt, std::nullopt, std::nullopt,
            std::nullopt, std::nullopt, std::nullopt, std::nullopt, std::nullopt),
        "00010001000000260008434f4e462d30303101405900000000000040492000000000000100000000000000000000044141504c020100000000000000000000000000000000000000");

    // sbe.order_book_snapshot.demo — exercises lists of inline structs and optionals
    std::vector<fig::sbe::PriceLevel> bids{{100.0, 5.0, uint32_t{1}}};
    std::vector<fig::sbe::PriceLevel> asks{};
    bad += check(
        "order_book_snapshot",
        fig::sbe::OrderBookSnapshotEncoder::encode(
            "AAPL", "SIM", bids, asks, 1700000000000000000LL, uint64_t{42}, uint8_t{1}),
        "000100090000001100044141504c000353494d00000001000000154059000000000000401400000000000000000001010000000017979cfe362a0000000000000000002a010101");

    // Round trip through the generated decoder
    auto raw = fig::sbe::NewOrderSingleEncoder::encode(
        "RT-001", fig::sbe::NewOrderSingleSide::Buy, 7.0, 12.5, std::nullopt, "MSFT",
        fig::sbe::NewOrderSingleOrderType::Limit, fig::sbe::NewOrderSingleTimeInForce::Day,
        std::nullopt, std::nullopt, std::nullopt, std::nullopt, std::nullopt, std::nullopt,
        std::nullopt, std::nullopt);
    auto value = fig::sbe::NewOrderSingleDecoder::decode(raw);
    if (value.cl_ord_id != "RT-001" || value.symbol != "MSFT" ||
        value.order_qty != 7.0 || !value.price || *value.price != 12.5) {
        std::printf("fig-cpp sbe parity round_trip FAILED\n");
        bad += 1;
    } else {
        std::printf("fig-cpp sbe parity round_trip OK\n");
    }

    return bad == 0 ? 0 : 1;
}
