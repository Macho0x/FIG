//! SBE encoders for §17 hot-path stream and batch messages.
//!
//! Template IDs align with FSL-generated `sbe_generated.rs` (schema_id=0x01).

use crate::messages::{
    BalanceEntry, BalanceSnapshot, CandleBar, CandleBarBatch, OrderBookSnapshot, PriceLevel,
    SymbolTicker,
};

const SCHEMA_ID: u16 = 0x01;

fn write_header(buf: &mut Vec<u8>, template_id: u16, block_length: u16) {
    buf.extend_from_slice(&SCHEMA_ID.to_be_bytes());
    buf.extend_from_slice(&template_id.to_be_bytes());
    buf.extend_from_slice(&0u16.to_be_bytes()); // version
    buf.extend_from_slice(&block_length.to_be_bytes());
}

fn write_str(buf: &mut Vec<u8>, s: &str) {
    let b = s.as_bytes();
    buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
    buf.extend_from_slice(b);
}

fn write_f64(buf: &mut Vec<u8>, v: f64) {
    buf.extend_from_slice(&v.to_be_bytes());
}

fn write_i64(buf: &mut Vec<u8>, v: i64) {
    buf.extend_from_slice(&v.to_be_bytes());
}

fn write_bool_opt(buf: &mut Vec<u8>, v: Option<bool>) {
    buf.push(v.unwrap_or(false) as u8);
    buf.push(u8::from(v.is_some()));
}

/// Encode a `CandleBar` (template 21 — wire body for batch items and events).
pub fn encode_candle_bar(bar: &CandleBar) -> Vec<u8> {
    let mut buf = Vec::new();
    write_header(&mut buf, 21, 0);
    write_str(&mut buf, &bar.symbol);
    write_str(&mut buf, &bar.interval);
    write_f64(&mut buf, bar.open.0);
    write_f64(&mut buf, bar.high.0);
    write_f64(&mut buf, bar.low.0);
    write_f64(&mut buf, bar.close.0);
    write_f64(&mut buf, bar.volume.0);
    write_i64(&mut buf, bar.bar_start);
    write_i64(&mut buf, bar.bar_end);
    buf.push(bar.is_final as u8);
    write_bool_opt(&mut buf, bar.is_snapshot);
    buf
}

/// Encode a `CandleBarBatch` (template 22).
pub fn encode_candle_bar_batch(batch: &CandleBarBatch) -> Vec<u8> {
    let mut buf = Vec::new();
    write_header(&mut buf, 22, 1);
    write_str(&mut buf, &batch.symbol);
    write_str(&mut buf, &batch.interval);
    buf.extend_from_slice(&(batch.bars.len() as u32).to_be_bytes());
    for bar in &batch.bars {
        let body = encode_candle_bar_body(bar);
        buf.extend_from_slice(&(body.len() as u32).to_be_bytes());
        buf.extend_from_slice(&body);
    }
    buf.push(batch.has_more as u8);
    if let Some(ref cursor) = batch.next_cursor {
        buf.push(1);
        write_str(&mut buf, cursor);
    } else {
        buf.push(0);
    }
    buf
}

fn encode_candle_bar_body(bar: &CandleBar) -> Vec<u8> {
    let mut buf = Vec::new();
    write_str(&mut buf, &bar.symbol);
    write_str(&mut buf, &bar.interval);
    write_f64(&mut buf, bar.open.0);
    write_f64(&mut buf, bar.high.0);
    write_f64(&mut buf, bar.low.0);
    write_f64(&mut buf, bar.close.0);
    write_f64(&mut buf, bar.volume.0);
    write_i64(&mut buf, bar.bar_start);
    write_i64(&mut buf, bar.bar_end);
    buf.push(bar.is_final as u8);
    write_bool_opt(&mut buf, bar.is_snapshot);
    buf
}

/// Encode a `SymbolTicker` (template 27).
pub fn encode_symbol_ticker(ticker: &SymbolTicker) -> Vec<u8> {
    let mut buf = Vec::new();
    write_header(&mut buf, 27, 65);
    write_str(&mut buf, &ticker.symbol);
    write_f64(&mut buf, ticker.last_price.0);
    write_f64(&mut buf, ticker.price_change);
    write_f64(&mut buf, ticker.price_change_pct);
    write_f64(&mut buf, ticker.volume.0);
    write_f64(&mut buf, ticker.high.0);
    write_f64(&mut buf, ticker.low.0);
    write_f64(&mut buf, ticker.open.0);
    write_i64(&mut buf, ticker.timestamp);
    write_bool_opt(&mut buf, ticker.is_snapshot);
    buf
}

fn write_price_level(buf: &mut Vec<u8>, level: &PriceLevel) {
    write_f64(buf, level.price.0);
    write_f64(buf, level.qty.0);
    if let Some(c) = level.order_count {
        buf.push(1);
        buf.extend_from_slice(&c.to_be_bytes());
    } else {
        buf.push(0);
    }
}

/// Encode an `OrderBookSnapshot` (template 9).
pub fn encode_order_book_snapshot(snap: &OrderBookSnapshot) -> Vec<u8> {
    let mut buf = Vec::new();
    write_header(&mut buf, 9, 0);
    write_str(&mut buf, &snap.symbol);
    write_str(&mut buf, &snap.exchange);
    buf.extend_from_slice(&(snap.bids.len() as u32).to_be_bytes());
    for level in &snap.bids {
        write_price_level(&mut buf, level);
    }
    buf.extend_from_slice(&(snap.asks.len() as u32).to_be_bytes());
    for level in &snap.asks {
        write_price_level(&mut buf, level);
    }
    write_i64(&mut buf, snap.timestamp);
    if let Some(seq) = snap.sequence {
        buf.push(1);
        buf.extend_from_slice(&seq.to_be_bytes());
    } else {
        buf.push(0);
    }
    write_bool_opt(&mut buf, snap.is_snapshot);
    buf
}

fn write_balance_entry(buf: &mut Vec<u8>, entry: &BalanceEntry) {
    write_str(buf, &entry.asset);
    write_f64(buf, entry.total);
    write_f64(buf, entry.available);
    write_f64(buf, entry.hold);
}

/// Encode a `BalanceSnapshot` (template 30).
pub fn encode_balance_snapshot(snap: &BalanceSnapshot) -> Vec<u8> {
    let mut buf = Vec::new();
    write_header(&mut buf, 30, 0);
    write_str(&mut buf, &snap.account);
    buf.extend_from_slice(&(snap.balances.len() as u32).to_be_bytes());
    for entry in &snap.balances {
        write_balance_entry(&mut buf, entry);
    }
    write_bool_opt(&mut buf, snap.is_snapshot);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{Price, Quantity};

    #[test]
    fn candle_bar_batch_round_trip_header() {
        let batch = CandleBarBatch {
            symbol: "AAPL".to_string(),
            interval: "5m".to_string(),
            bars: vec![CandleBar {
                symbol: "AAPL".to_string(),
                interval: "5m".to_string(),
                open: Price(150.0),
                high: Price(151.0),
                low: Price(149.5),
                close: Price(150.5),
                volume: Quantity(1000.0),
                bar_start: 1_700_000_000_000_000_000,
                bar_end: 1_700_000_300_000_000_000,
                is_final: true,
                is_snapshot: Some(true),
            }],
            has_more: false,
            next_cursor: None,
        };
        let encoded = encode_candle_bar_batch(&batch);
        assert_eq!(u16::from_be_bytes([encoded[0], encoded[1]]), SCHEMA_ID);
        assert_eq!(u16::from_be_bytes([encoded[2], encoded[3]]), 22);
    }
}
