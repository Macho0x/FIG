//! Cross-symbol mid / last-price cache (Hyperliquid `allMids` style).

use std::collections::HashMap;

use fig_core::messages::{AllMidsBatch, MiniTicker};

#[derive(Debug, Clone, Default)]
pub struct MidsState {
    /// Symbol → last traded price (from `MiniTicker.last_price`).
    mids: HashMap<String, f64>,
}

impl MidsState {
    pub fn apply_ticker(&mut self, ticker: &MiniTicker) {
        self.mids.insert(ticker.symbol.clone(), ticker.last_price.0);
    }

    pub fn apply_batch(&mut self, batch: &AllMidsBatch) {
        for ticker in &batch.tickers {
            self.apply_ticker(ticker);
        }
    }

    /// Last price for `symbol` (HL-style dashboard quote).
    pub fn mid(&self, symbol: &str) -> Option<f64> {
        self.mids.get(symbol).copied()
    }

    pub fn symbols(&self) -> impl Iterator<Item = &str> {
        self.mids.keys().map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.mids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.mids.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::{Price, Quantity};

    #[test]
    fn batch_and_lookup() {
        let mut state = MidsState::default();
        state.apply_batch(&AllMidsBatch {
            tickers: vec![
                MiniTicker {
                    symbol: "BTC".into(),
                    last_price: Price(42_000.0),
                    volume: Quantity(1.0),
                    timestamp: 0,
                    is_snapshot: Some(true),
                },
                MiniTicker {
                    symbol: "ETH".into(),
                    last_price: Price(2_200.0),
                    volume: Quantity(1.0),
                    timestamp: 0,
                    is_snapshot: None,
                },
            ],
        });
        assert_eq!(state.mid("BTC"), Some(42_000.0));
        assert_eq!(state.mid("ETH"), Some(2_200.0));
        assert_eq!(state.mid("SOL"), None);
    }
}
