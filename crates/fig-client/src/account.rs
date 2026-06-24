//! Private account stream merge (balances, positions).

use std::collections::HashMap;

use fig_core::messages::{
    BalanceSnapshot, BalanceUpdate, MarginSummary, MarginUpdate, PositionSnapshot, PositionUpdate,
};

#[derive(Debug, Clone, Default)]
pub struct AccountCache {
    pub account: String,
    pub balances: HashMap<String, f64>,
    pub positions: HashMap<String, f64>,
    pub margin: Option<MarginSummary>,
}

impl AccountCache {
    pub fn apply_balance_snapshot(&mut self, snap: &BalanceSnapshot) {
        self.account = snap.account.clone();
        self.balances.clear();
        for b in &snap.balances {
            self.balances.insert(b.asset.clone(), b.total);
        }
    }

    pub fn apply_balance_update(&mut self, update: &BalanceUpdate) {
        self.account = update.account.clone();
        self.balances.insert(update.asset.clone(), update.total);
    }

    pub fn apply_position_snapshot(&mut self, snap: &PositionSnapshot) {
        self.account = snap.account.clone();
        self.positions.clear();
        for p in &snap.positions {
            self.positions.insert(p.symbol.clone(), p.qty.0);
        }
    }

    pub fn apply_position_update(&mut self, update: &PositionUpdate) {
        self.account = update.account.clone();
        self.positions.insert(update.symbol.clone(), update.qty.0);
    }

    pub fn apply_margin_update(&mut self, update: &MarginUpdate) {
        self.account = update.account.clone();
        self.margin = Some(update.summary.clone());
    }
}
