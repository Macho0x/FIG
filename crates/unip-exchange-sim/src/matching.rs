//! Matching engine for the UNIP exchange simulator.
//!
//! Processes incoming orders against the order book, producing fills
//! and execution reports. Supports market and limit orders with
//! price-time priority.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use unip_core::messages::*;
use super::orderbook::{OrderBook, RestingOrder};

/// A fill resulting from matching an order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fill {
    pub fill_id: String,
    pub cl_ord_id: ClientOrderId,
    pub contra_cl_ord_id: ClientOrderId,
    pub side: Side,
    pub symbol: Symbol,
    pub fill_price: Price,
    pub fill_qty: Quantity,
    pub leaves_qty: Quantity,
    pub cum_qty: Quantity,
    pub avg_price: Price,
    pub exec_type: ExecType,
    pub ord_status: OrdStatus,
}

/// Result of processing an order.
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub fills: Vec<Fill>,
    pub resting_order: Option<RestingOrder>,
    pub reject_reason: Option<String>,
}

/// The matching engine, holding one order book per symbol.
pub struct MatchingEngine {
    books: HashMap<Symbol, OrderBook>,
    order_index: HashMap<ClientOrderId, Symbol>,
    fill_seq: u64,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            order_index: HashMap::new(),
            fill_seq: 0,
        }
    }

    fn next_fill_id(&mut self) -> String {
        let id = self.fill_seq;
        self.fill_seq += 1;
        format!("FILL-{}", id)
    }

    /// Get or create the order book for a symbol.
    fn book_for(&mut self, symbol: &Symbol) -> &mut OrderBook {
        self.books
            .entry(symbol.clone())
            .or_insert_with(|| OrderBook::new(symbol.clone()))
    }

    /// Process a new order. Returns fills and any resting order.
    pub fn process_new_order(&mut self, order: &NewOrderSingle) -> MatchResult {
        let mut fills = Vec::new();
        let mut remaining_qty = order.order_qty.0;
        let mut cum_qty = 0.0;
        let mut total_value = 0.0;

        // Determine which side to match against
        let is_buy = matches!(order.side, Side::Buy | Side::SellShortExempt);

        // Collect matching info first (immutable borrow)
        let matching_info: Vec<(Price, Vec<(ClientOrderId, f64)>)> = {
            let book = self.book_for(&order.symbol);
            let opposite_side = if is_buy { &book.asks } else { &book.bids };

            let mut info = Vec::new();
            let mut rem = remaining_qty;

            for level in opposite_side.levels() {
                if rem <= 0.0 {
                    break;
                }

                // For limit orders, check if the price crosses
                if order.order_type == OrderType::Limit {
                    if is_buy && level.price.0 > order.price.as_ref().map_or(f64::MAX, |p| p.0) {
                        break;
                    }
                    if !is_buy && level.price.0 < order.price.as_ref().map_or(0.0, |p| p.0) {
                        break;
                    }
                }

                let mut level_matches = Vec::new();
                for resting in &level.orders {
                    if rem <= 0.0 {
                        break;
                    }
                    let fill_qty = rem.min(resting.leaves_qty.0);
                    level_matches.push((resting.cl_ord_id.clone(), fill_qty));
                    rem -= fill_qty;
                }

                if !level_matches.is_empty() {
                    info.push((level.price.clone(), level_matches));
                }
            }
            info
        };

        // Apply fills (mutable borrow)
        for (price, level_matches) in &matching_info {
            for (cl_ord_id, fill_qty) in level_matches {
                let book = self.book_for(&order.symbol);
                let opposite_side = if is_buy { &mut book.asks } else { &mut book.bids };
                opposite_side.reduce(cl_ord_id, *fill_qty);

                remaining_qty -= fill_qty;
                cum_qty += fill_qty;
                total_value += fill_qty * price.0;

                let fill_id = self.next_fill_id();
                fills.push(Fill {
                    fill_id: fill_id,
                    cl_ord_id: order.cl_ord_id.clone(),
                    contra_cl_ord_id: cl_ord_id.clone(),
                    side: order.side.clone(),
                    symbol: order.symbol.clone(),
                    fill_price: price.clone(),
                    fill_qty: Quantity(*fill_qty),
                    leaves_qty: Quantity(remaining_qty),
                    cum_qty: Quantity(cum_qty),
                    avg_price: Price(if cum_qty > 0.0 { total_value / cum_qty } else { 0.0 }),
                    exec_type: if remaining_qty <= 0.0 {
                        ExecType::Fill
                    } else {
                        ExecType::PartialFill
                    },
                    ord_status: if remaining_qty <= 0.0 {
                        OrdStatus::Filled
                    } else {
                        OrdStatus::PartiallyFilled
                    },
                });
            }
        }

        // If there's remaining qty and it's a limit order, rest it on the book
        let resting_order = if remaining_qty > 0.0 && order.order_type == OrderType::Limit {
            let mut resting = RestingOrder {
                order_id: format!("OX-{}", order.cl_ord_id),
                cl_ord_id: order.cl_ord_id.clone(),
                side: order.side.clone(),
                symbol: order.symbol.clone(),
                price: order.price.clone().unwrap_or(Price(0.0)),
                qty: order.order_qty.clone(),
                leaves_qty: Quantity(remaining_qty),
                time_in_force: order.time_in_force.clone(),
                account: order.account.clone(),
                seq: 0,
            };
            self.order_index.insert(order.cl_ord_id.clone(), order.symbol.clone());
            let book = self.book_for(&order.symbol);
            book.add_order(resting);
            // Return a clone for the caller to inspect
            resting = RestingOrder {
                order_id: format!("OX-{}", order.cl_ord_id),
                cl_ord_id: order.cl_ord_id.clone(),
                side: order.side.clone(),
                symbol: order.symbol.clone(),
                price: order.price.clone().unwrap_or(Price(0.0)),
                qty: order.order_qty.clone(),
                leaves_qty: Quantity(remaining_qty),
                time_in_force: order.time_in_force.clone(),
                account: order.account.clone(),
                seq: 0,
            };
            Some(resting)
        } else if remaining_qty > 0.0 && order.order_type == OrderType::Market {
            None
        } else {
            None
        };

        let reject_reason = if fills.is_empty() && resting_order.is_none() {
            Some("No liquidity".to_string())
        } else {
            None
        };

        MatchResult {
            fills,
            resting_order,
            reject_reason,
        }
    }

    /// Process a cancel request. Returns the cancelled order or None.
    pub fn process_cancel(&mut self, cancel: &CancelRequest) -> Option<RestingOrder> {
        let symbol = self.order_index.get(&cancel.orig_cl_ord_id)?;
        let book = self.books.get_mut(symbol)?;
        book.cancel_order(&cancel.orig_cl_ord_id)
    }

    /// Process a cancel/replace request. Cancels the old order and adds a new one.
    pub fn process_replace(&mut self, replace: &CancelReplaceRequest) -> MatchResult {
        // First cancel the old order
        let old_order = self.process_cancel(&CancelRequest {
            cl_ord_id: replace.cl_ord_id.clone(),
            orig_cl_ord_id: replace.orig_cl_ord_id.clone(),
            symbol: replace.symbol.clone(),
            side: replace.side.clone(),
            order_qty: None,
        });

        // Then submit the new order
        let new_order = NewOrderSingle {
            cl_ord_id: replace.cl_ord_id.clone(),
            side: replace.side.clone(),
            order_qty: replace.order_qty.clone(),
            price: replace.price.clone(),
            symbol: replace.symbol.clone(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: old_order.as_ref().and_then(|o| o.account.clone()),
            strategy_id: None,
        };

        self.process_new_order(&new_order)
    }

    /// Get the order book for a symbol (read-only).
    pub fn get_book(&self, symbol: &Symbol) -> Option<&OrderBook> {
        self.books.get(symbol)
    }

    /// Get or create the order book for a symbol (mutable).
    pub fn get_book_mut(&mut self, symbol: &Symbol) -> &mut OrderBook {
        self.book_for(symbol)
    }

    /// List all symbols with active books.
    pub fn symbols(&self) -> Vec<Symbol> {
        self.books.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_limit_order(id: &str, side: Side, symbol: &str, price: f64, qty: f64) -> NewOrderSingle {
        NewOrderSingle {
            cl_ord_id: id.to_string(),
            side,
            order_qty: Quantity(qty),
            price: Some(Price(price)),
            symbol: symbol.to_string(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: None,
            strategy_id: None,
        }
    }

    fn make_market_order(id: &str, side: Side, symbol: &str, qty: f64) -> NewOrderSingle {
        NewOrderSingle {
            cl_ord_id: id.to_string(),
            side,
            order_qty: Quantity(qty),
            price: None,
            symbol: symbol.to_string(),
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: None,
            strategy_id: None,
        }
    }

    #[test]
    fn test_limit_order_rests_when_no_counterparty() {
        let mut engine = MatchingEngine::new();
        let order = make_limit_order("B1", Side::Buy, "AAPL", 100.00, 50.0);
        let result = engine.process_new_order(&order);

        assert!(result.fills.is_empty());
        assert!(result.resting_order.is_some());
        assert!(result.reject_reason.is_none());
    }

    #[test]
    fn test_matching_fill() {
        let mut engine = MatchingEngine::new();

        // Add a sell order
        let sell = make_limit_order("S1", Side::Sell, "AAPL", 100.00, 50.0);
        engine.process_new_order(&sell);

        // Add a buy order that crosses
        let buy = make_limit_order("B1", Side::Buy, "AAPL", 101.00, 30.0);
        let result = engine.process_new_order(&buy);

        assert_eq!(result.fills.len(), 1);
        assert_eq!(result.fills[0].fill_price.0, 100.00); // Fill at resting order's price
        assert_eq!(result.fills[0].fill_qty.0, 30.0);
    }

    #[test]
    fn test_full_fill_removes_resting() {
        let mut engine = MatchingEngine::new();

        // Add a sell order for 50
        let sell = make_limit_order("S1", Side::Sell, "AAPL", 100.00, 50.0);
        engine.process_new_order(&sell);

        // Buy exactly 50 — should fully fill
        let buy = make_limit_order("B1", Side::Buy, "AAPL", 101.00, 50.0);
        let result = engine.process_new_order(&buy);

        assert_eq!(result.fills.len(), 1);
        assert!(result.resting_order.is_none()); // Fully filled, no remainder

        // The sell order should be removed from the book
        let book = engine.get_book(&"AAPL".to_string()).unwrap();
        assert_eq!(book.asks.order_count(), 0);
    }

    #[test]
    fn test_partial_fill_leaves_remainder() {
        let mut engine = MatchingEngine::new();

        // Add a sell order for 50
        let sell = make_limit_order("S1", Side::Sell, "AAPL", 100.00, 50.0);
        engine.process_new_order(&sell);

        // Buy 30 — partial fill
        let buy = make_limit_order("B1", Side::Buy, "AAPL", 101.00, 30.0);
        let result = engine.process_new_order(&buy);

        assert_eq!(result.fills.len(), 1);
        assert!(result.resting_order.is_none()); // Buy fully filled

        // Sell order should have 20 remaining
        let book = engine.get_book(&"AAPL".to_string()).unwrap();
        assert_eq!(book.asks.order_count(), 1);
    }

    #[test]
    fn test_market_order_no_liquidity() {
        let mut engine = MatchingEngine::new();

        // Market buy with no sell orders
        let buy = make_market_order("B1", Side::Buy, "AAPL", 50.0);
        let result = engine.process_new_order(&buy);

        assert!(result.fills.is_empty());
        assert!(result.reject_reason.is_some());
    }

    #[test]
    fn test_cancel_order() {
        let mut engine = MatchingEngine::new();

        let sell = make_limit_order("S1", Side::Sell, "AAPL", 100.00, 50.0);
        engine.process_new_order(&sell);

        let cancel = CancelRequest {
            cl_ord_id: "C1".to_string(),
            orig_cl_ord_id: "S1".to_string(),
            symbol: "AAPL".to_string(),
            side: Side::Sell,
            order_qty: None,
        };

        let cancelled = engine.process_cancel(&cancel);
        assert!(cancelled.is_some());
        assert_eq!(cancelled.unwrap().cl_ord_id, "S1");

        let book = engine.get_book(&"AAPL".to_string()).unwrap();
        assert_eq!(book.asks.order_count(), 0);
    }
}