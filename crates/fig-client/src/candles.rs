//! Candle bar merge (partial + final bars).

use fig_core::messages::{CandleBar, CandleBarEvent};

#[derive(Debug, Clone, Default)]
pub struct CandleState {
    pub symbol: String,
    pub interval: String,
    pub bar: Option<CandleBar>,
}

impl CandleState {
    pub fn apply_event(&mut self, event: &CandleBarEvent) {
        self.symbol = event.bar.symbol.clone();
        self.interval = event.bar.interval.clone();
        self.bar = Some(event.bar.clone());
    }

    pub fn current(&self) -> Option<&CandleBar> {
        self.bar.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::{Price, Quantity};

    #[test]
    fn updates_bar() {
        let mut state = CandleState::default();
        state.apply_event(&CandleBarEvent {
            bar: CandleBar {
                symbol: "BTC".into(),
                interval: "5m".into(),
                open: Price(1.0),
                high: Price(2.0),
                low: Price(0.5),
                close: Price(1.5),
                volume: Quantity(100.0),
                bar_start: 0,
                bar_end: 300,
                is_final: false,
                is_snapshot: None,
            },
        });
        assert_eq!(state.current().unwrap().close.0, 1.5);
    }
}
