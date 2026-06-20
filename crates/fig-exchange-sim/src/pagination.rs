//! Shared pagination helpers for historical batch queries (§17.4b).

use fig_core::messages::{CandleBar, ExecutionReport, PublicTrade};

/// Paginate a sorted slice using an opaque cursor (exclusive start key).
pub fn paginate<T, F>(
    items: Vec<T>,
    limit: usize,
    cursor: Option<&str>,
    key: F,
) -> (Vec<T>, bool, Option<String>)
where
    T: Clone,
    F: Fn(&T) -> String,
{
    let mut slice = items;
    if let Some(c) = cursor {
        if let Some(pos) = slice.iter().position(|i| key(i) == c) {
            slice = slice.split_off(pos + 1);
        }
    }
    let has_more = slice.len() > limit;
    let page: Vec<T> = slice.into_iter().take(limit).collect();
    let next_cursor = if has_more {
        page.last().map(|i| key(i))
    } else {
        None
    };
    (page, has_more, next_cursor)
}

pub fn exec_cursor(report: &ExecutionReport) -> String {
    report.exec_id.clone()
}

pub fn candle_cursor(bar: &CandleBar) -> String {
    format!("{}:{}", bar.bar_end, bar.symbol)
}

pub fn trade_cursor(trade: &PublicTrade) -> String {
    format!("{}:{}", trade.timestamp, trade.trade_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::{ExecType, OrdStatus, Price, Quantity, Side};

    fn sample_report(id: &str) -> ExecutionReport {
        ExecutionReport {
            cl_ord_id: id.to_string(),
            order_id: id.to_string(),
            exec_id: id.to_string(),
            exec_type: ExecType::Fill,
            ord_status: OrdStatus::Filled,
            side: Side::Buy,
            last_qty: Some(Quantity(1.0)),
            last_price: Some(Price(1.0)),
            leaves_qty: Quantity(0.0),
            cum_qty: Quantity(1.0),
            avg_price: Price(1.0),
            symbol: "AAPL".to_string(),
            transact_time: 1,
        }
    }

    #[test]
    fn paginate_returns_next_cursor() {
        let items: Vec<_> = (0..5).map(|i| sample_report(&format!("E{i}"))).collect();
        let (page, has_more, next) = paginate(items, 2, None, exec_cursor);
        assert_eq!(page.len(), 2);
        assert!(has_more);
        assert_eq!(next.as_deref(), Some("E1"));

        let all: Vec<_> = (0..5).map(|i| sample_report(&format!("E{i}"))).collect();
        let (page2, has_more2, next2) = paginate(all, 2, next.as_deref(), exec_cursor);
        assert_eq!(page2.len(), 2);
        assert_eq!(page2[0].exec_id, "E2");
        assert!(has_more2);
        assert_eq!(next2.as_deref(), Some("E3"));
    }
}
