//! Top-of-book BBO with implied mid.

use fig_core::messages::BestBidOffer;

#[derive(Debug, Clone, Default)]
pub struct BboState {
    pub symbol: String,
    pub bbo: Option<BestBidOffer>,
}

impl BboState {
    pub fn apply(&mut self, bbo: &BestBidOffer) {
        self.symbol = bbo.symbol.clone();
        self.bbo = Some(bbo.clone());
    }

    pub fn best_bid(&self) -> Option<f64> {
        self.bbo.as_ref()?.bid_price.as_ref().map(|p| p.0)
    }

    pub fn best_ask(&self) -> Option<f64> {
        self.bbo.as_ref()?.ask_price.as_ref().map(|p| p.0)
    }

    /// Mid from bid/ask when both sides are present.
    pub fn implied_mid(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / 2.0),
            (Some(bid), None) => Some(bid),
            (None, Some(ask)) => Some(ask),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::{Price, Quantity};

    #[test]
    fn implied_mid_from_both_sides() {
        let mut state = BboState::default();
        state.apply(&BestBidOffer {
            symbol: "AAPL".into(),
            bid_price: Some(Price(100.0)),
            bid_qty: Some(Quantity(10.0)),
            ask_price: Some(Price(101.0)),
            ask_qty: Some(Quantity(5.0)),
            timestamp: 0,
            is_snapshot: Some(true),
        });
        assert_eq!(state.implied_mid(), Some(100.5));
    }
}
