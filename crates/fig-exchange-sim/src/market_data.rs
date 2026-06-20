//! Public market data: candles, trades, BBO, quote subscriptions.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use fig_core::messages::*;

fn now_ns() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64
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
    let width = interval_ns(interval).unwrap_or(300 * 1_000_000_000);
    (time_ns / width) * width
}

#[derive(Debug, Clone)]
pub enum SubscriptionKind {
    Quotes { symbol: String },
    Candles { symbol: String, interval: String },
    Trades { symbol: String },
    Bbo { symbol: String },
    Ticker { symbol: String },
}

#[derive(Debug, Clone)]
pub struct StreamSubscription {
    pub channel_id: u16,
    pub routing_key: String,
    pub kind: SubscriptionKind,
}

#[derive(Default)]
pub struct MarketDataHub {
    trades: HashMap<String, Vec<PublicTrade>>,
    closed_candles: HashMap<(String, String), Vec<CandleBar>>,
    partial_candles: HashMap<(String, String), CandleBar>,
    last_bbo: HashMap<String, BestBidOffer>,
    tickers: HashMap<String, SymbolTicker>,
    trade_seq: u64,
}

impl MarketDataHub {
    pub fn on_trade(&mut self, symbol: &str, price: f64, qty: f64, side: Side) {
        let ts = now_ns();
        self.trade_seq += 1;
        let trade = PublicTrade {
            symbol: symbol.to_string(),
            trade_id: format!("T-{}", self.trade_seq),
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

        for interval in ["1m", "5m"] {
            self.update_candle(symbol, interval, price, qty, ts);
        }
        self.update_ticker(symbol, price, qty, ts);
    }

    fn update_ticker(&mut self, symbol: &str, price: f64, qty: f64, ts: i64) {
        let entry = self.tickers.entry(symbol.to_string()).or_insert_with(|| SymbolTicker {
            symbol: symbol.to_string(),
            last_price: Price(price),
            price_change: 0.0,
            price_change_pct: 0.0,
            volume: Quantity(0.0),
            high: Price(price),
            low: Price(price),
            open: Price(price),
            timestamp: ts,
            is_snapshot: None,
        });
        let open = entry.open.0;
        entry.last_price = Price(price);
        entry.high = Price(entry.high.0.max(price));
        entry.low = Price(entry.low.0.min(price));
        entry.volume = Quantity(entry.volume.0 + qty);
        entry.price_change = price - open;
        entry.price_change_pct = if open.abs() > f64::EPSILON {
            (price - open) / open * 100.0
        } else {
            0.0
        };
        entry.timestamp = ts;
        entry.is_snapshot = Some(false);
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

    fn update_candle(&mut self, symbol: &str, interval: &str, price: f64, qty: f64, ts: i64) {
        let key = (symbol.to_string(), interval.to_string());
        let start = bar_start(ts, interval);
        let end = start + interval_ns(interval).unwrap_or(300 * 1_000_000_000);

        if let Some(partial) = self.partial_candles.get(&key) {
            if partial.bar_start == start {
                let mut bar = partial.clone();
                bar.high = Price(bar.high.0.max(price));
                bar.low = Price(bar.low.0.min(price));
                bar.close = Price(price);
                bar.volume = Quantity(bar.volume.0 + qty);
                bar.is_final = false;
                self.partial_candles.insert(key.clone(), bar);
                return;
            }
            if partial.bar_start < start {
                let mut finalized = partial.clone();
                finalized.is_final = true;
                finalized.bar_end = end;
                self.closed_candles
                    .entry(key.clone())
                    .or_default()
                    .push(finalized);
            }
        }

        let bar = CandleBar {
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
        };
        self.partial_candles.insert(key, bar);
    }

    pub fn query_candles(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> CandleBarBatch {
        let key = (symbol.to_string(), interval.to_string());
        let mut bars = self
            .closed_candles
            .get(&key)
            .cloned()
            .unwrap_or_default();
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
        let has_more = bars.len() > limit;
        bars.truncate(limit);
        CandleBarBatch {
            symbol: symbol.to_string(),
            interval: interval.to_string(),
            bars,
            has_more,
            next_cursor: None,
        }
    }

    pub fn query_trades(
        &self,
        symbol: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> PublicTradeBatch {
        let mut trades = self.trades.get(symbol).cloned().unwrap_or_default();
        if let Some(st) = start_time {
            trades.retain(|t| t.timestamp >= st);
        }
        if let Some(et) = end_time {
            trades.retain(|t| t.timestamp <= et);
        }
        let limit = limit.unwrap_or(500) as usize;
        let has_more = trades.len() > limit;
        trades.truncate(limit);
        PublicTradeBatch {
            symbol: symbol.to_string(),
            trades,
            has_more,
            next_cursor: None,
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
            is_snapshot: None,
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
}

pub fn parse_md_subscription(routing_key: &str, channel_path: &str) -> Option<SubscriptionKind> {
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
        Some("bbo") => Some(SubscriptionKind::Bbo { symbol }),
        Some("ticker") => Some(SubscriptionKind::Ticker { symbol }),
        _ => None,
    }
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
}
