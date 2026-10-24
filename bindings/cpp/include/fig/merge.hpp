#pragma once

#ifdef __cplusplus
extern "C" {
#endif
#include "../../../../crates/fig-ffi/include/fig.h"
#ifdef __cplusplus
}
#endif

#include <cstddef>
#include <memory>
#include <string>
#include <vector>

namespace fig {

template <typename Fn>
int apply_payload(const std::vector<uint8_t>& payload, Fn fn) {
    if (payload.empty()) {
        return -1;
    }
    return fn(payload.data(), payload.size());
}

class FundingState {
public:
    explicit FundingState(std::size_t capacity = 128) {
        handle_.reset(fig_funding_new(capacity));
    }

    int apply(const std::vector<uint8_t>& payload) {
        if (payload.empty()) {
            return -1;
        }
        return fig_funding_apply(handle_.get(), payload.data(), payload.size());
    }

    std::size_t len() const { return fig_funding_len(handle_.get()); }

    double latest_amount() const { return fig_funding_latest_amount(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigFundingHandle* h) const {
            if (h != nullptr) {
                fig_funding_free(h);
            }
        }
    };
    std::unique_ptr<FigFundingHandle, Deleter> handle_;
};

class AggTradesState {
public:
    explicit AggTradesState(std::size_t capacity = 256) {
        handle_.reset(fig_agg_trades_new(capacity));
    }

    int apply(const std::vector<uint8_t>& payload) {
        if (payload.empty()) {
            return -1;
        }
        return fig_agg_trades_apply(handle_.get(), payload.data(), payload.size());
    }

    double latest_price() const { return fig_agg_trades_latest_price(handle_.get()); }

    std::size_t len() const { return fig_agg_trades_len(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigAggTradesHandle* h) const {
            if (h != nullptr) {
                fig_agg_trades_free(h);
            }
        }
    };
    std::unique_ptr<FigAggTradesHandle, Deleter> handle_;
};

class LedgerState {
public:
    explicit LedgerState(std::size_t capacity = 128) {
        handle_.reset(fig_ledger_new(capacity));
    }

    int apply(const std::vector<uint8_t>& payload) {
        if (payload.empty()) {
            return -1;
        }
        return fig_ledger_apply(handle_.get(), payload.data(), payload.size());
    }

    std::size_t len() const { return fig_ledger_len(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigLedgerHandle* h) const {
            if (h != nullptr) {
                fig_ledger_free(h);
            }
        }
    };
    std::unique_ptr<FigLedgerHandle, Deleter> handle_;
};

class LiquidationState {
public:
    explicit LiquidationState(std::size_t capacity = 64) {
        handle_.reset(fig_liquidation_new(capacity));
    }

    int apply_user(const std::vector<uint8_t>& payload) {
        if (payload.empty()) {
            return -1;
        }
        return fig_liquidation_apply_user(handle_.get(), payload.data(), payload.size());
    }

    int apply_public(const std::vector<uint8_t>& payload) {
        if (payload.empty()) {
            return -1;
        }
        return fig_liquidation_apply_public(handle_.get(), payload.data(), payload.size());
    }

    std::size_t user_count() const { return fig_liquidation_user_count(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigLiquidationHandle* h) const {
            if (h != nullptr) {
                fig_liquidation_free(h);
            }
        }
    };
    std::unique_ptr<FigLiquidationHandle, Deleter> handle_;
};

class OrderBookState {
public:
    OrderBookState() { handle_.reset(fig_order_book_new()); }

    int apply_snapshot(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_order_book_apply_snapshot(handle_.get(), data, len);
        });
    }

    int apply_delta(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_order_book_apply_delta(handle_.get(), data, len);
        });
    }

    double best_bid() const { return fig_order_book_best_bid(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigOrderBookHandle* h) const {
            if (h != nullptr) {
                fig_order_book_free(h);
            }
        }
    };
    std::unique_ptr<FigOrderBookHandle, Deleter> handle_;
};

class MidsState {
public:
    MidsState() { handle_.reset(fig_mids_new()); }

    int apply_ticker(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_mids_apply_ticker(handle_.get(), data, len);
        });
    }

    int apply_batch(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_mids_apply_batch(handle_.get(), data, len);
        });
    }

    double mid(const char* symbol) const { return fig_mids_mid(handle_.get(), symbol); }

private:
    struct Deleter {
        void operator()(FigMidsHandle* h) const {
            if (h != nullptr) {
                fig_mids_free(h);
            }
        }
    };
    std::unique_ptr<FigMidsHandle, Deleter> handle_;
};

class BboState {
public:
    BboState() { handle_.reset(fig_bbo_new()); }

    int apply(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_bbo_apply(handle_.get(), data, len);
        });
    }

    double implied_mid() const { return fig_bbo_implied_mid(handle_.get()); }
    double best_bid() const { return fig_bbo_best_bid(handle_.get()); }
    double best_ask() const { return fig_bbo_best_ask(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigBboHandle* h) const {
            if (h != nullptr) {
                fig_bbo_free(h);
            }
        }
    };
    std::unique_ptr<FigBboHandle, Deleter> handle_;
};

class TradeTape {
public:
    explicit TradeTape(std::size_t capacity = 256) {
        handle_.reset(fig_trade_tape_new(capacity));
    }

    int push(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_trade_tape_push(handle_.get(), data, len);
        });
    }

    std::size_t len() const { return fig_trade_tape_len(handle_.get()); }
    double latest_price() const { return fig_trade_tape_latest_price(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigTradeTapeHandle* h) const {
            if (h != nullptr) {
                fig_trade_tape_free(h);
            }
        }
    };
    std::unique_ptr<FigTradeTapeHandle, Deleter> handle_;
};

class MarkPriceState {
public:
    MarkPriceState() { handle_.reset(fig_mark_price_new()); }

    int apply(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_mark_price_apply(handle_.get(), data, len);
        });
    }

    double get(const char* symbol) const { return fig_mark_price_get(handle_.get(), symbol); }

private:
    struct Deleter {
        void operator()(FigMarkPriceHandle* h) const {
            if (h != nullptr) {
                fig_mark_price_free(h);
            }
        }
    };
    std::unique_ptr<FigMarkPriceHandle, Deleter> handle_;
};

class OrdersState {
public:
    explicit OrdersState(std::size_t exec_capacity = 256) {
        handle_.reset(fig_orders_new(exec_capacity));
    }

    int apply_snapshot(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_orders_apply_snapshot(handle_.get(), data, len);
        });
    }

    int apply_execution(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_orders_apply_execution(handle_.get(), data, len);
        });
    }

    std::size_t open_count() const { return fig_orders_open_count(handle_.get()); }
    std::size_t execution_count() const { return fig_orders_execution_count(handle_.get()); }

private:
    struct Deleter {
        void operator()(FigOrdersHandle* h) const {
            if (h != nullptr) {
                fig_orders_free(h);
            }
        }
    };
    std::unique_ptr<FigOrdersHandle, Deleter> handle_;
};

class CandleState {
public:
    CandleState() { handle_.reset(fig_candle_new()); }

    int apply(const std::vector<uint8_t>& payload) {
        return apply_payload(payload, [&](const uint8_t* data, std::size_t len) {
            return fig_candle_apply(handle_.get(), data, len);
        });
    }

    double close() const { return fig_candle_close(handle_.get()); }
    int is_final() const { return fig_candle_is_final(handle_.get()); }

    std::string symbol() const { return take_text(fig_candle_symbol); }
    std::string interval() const { return take_text(fig_candle_interval); }

private:
    std::string take_text(int (*fn)(const FigCandleHandle*, char**)) const {
        char* out = nullptr;
        if (fn(handle_.get(), &out) != 0 || out == nullptr) {
            return {};
        }
        std::string text(out);
        fig_string_free(out);
        return text;
    }

    struct Deleter {
        void operator()(FigCandleHandle* h) const {
            if (h != nullptr) {
                fig_candle_free(h);
            }
        }
    };
    std::unique_ptr<FigCandleHandle, Deleter> handle_;
};

} // namespace fig
