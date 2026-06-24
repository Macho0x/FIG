//! Live open-order book + execution tape merge.

use std::collections::{HashMap, VecDeque};

use fig_core::messages::{ExecutionReport, OpenOrdersSnapshot, OrdStatus};

const DEFAULT_EXEC_CAPACITY: usize = 256;

fn is_open(status: OrdStatus) -> bool {
    matches!(
        status,
        OrdStatus::New
            | OrdStatus::PartiallyFilled
            | OrdStatus::PendingNew
            | OrdStatus::PendingCancel
    )
}

#[derive(Debug, Clone)]
pub struct OrdersState {
    pub account: String,
    open: HashMap<String, ExecutionReport>,
    executions: VecDeque<ExecutionReport>,
    exec_capacity: usize,
}

impl Default for OrdersState {
    fn default() -> Self {
        Self::with_exec_capacity(DEFAULT_EXEC_CAPACITY)
    }
}

impl OrdersState {
    pub fn with_exec_capacity(exec_capacity: usize) -> Self {
        Self {
            account: String::new(),
            open: HashMap::new(),
            executions: VecDeque::new(),
            exec_capacity: exec_capacity.max(1),
        }
    }

    pub fn apply_open_orders_snapshot(&mut self, snap: &OpenOrdersSnapshot) {
        self.account = snap.account.clone();
        if snap.is_snapshot.unwrap_or(true) {
            self.open.clear();
        }
        for order in &snap.orders {
            self.upsert_open(order);
        }
    }

    pub fn apply_execution_report(&mut self, report: &ExecutionReport) {
        if is_open(report.ord_status) {
            self.upsert_open(report);
        } else {
            self.open.remove(&report.order_id);
        }
        if self.executions.len() >= self.exec_capacity {
            self.executions.pop_front();
        }
        self.executions.push_back(report.clone());
    }

    fn upsert_open(&mut self, report: &ExecutionReport) {
        self.open.insert(report.order_id.clone(), report.clone());
    }

    pub fn open_orders(&self) -> Vec<&ExecutionReport> {
        let mut v: Vec<_> = self.open.values().collect();
        v.sort_by(|a, b| a.cl_ord_id.cmp(&b.cl_ord_id));
        v
    }

    pub fn open_count(&self) -> usize {
        self.open.len()
    }

    pub fn executions(&self) -> &VecDeque<ExecutionReport> {
        &self.executions
    }

    pub fn latest_execution(&self) -> Option<&ExecutionReport> {
        self.executions.back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::{ExecType, Price, Quantity, Side};

    fn report(order_id: &str, status: OrdStatus, exec_type: ExecType) -> ExecutionReport {
        ExecutionReport {
            cl_ord_id: format!("c-{order_id}"),
            order_id: order_id.into(),
            exec_id: format!("e-{order_id}"),
            exec_type,
            ord_status: status,
            side: Side::Buy,
            last_qty: Some(Quantity(10.0)),
            last_price: Some(Price(50.0)),
            leaves_qty: Quantity(0.0),
            cum_qty: Quantity(10.0),
            avg_price: Price(50.0),
            symbol: "AAPL".into(),
            transact_time: 0,
        }
    }

    #[test]
    fn snapshot_then_fill_removes_open() {
        let mut state = OrdersState::default();
        state.apply_open_orders_snapshot(&OpenOrdersSnapshot {
            account: "A".into(),
            orders: vec![report("1", OrdStatus::New, ExecType::New)],
            is_snapshot: Some(true),
        });
        assert_eq!(state.open_count(), 1);
        state.apply_execution_report(&report("1", OrdStatus::Filled, ExecType::Fill));
        assert_eq!(state.open_count(), 0);
        assert_eq!(state.executions().len(), 1);
    }
}
