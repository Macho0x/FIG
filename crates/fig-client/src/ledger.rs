//! Ledger update stream merge (deposits, withdrawals, fees, transfers).

use std::collections::VecDeque;

use fig_core::messages::LedgerUpdate;

const DEFAULT_CAPACITY: usize = 256;

#[derive(Debug, Clone)]
pub struct LedgerState {
    pub account: String,
    capacity: usize,
    entries: VecDeque<LedgerUpdate>,
}

impl Default for LedgerState {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl LedgerState {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            account: String::new(),
            capacity: capacity.max(1),
            entries: VecDeque::new(),
        }
    }

    pub fn apply_update(&mut self, update: &LedgerUpdate) {
        self.account = update.account.clone();
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(update.clone());
    }

    pub fn entries(&self) -> &VecDeque<LedgerUpdate> {
        &self.entries
    }

    pub fn latest(&self) -> Option<&LedgerUpdate> {
        self.entries.back()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
