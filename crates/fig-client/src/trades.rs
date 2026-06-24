//! Ring buffer of public trade prints.

use std::collections::VecDeque;

use fig_core::messages::{PublicTrade, PublicTradeEvent};

const DEFAULT_CAPACITY: usize = 256;

#[derive(Debug, Clone)]
pub struct TradeTape {
    capacity: usize,
    trades: VecDeque<PublicTrade>,
}

impl Default for TradeTape {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl TradeTape {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            trades: VecDeque::new(),
        }
    }

    pub fn push_event(&mut self, event: &PublicTradeEvent) {
        self.push_trade(&event.trade);
    }

    pub fn push_trade(&mut self, trade: &PublicTrade) {
        if self.trades.len() >= self.capacity {
            self.trades.pop_front();
        }
        self.trades.push_back(trade.clone());
    }

    pub fn trades(&self) -> &VecDeque<PublicTrade> {
        &self.trades
    }

    pub fn latest(&self) -> Option<&PublicTrade> {
        self.trades.back()
    }

    pub fn len(&self) -> usize {
        self.trades.len()
    }

    pub fn is_empty(&self) -> bool {
        self.trades.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::{Price, Quantity, Side};

    fn sample_trade(id: &str, price: f64) -> PublicTrade {
        PublicTrade {
            symbol: "AAPL".into(),
            trade_id: id.into(),
            price: Price(price),
            qty: Quantity(1.0),
            side: Side::Buy,
            timestamp: 0,
        }
    }

    #[test]
    fn ring_evicts_oldest() {
        let mut tape = TradeTape::with_capacity(2);
        tape.push_trade(&sample_trade("1", 100.0));
        tape.push_trade(&sample_trade("2", 101.0));
        tape.push_trade(&sample_trade("3", 102.0));
        assert_eq!(tape.len(), 2);
        assert_eq!(tape.trades().front().unwrap().trade_id, "2");
        assert_eq!(tape.latest().unwrap().price.0, 102.0);
    }
}
