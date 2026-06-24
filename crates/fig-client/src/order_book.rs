//! Local order book merge from snapshot + delta stream items.

use fig_core::messages::{
    MarketDataAction, MarketDataSnapshot, OrderBookDelta, OrderBookSnapshot, PriceLevel, Side,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BookGap {
    #[error("sequence gap: expected > {last}, got {got}")]
    Sequence { last: u64, got: u64 },
}

#[derive(Debug, Clone, Default)]
pub struct OrderBookState {
    pub symbol: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    last_sequence: Option<u64>,
}

impl OrderBookState {
    pub fn apply_snapshot(&mut self, snap: &OrderBookSnapshot) -> Result<(), BookGap> {
        if let (Some(last), Some(seq)) = (self.last_sequence, snap.sequence) {
            if seq < last {
                return Err(BookGap::Sequence { last, got: seq });
            }
        }
        self.symbol = snap.symbol.clone();
        self.bids = snap.bids.clone();
        self.asks = snap.asks.clone();
        if let Some(seq) = snap.sequence {
            self.last_sequence = Some(seq);
        }
        Ok(())
    }

    pub fn apply_delta(&mut self, delta: &OrderBookDelta) -> Result<(), BookGap> {
        if let (Some(last), Some(seq)) = (self.last_sequence, delta.sequence) {
            if seq <= last {
                return Err(BookGap::Sequence { last, got: seq });
            }
        }
        self.symbol = delta.symbol.clone();
        for update in &delta.updates {
            let book = match update.side {
                Side::Buy => &mut self.bids,
                Side::Sell | Side::SellShort | Side::SellShortExempt => &mut self.asks,
            };
            apply_level(book, update.price.0, update.qty.0, update.action);
        }
        if let Some(seq) = delta.sequence {
            self.last_sequence = Some(seq);
        }
        Ok(())
    }

    pub fn apply_market_data_snapshot(&mut self, snap: &MarketDataSnapshot) -> Result<(), BookGap> {
        let ob = OrderBookSnapshot {
            symbol: snap.symbol.clone(),
            exchange: snap.exchange.clone(),
            bids: snap.bids.clone(),
            asks: snap.asks.clone(),
            timestamp: snap.timestamp,
            sequence: snap.sequence,
            is_snapshot: Some(true),
        };
        self.apply_snapshot(&ob)
    }

    pub fn last_sequence(&self) -> Option<u64> {
        self.last_sequence
    }
}

fn apply_level(book: &mut Vec<PriceLevel>, price: f64, qty: f64, action: MarketDataAction) {
    if let Some(idx) = book.iter().position(|l| l.price.0 == price) {
        match action {
            MarketDataAction::Delete | MarketDataAction::Change if qty == 0.0 => {
                book.remove(idx);
            }
            MarketDataAction::New | MarketDataAction::Change => {
                book[idx].qty = fig_core::messages::Quantity(qty);
            }
            MarketDataAction::Delete => {
                book.remove(idx);
            }
        }
    } else if matches!(action, MarketDataAction::New | MarketDataAction::Change) && qty > 0.0 {
        book.push(PriceLevel {
            price: fig_core::messages::Price(price),
            qty: fig_core::messages::Quantity(qty),
            order_count: None,
        });
        if book.len() > 1 {
            let is_bid = book[0].price.0 >= book[book.len() - 1].price.0;
            book.sort_by(|a, b| {
                if is_bid {
                    b.price.0.partial_cmp(&a.price.0).unwrap()
                } else {
                    a.price.0.partial_cmp(&b.price.0).unwrap()
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::MarketDataUpdate;

    #[test]
    fn snapshot_then_delta() {
        let mut book = OrderBookState::default();
        book.apply_snapshot(&OrderBookSnapshot {
            symbol: "AAPL".into(),
            exchange: "SIM".into(),
            bids: vec![PriceLevel {
                price: fig_core::messages::Price(100.0),
                qty: fig_core::messages::Quantity(10.0),
                order_count: None,
            }],
            asks: vec![],
            timestamp: 0,
            sequence: Some(1),
            is_snapshot: Some(true),
        })
        .unwrap();

        book.apply_delta(&OrderBookDelta {
            symbol: "AAPL".into(),
            updates: vec![MarketDataUpdate {
                side: Side::Buy,
                action: MarketDataAction::Change,
                price: fig_core::messages::Price(100.0),
                qty: fig_core::messages::Quantity(5.0),
            }],
            timestamp: 1,
            sequence: Some(2),
        })
        .unwrap();

        assert_eq!(book.bids[0].qty.0, 5.0);
    }

    #[test]
    fn rejects_sequence_regression() {
        let mut book = OrderBookState::default();
        book.last_sequence = Some(5);
        let err = book
            .apply_delta(&OrderBookDelta {
                symbol: "AAPL".into(),
                updates: vec![],
                timestamp: 0,
                sequence: Some(3),
            })
            .unwrap_err();
        assert!(matches!(err, BookGap::Sequence { .. }));
    }
}
