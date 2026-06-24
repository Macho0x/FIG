//! Funding payment stream merge.

use std::collections::VecDeque;

use fig_core::messages::FundingPayment;

const DEFAULT_CAPACITY: usize = 128;

#[derive(Debug, Clone)]
pub struct FundingState {
    pub account: String,
    capacity: usize,
    payments: VecDeque<FundingPayment>,
}

impl Default for FundingState {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl FundingState {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            account: String::new(),
            capacity: capacity.max(1),
            payments: VecDeque::new(),
        }
    }

    pub fn apply_payment(&mut self, payment: &FundingPayment) {
        self.account = payment.account.clone();
        if self.payments.len() >= self.capacity {
            self.payments.pop_front();
        }
        self.payments.push_back(payment.clone());
    }

    pub fn payments(&self) -> &VecDeque<FundingPayment> {
        &self.payments
    }

    pub fn latest(&self) -> Option<&FundingPayment> {
        self.payments.back()
    }

    pub fn len(&self) -> usize {
        self.payments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.payments.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_latest() {
        let mut state = FundingState::default();
        let p = FundingPayment {
            account: "A".into(),
            symbol: Some("BTC".into()),
            amount: -1.5,
            rate: 0.0001,
            timestamp: 1,
        };
        state.apply_payment(&p);
        assert_eq!(state.latest().unwrap().amount, -1.5);
    }
}
