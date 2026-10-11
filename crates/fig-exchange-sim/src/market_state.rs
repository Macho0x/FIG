//! Trading-session and security status for the reference broker.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use fig_core::messages::*;

fn now_ns() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64
}

/// Venue-wide session plus per-symbol security status.
#[derive(Debug, Clone)]
pub struct MarketStateHub {
    session_id: String,
    session_status: TradingSessionStatusStatus,
    halt_reason: Option<TradingSessionStatusHaltReason>,
    securities: HashMap<String, SecurityStatus>,
}

impl Default for MarketStateHub {
    fn default() -> Self {
        Self {
            session_id: "REG".to_string(),
            session_status: TradingSessionStatusStatus::Open,
            halt_reason: None,
            securities: HashMap::new(),
        }
    }
}

impl MarketStateHub {
    pub fn session_status(&self, symbol: Option<&str>) -> TradingSessionStatus {
        TradingSessionStatus {
            symbol: symbol.map(str::to_string),
            trading_session_id: self.session_id.clone(),
            status: self.session_status,
            halt_reason: self.halt_reason,
            timestamp: now_ns(),
            is_snapshot: Some(true),
        }
    }

    pub fn security_status(&self, symbol: &str) -> SecurityStatus {
        self.securities
            .get(symbol)
            .cloned()
            .unwrap_or_else(|| SecurityStatus {
                symbol: symbol.to_string(),
                status: SecurityStatusStatus::Trading,
                halt_reason: None,
                timestamp: now_ns(),
                is_snapshot: Some(true),
            })
    }

    pub fn set_session(
        &mut self,
        status: TradingSessionStatusStatus,
        halt_reason: Option<TradingSessionStatusHaltReason>,
    ) {
        self.session_status = status;
        self.halt_reason = halt_reason;
    }

    pub fn set_security(
        &mut self,
        symbol: &str,
        status: SecurityStatusStatus,
        halt_reason: Option<SecurityStatusHaltReason>,
    ) {
        self.securities.insert(
            symbol.to_string(),
            SecurityStatus {
                symbol: symbol.to_string(),
                status,
                halt_reason,
                timestamp: now_ns(),
                is_snapshot: Some(true),
            },
        );
    }

    /// Whether the matcher may accept a new order for `symbol`.
    pub fn orders_allowed(&self, symbol: &str) -> Result<(), &'static str> {
        match self.session_status {
            TradingSessionStatusStatus::Open | TradingSessionStatusStatus::Auction => {}
            TradingSessionStatusStatus::Halt => return Err("SESSION_HALTED"),
            TradingSessionStatusStatus::PreOpen => return Err("MARKET_NOT_OPEN"),
            TradingSessionStatusStatus::Break => return Err("SESSION_BREAK"),
            TradingSessionStatusStatus::Close => return Err("MARKET_CLOSED"),
        }
        match self.securities.get(symbol).map(|s| s.status) {
            Some(SecurityStatusStatus::Halt) => Err("SECURITY_HALTED"),
            Some(SecurityStatusStatus::Suspended) => Err("SECURITY_SUSPENDED"),
            _ => Ok(()),
        }
    }
}

/// `marketdata/session`
pub fn parse_session_path(path: &str) -> bool {
    path.trim_start_matches('/') == "marketdata/session"
}

/// `marketdata/{symbol}/status`
pub fn parse_security_status_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() == 3 && parts[0] == "marketdata" && parts[2] == "status" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_session_is_open() {
        let hub = MarketStateHub::default();
        let status = hub.session_status(None);
        assert_eq!(status.status, TradingSessionStatusStatus::Open);
        assert_eq!(status.trading_session_id, "REG");
        assert!(hub.orders_allowed("AAPL").is_ok());
    }

    #[test]
    fn halt_rejects_orders() {
        let mut hub = MarketStateHub::default();
        hub.set_session(
            TradingSessionStatusStatus::Halt,
            Some(TradingSessionStatusHaltReason::News),
        );
        assert_eq!(hub.orders_allowed("AAPL"), Err("SESSION_HALTED"));
        hub.set_session(TradingSessionStatusStatus::Open, None);
        hub.set_security(
            "AAPL",
            SecurityStatusStatus::Halt,
            Some(SecurityStatusHaltReason::Regulatory),
        );
        assert_eq!(hub.orders_allowed("AAPL"), Err("SECURITY_HALTED"));
        assert!(hub.orders_allowed("BTC").is_ok());
    }

    #[test]
    fn parse_session_and_status_paths() {
        assert!(parse_session_path("marketdata/session"));
        assert!(parse_session_path("/marketdata/session"));
        assert!(!parse_session_path("marketdata/AAPL/status"));
        assert_eq!(
            parse_security_status_path("marketdata/AAPL/status").as_deref(),
            Some("AAPL")
        );
        assert!(parse_security_status_path("marketdata/session").is_none());
    }
}
