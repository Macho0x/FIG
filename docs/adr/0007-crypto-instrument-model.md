# ADR 0007: Crypto instrument model

## Status

Accepted

## Context

Hyperliquid-shaped venues use long coin symbols, perpetual product metadata, and
margin-asset denomination (e.g. USDC). FIG's original `Symbol` type was limited to
8 uppercase characters (equity-style tickers).

## Decision

1. Extend `Symbol` to **32 characters** (backward compatible for existing symbols).
2. Add FSL types in [`schemas/instruments.fsl`](../../schemas/instruments.fsl):
   - `InstrumentId` (string, e.g. `BTC-PERP`)
   - `InstrumentMetadata` (symbol, product_kind, margin_asset, tick/lot sizes)
   - `InstrumentCatalogRequest` / `InstrumentCatalogResponse`
3. Expose catalog at native path **`.well-known/instruments`** (GET / CBOR).
4. Reference sim registers **BTC** and **ETH** perp metadata with USDC margin.

## Consequences

- Codegen must include `instruments.fsl` in the merge list (`fig-fsl` parser).
- Gateway HL `coin` fields map to FIG `Symbol` (uppercase, max 32).
- Full §20.1 deferred items (EIP-712, TWAP, vaults, chain events) remain out of scope.

## Evidence

- `instrument_catalog_response()` in `fig-exchange-sim`
- `GET /.well-known/instruments` handler in `broker_api.rs`
