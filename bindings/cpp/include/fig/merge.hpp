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
#include <vector>

namespace fig {

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

} // namespace fig
