//! Public market data: candles, trades, BBO, quote subscriptions.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use fig_core::messages::*;
use fig_core::transport::FigConnection;

pub fn now_ns() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64
}

/// FIG interval names from SPEC §9.1. The reference broker builds all four.
pub const CANDLE_INTERVALS: [&str; 4] = ["1m", "5m", "1h", "1d"];

fn interval_width(interval: &str) -> i64 {
    interval_ns(interval).unwrap_or(300 * 1_000_000_000)
}

/// Parse interval strings like `5m`, `1h`, `1d` into nanoseconds.
pub fn interval_ns(interval: &str) -> Option<i64> {
    if interval.is_empty() {
        return None;
    }
    let (num, unit) = interval.split_at(interval.len() - 1);
    let n: i64 = num.parse().ok()?;
    let secs = match unit {
        "s" => n,
        "m" => n * 60,
        "h" => n * 3600,
        "d" => n * 86400,
        _ => return None,
    };
    Some(secs * 1_000_000_000)
}

pub fn bar_start(time_ns: i64, interval: &str) -> i64 {
    let width = interval_width(interval);
    (time_ns / width) * width
}

impl SubscriptionKind {
    /// Native `CHANNEL_PATH` for this subscription. Used to match UNSUBSCRIBE
    /// by equality, so a prefix cannot drop both `1m` and `5m`.
    pub fn channel_path(&self) -> String {
        match self {
            SubscriptionKind::Quotes { symbol } => format!("marketdata/{symbol}/quotes"),
            SubscriptionKind::Candles { symbol, interval } => {
                format!("marketdata/{symbol}/candles/{interval}")
            }
            SubscriptionKind::Trades { symbol } => format!("marketdata/{symbol}/trades"),
            SubscriptionKind::AggTrades { symbol } => format!("marketdata/{symbol}/aggtrades"),
            SubscriptionKind::Bbo { symbol } => format!("marketdata/{symbol}/bbo"),
            SubscriptionKind::Ticker { symbol } => format!("marketdata/{symbol}/ticker"),
            SubscriptionKind::MiniTicker { symbol: None } => "marketdata/ticker/all".to_string(),
            SubscriptionKind::MiniTicker {
                symbol: Some(symbol),
            } => {
                format!("marketdata/{symbol}/miniticker")
            }
            SubscriptionKind::MarkPrice { symbol } => format!("marketdata/{symbol}/mark"),
            SubscriptionKind::Liquidations => "marketdata/liquidations".to_string(),
            SubscriptionKind::Session => "marketdata/session".to_string(),
            SubscriptionKind::SecurityStatus { symbol } => format!("marketdata/{symbol}/status"),
            SubscriptionKind::PriceLimits { symbol } => format!("marketdata/{symbol}/limits"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionKind {
    Quotes { symbol: String },
    Candles { symbol: String, interval: String },
    Trades { symbol: String },
    AggTrades { symbol: String },
    Bbo { symbol: String },
    Ticker { symbol: String },
    MiniTicker { symbol: Option<String> },
    MarkPrice { symbol: String },
    Liquidations,
    Session,
    SecurityStatus { symbol: String },
    PriceLimits { symbol: String },
}

#[derive(Clone)]
pub struct StreamSubscription {
    pub channel_id: u16,
    pub routing_key: String,
    pub kind: SubscriptionKind,
    pub conn: Option<Arc<FigConnection>>,
}

#[derive(Default)]
pub struct MarketDataHub {
    trades: HashMap<String, Vec<PublicTrade>>,
    agg_trades: HashMap<String, Vec<AggregateTrade>>,
    closed_candles: HashMap<(String, String), Vec<CandleBar>>,
    partial_candles: HashMap<(String, String), CandleBar>,
    last_bbo: HashMap<String, BestBidOffer>,
    tickers: HashMap<String, SymbolTicker>,
    ticker_trades: HashMap<String, Vec<(i64, f64, f64)>>,
    mini_tickers: HashMap<String, MiniTicker>,
    mark_prices: HashMap<String, MarkPriceUpdate>,
    liquidations: Vec<LiquidationTrade>,
    book_history: HashMap<String, Vec<OrderBookSnapshot>>,
    trade_seq: u64,
    agg_seq: u64,
}

impl MarketDataHub {
    /// Record a fill. The `Vec` is every candle that rolled closed on this trade.
    pub fn on_trade(
        &mut self,
        symbol: &str,
        price: f64,
        qty: f64,
        side: Side,
    ) -> (Option<LiquidationTrade>, Vec<CandleBar>) {
        let ts = now_ns();
        self.trade_seq += 1;
        let trade_id = format!("T-{}", self.trade_seq);
        let trade = PublicTrade {
            symbol: symbol.to_string(),
            trade_id: trade_id.clone(),
            price: Price(price),
            qty: Quantity(qty),
            side: match side {
                Side::Buy => Side::Buy,
                Side::Sell | Side::SellShort | Side::SellShortExempt => Side::Sell,
            },
            timestamp: ts,
        };
        self.trades
            .entry(symbol.to_string())
            .or_default()
            .push(trade);

        self.agg_seq += 1;
        let agg = AggregateTrade {
            symbol: symbol.to_string(),
            agg_trade_id: format!("A-{}", self.agg_seq),
            price: Price(price),
            qty: Quantity(qty),
            side: match side {
                Side::Buy => Side::Buy,
                _ => Side::Sell,
            },
            first_trade_id: trade_id.clone(),
            last_trade_id: trade_id,
            timestamp: ts,
        };
        self.agg_trades
            .entry(symbol.to_string())
            .or_default()
            .push(agg);

        let mut closed = Vec::new();
        for interval in CANDLE_INTERVALS {
            if let Some(bar) = self.update_candle(symbol, interval, price, qty, ts) {
                closed.push(bar);
            }
        }
        self.update_ticker(symbol, price, qty, ts);
        self.update_mini_ticker(symbol, price, qty, ts);
        self.update_mark_price(symbol, price, ts);

        if qty >= 500.0 {
            let liq = LiquidationTrade {
                symbol: symbol.to_string(),
                side: match side {
                    Side::Buy => Side::Buy,
                    _ => Side::Sell,
                },
                price: Price(price),
                qty: Quantity(qty),
                timestamp: ts,
            };
            self.liquidations.push(liq.clone());
            return (Some(liq), closed);
        }
        (None, closed)
    }

    /// Move open bars whose window has ended into `closed_candles`. No new
    /// partial is opened until the next trade. A quiet book therefore finalizes
    /// on the next trade, query, or subscribe, not on a timer.
    pub fn roll_elapsed(&mut self, now: i64) {
        let keys: Vec<_> = self.partial_candles.keys().cloned().collect();
        for key in keys {
            let Some(partial) = self.partial_candles.get(&key).cloned() else {
                continue;
            };
            let width = interval_width(&key.1);
            if now < partial.bar_start.saturating_add(width) {
                continue;
            }
            self.partial_candles.remove(&key);
            self.closed_candles
                .entry(key)
                .or_default()
                .push(finalize_bar(partial, width));
        }
    }

    fn update_ticker(&mut self, symbol: &str, price: f64, qty: f64, ts: i64) {
        const DAY_NS: i64 = 24 * 3_600 * 1_000_000_000;
        let window = self.ticker_trades.entry(symbol.to_string()).or_default();
        window.push((ts, price, qty));
        window.retain(|(t, _, _)| *t >= ts.saturating_sub(DAY_NS));

        let open = window.first().map(|(_, p, _)| *p).unwrap_or(price);
        let high = window.iter().map(|(_, p, _)| *p).fold(price, f64::max);
        let low = window.iter().map(|(_, p, _)| *p).fold(price, f64::min);
        let volume: f64 = window.iter().map(|(_, _, q)| *q).sum();

        let entry = self
            .tickers
            .entry(symbol.to_string())
            .or_insert_with(|| SymbolTicker {
                symbol: symbol.to_string(),
                last_price: Price(price),
                price_change: 0.0,
                price_change_pct: 0.0,
                volume: Quantity(0.0),
                high: Price(price),
                low: Price(price),
                open: Price(open),
                timestamp: ts,
                is_snapshot: None,
            });
        entry.last_price = Price(price);
        entry.high = Price(high);
        entry.low = Price(low);
        entry.open = Price(open);
        entry.volume = Quantity(volume);
        entry.price_change = price - open;
        entry.price_change_pct = if open.abs() > f64::EPSILON {
            (price - open) / open * 100.0
        } else {
            0.0
        };
        entry.timestamp = ts;
        entry.is_snapshot = Some(false);
    }

    fn update_mini_ticker(&mut self, symbol: &str, price: f64, qty: f64, ts: i64) {
        let entry = self
            .mini_tickers
            .entry(symbol.to_string())
            .or_insert_with(|| MiniTicker {
                symbol: symbol.to_string(),
                last_price: Price(price),
                volume: Quantity(0.0),
                timestamp: ts,
                is_snapshot: None,
            });
        entry.last_price = Price(price);
        entry.volume = Quantity(entry.volume.0 + qty);
        entry.timestamp = ts;
        entry.is_snapshot = Some(false);
    }

    fn update_mark_price(&mut self, symbol: &str, price: f64, ts: i64) {
        self.mark_prices.insert(
            symbol.to_string(),
            MarkPriceUpdate {
                symbol: symbol.to_string(),
                mark_price: Price(price * 1.0001),
                index_price: Some(Price(price)),
                funding_rate: Some(0.0001),
                timestamp: ts,
                is_snapshot: Some(false),
            },
        );
    }

    pub fn mark_price(&self, symbol: &str) -> MarkPriceUpdate {
        self.mark_prices
            .get(symbol)
            .cloned()
            .unwrap_or(MarkPriceUpdate {
                symbol: symbol.to_string(),
                mark_price: Price(0.0),
                index_price: Some(Price(0.0)),
                funding_rate: Some(0.0),
                timestamp: now_ns(),
                is_snapshot: Some(true),
            })
    }

    pub fn all_mini_tickers(&self) -> AllMidsBatch {
        AllMidsBatch {
            tickers: self.mini_tickers.values().cloned().collect(),
        }
    }

    pub fn mini_ticker(&self, symbol: &str) -> MiniTicker {
        self.mini_tickers
            .get(symbol)
            .cloned()
            .unwrap_or(MiniTicker {
                symbol: symbol.to_string(),
                last_price: Price(0.0),
                volume: Quantity(0.0),
                timestamp: now_ns(),
                is_snapshot: Some(true),
            })
    }

    pub fn query_agg_trades(
        &self,
        symbol: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> AggregateTradeBatch {
        let mut trades = self.agg_trades.get(symbol).cloned().unwrap_or_default();
        if let Some(st) = start_time {
            trades.retain(|t| t.timestamp >= st);
        }
        if let Some(et) = end_time {
            trades.retain(|t| t.timestamp <= et);
        }
        let limit = limit.unwrap_or(500) as usize;
        let (page, has_more, next_cursor) =
            crate::pagination::paginate(trades, limit, cursor, |t| {
                format!("{}:{}", t.timestamp, t.agg_trade_id)
            });
        AggregateTradeBatch {
            symbol: symbol.to_string(),
            trades: page,
            has_more,
            next_cursor,
        }
    }

    pub fn last_agg_trade(&self, symbol: &str) -> Option<AggregateTrade> {
        self.agg_trades.get(symbol).and_then(|t| t.last().cloned())
    }

    pub fn recent_liquidations(&self, limit: usize) -> Vec<LiquidationTrade> {
        self.liquidations
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn ticker(&self, symbol: &str) -> Option<SymbolTicker> {
        self.tickers.get(symbol).cloned()
    }

    pub fn ticker_snapshot(&self, symbol: &str) -> SymbolTicker {
        self.tickers.get(symbol).cloned().unwrap_or(SymbolTicker {
            symbol: symbol.to_string(),
            last_price: Price(0.0),
            price_change: 0.0,
            price_change_pct: 0.0,
            volume: Quantity(0.0),
            high: Price(0.0),
            low: Price(0.0),
            open: Price(0.0),
            timestamp: now_ns(),
            is_snapshot: Some(true),
        })
    }

    /// `Some` is the bar that just closed. Its `bar_end` is the old window's end.
    fn update_candle(
        &mut self,
        symbol: &str,
        interval: &str,
        price: f64,
        qty: f64,
        ts: i64,
    ) -> Option<CandleBar> {
        let key = (symbol.to_string(), interval.to_string());
        let width = interval_width(interval);
        let start = bar_start(ts, interval);
        let end = start + width;
        let mut closed = None;

        if let Some(partial) = self.partial_candles.get(&key) {
            if partial.bar_start == start {
                let mut bar = partial.clone();
                bar.high = Price(bar.high.0.max(price));
                bar.low = Price(bar.low.0.min(price));
                bar.close = Price(price);
                bar.volume = Quantity(bar.volume.0 + qty);
                bar.is_final = false;
                self.partial_candles.insert(key, bar);
                return None;
            }
            if partial.bar_start < start {
                let finalized = finalize_bar(partial.clone(), width);
                self.closed_candles
                    .entry(key.clone())
                    .or_default()
                    .push(finalized.clone());
                closed = Some(finalized);
            }
        }

        self.partial_candles.insert(
            key,
            CandleBar {
                symbol: symbol.to_string(),
                interval: interval.to_string(),
                open: Price(price),
                high: Price(price),
                low: Price(price),
                close: Price(price),
                volume: Quantity(qty),
                bar_start: start,
                bar_end: end,
                is_final: false,
                is_snapshot: None,
            },
        );
        closed
    }

    pub fn query_candles(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> CandleBarBatch {
        let key = (symbol.to_string(), interval.to_string());
        let mut bars = self.closed_candles.get(&key).cloned().unwrap_or_default();
        if let Some(partial) = self.partial_candles.get(&key) {
            bars.push(partial.clone());
        }
        if let Some(st) = start_time {
            bars.retain(|b| b.bar_start >= st);
        }
        if let Some(et) = end_time {
            bars.retain(|b| b.bar_end <= et);
        }
        let limit = limit.unwrap_or(500) as usize;
        let (page, has_more, next_cursor) =
            crate::pagination::paginate(bars, limit, cursor, crate::pagination::candle_cursor);
        CandleBarBatch {
            symbol: symbol.to_string(),
            interval: interval.to_string(),
            bars: page,
            has_more,
            next_cursor,
        }
    }

    pub fn query_trades(
        &self,
        symbol: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> PublicTradeBatch {
        let mut trades = self.trades.get(symbol).cloned().unwrap_or_default();
        if let Some(st) = start_time {
            trades.retain(|t| t.timestamp >= st);
        }
        if let Some(et) = end_time {
            trades.retain(|t| t.timestamp <= et);
        }
        let limit = limit.unwrap_or(500) as usize;
        let (page, has_more, next_cursor) =
            crate::pagination::paginate(trades, limit, cursor, crate::pagination::trade_cursor);
        PublicTradeBatch {
            symbol: symbol.to_string(),
            trades: page,
            has_more,
            next_cursor,
        }
    }

    pub fn update_bbo(&mut self, symbol: &str, bid: Option<(f64, f64)>, ask: Option<(f64, f64)>) {
        let bbo = BestBidOffer {
            symbol: symbol.to_string(),
            bid_price: bid.map(|(p, _)| Price(p)),
            bid_qty: bid.map(|(_, q)| Quantity(q)),
            ask_price: ask.map(|(p, _)| Price(p)),
            ask_qty: ask.map(|(_, q)| Quantity(q)),
            timestamp: now_ns(),
            is_snapshot: Some(false),
        };
        self.last_bbo.insert(symbol.to_string(), bbo);
    }

    pub fn last_bbo(&self, symbol: &str) -> Option<BestBidOffer> {
        self.last_bbo.get(symbol).cloned()
    }

    pub fn partial_candle(&self, symbol: &str, interval: &str) -> Option<CandleBar> {
        self.partial_candles
            .get(&(symbol.to_string(), interval.to_string()))
            .cloned()
    }

    pub fn last_trade(&self, symbol: &str) -> Option<PublicTrade> {
        self.trades.get(symbol).and_then(|t| t.last().cloned())
    }

    pub fn record_book_snapshot(&mut self, mut snap: OrderBookSnapshot) {
        snap.is_snapshot = Some(true);
        let history = self.book_history.entry(snap.symbol.clone()).or_default();
        history.push(snap);
        if history.len() > 256 {
            history.remove(0);
        }
    }

    pub fn book_at_time(
        &self,
        symbol: &str,
        at_time: i64,
        depth: usize,
    ) -> Option<OrderBookSnapshot> {
        let history = self.book_history.get(symbol)?;
        let mut snap = history
            .iter()
            .rev()
            .find(|s| s.timestamp <= at_time)?
            .clone();
        snap.bids.truncate(depth);
        snap.asks.truncate(depth);
        snap.is_snapshot = Some(true);
        Some(snap)
    }
}

fn finalize_bar(mut bar: CandleBar, width: i64) -> CandleBar {
    bar.is_final = true;
    bar.is_snapshot = Some(false);
    bar.bar_end = bar.bar_start.saturating_add(width);
    bar
}

pub fn parse_md_subscription(routing_key: &str, channel_path: &str) -> Option<SubscriptionKind> {
    if crate::market_state::parse_session_path(channel_path)
        || crate::market_state::parse_session_path(routing_key)
    {
        return Some(SubscriptionKind::Session);
    }
    if let Some(symbol) = crate::market_state::parse_security_status_path(channel_path)
        .or_else(|| crate::market_state::parse_security_status_path(routing_key))
    {
        return Some(SubscriptionKind::SecurityStatus { symbol });
    }
    if let Some(symbol) = crate::market_state::parse_price_limits_path(channel_path)
        .or_else(|| crate::market_state::parse_price_limits_path(routing_key))
    {
        return Some(SubscriptionKind::PriceLimits { symbol });
    }
    if parse_liquidations_subscription(routing_key, channel_path) {
        return Some(SubscriptionKind::Liquidations);
    }
    if let Some(symbol) = parse_mini_ticker_subscription(routing_key, channel_path) {
        return Some(SubscriptionKind::MiniTicker { symbol });
    }
    let path = if !channel_path.is_empty() {
        channel_path.to_string()
    } else {
        routing_key.replace('.', "/")
    };
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.first() != Some(&"marketdata") {
        return None;
    }
    let symbol = parts.get(1)?.to_string();
    match parts.get(2).copied() {
        Some("quotes") | Some("book") => Some(SubscriptionKind::Quotes { symbol }),
        Some("candles") => {
            let interval = parts.get(3)?.to_string();
            Some(SubscriptionKind::Candles { symbol, interval })
        }
        Some("trades") => Some(SubscriptionKind::Trades { symbol }),
        Some("aggtrades") => Some(SubscriptionKind::AggTrades { symbol }),
        Some("bbo") => Some(SubscriptionKind::Bbo { symbol }),
        Some("ticker") => Some(SubscriptionKind::Ticker { symbol }),
        Some("mark") => Some(SubscriptionKind::MarkPrice { symbol }),
        _ => None,
    }
}

pub fn parse_all_mids_path(path: &str) -> bool {
    path == "marketdata/ticker/all"
}

pub fn parse_mark_query_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "mark" {
        return Some(parts[1].to_string());
    }
    None
}

pub fn parse_agg_trade_query_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "aggtrades" {
        return Some(parts[1].to_string());
    }
    None
}

pub fn parse_liquidations_subscription(routing_key: &str, channel_path: &str) -> bool {
    let path = if !channel_path.is_empty() {
        channel_path.to_string()
    } else {
        routing_key.replace('.', "/")
    };
    path == "marketdata/liquidations"
}

pub fn parse_mini_ticker_subscription(
    routing_key: &str,
    channel_path: &str,
) -> Option<Option<String>> {
    let path = if !channel_path.is_empty() {
        channel_path.to_string()
    } else {
        routing_key.replace('.', "/")
    };
    if path == "marketdata/ticker/all" {
        return Some(None);
    }
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "miniticker" {
        return Some(Some(parts[1].to_string()));
    }
    None
}

pub fn parse_ticker_query_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "ticker" {
        return Some(parts[1].to_string());
    }
    None
}

pub fn parse_candle_query_path(path: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 4 && parts[0] == "marketdata" && parts[2] == "candles" {
        return Some((parts[1].to_string(), parts[3].to_string()));
    }
    None
}

pub fn parse_trade_query_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "trades" {
        return Some(parts[1].to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_parsing() {
        assert_eq!(interval_ns("5m"), Some(300 * 1_000_000_000));
    }

    #[test]
    fn closed_bar_end_is_the_old_window() {
        let mut hub = MarketDataHub::default();
        let width = interval_ns("1m").unwrap();
        let start = 1_700_000_000_000_000_000 / width * width;
        assert!(hub
            .update_candle("BTCUSDT", "1m", 10.0, 1.0, start + 1)
            .is_none());
        let closed = hub
            .update_candle("BTCUSDT", "1m", 12.0, 1.0, start + width + 1)
            .expect("roll");
        assert!(closed.is_final);
        assert_eq!(closed.is_snapshot, Some(false));
        assert_eq!(closed.bar_start, start);
        assert_eq!(closed.bar_end, start + width);
        let partial = hub.partial_candle("BTCUSDT", "1m").unwrap();
        assert!(!partial.is_final);
        assert_eq!(partial.bar_start, start + width);
    }

    #[test]
    fn fill_builds_hourly_and_daily_bars() {
        let mut hub = MarketDataHub::default();
        let (_liq, _closed) = hub.on_trade("BTCUSDT", 10.0, 1.0, Side::Buy);
        assert!(hub.partial_candle("BTCUSDT", "1h").is_some());
        assert!(hub.partial_candle("BTCUSDT", "1d").is_some());
    }

    #[test]
    fn roll_elapsed_finalizes_without_a_new_trade() {
        let mut hub = MarketDataHub::default();
        let width = interval_ns("1m").unwrap();
        let start = 1_700_000_000_000_000_000 / width * width;
        hub.update_candle("BTCUSDT", "1m", 10.0, 1.0, start + 1);
        hub.roll_elapsed(start + width);
        assert!(hub.partial_candle("BTCUSDT", "1m").is_none());
        let page = hub.query_candles("BTCUSDT", "1m", None, None, None, None);
        assert_eq!(page.bars.len(), 1);
        assert!(page.bars[0].is_final);
        assert_eq!(page.bars[0].bar_end, start + width);
    }
}
