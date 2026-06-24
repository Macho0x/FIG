//! Per-symbol mark / index / funding cache for perp UIs.

use std::collections::HashMap;

use fig_core::messages::MarkPriceUpdate;

#[derive(Debug, Clone, Default)]
pub struct MarkPriceState {
    marks: HashMap<String, MarkPriceUpdate>,
}

impl MarkPriceState {
    pub fn apply(&mut self, update: &MarkPriceUpdate) {
        self.marks.insert(update.symbol.clone(), update.clone());
    }

    pub fn mark(&self, symbol: &str) -> Option<f64> {
        self.marks.get(symbol).map(|m| m.mark_price.0)
    }

    pub fn index(&self, symbol: &str) -> Option<f64> {
        self.marks
            .get(symbol)
            .and_then(|m| m.index_price.as_ref().map(|p| p.0))
    }

    pub fn funding_rate(&self, symbol: &str) -> Option<f64> {
        self.marks.get(symbol).and_then(|m| m.funding_rate)
    }

    pub fn get(&self, symbol: &str) -> Option<&MarkPriceUpdate> {
        self.marks.get(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::Price;

    #[test]
    fn stores_mark_and_funding() {
        let mut state = MarkPriceState::default();
        state.apply(&MarkPriceUpdate {
            symbol: "BTC-PERP".into(),
            mark_price: Price(42_100.0),
            index_price: Some(Price(42_050.0)),
            funding_rate: Some(0.0001),
            timestamp: 0,
            is_snapshot: Some(true),
        });
        assert_eq!(state.mark("BTC-PERP"), Some(42_100.0));
        assert_eq!(state.index("BTC-PERP"), Some(42_050.0));
        assert_eq!(state.funding_rate("BTC-PERP"), Some(0.0001));
    }
}
