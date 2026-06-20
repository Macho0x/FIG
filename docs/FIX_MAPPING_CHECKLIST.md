# FIX 4.4 Mapping Checklist

Field-by-field audit of `schemas/orders.fsl` gateway mappings vs `crates/fig-gateways/src/fix.rs`.

Status key: **Done** | **Partial** | **N/A** (not in FIG schema)

## NewOrderSingle (35=D)

| FIG field | FIX tag | FSL | fix.rs | Notes |
|-----------|---------|-----|--------|-------|
| cl_ord_id | 11 | Done | Done | Required |
| side | 54 | Done | Done | 1/2/5/6 |
| order_qty | 38 | Done | Done | Required |
| price | 44 | Done | Done | Required for Limit/StopLimit |
| stop_price | 99 | Done | Done | Required for Stop/StopLimit |
| symbol | 55 | Done | Done | Required |
| order_type | 40 | Done | Done | 1–4, 5 (MOC), B (LOC), P (Pegged) |
| time_in_force | 59 | Done | Done | 0/1/3/4/6 |
| expire_time | 432 | Done | Done | Required when TIF=GTD |
| account | 1 | Done | Done | Optional |
| security_id | 48 | Done | Done | Optional symbology |
| id_source | 22 | Done | Done | CUSIP/SEDOL/ISIN/RIC/ExchangeSymbol |
| security_exchange | 207 | Done | Done | Optional MIC |
| strategy_id | — | N/A | N/A | No standard FIX tag |

## CancelRequest (35=F)

| FIG field | FIX tag | FSL | fix.rs | Notes |
|-----------|---------|-----|--------|-------|
| cl_ord_id | 11 | Done | Done | Cancel request ID |
| orig_cl_ord_id | 41 | Done | Done | Original order ID |
| symbol | 55 | Done | Done | Required |
| side | 54 | Done | Done | Required |
| order_qty | 38 | Done | Done | Optional partial cancel |

## CancelReplaceRequest (35=G)

| FIG field | FIX tag | FSL | fix.rs | Notes |
|-----------|---------|-----|--------|-------|
| cl_ord_id | 11 | Done | Done | New replace ID |
| orig_cl_ord_id | 41 | Done | Done | Order to replace |
| symbol | 55 | Done | Done | Required |
| side | 54 | Done | Done | Required |
| order_qty | 38 | Done | Done | Required |
| price | 44 | Done | Done | Optional |

## ExecutionReport (35=8)

| FIG field | FIX tag | FSL | fix.rs | Notes |
|-----------|---------|-----|--------|-------|
| cl_ord_id | 11 | Done | Done | |
| order_id | 37 | Done | Done | |
| exec_id | 17 | Done | Done | |
| exec_type | 150 | Done | Done | FIG→FIX only |
| ord_status | 39 | Done | Done | FIG→FIX only |
| side | 54 | Done | Done | |
| last_qty | 32 | Done | Done | Optional |
| last_price | 31 | Done | Done | Optional |
| leaves_qty | 151 | Done | Done | |
| cum_qty | 14 | Done | Done | |
| avg_price | 6 | Done | Done | |
| symbol | 55 | Done | Done | |
| transact_time | 60 | Done | Done | UTCTimestamp format |
| Session header | 49/56/34/52 | Partial | Done | Via `FixOutboundContext` |

## CancelReject (35=9)

| FIG field | FIX tag | FSL | fix.rs | Notes |
|-----------|---------|-----|--------|-------|
| cl_ord_id | 11 | Done | Done | |
| orig_cl_ord_id | 41 | Done | Done | |
| reject_reason | 102 | Done | Done | Mapped to CxlRejReason |
| symbol | 55 | Done | Done | |
| CxlRejResponseTo | 434 | Partial | Done | 1=cancel, 2=replace |
| ord_status | 39 | N/A | Done | Always Rejected (8) |

## BusinessMessageReject (35=j)

| FIG field | FIX tag | FSL | fix.rs | Notes |
|-----------|---------|-----|--------|-------|
| ref_msg_type | 372 | N/A | Done | Referenced application msg type |
| ref_seq_num | 45 | N/A | Done | Referenced MsgSeqNum |
| business_reject_reason | 380 | N/A | Done | Mapped to `BusinessRejectReason` |
| text | 58 | N/A | Done | Human-readable detail |

Gateway sends BMR on FIX→FIG translation failures (`fig-gateway.rs`).

## Session / infrastructure

| Feature | Status | Location |
|---------|--------|----------|
| Logon ↔ STREAM_OPEN | Done | `logon_to_stream_open` |
| Resend ↔ CONTROL | Done | `resend_request_to_control` |
| FixSession state machine | Done | `fix_session.rs` |
| Gateway session loop | Done | `fig-gateway.rs` |
| Message framing | Done | `split_fix_messages` |
| Checksum validate/serialize | Done | `parse_fix_message` / `serialize_fix_message` |
| TLS FIX acceptor | Done | `fix_tls.rs`, `--fix-tls` on `fig-gateway` |
| Multi-node seq persistence | Done | `fix_seq_store.rs`, `FixSession::with_seq_store` |
| BusinessMessageReject on translate errors | Done | `fig-gateway.rs` |

## Matching engine semantics

| Feature | Status | Location |
|---------|--------|----------|
| Market / Limit | Done | `matching.rs` |
| Stop / StopLimit | Done | Pending stop queue + trigger |
| IOC | Done | No resting remainder |
| FOK | Done | All-or-nothing reject |
| GTD expire at entry | Done | Validates `expire_time` |
| CancelReject on failed cancel | Done | `server.rs` + `CancelOutcome` |
| MOC / LOC / Pegged | Done | Mapped to Market/Limit/Limit in `matching.rs` |

## Still out of scope (production FIX venues)

- SecurityDefinition reference data workflow
- Session Reject (35=3) application-level handling beyond ignore
- Drop copy, allocations, locates
