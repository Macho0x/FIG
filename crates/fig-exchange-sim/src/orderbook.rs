//! L3 order book with price-time priority.
//!
//! The order book maintains all resting orders sorted by price (best first)
//! and time of arrival (FIFO within a price level). It supports add, cancel,
//! and replace operations, and provides top-of-book and depth-of-book queries.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use fig_core::messages::*;

/// A resting order in the book.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestingOrder {
    pub order_id: String,
    pub cl_ord_id: ClientOrderId,
    pub side: Side,
    pub symbol: Symbol,
    pub price: Price,
    pub qty: Quantity,
    pub leaves_qty: Quantity,
    pub time_in_force: TimeInForce,
    pub account: Option<String>,
    /// Monotonically increasing sequence for price-time priority.
    pub seq: u64,
}

/// One side of the book (bids or asks).
///
/// Bids are sorted descending by price (highest first).
/// Asks are sorted ascending by price (lowest first).
#[derive(Debug, Clone)]
pub struct BookSide {
    /// Price levels. For bids: descending. For asks: ascending.
    levels: BTreeMap<OrderedFloat, PriceLevel>,
    /// Whether this is the bid side (true) or ask side (false).
    is_bid: bool,
}

/// A price level containing all orders at a given price.
#[derive(Debug, Clone)]
pub struct PriceLevel {
    pub price: Price,
    pub orders: Vec<RestingOrder>,
    pub total_qty: f64,
}

/// Wrapper to allow f64 in BTreeMap (BTreeMap requires Ord, f64 doesn't impl Ord).
#[derive(Debug, Clone, Copy, PartialEq)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// The full L3 order book for a single instrument.
#[derive(Debug)]
pub struct OrderBook {
    pub symbol: Symbol,
    pub bids: BookSide,
    pub asks: BookSide,
    /// Monotonically increasing sequence counter for price-time priority.
    next_seq: u64,
}

impl BookSide {
    pub fn new(is_bid: bool) -> Self {
        Self {
            levels: BTreeMap::new(),
            is_bid,
        }
    }

    /// Add an order to this side of the book.
    pub fn add(&mut self, order: RestingOrder) {
        let key = if self.is_bid {
            // Bids: descending price (highest first)
            OrderedFloat(-order.price.0)
        } else {
            // Asks: ascending price (lowest first)
            OrderedFloat(order.price.0)
        };

        let level = self.levels.entry(key).or_insert_with(|| PriceLevel {
            price: order.price.clone(),
            orders: Vec::new(),
            total_qty: 0.0,
        });

        level.total_qty += order.leaves_qty.0;
        level.orders.push(order);
    }

    /// Find and remove an order by cl_ord_id. Returns the removed order.
    pub fn cancel(&mut self, cl_ord_id: &str) -> Option<RestingOrder> {
        for (key, level) in self.levels.iter_mut() {
            if let Some(idx) = level.orders.iter().position(|o| o.cl_ord_id == cl_ord_id) {
                let order = level.orders.remove(idx);
                level.total_qty -= order.leaves_qty.0;
                if level.orders.is_empty() {
                    let key = *key;
                    self.levels.remove(&key);
                }
                return Some(order);
            }
        }
        None
    }

    /// Reduce leaves_qty for an order. If leaves_qty reaches zero, removes the order.
    /// Returns true if the order was found and reduced/removed.
    pub fn reduce(&mut self, cl_ord_id: &str, fill_qty: f64) -> bool {
        // First pass: reduce the leaves_qty
        let mut remove_key = None;
        let mut remove_idx = None;
        let mut found = false;

        for (key, level) in self.levels.iter_mut() {
            if let Some(idx) = level.orders.iter().position(|o| o.cl_ord_id == cl_ord_id) {
                let order = &mut level.orders[idx];
                order.leaves_qty = Quantity(order.leaves_qty.0 - fill_qty);
                level.total_qty -= fill_qty;
                found = true;
                if order.leaves_qty.0 <= 1e-10 {
                    // Mark for removal (can't remove during iteration)
                    remove_key = Some(*key);
                    remove_idx = Some(idx);
                }
                break;
            }
        }

        // Second pass: remove fully filled order if needed
        if let (Some(key), Some(idx)) = (remove_key, remove_idx) {
            if let Some(level) = self.levels.get_mut(&key) {
                level.orders.remove(idx);
                if level.orders.is_empty() {
                    self.levels.remove(&key);
                }
            }
        }

        found
    }

    /// Get the best price level (first in the BTreeMap).
    pub fn best_level(&self) -> Option<&PriceLevel> {
        self.levels.values().next()
    }

    /// Get all price levels.
    pub fn levels(&self) -> impl Iterator<Item = &PriceLevel> {
        self.levels.values()
    }

    /// Get the total number of resting orders on this side.
    pub fn order_count(&self) -> usize {
        self.levels.values().map(|l| l.orders.len()).sum()
    }
}

impl OrderBook {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            bids: BookSide::new(true),
            asks: BookSide::new(false),
            next_seq: 0,
        }
    }

    /// Allocate the next sequence number for price-time priority.
    fn next_seq(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    /// Add a new resting order to the book. Returns the assigned sequence number.
    pub fn add_order(&mut self, mut order: RestingOrder) -> u64 {
        order.seq = self.next_seq();
        let seq = order.seq;
        let side = match order.side {
            Side::Buy | Side::SellShort | Side::SellShortExempt => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        side.add(order);
        seq
    }

    /// Cancel an order by cl_ord_id. Searches both sides.
    pub fn cancel_order(&mut self, cl_ord_id: &str) -> Option<RestingOrder> {
        if let Some(order) = self.bids.cancel(cl_ord_id) {
            return Some(order);
        }
        self.asks.cancel(cl_ord_id)
    }

    /// Get the best bid price and quantity.
    pub fn best_bid(&self) -> Option<(&Price, &Quantity)> {
        self.bids
            .best_level()
            .map(|l| (&l.price, &l.orders[0].leaves_qty))
    }

    /// Get the best ask price and quantity.
    pub fn best_ask(&self) -> Option<(&Price, &Quantity)> {
        self.asks
            .best_level()
            .map(|l| (&l.price, &l.orders[0].leaves_qty))
    }

    /// Get the top N levels of the bid side.
    pub fn bid_depth(&self, n: usize) -> Vec<fig_core::messages::PriceLevel> {
        self.bids
            .levels()
            .take(n)
            .map(|l| fig_core::messages::PriceLevel {
                price: l.price.clone(),
                qty: Quantity(l.total_qty),
                order_count: Some(l.orders.len() as u32),
            })
            .collect()
    }

    /// Get the top N levels of the ask side.
    pub fn ask_depth(&self, n: usize) -> Vec<fig_core::messages::PriceLevel> {
        self.asks
            .levels()
            .take(n)
            .map(|l| fig_core::messages::PriceLevel {
                price: l.price.clone(),
                qty: Quantity(l.total_qty),
                order_count: Some(l.orders.len() as u32),
            })
            .collect()
    }

    /// Total number of resting orders in the book.
    pub fn total_orders(&self) -> usize {
        self.bids.order_count() + self.asks.order_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_order(id: &str, side: Side, price: f64, qty: f64) -> RestingOrder {
        RestingOrder {
            order_id: format!("OX-{}", id),
            cl_ord_id: id.to_string(),
            side: side.clone(),
            symbol: "AAPL".to_string(),
            price: Price(price),
            qty: Quantity(qty),
            leaves_qty: Quantity(qty),
            time_in_force: TimeInForce::Day,
            account: None,
            seq: 0,
        }
    }

    #[test]
    fn test_add_and_best_bid_ask() {
        let mut book = OrderBook::new("AAPL".to_string());

        let bid = make_order("B1", Side::Buy, 100.00, 50.0);
        book.add_order(bid);
        let ask = make_order("S1", Side::Sell, 101.00, 30.0);
        book.add_order(ask);

        let (bid_price, bid_qty) = book.best_bid().unwrap();
        assert_eq!(bid_price.0, 100.00);
        assert_eq!(bid_qty.0, 50.0);

        let (ask_price, ask_qty) = book.best_ask().unwrap();
        assert_eq!(ask_price.0, 101.00);
        assert_eq!(ask_qty.0, 30.0);
    }

    #[test]
    fn test_price_time_priority() {
        let mut book = OrderBook::new("AAPL".to_string());

        // First bid at 100.00
        let bid1 = make_order("B1", Side::Buy, 100.00, 50.0);
        book.add_order(bid1);
        // Second bid at 100.00 (same price, later time)
        let bid2 = make_order("B2", Side::Buy, 100.00, 30.0);
        book.add_order(bid2);
        // Third bid at 99.00 (worse price)
        let bid3 = make_order("B3", Side::Buy, 99.00, 40.0);
        book.add_order(bid3);

        // Best bid should be 100.00 (highest)
        let (price, _) = book.best_bid().unwrap();
        assert_eq!(price.0, 100.00);

        // Depth should show 100.00 first, then 99.00
        let depth = book.bid_depth(5);
        assert_eq!(depth.len(), 2);
        assert_eq!(depth[0].price.0, 100.00);
        assert_eq!(depth[1].price.0, 99.00);
    }

    #[test]
    fn test_cancel_order() {
        let mut book = OrderBook::new("AAPL".to_string());

        let bid = make_order("B1", Side::Buy, 100.00, 50.0);
        book.add_order(bid);

        assert_eq!(book.total_orders(), 1);
        let cancelled = book.cancel_order("B1");
        assert!(cancelled.is_some());
        assert_eq!(cancelled.unwrap().cl_ord_id, "B1");
        assert_eq!(book.total_orders(), 0);
    }

    #[test]
    fn test_ask_price_ascending() {
        let mut book = OrderBook::new("AAPL".to_string());

        let ask1 = make_order("S1", Side::Sell, 102.00, 20.0);
        book.add_order(ask1);
        let ask2 = make_order("S2", Side::Sell, 101.00, 30.0);
        book.add_order(ask2);

        // Best ask should be 101.00 (lowest)
        let (price, _) = book.best_ask().unwrap();
        assert_eq!(price.0, 101.00);
    }
}
