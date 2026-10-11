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

/// Venue-wide session plus per-symbol security status and price collars.
#[derive(Debug, Clone)]
pub struct MarketStateHub {
    session_id: String,
    session_status: TradingSessionStatusStatus,
    halt_reason: Option<TradingSessionStatusHaltReason>,
    securities: HashMap<String, SecurityStatus>,
    price_limits: HashMap<String, PriceLimits>,
}

impl Default for MarketStateHub {
    fn default() -> Self {
        Self {
            session_id: "REG".to_string(),
            session_status: TradingSessionStatusStatus::Open,
            halt_reason: None,
            securities: HashMap::new(),
            price_limits: HashMap::new(),
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

    pub fn price_limits(&self, symbol: &str) -> PriceLimits {
        self.price_limits
            .get(symbol)
            .cloned()
            .unwrap_or_else(|| PriceLimits {
                symbol: symbol.to_string(),
                lower_limit: None,
                upper_limit: None,
                reference_price: None,
                timestamp: now_ns(),
                is_snapshot: Some(true),
            })
    }

    pub fn set_price_limits(
        &mut self,
        symbol: &str,
        lower: Option<f64>,
        upper: Option<f64>,
        reference: Option<f64>,
    ) {
        self.price_limits.insert(
            symbol.to_string(),
            PriceLimits {
                symbol: symbol.to_string(),
                lower_limit: lower.map(Price),
                upper_limit: upper.map(Price),
                reference_price: reference.map(Price),
                timestamp: now_ns(),
                is_snapshot: Some(true),
            },
        );
    }

    /// Limit/stop prices must sit inside the collar when one is configured.
    /// Market orders with no price are not collared here.
    pub fn price_allowed(
        &self,
        symbol: &str,
        price: Option<f64>,
        stop_price: Option<f64>,
    ) -> Result<(), &'static str> {
        let Some(lim) = self.price_limits.get(symbol) else {
            return Ok(());
        };
        for px in [price, stop_price].into_iter().flatten() {
            if let Some(lo) = lim.lower_limit.as_ref() {
                if px + f64::EPSILON < lo.0 {
                    return Err("PRICE_LIMIT_EXCEEDED");
                }
            }
            if let Some(hi) = lim.upper_limit.as_ref() {
                if px > hi.0 + f64::EPSILON {
                    return Err("PRICE_LIMIT_EXCEEDED");
                }
            }
        }
        Ok(())
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

/// `marketdata/{symbol}/limits`
pub fn parse_price_limits_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() == 3 && parts[0] == "marketdata" && parts[2] == "limits" {
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
        assert_eq!(
            parse_price_limits_path("marketdata/AAPL/limits").as_deref(),
            Some("AAPL")
        );
        assert!(parse_price_limits_path("marketdata/AAPL/status").is_none());
    }

    #[test]
    fn collar_rejects_outside_band() {
        let mut hub = MarketStateHub::default();
        hub.set_price_limits("AAPL", Some(10.0), Some(20.0), Some(15.0));
        assert!(hub.price_allowed("AAPL", Some(15.0), None).is_ok());
        assert_eq!(
            hub.price_allowed("AAPL", Some(9.0), None),
            Err("PRICE_LIMIT_EXCEEDED")
        );
        assert_eq!(
            hub.price_allowed("AAPL", Some(21.0), None),
            Err("PRICE_LIMIT_EXCEEDED")
        );
        assert!(hub.price_allowed("BTC", Some(1.0), None).is_ok());
        assert!(hub.price_allowed("AAPL", None, None).is_ok());
    }
}
