//! Aggregate trade tape merge from snapshot + live stream items.

use std::collections::VecDeque;

use fig_core::messages::{AggregateTrade, AggregateTradeEvent};

const DEFAULT_CAPACITY: usize = 256;

#[derive(Debug, Clone)]
pub struct AggTradeState {
    capacity: usize,
    trades: VecDeque<AggregateTrade>,
}

impl Default for AggTradeState {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl AggTradeState {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            trades: VecDeque::new(),
        }
    }

    pub fn push_event(&mut self, event: &AggregateTradeEvent) {
        self.push_trade(&event.trade);
    }

    pub fn push_trade(&mut self, trade: &AggregateTrade) {
        if self.trades.len() >= self.capacity {
            self.trades.pop_front();
        }
        self.trades.push_back(trade.clone());
    }

    pub fn trades(&self) -> &VecDeque<AggregateTrade> {
        &self.trades
    }

    pub fn latest(&self) -> Option<&AggregateTrade> {
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

    fn sample(id: &str, price: f64) -> AggregateTrade {
        AggregateTrade {
            symbol: "BTC".into(),
            agg_trade_id: id.into(),
            price: Price(price),
            qty: Quantity(1.0),
            side: Side::Buy,
            timestamp: 0,
            first_trade_id: id.into(),
            last_trade_id: id.into(),
        }
    }

    #[test]
    fn ring_evicts_oldest() {
        let mut state = AggTradeState::with_capacity(2);
        state.push_trade(&sample("1", 100.0));
        state.push_trade(&sample("2", 101.0));
        state.push_trade(&sample("3", 102.0));
        assert_eq!(state.len(), 2);
        assert_eq!(state.trades().front().unwrap().agg_trade_id, "2");
    }
}
