# FIG FSL schemas

The trading schema is split into three logical files merged at codegen time
(`cargo xtask codegen`):

| File | Domain |
|---|---|
| [`orders.fsl`](orders.fsl) | Order entry, execution, FIX/REST gateway mappings |
| [`marketdata.fsl`](marketdata.fsl) | Public market data (book, candles, trades, ticker) |
| [`account.fsl`](account.fsl) | Private account streams and historical queries |

All fragments share well-known schema id `0x01` (`trading.orders`). Generated
Rust lives in `crates/fig-core/src/generated/`.
