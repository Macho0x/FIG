//! Matching engine for the FIG exchange simulator.
//!
//! Processes incoming orders against the order book, producing fills
//! and execution reports. Supports market and limit orders with
//! price-time priority.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::orderbook::{OrderBook, RestingOrder};
use fig_core::messages::*;

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

/// Result of processing a cancel request.
#[derive(Debug, Clone, PartialEq)]
pub enum CancelOutcome {
    Cancelled(RestingOrder),
    Rejected(CancelRejectReason),
}

/// A stop order waiting for trigger.
#[derive(Debug, Clone)]
struct PendingStopOrder {
    order: NewOrderSingle,
}

/// Result of processing an order.
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub fills: Vec<Fill>,
    pub resting_order: Option<RestingOrder>,
    pub reject_reason: Option<String>,
}

/// Outcome of [`MatchingEngine::process_batch`].
#[derive(Debug, Clone)]
pub struct BatchApply {
    pub report: BatchReport,
    pub fills: Vec<Fill>,
    pub reports: Vec<ExecutionReport>,
    pub symbols: Vec<String>,
}

struct BatchStep {
    result: BatchResult,
    fills: Vec<Fill>,
    reports: Vec<ExecutionReport>,
    symbol: Option<String>,
}

fn action_identity(action: &BatchAction) -> (BatchResultKind, String) {
    let kind = match action.kind {
        BatchActionKind::NewOrder => BatchResultKind::NewOrder,
        BatchActionKind::Cancel => BatchResultKind::Cancel,
        BatchActionKind::Replace => BatchResultKind::Replace,
    };
    let cl_ord_id = match action.kind {
        BatchActionKind::NewOrder => action
            .new_order
            .as_ref()
            .map(|o| o.cl_ord_id.clone())
            .unwrap_or_default(),
        BatchActionKind::Cancel => action
            .cancel
            .as_ref()
            .map(|o| o.cl_ord_id.clone())
            .unwrap_or_default(),
        BatchActionKind::Replace => action
            .replace
            .as_ref()
            .map(|o| o.cl_ord_id.clone())
            .unwrap_or_default(),
    };
    (kind, cl_ord_id)
}

fn cancel_reason(reason: CancelRejectReason) -> String {
    match reason {
        CancelRejectReason::OrderNotFound => "ORDER_NOT_FOUND".into(),
        CancelRejectReason::AlreadyCanceled => "ALREADY_CANCELED".into(),
        CancelRejectReason::AlreadyFilled => "ALREADY_FILLED".into(),
        CancelRejectReason::TooLateToCancel => "TOO_LATE_TO_CANCEL".into(),
    }
}

/// `exec_id` is `string(max_len: 20)`. Keep a short id as written. A longer id
/// becomes a 17-character hash so two long client order ids do not collapse
/// onto the same truncated suffix.
fn fit_exec_id(preferred: String) -> String {
    const MAX: usize = 20;
    if preferred.len() <= MAX {
        return preferred;
    }
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in preferred.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("X{hash:016x}")
}

fn apply_one(
    engine: &mut MatchingEngine,
    action: &BatchAction,
    index: u32,
) -> Result<BatchStep, String> {
    let (kind, cl_ord_id) = action_identity(action);
    match action.kind {
        BatchActionKind::NewOrder => {
            let order = action
                .new_order
                .as_ref()
                .ok_or_else(|| "BATCH_ACTION_INVALID".to_string())?;
            let matched = engine.process_new_order(order);
            if let Some(reason) = matched.reject_reason {
                return Err(reason);
            }
            let mut reports = Vec::new();
            for fill in &matched.fills {
                reports.push(ExecutionReport {
                    cl_ord_id: order.cl_ord_id.clone(),
                    order_id: fill.fill_id.clone(),
                    exec_id: fit_exec_id(format!("EX-{}", fill.fill_id)),
                    exec_type: fill.exec_type,
                    ord_status: fill.ord_status,
                    side: fill.side,
                    last_qty: Some(fill.fill_qty.clone()),
                    last_price: Some(fill.fill_price.clone()),
                    leaves_qty: fill.leaves_qty.clone(),
                    cum_qty: fill.cum_qty.clone(),
                    avg_price: fill.avg_price.clone(),
                    symbol: fill.symbol.clone(),
                    transact_time: MatchingEngine::now_nanos(),
                });
            }
            if let Some(resting) = &matched.resting_order {
                reports.push(resting_to_report(resting));
            }
            Ok(BatchStep {
                result: BatchResult {
                    index,
                    kind,
                    cl_ord_id,
                    status: BatchResultStatus::Applied,
                    reject_reason: None,
                },
                fills: matched.fills,
                reports,
                symbol: Some(order.symbol.clone()),
            })
        }
        BatchActionKind::Cancel => {
            let cancel = action
                .cancel
                .as_ref()
                .ok_or_else(|| "BATCH_ACTION_INVALID".to_string())?;
            match engine.process_cancel(cancel) {
                CancelOutcome::Cancelled(order) => {
                    let mut report = resting_to_report(&order);
                    report.cl_ord_id = cancel.cl_ord_id.clone();
                    report.exec_type = ExecType::Canceled;
                    report.ord_status = OrdStatus::Canceled;
                    report.leaves_qty = Quantity(0.0);
                    Ok(BatchStep {
                        result: BatchResult {
                            index,
                            kind,
                            cl_ord_id,
                            status: BatchResultStatus::Applied,
                            reject_reason: None,
                        },
                        fills: Vec::new(),
                        reports: vec![report],
                        symbol: Some(cancel.symbol.clone()),
                    })
                }
                CancelOutcome::Rejected(reason) => Err(cancel_reason(reason)),
            }
        }
        BatchActionKind::Replace => {
            let replace = action
                .replace
                .as_ref()
                .ok_or_else(|| "BATCH_ACTION_INVALID".to_string())?;
            let matched = engine.process_replace(replace);
            if let Some(reason) = matched.reject_reason {
                return Err(reason);
            }
            let mut reports = Vec::new();
            for fill in &matched.fills {
                reports.push(ExecutionReport {
                    cl_ord_id: replace.cl_ord_id.clone(),
                    order_id: fill.fill_id.clone(),
                    exec_id: fit_exec_id(format!("EX-{}", fill.fill_id)),
                    exec_type: fill.exec_type,
                    ord_status: fill.ord_status,
                    side: fill.side,
                    last_qty: Some(fill.fill_qty.clone()),
                    last_price: Some(fill.fill_price.clone()),
                    leaves_qty: fill.leaves_qty.clone(),
                    cum_qty: fill.cum_qty.clone(),
                    avg_price: fill.avg_price.clone(),
                    symbol: fill.symbol.clone(),
                    transact_time: MatchingEngine::now_nanos(),
                });
            }
            if let Some(resting) = &matched.resting_order {
                reports.push(resting_to_report(resting));
            }
            Ok(BatchStep {
                result: BatchResult {
                    index,
                    kind,
                    cl_ord_id,
                    status: BatchResultStatus::Applied,
                    reject_reason: None,
                },
                fills: matched.fills,
                reports,
                symbol: Some(replace.symbol.clone()),
            })
        }
    }
}

/// The matching engine, holding one order book per symbol.
#[derive(Clone)]
pub struct MatchingEngine {
    books: HashMap<Symbol, OrderBook>,
    order_index: HashMap<ClientOrderId, Symbol>,
    pending_stops: HashMap<Symbol, Vec<PendingStopOrder>>,
    fill_seq: u64,
    order_history: Vec<(String, ExecutionReport)>,
}

impl Default for MatchingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Short sales take liquidity from the bid, the same way a long sell does.
fn is_aggressor_buy(side: Side) -> bool {
    matches!(side, Side::Buy)
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            order_index: HashMap::new(),
            pending_stops: HashMap::new(),
            fill_seq: 0,
            order_history: Vec::new(),
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

    fn now_nanos() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64
    }

    /// Direct struct construction bypasses the codec, so the matcher rejects
    /// non-finite or negative price and quantity itself.
    fn reject_bad_economics(
        &self,
        order: &NewOrderSingle,
        allow_stop_rest: bool,
    ) -> Option<String> {
        let qty = order.order_qty.0;
        let on_whole_share = (qty - qty.round()).abs() <= 1e-3;
        if !qty.is_finite() || qty < 0.0 || qty > 999_999_999.0 || !on_whole_share {
            return Some("INVALID_QUANTITY".to_string());
        }
        if let Some(px) = order.price.as_ref() {
            if !px.0.is_finite() || px.0 < 0.0 {
                return Some("INVALID_PRICE".to_string());
            }
        }
        if let Some(px) = order.stop_price.as_ref() {
            if !px.0.is_finite() || px.0 < 0.0 {
                return Some("INVALID_PRICE".to_string());
            }
        }
        // Triggered stops already own their index entry. A new order that
        // reuses a live id is a duplicate. Replace cancels first, so the
        // same id can be reused after the cancel removes it.
        if allow_stop_rest && self.order_index.contains_key(&order.cl_ord_id) {
            return Some("DUPLICATE_CL_ORD_ID".to_string());
        }
        None
    }

    fn validate_order(&self, order: &NewOrderSingle) -> Option<String> {
        if let Some(expire) = order.expire_time {
            if order.time_in_force == TimeInForce::Gtd && expire <= Self::now_nanos() {
                return Some("Order expired".to_string());
            }
        }
        match order.order_type {
            OrderType::Stop | OrderType::StopLimit if order.stop_price.is_none() => {
                Some("Stop orders require stop_price".to_string())
            }
            OrderType::Limit | OrderType::StopLimit if order.price.is_none() => {
                Some("Limit orders require price".to_string())
            }
            _ => None,
        }
    }

    fn would_cross(&mut self, order: &NewOrderSingle) -> bool {
        if !matches!(
            order.order_type,
            OrderType::Limit | OrderType::StopLimit | OrderType::LimitOnClose
        ) {
            return false;
        }
        let Some(limit_px) = order.price.as_ref().map(|p| p.0) else {
            return false;
        };
        let is_buy = is_aggressor_buy(order.side);
        let book = self.book_for(&order.symbol);
        if is_buy {
            book.asks.best_price().is_some_and(|ask| ask.0 <= limit_px)
        } else {
            book.bids.best_price().is_some_and(|bid| bid.0 >= limit_px)
        }
    }

    fn available_liquidity(&mut self, order: &NewOrderSingle) -> f64 {
        let order_type = Self::effective_match_type(&order.order_type);
        let is_buy = is_aggressor_buy(order.side);
        let book = self.book_for(&order.symbol);
        let opposite_side = if is_buy { &book.asks } else { &book.bids };
        let mut available = 0.0;
        for level in opposite_side.levels() {
            if order_type == OrderType::Limit || order_type == OrderType::StopLimit {
                if is_buy && level.price.0 > order.price.as_ref().map_or(f64::MAX, |p| p.0) {
                    break;
                }
                if !is_buy && level.price.0 < order.price.as_ref().map_or(0.0, |p| p.0) {
                    break;
                }
            }
            available += level.orders.iter().map(|o| o.leaves_qty.0).sum::<f64>();
        }
        available
    }

    fn trigger_stops(&mut self, symbol: &Symbol) -> Vec<MatchResult> {
        let mut results = Vec::new();
        let Some(stops) = self.pending_stops.get(symbol) else {
            return results;
        };
        if stops.is_empty() {
            return results;
        }

        let reference = self.reference_price(symbol);
        let Some(reference) = reference else {
            return results;
        };

        let pending = self.pending_stops.remove(symbol).unwrap_or_default();
        let mut remaining = Vec::new();

        for stop in pending {
            let stop_px = stop.order.stop_price.as_ref().map(|p| p.0).unwrap_or(0.0);
            let is_buy = is_aggressor_buy(stop.order.side);
            let triggered = if is_buy {
                reference >= stop_px
            } else {
                reference <= stop_px
            };

            if !triggered {
                remaining.push(stop);
                continue;
            }

            let mut triggered_order = stop.order.clone();
            triggered_order.order_type = match triggered_order.order_type {
                OrderType::Stop => OrderType::Market,
                OrderType::StopLimit => OrderType::Limit,
                other => other,
            };
            results.push(self.process_new_order_inner(&triggered_order, false));
        }

        if !remaining.is_empty() {
            self.pending_stops.insert(symbol.clone(), remaining);
        }
        results
    }

    fn reference_price(&self, symbol: &Symbol) -> Option<f64> {
        let book = self.books.get(symbol)?;
        book.asks
            .best_price()
            .map(|p| p.0)
            .or_else(|| book.bids.best_price().map(|p| p.0))
    }

    /// Process a new order. Returns fills and any resting order.
    pub fn process_new_order(&mut self, order: &NewOrderSingle) -> MatchResult {
        let mut result = self.process_new_order_inner(order, true);
        for triggered in self.trigger_stops(&order.symbol) {
            result.fills.extend(triggered.fills);
            if triggered.resting_order.is_some() {
                result.resting_order = triggered.resting_order;
            }
            if triggered.reject_reason.is_some() {
                result.reject_reason = triggered.reject_reason;
            }
        }
        result
    }

    fn effective_match_type(order_type: &OrderType) -> OrderType {
        match order_type {
            OrderType::MarketOnClose => OrderType::Market,
            OrderType::LimitOnClose => OrderType::Limit,
            OrderType::Pegged => OrderType::Limit,
            other => *other,
        }
    }

    fn process_new_order_inner(
        &mut self,
        order: &NewOrderSingle,
        allow_stop_rest: bool,
    ) -> MatchResult {
        if let Some(reason) = self.reject_bad_economics(order, allow_stop_rest) {
            return MatchResult {
                fills: vec![],
                resting_order: None,
                reject_reason: Some(reason),
            };
        }
        if let Some(reason) = self.validate_order(order) {
            return MatchResult {
                fills: vec![],
                resting_order: None,
                reject_reason: Some(reason),
            };
        }

        if matches!(order.order_type, OrderType::Stop | OrderType::StopLimit) && allow_stop_rest {
            self.pending_stops
                .entry(order.symbol.clone())
                .or_default()
                .push(PendingStopOrder {
                    order: order.clone(),
                });
            self.order_index
                .insert(order.cl_ord_id.clone(), order.symbol.clone());
            return MatchResult {
                fills: vec![],
                resting_order: None,
                reject_reason: None,
            };
        }

        if order.time_in_force == TimeInForce::Fok {
            let available = self.available_liquidity(order);
            if available < order.order_qty.0 {
                return MatchResult {
                    fills: vec![],
                    resting_order: None,
                    reject_reason: Some("FOK not fully fillable".to_string()),
                };
            }
        }

        if order.post_only == Some(true) && self.would_cross(order) {
            return MatchResult {
                fills: vec![],
                resting_order: None,
                reject_reason: Some("Post-only would cross".to_string()),
            };
        }

        let match_type = Self::effective_match_type(&order.order_type);

        let mut fills = Vec::new();
        let mut remaining_qty = order.order_qty.0;
        let mut cum_qty = 0.0;
        let mut total_value = 0.0;

        // Determine which side to match against. Short sales are sells.
        let is_buy = is_aggressor_buy(order.side);

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
                if match_type == OrderType::Limit || match_type == OrderType::StopLimit {
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
                let opposite_side = if is_buy {
                    &mut book.asks
                } else {
                    &mut book.bids
                };
                opposite_side.reduce(cl_ord_id, *fill_qty);
                if opposite_side.order_count() == 0 || !opposite_side.contains(cl_ord_id) {
                    self.order_index.remove(cl_ord_id);
                }

                remaining_qty -= fill_qty;
                cum_qty += fill_qty;
                total_value += fill_qty * price.0;

                let fill_id = self.next_fill_id();
                fills.push(Fill {
                    fill_id,
                    cl_ord_id: order.cl_ord_id.clone(),
                    contra_cl_ord_id: cl_ord_id.clone(),
                    side: order.side,
                    symbol: order.symbol.clone(),
                    fill_price: price.clone(),
                    fill_qty: Quantity(*fill_qty),
                    leaves_qty: Quantity(remaining_qty),
                    cum_qty: Quantity(cum_qty),
                    avg_price: Price(if cum_qty > 0.0 {
                        total_value / cum_qty
                    } else {
                        0.0
                    }),
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

        let can_rest = remaining_qty > 0.0
            && match_type == OrderType::Limit
            && !matches!(order.time_in_force, TimeInForce::Ioc | TimeInForce::Fok);

        // If there's remaining qty and it's a limit order, rest it on the book
        let resting_order = if can_rest {
            let resting = RestingOrder {
                order_id: format!("OX-{}", order.cl_ord_id),
                cl_ord_id: order.cl_ord_id.clone(),
                side: order.side,
                symbol: order.symbol.clone(),
                price: order.price.clone().unwrap_or(Price(0.0)),
                qty: order.order_qty.clone(),
                leaves_qty: Quantity(remaining_qty),
                time_in_force: order.time_in_force,
                expire_time: order.expire_time,
                account: order.account.clone(),
                seq: 0,
            };
            self.order_index
                .insert(order.cl_ord_id.clone(), order.symbol.clone());
            let book = self.book_for(&order.symbol);
            book.add_order(resting.clone());
            Some(resting)
        } else {
            None
        };

        let reject_reason = if fills.is_empty() && resting_order.is_none() {
            if matches!(order.time_in_force, TimeInForce::Ioc | TimeInForce::Fok)
                || matches!(match_type, OrderType::Market)
            {
                Some("No liquidity".to_string())
            } else {
                None
            }
        } else {
            None
        };

        MatchResult {
            fills,
            resting_order,
            reject_reason,
        }
    }

    /// Process a cancel request.
    pub fn process_cancel(&mut self, cancel: &CancelRequest) -> CancelOutcome {
        if let Some(stops) = self.pending_stops.get_mut(&cancel.symbol) {
            if let Some(idx) = stops
                .iter()
                .position(|s| s.order.cl_ord_id == cancel.orig_cl_ord_id)
            {
                stops.remove(idx);
                self.order_index.remove(&cancel.orig_cl_ord_id);
                return CancelOutcome::Cancelled(RestingOrder {
                    order_id: format!("OX-{}", cancel.orig_cl_ord_id),
                    cl_ord_id: cancel.orig_cl_ord_id.clone(),
                    side: cancel.side,
                    symbol: cancel.symbol.clone(),
                    price: Price(0.0),
                    qty: Quantity(0.0),
                    leaves_qty: Quantity(0.0),
                    time_in_force: TimeInForce::Day,
                    expire_time: None,
                    account: None,
                    seq: 0,
                });
            }
        }

        let Some(symbol) = self.order_index.get(&cancel.orig_cl_ord_id).cloned() else {
            return CancelOutcome::Rejected(CancelRejectReason::OrderNotFound);
        };

        let book = match self.books.get_mut(&symbol) {
            Some(b) => b,
            None => return CancelOutcome::Rejected(CancelRejectReason::OrderNotFound),
        };

        if let Some(order) = book.cancel_order(&cancel.orig_cl_ord_id) {
            self.order_index.remove(&cancel.orig_cl_ord_id);
            CancelOutcome::Cancelled(order)
        } else {
            CancelOutcome::Rejected(CancelRejectReason::AlreadyFilled)
        }
    }

    /// Process a cancel/replace request. Cancels the old order and adds a new one.
    pub fn process_replace(&mut self, replace: &CancelReplaceRequest) -> MatchResult {
        // First cancel the old order
        let old_order = match self.process_cancel(&CancelRequest {
            cl_ord_id: replace.cl_ord_id.clone(),
            orig_cl_ord_id: replace.orig_cl_ord_id.clone(),
            symbol: replace.symbol.clone(),
            side: replace.side,
            order_qty: None,
        }) {
            CancelOutcome::Cancelled(order) => Some(order),
            CancelOutcome::Rejected(_) => None,
        };

        // Then submit the new order
        let new_order = NewOrderSingle {
            cl_ord_id: replace.cl_ord_id.clone(),
            side: replace.side,
            order_qty: replace.order_qty.clone(),
            price: replace.price.clone(),
            stop_price: None,
            symbol: replace.symbol.clone(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: old_order.as_ref().and_then(|o| o.account.clone()),
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
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

    /// Working order present (resting or pending stop).
    pub fn has_working(&self, cl_ord_id: &str) -> bool {
        self.order_index.contains_key(cl_ord_id)
    }

    /// Record an execution report in per-account order history.
    pub fn record_execution(&mut self, account: &str, report: ExecutionReport) {
        self.order_history.push((account.to_string(), report));
    }

    /// Cancel working orders for an account, optionally filtered by symbol/side.
    pub fn process_mass_cancel(&mut self, req: &MassCancelRequest) -> (u32, Vec<ExecutionReport>) {
        let snapshot = self.open_orders(&req.account, req.symbol.as_deref());
        let targets: Vec<(ClientOrderId, Symbol, Side)> = snapshot
            .orders
            .into_iter()
            .filter(|o| req.side.is_none_or(|s| o.side == s))
            .map(|o| (o.cl_ord_id, o.symbol, o.side))
            .collect();
        let mut affected = 0u32;
        let mut reports = Vec::new();
        for (orig, symbol, side) in targets {
            match self.process_cancel(&CancelRequest {
                cl_ord_id: req.cl_ord_id.clone(),
                orig_cl_ord_id: orig,
                symbol,
                side,
                order_qty: None,
            }) {
                CancelOutcome::Cancelled(order) => {
                    affected += 1;
                    let mut report = resting_to_report(&order);
                    report.cl_ord_id = req.cl_ord_id.clone();
                    report.exec_type = ExecType::Canceled;
                    report.ord_status = OrdStatus::Canceled;
                    report.leaves_qty = Quantity(0.0);
                    reports.push(report);
                }
                CancelOutcome::Rejected(_) => {}
            }
        }
        (affected, reports)
    }

    /// Max actions the reference broker accepts in one `BatchRequest` (ADR 0010).
    pub const MAX_BATCH_ACTIONS: usize = 16;

    /// Apply an atomic trading batch. On any item reject, restore pre-batch state.
    pub fn process_batch(&mut self, req: &BatchRequest) -> BatchApply {
        let snapshot = self.clone();
        let mut results = Vec::with_capacity(req.actions.len());
        let mut fills = Vec::new();
        let mut reports = Vec::new();
        let mut fill_symbols: Vec<String> = Vec::new();
        let mut fail_at: Option<(usize, String)> = None;

        for (index, action) in req.actions.iter().enumerate() {
            match apply_one(self, action, index as u32) {
                Ok(step) => {
                    fills.extend(step.fills);
                    reports.extend(step.reports);
                    if let Some(sym) = step.symbol {
                        if !fill_symbols.iter().any(|s| s == &sym) {
                            fill_symbols.push(sym);
                        }
                    }
                    results.push(step.result);
                }
                Err(reason) => {
                    fail_at = Some((index, reason));
                    break;
                }
            }
        }

        let now = Self::now_nanos();
        if let Some((fail_index, reason)) = fail_at {
            *self = snapshot;
            let mut rolled = Vec::with_capacity(req.actions.len());
            for (index, action) in req.actions.iter().enumerate() {
                let (kind, cl_ord_id) = action_identity(action);
                let reject = if index == fail_index {
                    reason.clone()
                } else {
                    "BATCH_ROLLED_BACK".to_string()
                };
                rolled.push(BatchResult {
                    index: index as u32,
                    kind,
                    cl_ord_id,
                    status: BatchResultStatus::Rejected,
                    reject_reason: Some(reject),
                });
            }
            BatchApply {
                report: BatchReport {
                    batch_id: req.batch_id.clone(),
                    account: req.account.clone(),
                    outcome: BatchReportOutcome::Rejected,
                    results: rolled,
                    timestamp: now,
                },
                fills: Vec::new(),
                reports: Vec::new(),
                symbols: Vec::new(),
            }
        } else {
            BatchApply {
                report: BatchReport {
                    batch_id: req.batch_id.clone(),
                    account: req.account.clone(),
                    outcome: BatchReportOutcome::Applied,
                    results,
                    timestamp: now,
                },
                fills,
                reports,
                symbols: fill_symbols,
            }
        }
    }

    /// Working orders for an account (optionally filtered by symbol).
    pub fn open_orders(&self, account: &str, symbol: Option<&str>) -> OpenOrdersSnapshot {
        let mut orders = Vec::new();
        for book in self.books.values() {
            for resting in book.resting_orders() {
                let acct = resting.account.as_deref().unwrap_or("DEMO-ACCT");
                if acct != account {
                    continue;
                }
                if let Some(sym) = symbol {
                    if resting.symbol != sym {
                        continue;
                    }
                }
                orders.push(resting_to_report(resting));
            }
        }
        OpenOrdersSnapshot {
            account: account.to_string(),
            orders,
            is_snapshot: Some(true),
        }
    }

    /// Historical order events for an account.
    pub fn query_order_history(
        &self,
        account: &str,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> OrderHistoryBatch {
        let orders: Vec<ExecutionReport> = self
            .order_history
            .iter()
            .filter(|(acct, r)| {
                acct == account
                    && symbol.is_none_or(|s| r.symbol == s)
                    && start_time.is_none_or(|st| r.transact_time >= st)
                    && end_time.is_none_or(|et| r.transact_time <= et)
            })
            .map(|(_, r)| r.clone())
            .collect();
        let limit = limit.unwrap_or(500) as usize;
        let (page, has_more, next_cursor) =
            crate::pagination::paginate(orders, limit, cursor, crate::pagination::exec_cursor);
        OrderHistoryBatch {
            account: account.to_string(),
            orders: page,
            has_more,
            next_cursor,
        }
    }

    /// Build a point-in-time order book snapshot with sequence metadata.
    pub fn order_book_snapshot(&self, symbol: &str, depth: usize) -> Option<MarketDataSnapshot> {
        let book = self.books.get(symbol)?;
        Some(MarketDataSnapshot {
            symbol: symbol.to_string(),
            exchange: "SIM".to_string(),
            bids: book.bid_depth(depth),
            asks: book.ask_depth(depth),
            timestamp: Self::now_nanos(),
            sequence: Some(book.book_sequence()),
            is_snapshot: Some(true),
        })
    }

    pub fn order_book_snapshot_typed(
        &self,
        symbol: &str,
        depth: usize,
    ) -> Option<OrderBookSnapshot> {
        let book = self.books.get(symbol)?;
        Some(OrderBookSnapshot {
            symbol: symbol.to_string(),
            exchange: "SIM".to_string(),
            bids: book.bid_depth(depth),
            asks: book.ask_depth(depth),
            timestamp: Self::now_nanos(),
            sequence: Some(book.book_sequence()),
            is_snapshot: Some(true),
        })
    }

    pub fn order_book_delta(&self, symbol: &str) -> Option<OrderBookDelta> {
        let book = self.books.get(symbol)?;
        let mut updates = Vec::new();
        if let Some((price, qty)) = book.best_bid() {
            updates.push(MarketDataUpdate {
                side: Side::Buy,
                action: MarketDataAction::Change,
                price: price.clone(),
                qty: qty.clone(),
            });
        }
        if let Some((price, qty)) = book.best_ask() {
            updates.push(MarketDataUpdate {
                side: Side::Sell,
                action: MarketDataAction::Change,
                price: price.clone(),
                qty: qty.clone(),
            });
        }
        Some(OrderBookDelta {
            symbol: symbol.to_string(),
            updates,
            timestamp: Self::now_nanos(),
            sequence: Some(book.book_sequence()),
        })
    }
}

pub fn resting_to_report(resting: &RestingOrder) -> ExecutionReport {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64;
    ExecutionReport {
        cl_ord_id: resting.cl_ord_id.clone(),
        order_id: resting.order_id.clone(),
        exec_id: fit_exec_id(format!("OPEN-{}", resting.cl_ord_id)),
        exec_type: ExecType::New,
        ord_status: OrdStatus::New,
        side: resting.side,
        last_qty: None,
        last_price: None,
        leaves_qty: resting.leaves_qty.clone(),
        cum_qty: Quantity(resting.qty.0 - resting.leaves_qty.0),
        avg_price: resting.price.clone(),
        symbol: resting.symbol.clone(),
        transact_time: now,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_id_stays_within_schema_max() {
        assert_eq!(fit_exec_id("OPEN-1".into()), "OPEN-1");
        let long = fit_exec_id(format!("OPEN-{}", "FIX-E2E-AAPL-001"));
        assert!(long.len() <= 20, "{long}");
        assert_ne!(long, fit_exec_id(format!("OPEN-{}", "FIX-E2E-AAPL-002")));
    }

    fn make_limit_order(
        id: &str,
        side: Side,
        symbol: &str,
        price: f64,
        qty: f64,
    ) -> NewOrderSingle {
        NewOrderSingle {
            cl_ord_id: id.to_string(),
            side,
            order_qty: Quantity(qty),
            price: Some(Price(price)),
            stop_price: None,
            symbol: symbol.to_string(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: None,
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
        }
    }

    fn make_market_order(id: &str, side: Side, symbol: &str, qty: f64) -> NewOrderSingle {
        NewOrderSingle {
            cl_ord_id: id.to_string(),
            side,
            order_qty: Quantity(qty),
            price: None,
            stop_price: None,
            symbol: symbol.to_string(),
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: None,
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
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
    fn batch_two_limits_apply() {
        let mut engine = MatchingEngine::new();
        let req = BatchRequest {
            batch_id: "BAT-1".into(),
            account: "TEST".into(),
            actions: vec![
                BatchAction {
                    kind: BatchActionKind::NewOrder,
                    new_order: Some(make_limit_order("B1", Side::Buy, "AAPL", 100.0, 10.0)),
                    cancel: None,
                    replace: None,
                },
                BatchAction {
                    kind: BatchActionKind::NewOrder,
                    new_order: Some(make_limit_order("B2", Side::Buy, "AAPL", 99.0, 10.0)),
                    cancel: None,
                    replace: None,
                },
            ],
        };
        let out = engine.process_batch(&req);
        assert_eq!(out.report.outcome, BatchReportOutcome::Applied);
        assert!(engine.has_working("B1"));
        assert!(engine.has_working("B2"));
    }

    #[test]
    fn batch_rolls_back_on_second_reject() {
        let mut engine = MatchingEngine::new();
        let req = BatchRequest {
            batch_id: "BAT-2".into(),
            account: "TEST".into(),
            actions: vec![
                BatchAction {
                    kind: BatchActionKind::NewOrder,
                    new_order: Some(make_limit_order("OK", Side::Buy, "AAPL", 100.0, 10.0)),
                    cancel: None,
                    replace: None,
                },
                BatchAction {
                    kind: BatchActionKind::Cancel,
                    new_order: None,
                    cancel: Some(CancelRequest {
                        cl_ord_id: "CXL".into(),
                        orig_cl_ord_id: "MISSING".into(),
                        symbol: "AAPL".into(),
                        side: Side::Buy,
                        order_qty: None,
                    }),
                    replace: None,
                },
            ],
        };
        let out = engine.process_batch(&req);
        assert_eq!(out.report.outcome, BatchReportOutcome::Rejected);
        assert!(!engine.has_working("OK"));
        assert_eq!(
            out.report.results[0].reject_reason.as_deref(),
            Some("BATCH_ROLLED_BACK")
        );
        assert_eq!(out.report.results[1].status, BatchResultStatus::Rejected);
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
        assert!(matches!(cancelled, CancelOutcome::Cancelled(_)));
        if let CancelOutcome::Cancelled(order) = cancelled {
            assert_eq!(order.cl_ord_id, "S1");
        }

        let book = engine.get_book(&"AAPL".to_string()).unwrap();
        assert_eq!(book.asks.order_count(), 0);
    }

    #[test]
    fn test_ioc_cancels_unfilled_remainder() {
        let mut engine = MatchingEngine::new();
        let sell = make_limit_order("S1", Side::Sell, "AAPL", 100.00, 50.0);
        engine.process_new_order(&sell);

        let mut buy = make_limit_order("B1", Side::Buy, "AAPL", 99.00, 30.0);
        buy.time_in_force = TimeInForce::Ioc;
        let result = engine.process_new_order(&buy);

        assert!(result.fills.is_empty());
        assert!(result.resting_order.is_none());
    }

    #[test]
    fn test_fok_rejects_partial_liquidity() {
        let mut engine = MatchingEngine::new();
        let sell = make_limit_order("S1", Side::Sell, "AAPL", 100.00, 10.0);
        engine.process_new_order(&sell);

        let mut buy = make_limit_order("B1", Side::Buy, "AAPL", 101.00, 50.0);
        buy.time_in_force = TimeInForce::Fok;
        let result = engine.process_new_order(&buy);

        assert!(result.fills.is_empty());
        assert_eq!(
            result.reject_reason.as_deref(),
            Some("FOK not fully fillable")
        );
    }

    #[test]
    fn test_post_only_rejects_when_would_cross() {
        let mut engine = MatchingEngine::new();
        let ask = make_limit_order("ASK-1", Side::Sell, "BTC", 50000.0, 1.0);
        engine.process_new_order(&ask);

        let mut buy = make_limit_order("BUY-1", Side::Buy, "BTC", 50000.0, 1.0);
        buy.post_only = Some(true);
        let result = engine.process_new_order(&buy);

        assert!(result.fills.is_empty());
        assert_eq!(
            result.reject_reason.as_deref(),
            Some("Post-only would cross")
        );
    }

    #[test]
    fn test_stop_order_rests_until_triggered() {
        let mut engine = MatchingEngine::new();
        let mut stop = make_limit_order("STOP-1", Side::Buy, "AAPL", 105.0, 10.0);
        stop.order_type = OrderType::Stop;
        stop.price = None;
        stop.stop_price = Some(Price(105.0));
        let result = engine.process_new_order(&stop);
        assert!(result.fills.is_empty());

        let sell = make_limit_order("S1", Side::Sell, "AAPL", 105.0, 10.0);
        let triggered = engine.process_new_order(&sell);
        assert!(!triggered.fills.is_empty());
    }

    #[test]
    fn test_cancel_unknown_order_rejected() {
        let mut engine = MatchingEngine::new();
        let cancel = CancelRequest {
            cl_ord_id: "C1".to_string(),
            orig_cl_ord_id: "MISSING".to_string(),
            symbol: "AAPL".to_string(),
            side: Side::Buy,
            order_qty: None,
        };
        assert!(matches!(
            engine.process_cancel(&cancel),
            CancelOutcome::Rejected(CancelRejectReason::OrderNotFound)
        ));
    }

    #[test]
    fn test_mass_cancel_by_symbol_and_side() {
        let mut engine = MatchingEngine::new();
        let mut buy = make_limit_order("B1", Side::Buy, "AAPL", 10.0, 1.0);
        buy.account = Some("DEMO-ACCT".to_string());
        let mut sell = make_limit_order("S1", Side::Sell, "AAPL", 20.0, 1.0);
        sell.account = Some("DEMO-ACCT".to_string());
        let mut other = make_limit_order("B2", Side::Buy, "BTC", 10.0, 1.0);
        other.account = Some("DEMO-ACCT".to_string());
        engine.process_new_order(&buy);
        engine.process_new_order(&sell);
        engine.process_new_order(&other);

        let (n, reports) = engine.process_mass_cancel(&MassCancelRequest {
            cl_ord_id: "MC-1".to_string(),
            account: "DEMO-ACCT".to_string(),
            symbol: Some("AAPL".to_string()),
            side: Some(Side::Buy),
        });
        assert_eq!(n, 1);
        assert_eq!(reports.len(), 1);
        assert_eq!(
            engine.open_orders("DEMO-ACCT", Some("AAPL")).orders.len(),
            1
        );
        assert_eq!(engine.open_orders("DEMO-ACCT", Some("BTC")).orders.len(), 1);
    }
}
