//! Simulated account balances, positions, and fill history.

use std::collections::HashMap;

use fig_core::messages::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountSubscriptionKind {
    Executions,
    Balances,
    Positions,
    Funding,
    Ledger,
}

#[derive(Debug, Clone)]
pub struct AccountSubscription {
    pub channel_id: u16,
    pub routing_key: String,
    pub account: String,
    pub kind: AccountSubscriptionKind,
}

#[derive(Debug, Clone)]
pub struct SimAccount {
    pub account: String,
    pub balance: f64,
    pub buying_power: f64,
    pub currency: String,
    pub assets: HashMap<String, BalanceEntry>,
    pub positions: HashMap<String, PositionEntry>,
    pub fills: Vec<ExecutionReport>,
    pub funding: Vec<FundingPayment>,
    pub ledger: Vec<LedgerUpdate>,
}

impl SimAccount {
    pub fn demo(account: &str) -> Self {
        let mut assets = HashMap::new();
        assets.insert(
            "USD".to_string(),
            BalanceEntry {
                asset: "USD".to_string(),
                total: 1_000_000.0,
                available: 2_000_000.0,
                hold: 0.0,
            },
        );
        Self {
            account: account.to_string(),
            balance: 1_000_000.0,
            buying_power: 2_000_000.0,
            currency: "USD".to_string(),
            assets,
            positions: HashMap::new(),
            fills: Vec::new(),
            funding: Vec::new(),
            ledger: Vec::new(),
        }
    }

    pub fn account_summary(&self) -> AccountSummary {
        AccountSummary {
            account: self.account.clone(),
            balance: self.balance,
            buying_power: self.buying_power,
            currency: self.currency.clone(),
        }
    }

    pub fn margin_summary(&self) -> MarginSummary {
        MarginSummary {
            account: self.account.clone(),
            balance: self.balance,
            buying_power: self.buying_power,
            equity: self.balance,
            margin_used: 0.0,
            available: self.buying_power,
            currency: self.currency.clone(),
        }
    }

    pub fn balance_snapshot(&self, is_snapshot: bool) -> BalanceSnapshot {
        BalanceSnapshot {
            account: self.account.clone(),
            balances: self.assets.values().cloned().collect(),
            is_snapshot: Some(is_snapshot),
        }
    }

    pub fn position_snapshot(&self, is_snapshot: bool) -> PositionSnapshot {
        PositionSnapshot {
            account: self.account.clone(),
            positions: self.positions.values().cloned().collect(),
            is_snapshot: Some(is_snapshot),
        }
    }

    pub fn record_fill(&mut self, report: ExecutionReport) -> BalanceUpdate {
        self.fills.push(report.clone());
        let delta = report
            .last_qty
            .as_ref()
            .map(|q| q.0 * report.last_price.as_ref().map(|p| p.0).unwrap_or(0.0))
            .unwrap_or(0.0)
            * match report.side {
                Side::Buy => -1.0,
                _ => 1.0,
            };
        self.balance += delta;
        self.buying_power += delta;
        if let Some(entry) = self.assets.get_mut("USD") {
            entry.total = self.balance;
            entry.available = self.buying_power;
        }
        BalanceUpdate {
            account: self.account.clone(),
            asset: "USD".to_string(),
            delta,
            total: self.balance,
            available: self.buying_power,
            reason: BalanceUpdateReason::Trade,
        }
    }

    pub fn record_funding(&mut self, payment: FundingPayment) {
        self.funding.push(payment);
    }

    pub fn record_ledger(&mut self, entry: LedgerUpdate) {
        self.balance += entry.delta;
        self.buying_power += entry.delta;
        if let Some(asset_entry) = self.assets.get_mut(&entry.asset) {
            asset_entry.total += entry.delta;
            asset_entry.available += entry.delta;
        }
        self.ledger.push(entry);
    }

    pub fn query_funding(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> FundingHistoryBatch {
        let mut payments: Vec<FundingPayment> = self
            .funding
            .iter()
            .filter(|p| start_time.is_none_or(|st| p.timestamp >= st))
            .filter(|p| end_time.is_none_or(|et| p.timestamp <= et))
            .cloned()
            .collect();
        let limit = limit.unwrap_or(500) as usize;
        let has_more = payments.len() > limit;
        payments.truncate(limit);
        FundingHistoryBatch {
            account: self.account.clone(),
            payments,
            has_more,
            next_cursor: None,
        }
    }

    pub fn query_ledger(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> LedgerHistoryBatch {
        let mut entries: Vec<LedgerUpdate> = self
            .ledger
            .iter()
            .filter(|e| start_time.is_none_or(|st| e.timestamp >= st))
            .filter(|e| end_time.is_none_or(|et| e.timestamp <= et))
            .cloned()
            .collect();
        let limit = limit.unwrap_or(500) as usize;
        let has_more = entries.len() > limit;
        entries.truncate(limit);
        LedgerHistoryBatch {
            account: self.account.clone(),
            entries,
            has_more,
            next_cursor: None,
        }
    }

    pub fn query_fills(
        &self,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> FillHistoryBatch {
        let mut fills: Vec<ExecutionReport> = self
            .fills
            .iter()
            .filter(|f| symbol.is_none_or(|s| f.symbol == s))
            .filter(|f| start_time.is_none_or(|st| f.transact_time >= st))
            .filter(|f| end_time.is_none_or(|et| f.transact_time <= et))
            .cloned()
            .collect();
        let limit = limit.unwrap_or(500) as usize;
        let has_more = fills.len() > limit;
        fills.truncate(limit);
        FillHistoryBatch {
            account: self.account.clone(),
            fills,
            has_more,
            next_cursor: None,
        }
    }
}

#[derive(Default)]
pub struct AccountHub {
    accounts: HashMap<String, SimAccount>,
}

impl AccountHub {
    pub fn get_or_create(&mut self, account: &str) -> &mut SimAccount {
        self.accounts
            .entry(account.to_string())
            .or_insert_with(|| SimAccount::demo(account))
    }

    pub fn get(&self, account: &str) -> Option<&SimAccount> {
        self.accounts.get(account)
    }
}

pub fn parse_account_subscription(routing_key: &str, channel_path: &str) -> Option<(String, AccountSubscriptionKind)> {
    let path = if !channel_path.is_empty() {
        channel_path.to_string()
    } else {
        routing_key.replace('.', "/")
    };
    if path.starts_with("trading/accounts/") && path.ends_with("/executions") {
        let account = path.split('/').nth(2)?.to_string();
        return Some((account, AccountSubscriptionKind::Executions));
    }
    if path.starts_with("accounts/") && path.ends_with("/balances") {
        let account = path.split('/').nth(1)?.to_string();
        return Some((account, AccountSubscriptionKind::Balances));
    }
    if path.starts_with("accounts/") && path.ends_with("/positions") {
        let account = path.split('/').nth(1)?.to_string();
        return Some((account, AccountSubscriptionKind::Positions));
    }
    if path.starts_with("accounts/") && path.ends_with("/funding") {
        let account = path.split('/').nth(1)?.to_string();
        return Some((account, AccountSubscriptionKind::Funding));
    }
    if path.starts_with("accounts/") && path.ends_with("/ledger") {
        let account = path.split('/').nth(1)?.to_string();
        return Some((account, AccountSubscriptionKind::Ledger));
    }
    None
}

pub fn account_from_path(path: &str) -> Option<String> {
    if path.starts_with("accounts/") {
        return path.split('/').nth(1).map(str::to_string);
    }
    if path.contains("/accounts/") {
        return path
            .split('/')
            .find(|s| *s != "trading" && *s != "accounts" && !s.is_empty())
            .map(str::to_string);
    }
    None
}
