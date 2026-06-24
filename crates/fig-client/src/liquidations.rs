//! User liquidation and public liquidation trade merge.

use std::collections::VecDeque;

use fig_core::messages::{LiquidationTrade, LiquidationTradeEvent, UserLiquidation};

const DEFAULT_CAPACITY: usize = 64;

#[derive(Debug, Clone)]
pub struct LiquidationState {
    pub account: String,
    user_liquidations: VecDeque<UserLiquidation>,
    public_trades: VecDeque<LiquidationTrade>,
    capacity: usize,
}

impl Default for LiquidationState {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl LiquidationState {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            account: String::new(),
            user_liquidations: VecDeque::new(),
            public_trades: VecDeque::new(),
            capacity: capacity.max(1),
        }
    }

    pub fn apply_user_liquidation(&mut self, liq: &UserLiquidation) {
        self.account = liq.account.clone();
        if self.user_liquidations.len() >= self.capacity {
            self.user_liquidations.pop_front();
        }
        self.user_liquidations.push_back(liq.clone());
    }

    pub fn apply_public_trade(&mut self, trade: &LiquidationTrade) {
        if self.public_trades.len() >= self.capacity {
            self.public_trades.pop_front();
        }
        self.public_trades.push_back(trade.clone());
    }

    pub fn apply_public_event(&mut self, event: &LiquidationTradeEvent) {
        self.apply_public_trade(&event.trade);
    }

    pub fn user_liquidations(&self) -> &VecDeque<UserLiquidation> {
        &self.user_liquidations
    }

    pub fn public_trades(&self) -> &VecDeque<LiquidationTrade> {
        &self.public_trades
    }

    pub fn latest_user(&self) -> Option<&UserLiquidation> {
        self.user_liquidations.back()
    }
}
