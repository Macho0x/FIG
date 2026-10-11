// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' v1.0.0
// Standard order entry and execution messages

export const SCHEMA_ID = 0x01 as const;

function writeStringU32(buf: number[], s: string): void { const b = new TextEncoder().encode(s); writeU32BE(buf, b.length); for (const x of b) buf.push(x); }
function readStringU32(buf: Uint8Array, pos: { value: number }): string { const n = readU32BE(buf, pos); const s = new TextDecoder().decode(buf.subarray(pos.value, pos.value + n)); pos.value += n; return s; }
function readU8(buf: Uint8Array, pos: { value: number }): number { return buf[pos.value++]; }
function f32Bits(v: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setFloat32(0, v, false); return dv.getUint32(0, false); }
function f32FromBits(bits: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setUint32(0, bits >>> 0, false); return dv.getFloat32(0, false); }
function writeU16BE(buf: number[], v: number): void { buf.push((v >> 8) & 0xff, v & 0xff); }
function writeU32BE(buf: number[], v: number): void { buf.push((v >> 24) & 0xff, (v >> 16) & 0xff, (v >> 8) & 0xff, v & 0xff); }
function writeU64BE(buf: number[], v: bigint): void { for (let i = 7; i >= 0; i--) buf.push(Number((v >> BigInt(i * 8)) & 0xffn)); }
function writeF64BE(buf: number[], v: number): void { const dv = new DataView(new ArrayBuffer(8)); dv.setFloat64(0, v, false); for (let i = 0; i < 8; i++) buf.push(dv.getUint8(i)); }
function writeI64BE(buf: number[], v: bigint): void { writeU64BE(buf, BigInt.asUintN(64, v)); }
function writeString(buf: number[], s: string): void { const bytes = new TextEncoder().encode(s); writeU16BE(buf, bytes.length); for (const b of bytes) buf.push(b); }
function readU8(buf: Uint8Array, pos: { value: number }): number { return buf[pos.value++]; }
function f32Bits(v: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setFloat32(0, v, false); return dv.getUint32(0, false); }
function f32FromBits(bits: number): number { const dv = new DataView(new ArrayBuffer(4)); dv.setUint32(0, bits >>> 0, false); return dv.getFloat32(0, false); }

function readU16BE(buf: Uint8Array, pos: { value: number }): number { const v = (buf[pos.value] << 8) | buf[pos.value + 1]; pos.value += 2; return v; }
function readU32BE(buf: Uint8Array, pos: { value: number }): number { const v = (buf[pos.value] << 24) | (buf[pos.value + 1] << 16) | (buf[pos.value + 2] << 8) | buf[pos.value + 3]; pos.value += 4; return v >>> 0; }
function readU64BE(buf: Uint8Array, pos: { value: number }): bigint { let v = 0n; for (let i = 0; i < 8; i++) v = (v << 8n) | BigInt(buf[pos.value + i]); pos.value += 8; return v; }
function readF64BE(buf: Uint8Array, pos: { value: number }): number { const dv = new DataView(buf.buffer, buf.byteOffset + pos.value, 8); const v = dv.getFloat64(0, false); pos.value += 8; return v; }
function readI64BE(buf: Uint8Array, pos: { value: number }): bigint { return readU64BE(buf, pos); }
function readString(buf: Uint8Array, pos: { value: number }): string { const len = readU16BE(buf, pos); const s = new TextDecoder().decode(buf.subarray(pos.value, pos.value + len)); pos.value += len; return s; }

export enum NewOrderSingleSide {
  Buy = 1,
  Sell = 2,
  SellShort = 3,
  SellShortExempt = 4,
}

export function NewOrderSingleSideFromValue(v: number): NewOrderSingleSide | null {
  switch (v) {
    case 1: return NewOrderSingleSide.Buy;
    case 2: return NewOrderSingleSide.Sell;
    case 3: return NewOrderSingleSide.SellShort;
    case 4: return NewOrderSingleSide.SellShortExempt;
    default: return null;
  }
}

export function NewOrderSingleSideToValue(e: NewOrderSingleSide): number { return e; }

export enum NewOrderSingleOrderType {
  Market = 1,
  Limit = 2,
  Stop = 3,
  StopLimit = 4,
  MarketOnClose = 5,
  LimitOnClose = 6,
  Pegged = 7,
}

export function NewOrderSingleOrderTypeFromValue(v: number): NewOrderSingleOrderType | null {
  switch (v) {
    case 1: return NewOrderSingleOrderType.Market;
    case 2: return NewOrderSingleOrderType.Limit;
    case 3: return NewOrderSingleOrderType.Stop;
    case 4: return NewOrderSingleOrderType.StopLimit;
    case 5: return NewOrderSingleOrderType.MarketOnClose;
    case 6: return NewOrderSingleOrderType.LimitOnClose;
    case 7: return NewOrderSingleOrderType.Pegged;
    default: return null;
  }
}

export function NewOrderSingleOrderTypeToValue(e: NewOrderSingleOrderType): number { return e; }

export enum NewOrderSingleTimeInForce {
  Day = 1,
  Gtc = 2,
  Ioc = 3,
  Fok = 4,
  Gtd = 5,
}

export function NewOrderSingleTimeInForceFromValue(v: number): NewOrderSingleTimeInForce | null {
  switch (v) {
    case 1: return NewOrderSingleTimeInForce.Day;
    case 2: return NewOrderSingleTimeInForce.Gtc;
    case 3: return NewOrderSingleTimeInForce.Ioc;
    case 4: return NewOrderSingleTimeInForce.Fok;
    case 5: return NewOrderSingleTimeInForce.Gtd;
    default: return null;
  }
}

export function NewOrderSingleTimeInForceToValue(e: NewOrderSingleTimeInForce): number { return e; }

export enum NewOrderSingleIdSource {
  Cusip = 1,
  Sedol = 2,
  Isin = 3,
  Ric = 4,
  ExchangeSymbol = 5,
}

export function NewOrderSingleIdSourceFromValue(v: number): NewOrderSingleIdSource | null {
  switch (v) {
    case 1: return NewOrderSingleIdSource.Cusip;
    case 2: return NewOrderSingleIdSource.Sedol;
    case 3: return NewOrderSingleIdSource.Isin;
    case 4: return NewOrderSingleIdSource.Ric;
    case 5: return NewOrderSingleIdSource.ExchangeSymbol;
    default: return null;
  }
}

export function NewOrderSingleIdSourceToValue(e: NewOrderSingleIdSource): number { return e; }

export enum CancelRequestSide {
  Buy = 1,
  Sell = 2,
  SellShort = 3,
}

export function CancelRequestSideFromValue(v: number): CancelRequestSide | null {
  switch (v) {
    case 1: return CancelRequestSide.Buy;
    case 2: return CancelRequestSide.Sell;
    case 3: return CancelRequestSide.SellShort;
    default: return null;
  }
}

export function CancelRequestSideToValue(e: CancelRequestSide): number { return e; }

export enum CancelReplaceRequestSide {
  Buy = 1,
  Sell = 2,
  SellShort = 3,
}

export function CancelReplaceRequestSideFromValue(v: number): CancelReplaceRequestSide | null {
  switch (v) {
    case 1: return CancelReplaceRequestSide.Buy;
    case 2: return CancelReplaceRequestSide.Sell;
    case 3: return CancelReplaceRequestSide.SellShort;
    default: return null;
  }
}

export function CancelReplaceRequestSideToValue(e: CancelReplaceRequestSide): number { return e; }

export enum ExecutionReportExecType {
  New = 1,
  PartialFill = 2,
  Fill = 3,
  DoneForDay = 4,
  Canceled = 5,
  Replaced = 6,
  PendingCancel = 7,
  Stopped = 8,
  Rejected = 9,
  Suspended = 10,
  PendingNew = 11,
  Expired = 12,
}

export function ExecutionReportExecTypeFromValue(v: number): ExecutionReportExecType | null {
  switch (v) {
    case 1: return ExecutionReportExecType.New;
    case 2: return ExecutionReportExecType.PartialFill;
    case 3: return ExecutionReportExecType.Fill;
    case 4: return ExecutionReportExecType.DoneForDay;
    case 5: return ExecutionReportExecType.Canceled;
    case 6: return ExecutionReportExecType.Replaced;
    case 7: return ExecutionReportExecType.PendingCancel;
    case 8: return ExecutionReportExecType.Stopped;
    case 9: return ExecutionReportExecType.Rejected;
    case 10: return ExecutionReportExecType.Suspended;
    case 11: return ExecutionReportExecType.PendingNew;
    case 12: return ExecutionReportExecType.Expired;
    default: return null;
  }
}

export function ExecutionReportExecTypeToValue(e: ExecutionReportExecType): number { return e; }

export enum ExecutionReportOrdStatus {
  New = 1,
  PartiallyFilled = 2,
  Filled = 3,
  DoneForDay = 4,
  Canceled = 5,
  PendingCancel = 6,
  Stopped = 7,
  Rejected = 8,
  Suspended = 9,
  PendingNew = 10,
  Expired = 11,
  Replaced = 12,
}

export function ExecutionReportOrdStatusFromValue(v: number): ExecutionReportOrdStatus | null {
  switch (v) {
    case 1: return ExecutionReportOrdStatus.New;
    case 2: return ExecutionReportOrdStatus.PartiallyFilled;
    case 3: return ExecutionReportOrdStatus.Filled;
    case 4: return ExecutionReportOrdStatus.DoneForDay;
    case 5: return ExecutionReportOrdStatus.Canceled;
    case 6: return ExecutionReportOrdStatus.PendingCancel;
    case 7: return ExecutionReportOrdStatus.Stopped;
    case 8: return ExecutionReportOrdStatus.Rejected;
    case 9: return ExecutionReportOrdStatus.Suspended;
    case 10: return ExecutionReportOrdStatus.PendingNew;
    case 11: return ExecutionReportOrdStatus.Expired;
    case 12: return ExecutionReportOrdStatus.Replaced;
    default: return null;
  }
}

export function ExecutionReportOrdStatusToValue(e: ExecutionReportOrdStatus): number { return e; }

export enum ExecutionReportSide {
  Buy = 1,
  Sell = 2,
  SellShort = 3,
}

export function ExecutionReportSideFromValue(v: number): ExecutionReportSide | null {
  switch (v) {
    case 1: return ExecutionReportSide.Buy;
    case 2: return ExecutionReportSide.Sell;
    case 3: return ExecutionReportSide.SellShort;
    default: return null;
  }
}

export function ExecutionReportSideToValue(e: ExecutionReportSide): number { return e; }

export enum CancelRejectRejectReason {
  OrderNotFound = 1,
  AlreadyCanceled = 2,
  AlreadyFilled = 3,
  TooLateToCancel = 4,
}

export function CancelRejectRejectReasonFromValue(v: number): CancelRejectRejectReason | null {
  switch (v) {
    case 1: return CancelRejectRejectReason.OrderNotFound;
    case 2: return CancelRejectRejectReason.AlreadyCanceled;
    case 3: return CancelRejectRejectReason.AlreadyFilled;
    case 4: return CancelRejectRejectReason.TooLateToCancel;
    default: return null;
  }
}

export function CancelRejectRejectReasonToValue(e: CancelRejectRejectReason): number { return e; }

export enum BalanceUpdateReason {
  Trade = 1,
  Deposit = 2,
  Withdrawal = 3,
  Transfer = 4,
  Fee = 5,
}

export function BalanceUpdateReasonFromValue(v: number): BalanceUpdateReason | null {
  switch (v) {
    case 1: return BalanceUpdateReason.Trade;
    case 2: return BalanceUpdateReason.Deposit;
    case 3: return BalanceUpdateReason.Withdrawal;
    case 4: return BalanceUpdateReason.Transfer;
    case 5: return BalanceUpdateReason.Fee;
    default: return null;
  }
}

export function BalanceUpdateReasonToValue(e: BalanceUpdateReason): number { return e; }

export enum OrderListStatusStatus {
  Executing = 1,
  AllDone = 2,
  Reject = 3,
}

export function OrderListStatusStatusFromValue(v: number): OrderListStatusStatus | null {
  switch (v) {
    case 1: return OrderListStatusStatus.Executing;
    case 2: return OrderListStatusStatus.AllDone;
    case 3: return OrderListStatusStatus.Reject;
    default: return null;
  }
}

export function OrderListStatusStatusToValue(e: OrderListStatusStatus): number { return e; }

export enum LedgerUpdateKind {
  Deposit = 1,
  Withdrawal = 2,
  Transfer = 3,
  Fee = 4,
  Funding = 5,
}

export function LedgerUpdateKindFromValue(v: number): LedgerUpdateKind | null {
  switch (v) {
    case 1: return LedgerUpdateKind.Deposit;
    case 2: return LedgerUpdateKind.Withdrawal;
    case 3: return LedgerUpdateKind.Transfer;
    case 4: return LedgerUpdateKind.Fee;
    case 5: return LedgerUpdateKind.Funding;
    default: return null;
  }
}

export function LedgerUpdateKindToValue(e: LedgerUpdateKind): number { return e; }

export enum TradingSessionStatusStatus {
  PreOpen = 1,
  Open = 2,
  Break = 3,
  Close = 4,
  Halt = 5,
  Auction = 6,
}

export function TradingSessionStatusStatusFromValue(v: number): TradingSessionStatusStatus | null {
  switch (v) {
    case 1: return TradingSessionStatusStatus.PreOpen;
    case 2: return TradingSessionStatusStatus.Open;
    case 3: return TradingSessionStatusStatus.Break;
    case 4: return TradingSessionStatusStatus.Close;
    case 5: return TradingSessionStatusStatus.Halt;
    case 6: return TradingSessionStatusStatus.Auction;
    default: return null;
  }
}

export function TradingSessionStatusStatusToValue(e: TradingSessionStatusStatus): number { return e; }

export enum TradingSessionStatusHaltReason {
  None = 1,
  News = 2,
  MatchingEngine = 3,
  Regulatory = 4,
  Auction = 5,
  Other = 6,
}

export function TradingSessionStatusHaltReasonFromValue(v: number): TradingSessionStatusHaltReason | null {
  switch (v) {
    case 1: return TradingSessionStatusHaltReason.None;
    case 2: return TradingSessionStatusHaltReason.News;
    case 3: return TradingSessionStatusHaltReason.MatchingEngine;
    case 4: return TradingSessionStatusHaltReason.Regulatory;
    case 5: return TradingSessionStatusHaltReason.Auction;
    case 6: return TradingSessionStatusHaltReason.Other;
    default: return null;
  }
}

export function TradingSessionStatusHaltReasonToValue(e: TradingSessionStatusHaltReason): number { return e; }

export enum SecurityStatusStatus {
  Trading = 1,
  Halt = 2,
  Auction = 3,
  Suspended = 4,
}

export function SecurityStatusStatusFromValue(v: number): SecurityStatusStatus | null {
  switch (v) {
    case 1: return SecurityStatusStatus.Trading;
    case 2: return SecurityStatusStatus.Halt;
    case 3: return SecurityStatusStatus.Auction;
    case 4: return SecurityStatusStatus.Suspended;
    default: return null;
  }
}

export function SecurityStatusStatusToValue(e: SecurityStatusStatus): number { return e; }

export enum SecurityStatusHaltReason {
  None = 1,
  News = 2,
  MatchingEngine = 3,
  Regulatory = 4,
  Auction = 5,
  Other = 6,
}

export function SecurityStatusHaltReasonFromValue(v: number): SecurityStatusHaltReason | null {
  switch (v) {
    case 1: return SecurityStatusHaltReason.None;
    case 2: return SecurityStatusHaltReason.News;
    case 3: return SecurityStatusHaltReason.MatchingEngine;
    case 4: return SecurityStatusHaltReason.Regulatory;
    case 5: return SecurityStatusHaltReason.Auction;
    case 6: return SecurityStatusHaltReason.Other;
    default: return null;
  }
}

export function SecurityStatusHaltReasonToValue(e: SecurityStatusHaltReason): number { return e; }

export enum MassCancelRequestSide {
  Buy = 1,
  Sell = 2,
  SellShort = 3,
  SellShortExempt = 4,
}

export function MassCancelRequestSideFromValue(v: number): MassCancelRequestSide | null {
  switch (v) {
    case 1: return MassCancelRequestSide.Buy;
    case 2: return MassCancelRequestSide.Sell;
    case 3: return MassCancelRequestSide.SellShort;
    case 4: return MassCancelRequestSide.SellShortExempt;
    default: return null;
  }
}

export function MassCancelRequestSideToValue(e: MassCancelRequestSide): number { return e; }

export enum MassCancelReportStatus {
  Accepted = 1,
  Completed = 2,
  Rejected = 3,
}

export function MassCancelReportStatusFromValue(v: number): MassCancelReportStatus | null {
  switch (v) {
    case 1: return MassCancelReportStatus.Accepted;
    case 2: return MassCancelReportStatus.Completed;
    case 3: return MassCancelReportStatus.Rejected;
    default: return null;
  }
}

export function MassCancelReportStatusToValue(e: MassCancelReportStatus): number { return e; }

export enum AggregateTradeSide {
  Buy = 1,
  Sell = 2,
}

export function AggregateTradeSideFromValue(v: number): AggregateTradeSide | null {
  switch (v) {
    case 1: return AggregateTradeSide.Buy;
    case 2: return AggregateTradeSide.Sell;
    default: return null;
  }
}

export function AggregateTradeSideToValue(e: AggregateTradeSide): number { return e; }

export enum LiquidationTradeSide {
  Buy = 1,
  Sell = 2,
}

export function LiquidationTradeSideFromValue(v: number): LiquidationTradeSide | null {
  switch (v) {
    case 1: return LiquidationTradeSide.Buy;
    case 2: return LiquidationTradeSide.Sell;
    default: return null;
  }
}

export function LiquidationTradeSideToValue(e: LiquidationTradeSide): number { return e; }

export enum MarketDataUpdateSide {
  Buy = 1,
  Sell = 2,
}

export function MarketDataUpdateSideFromValue(v: number): MarketDataUpdateSide | null {
  switch (v) {
    case 1: return MarketDataUpdateSide.Buy;
    case 2: return MarketDataUpdateSide.Sell;
    default: return null;
  }
}

export function MarketDataUpdateSideToValue(e: MarketDataUpdateSide): number { return e; }

export enum MarketDataUpdateAction {
  New = 1,
  Change = 2,
  Delete = 3,
}

export function MarketDataUpdateActionFromValue(v: number): MarketDataUpdateAction | null {
  switch (v) {
    case 1: return MarketDataUpdateAction.New;
    case 2: return MarketDataUpdateAction.Change;
    case 3: return MarketDataUpdateAction.Delete;
    default: return null;
  }
}

export function MarketDataUpdateActionToValue(e: MarketDataUpdateAction): number { return e; }

export enum PublicTradeSide {
  Buy = 1,
  Sell = 2,
}

export function PublicTradeSideFromValue(v: number): PublicTradeSide | null {
  switch (v) {
    case 1: return PublicTradeSide.Buy;
    case 2: return PublicTradeSide.Sell;
    default: return null;
  }
}

export function PublicTradeSideToValue(e: PublicTradeSide): number { return e; }

export enum CapabilityPathPattern {
  PubSub = 1,
  RequestResponse = 2,
  RequestStream = 3,
}

export function CapabilityPathPatternFromValue(v: number): CapabilityPathPattern | null {
  switch (v) {
    case 1: return CapabilityPathPattern.PubSub;
    case 2: return CapabilityPathPattern.RequestResponse;
    case 3: return CapabilityPathPattern.RequestStream;
    default: return null;
  }
}

export function CapabilityPathPatternToValue(e: CapabilityPathPattern): number { return e; }

/** SBE struct AggregateTrade */
export interface AggregateTrade {
  symbol: string;
  agg_trade_id: string;
  price: number;
  qty: number;
  side: AggregateTradeSide;
  first_trade_id: string;
  last_trade_id: string;
  timestamp: bigint;
}

/** Encode AggregateTrade fields (no header). */
export function AggregateTradeEncoderEncode(value: AggregateTrade, buf: number[]): void {
  writeString(buf, value.symbol);
  writeString(buf, value.agg_trade_id);
  writeF64BE(buf, value.price);
  writeF64BE(buf, value.qty);
  buf.push(AggregateTradeSideToValue(value.side));
  writeString(buf, value.first_trade_id);
  writeString(buf, value.last_trade_id);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
}

/** Decode AggregateTrade fields; advances `pos`. */
export function AggregateTradeDecoderDecode(buf: Uint8Array, pos: { value: number }): AggregateTrade {
  const symbol = readString(buf, pos);
  const agg_trade_id = readString(buf, pos);
  const price = readF64BE(buf, pos);
  const qty = readF64BE(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = AggregateTradeSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid AggregateTradeSide');
  const first_trade_id = readString(buf, pos);
  const last_trade_id = readString(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: AggregateTrade = {
    symbol: symbol,
    agg_trade_id: agg_trade_id,
    price: price,
    qty: qty,
    side: side,
    first_trade_id: first_trade_id,
    last_trade_id: last_trade_id,
    timestamp: timestamp,
  };
  return value;
}

/** SBE struct LiquidationTrade */
export interface LiquidationTrade {
  symbol: string;
  side: LiquidationTradeSide;
  price: number;
  qty: number;
  timestamp: bigint;
}

/** Encode LiquidationTrade fields (no header). */
export function LiquidationTradeEncoderEncode(value: LiquidationTrade, buf: number[]): void {
  writeString(buf, value.symbol);
  buf.push(LiquidationTradeSideToValue(value.side));
  writeF64BE(buf, value.price);
  writeF64BE(buf, value.qty);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
}

/** Decode LiquidationTrade fields; advances `pos`. */
export function LiquidationTradeDecoderDecode(buf: Uint8Array, pos: { value: number }): LiquidationTrade {
  const symbol = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = LiquidationTradeSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid LiquidationTradeSide');
  const price = readF64BE(buf, pos);
  const qty = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: LiquidationTrade = {
    symbol: symbol,
    side: side,
    price: price,
    qty: qty,
    timestamp: timestamp,
  };
  return value;
}

/** SBE struct PriceLevel */
export interface PriceLevel {
  price: number;
  qty: number;
  order_count: number | null;
}

/** Encode PriceLevel fields (no header). */
export function PriceLevelEncoderEncode(value: PriceLevel, buf: number[]): void {
  writeF64BE(buf, value.price);
  writeF64BE(buf, value.qty);
  writeU32BE(buf, value.order_count ?? 0);
  buf.push(value.order_count != null ? 1 : 0);
}

/** Decode PriceLevel fields; advances `pos`. */
export function PriceLevelDecoderDecode(buf: Uint8Array, pos: { value: number }): PriceLevel {
  const price = readF64BE(buf, pos);
  const qty = readF64BE(buf, pos);
  const order_count_v = readU32BE(buf, pos);
  const order_count = buf[pos.value++] === 1 ? order_count_v : null;
  const value: PriceLevel = {
    price: price,
    qty: qty,
    order_count: order_count,
  };
  return value;
}

/** SBE struct MarketDataUpdate */
export interface MarketDataUpdate {
  side: MarketDataUpdateSide;
  action: MarketDataUpdateAction;
  price: number;
  qty: number;
}

/** Encode MarketDataUpdate fields (no header). */
export function MarketDataUpdateEncoderEncode(value: MarketDataUpdate, buf: number[]): void {
  buf.push(MarketDataUpdateSideToValue(value.side));
  buf.push(MarketDataUpdateActionToValue(value.action));
  writeF64BE(buf, value.price);
  writeF64BE(buf, value.qty);
}

/** Decode MarketDataUpdate fields; advances `pos`. */
export function MarketDataUpdateDecoderDecode(buf: Uint8Array, pos: { value: number }): MarketDataUpdate {
  const sideRaw = readU8(buf, pos);
  const side = MarketDataUpdateSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid MarketDataUpdateSide');
  const actionRaw = readU8(buf, pos);
  const action = MarketDataUpdateActionFromValue(actionRaw);
  if (action == null) throw new Error('invalid MarketDataUpdateAction');
  const price = readF64BE(buf, pos);
  const qty = readF64BE(buf, pos);
  const value: MarketDataUpdate = {
    side: side,
    action: action,
    price: price,
    qty: qty,
  };
  return value;
}

/** SBE struct CandleBar */
export interface CandleBar {
  symbol: string;
  interval: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  bar_start: bigint;
  bar_end: bigint;
  is_final: number;
  is_snapshot: number | null;
}

/** Encode CandleBar fields (no header). */
export function CandleBarEncoderEncode(value: CandleBar, buf: number[]): void {
  writeString(buf, value.symbol);
  writeString(buf, value.interval);
  writeF64BE(buf, value.open);
  writeF64BE(buf, value.high);
  writeF64BE(buf, value.low);
  writeF64BE(buf, value.close);
  writeF64BE(buf, value.volume);
  writeU64BE(buf, BigInt.asUintN(64, value.bar_start));
  writeU64BE(buf, BigInt.asUintN(64, value.bar_end));
  buf.push(value.is_final);
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** Decode CandleBar fields; advances `pos`. */
export function CandleBarDecoderDecode(buf: Uint8Array, pos: { value: number }): CandleBar {
  const symbol = readString(buf, pos);
  const interval = readString(buf, pos);
  const open = readF64BE(buf, pos);
  const high = readF64BE(buf, pos);
  const low = readF64BE(buf, pos);
  const close = readF64BE(buf, pos);
  const volume = readF64BE(buf, pos);
  const bar_start = BigInt.asIntN(64, readU64BE(buf, pos));
  const bar_end = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_final = readU8(buf, pos);
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: CandleBar = {
    symbol: symbol,
    interval: interval,
    open: open,
    high: high,
    low: low,
    close: close,
    volume: volume,
    bar_start: bar_start,
    bar_end: bar_end,
    is_final: is_final,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE struct PublicTrade */
export interface PublicTrade {
  symbol: string;
  trade_id: string;
  price: number;
  qty: number;
  side: PublicTradeSide;
  timestamp: bigint;
}

/** Encode PublicTrade fields (no header). */
export function PublicTradeEncoderEncode(value: PublicTrade, buf: number[]): void {
  writeString(buf, value.symbol);
  writeString(buf, value.trade_id);
  writeF64BE(buf, value.price);
  writeF64BE(buf, value.qty);
  buf.push(PublicTradeSideToValue(value.side));
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
}

/** Decode PublicTrade fields; advances `pos`. */
export function PublicTradeDecoderDecode(buf: Uint8Array, pos: { value: number }): PublicTrade {
  const symbol = readString(buf, pos);
  const trade_id = readString(buf, pos);
  const price = readF64BE(buf, pos);
  const qty = readF64BE(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = PublicTradeSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid PublicTradeSide');
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: PublicTrade = {
    symbol: symbol,
    trade_id: trade_id,
    price: price,
    qty: qty,
    side: side,
    timestamp: timestamp,
  };
  return value;
}

/** SBE struct InstrumentMetadata */
export interface InstrumentMetadata {
  instrument_id: string;
  symbol: string;
  product_kind: string;
  margin_asset: string | null;
  display_name: string | null;
  tick_size: number | null;
  lot_size: number | null;
}

/** Encode InstrumentMetadata fields (no header). */
export function InstrumentMetadataEncoderEncode(value: InstrumentMetadata, buf: number[]): void {
  writeString(buf, value.instrument_id);
  writeString(buf, value.symbol);
  writeString(buf, value.product_kind);
  if (value.margin_asset != null) { buf.push(1); writeString(buf, value.margin_asset); } else { buf.push(0); }
  if (value.display_name != null) { buf.push(1); writeString(buf, value.display_name); } else { buf.push(0); }
  writeF64BE(buf, value.tick_size ?? 0);
  buf.push(value.tick_size != null ? 1 : 0);
  writeF64BE(buf, value.lot_size ?? 0);
  buf.push(value.lot_size != null ? 1 : 0);
}

/** Decode InstrumentMetadata fields; advances `pos`. */
export function InstrumentMetadataDecoderDecode(buf: Uint8Array, pos: { value: number }): InstrumentMetadata {
  let margin_asset: string | null = null;
  let display_name: string | null = null;
  const instrument_id = readString(buf, pos);
  const symbol = readString(buf, pos);
  const product_kind = readString(buf, pos);
  if (readU8(buf, pos) === 1) margin_asset = readString(buf, pos);
  if (readU8(buf, pos) === 1) display_name = readString(buf, pos);
  const tick_size_v = readF64BE(buf, pos);
  const tick_size = buf[pos.value++] === 1 ? tick_size_v : null;
  const lot_size_v = readF64BE(buf, pos);
  const lot_size = buf[pos.value++] === 1 ? lot_size_v : null;
  const value: InstrumentMetadata = {
    instrument_id: instrument_id,
    symbol: symbol,
    product_kind: product_kind,
    margin_asset: margin_asset,
    display_name: display_name,
    tick_size: tick_size,
    lot_size: lot_size,
  };
  return value;
}

/** SBE struct BalanceEntry */
export interface BalanceEntry {
  asset: string;
  total: number;
  available: number;
  hold: number;
}

/** Encode BalanceEntry fields (no header). */
export function BalanceEntryEncoderEncode(value: BalanceEntry, buf: number[]): void {
  writeString(buf, value.asset);
  writeF64BE(buf, value.total);
  writeF64BE(buf, value.available);
  writeF64BE(buf, value.hold);
}

/** Decode BalanceEntry fields; advances `pos`. */
export function BalanceEntryDecoderDecode(buf: Uint8Array, pos: { value: number }): BalanceEntry {
  const asset = readString(buf, pos);
  const total = readF64BE(buf, pos);
  const available = readF64BE(buf, pos);
  const hold = readF64BE(buf, pos);
  const value: BalanceEntry = {
    asset: asset,
    total: total,
    available: available,
    hold: hold,
  };
  return value;
}

/** SBE struct PositionEntry */
export interface PositionEntry {
  symbol: string;
  qty: number;
  entry_price: number;
  unrealized_pnl: number;
}

/** Encode PositionEntry fields (no header). */
export function PositionEntryEncoderEncode(value: PositionEntry, buf: number[]): void {
  writeString(buf, value.symbol);
  writeF64BE(buf, value.qty);
  writeF64BE(buf, value.entry_price);
  writeF64BE(buf, value.unrealized_pnl);
}

/** Decode PositionEntry fields; advances `pos`. */
export function PositionEntryDecoderDecode(buf: Uint8Array, pos: { value: number }): PositionEntry {
  const symbol = readString(buf, pos);
  const qty = readF64BE(buf, pos);
  const entry_price = readF64BE(buf, pos);
  const unrealized_pnl = readF64BE(buf, pos);
  const value: PositionEntry = {
    symbol: symbol,
    qty: qty,
    entry_price: entry_price,
    unrealized_pnl: unrealized_pnl,
  };
  return value;
}

/** SBE struct PageInfo */
export interface PageInfo {
  has_more: number;
  next_cursor: string | null;
}

/** Encode PageInfo fields (no header). */
export function PageInfoEncoderEncode(value: PageInfo, buf: number[]): void {
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** Decode PageInfo fields; advances `pos`. */
export function PageInfoDecoderDecode(buf: Uint8Array, pos: { value: number }): PageInfo {
  let next_cursor: string | null = null;
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: PageInfo = {
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE struct CapabilityPath */
export interface CapabilityPath {
  path: string;
  pattern: CapabilityPathPattern;
  auth_required: number;
}

/** Encode CapabilityPath fields (no header). */
export function CapabilityPathEncoderEncode(value: CapabilityPath, buf: number[]): void {
  writeString(buf, value.path);
  buf.push(CapabilityPathPatternToValue(value.pattern));
  buf.push(value.auth_required);
}

/** Decode CapabilityPath fields; advances `pos`. */
export function CapabilityPathDecoderDecode(buf: Uint8Array, pos: { value: number }): CapabilityPath {
  const path = readString(buf, pos);
  const patternRaw = readU8(buf, pos);
  const pattern = CapabilityPathPatternFromValue(patternRaw);
  if (pattern == null) throw new Error('invalid CapabilityPathPattern');
  const auth_required = readU8(buf, pos);
  const value: CapabilityPath = {
    path: path,
    pattern: pattern,
    auth_required: auth_required,
  };
  return value;
}

/** SBE message NewOrderSingle */
export interface NewOrderSingle {
  cl_ord_id: string;
  side: NewOrderSingleSide;
  order_qty: number;
  price: number | null;
  stop_price: number | null;
  symbol: string;
  order_type: NewOrderSingleOrderType;
  time_in_force: NewOrderSingleTimeInForce;
  expire_time: bigint | null;
  account: string | null;
  strategy_id: string | null;
  security_id: string | null;
  id_source: NewOrderSingleIdSource | null;
  security_exchange: string | null;
  post_only: number | null;
  reduce_only: number | null;
}

/** SBE body encoder for NewOrderSingle (fields only, no header). */
export function NewOrderSingleBodyEncoderEncode(value: NewOrderSingle, buf: number[]): void {
  writeString(buf, value.cl_ord_id);
  buf.push(NewOrderSingleSideToValue(value.side));
  writeF64BE(buf, value.order_qty);
  writeF64BE(buf, value.price ?? 0);
  buf.push(value.price != null ? 1 : 0);
  writeF64BE(buf, value.stop_price ?? 0);
  buf.push(value.stop_price != null ? 1 : 0);
  writeString(buf, value.symbol);
  buf.push(NewOrderSingleOrderTypeToValue(value.order_type));
  buf.push(NewOrderSingleTimeInForceToValue(value.time_in_force));
  writeU64BE(buf, BigInt.asUintN(64, value.expire_time ?? 0n));
  buf.push(value.expire_time != null ? 1 : 0);
  if (value.account != null) { buf.push(1); writeString(buf, value.account); } else { buf.push(0); }
  if (value.strategy_id != null) { buf.push(1); writeString(buf, value.strategy_id); } else { buf.push(0); }
  if (value.security_id != null) { buf.push(1); writeString(buf, value.security_id); } else { buf.push(0); }
  buf.push(value.id_source != null ? NewOrderSingleIdSourceToValue(value.id_source) : 0);
  buf.push(value.id_source != null ? 1 : 0);
  if (value.security_exchange != null) { buf.push(1); writeString(buf, value.security_exchange); } else { buf.push(0); }
  buf.push(value.post_only ?? 0);
  buf.push(value.post_only != null ? 1 : 0);
  buf.push(value.reduce_only ?? 0);
  buf.push(value.reduce_only != null ? 1 : 0);
}

/** SBE body decoder for NewOrderSingle: reads fields at `pos`. */
export function NewOrderSingleBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): NewOrderSingle {
  let account: string | null = null;
  let strategy_id: string | null = null;
  let security_id: string | null = null;
  let security_exchange: string | null = null;
  const cl_ord_id = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = NewOrderSingleSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid NewOrderSingleSide');
  const order_qty = readF64BE(buf, pos);
  const price_v = readF64BE(buf, pos);
  const price = buf[pos.value++] === 1 ? price_v : null;
  const stop_price_v = readF64BE(buf, pos);
  const stop_price = buf[pos.value++] === 1 ? stop_price_v : null;
  const symbol = readString(buf, pos);
  const order_typeRaw = readU8(buf, pos);
  const order_type = NewOrderSingleOrderTypeFromValue(order_typeRaw);
  if (order_type == null) throw new Error('invalid NewOrderSingleOrderType');
  const time_in_forceRaw = readU8(buf, pos);
  const time_in_force = NewOrderSingleTimeInForceFromValue(time_in_forceRaw);
  if (time_in_force == null) throw new Error('invalid NewOrderSingleTimeInForce');
  const expire_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const expire_time = buf[pos.value++] === 1 ? expire_time_v : null;
  if (readU8(buf, pos) === 1) account = readString(buf, pos);
  if (readU8(buf, pos) === 1) strategy_id = readString(buf, pos);
  if (readU8(buf, pos) === 1) security_id = readString(buf, pos);
  const id_sourceRaw = readU8(buf, pos);
  let id_source: NewOrderSingleIdSource | null = null;
  if (buf[pos.value++] === 1) {
    const v = NewOrderSingleIdSourceFromValue(id_sourceRaw);
    if (v == null) throw new Error('invalid NewOrderSingleIdSource');
    id_source = v;
  }
  if (readU8(buf, pos) === 1) security_exchange = readString(buf, pos);
  const post_only_v = readU8(buf, pos);
  const post_only = buf[pos.value++] === 1 ? post_only_v : null;
  const reduce_only_v = readU8(buf, pos);
  const reduce_only = buf[pos.value++] === 1 ? reduce_only_v : null;
  const value: NewOrderSingle = {
    cl_ord_id: cl_ord_id,
    side: side,
    order_qty: order_qty,
    price: price,
    stop_price: stop_price,
    symbol: symbol,
    order_type: order_type,
    time_in_force: time_in_force,
    expire_time: expire_time,
    account: account,
    strategy_id: strategy_id,
    security_id: security_id,
    id_source: id_source,
    security_exchange: security_exchange,
    post_only: post_only,
    reduce_only: reduce_only,
  };
  return value;
}

/** SBE encoder for NewOrderSingle — returns the complete message. */
export function NewOrderSingleEncoderEncode(cl_ord_id: string, side: NewOrderSingleSide, order_qty: number, price: number | null, stop_price: number | null, symbol: string, order_type: NewOrderSingleOrderType, time_in_force: NewOrderSingleTimeInForce, expire_time: bigint | null, account: string | null, strategy_id: string | null, security_id: string | null, id_source: NewOrderSingleIdSource | null, security_exchange: string | null, post_only: number | null, reduce_only: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 1);
  writeU16BE(buf, 0);
  writeU16BE(buf, 38);
  writeString(buf, cl_ord_id);
  buf.push(NewOrderSingleSideToValue(side));
  writeF64BE(buf, order_qty);
  writeF64BE(buf, price ?? 0);
  buf.push(price != null ? 1 : 0);
  writeF64BE(buf, stop_price ?? 0);
  buf.push(stop_price != null ? 1 : 0);
  writeString(buf, symbol);
  buf.push(NewOrderSingleOrderTypeToValue(order_type));
  buf.push(NewOrderSingleTimeInForceToValue(time_in_force));
  writeU64BE(buf, BigInt.asUintN(64, expire_time ?? 0n));
  buf.push(expire_time != null ? 1 : 0);
  if (account != null) { buf.push(1); writeString(buf, account); } else { buf.push(0); }
  if (strategy_id != null) { buf.push(1); writeString(buf, strategy_id); } else { buf.push(0); }
  if (security_id != null) { buf.push(1); writeString(buf, security_id); } else { buf.push(0); }
  buf.push(id_source != null ? NewOrderSingleIdSourceToValue(id_source) : 0);
  buf.push(id_source != null ? 1 : 0);
  if (security_exchange != null) { buf.push(1); writeString(buf, security_exchange); } else { buf.push(0); }
  buf.push(post_only ?? 0);
  buf.push(post_only != null ? 1 : 0);
  buf.push(reduce_only ?? 0);
  buf.push(reduce_only != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for NewOrderSingle: validates the header and reads the body. */
export function NewOrderSingleDecoderDecode(buf: Uint8Array): NewOrderSingle {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 1) throw new Error("invalid template_id");
  let account: string | null = null;
  let strategy_id: string | null = null;
  let security_id: string | null = null;
  let security_exchange: string | null = null;
  const cl_ord_id = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = NewOrderSingleSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid NewOrderSingleSide');
  const order_qty = readF64BE(buf, pos);
  const price_v = readF64BE(buf, pos);
  const price = buf[pos.value++] === 1 ? price_v : null;
  const stop_price_v = readF64BE(buf, pos);
  const stop_price = buf[pos.value++] === 1 ? stop_price_v : null;
  const symbol = readString(buf, pos);
  const order_typeRaw = readU8(buf, pos);
  const order_type = NewOrderSingleOrderTypeFromValue(order_typeRaw);
  if (order_type == null) throw new Error('invalid NewOrderSingleOrderType');
  const time_in_forceRaw = readU8(buf, pos);
  const time_in_force = NewOrderSingleTimeInForceFromValue(time_in_forceRaw);
  if (time_in_force == null) throw new Error('invalid NewOrderSingleTimeInForce');
  const expire_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const expire_time = buf[pos.value++] === 1 ? expire_time_v : null;
  if (readU8(buf, pos) === 1) account = readString(buf, pos);
  if (readU8(buf, pos) === 1) strategy_id = readString(buf, pos);
  if (readU8(buf, pos) === 1) security_id = readString(buf, pos);
  const id_sourceRaw = readU8(buf, pos);
  let id_source: NewOrderSingleIdSource | null = null;
  if (buf[pos.value++] === 1) {
    const v = NewOrderSingleIdSourceFromValue(id_sourceRaw);
    if (v == null) throw new Error('invalid NewOrderSingleIdSource');
    id_source = v;
  }
  if (readU8(buf, pos) === 1) security_exchange = readString(buf, pos);
  const post_only_v = readU8(buf, pos);
  const post_only = buf[pos.value++] === 1 ? post_only_v : null;
  const reduce_only_v = readU8(buf, pos);
  const reduce_only = buf[pos.value++] === 1 ? reduce_only_v : null;
  const value: NewOrderSingle = {
    cl_ord_id: cl_ord_id,
    side: side,
    order_qty: order_qty,
    price: price,
    stop_price: stop_price,
    symbol: symbol,
    order_type: order_type,
    time_in_force: time_in_force,
    expire_time: expire_time,
    account: account,
    strategy_id: strategy_id,
    security_id: security_id,
    id_source: id_source,
    security_exchange: security_exchange,
    post_only: post_only,
    reduce_only: reduce_only,
  };
  return value;
}

/** SBE message CancelRequest */
export interface CancelRequest {
  cl_ord_id: string;
  orig_cl_ord_id: string;
  symbol: string;
  side: CancelRequestSide;
  order_qty: number | null;
}

/** SBE body encoder for CancelRequest (fields only, no header). */
export function CancelRequestBodyEncoderEncode(value: CancelRequest, buf: number[]): void {
  writeString(buf, value.cl_ord_id);
  writeString(buf, value.orig_cl_ord_id);
  writeString(buf, value.symbol);
  buf.push(CancelRequestSideToValue(value.side));
  writeF64BE(buf, value.order_qty ?? 0);
  buf.push(value.order_qty != null ? 1 : 0);
}

/** SBE body decoder for CancelRequest: reads fields at `pos`. */
export function CancelRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CancelRequest {
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const symbol = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = CancelRequestSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid CancelRequestSide');
  const order_qty_v = readF64BE(buf, pos);
  const order_qty = buf[pos.value++] === 1 ? order_qty_v : null;
  const value: CancelRequest = {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    symbol: symbol,
    side: side,
    order_qty: order_qty,
  };
  return value;
}

/** SBE encoder for CancelRequest — returns the complete message. */
export function CancelRequestEncoderEncode(cl_ord_id: string, orig_cl_ord_id: string, symbol: string, side: CancelRequestSide, order_qty: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 2);
  writeU16BE(buf, 0);
  writeU16BE(buf, 9);
  writeString(buf, cl_ord_id);
  writeString(buf, orig_cl_ord_id);
  writeString(buf, symbol);
  buf.push(CancelRequestSideToValue(side));
  writeF64BE(buf, order_qty ?? 0);
  buf.push(order_qty != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for CancelRequest: validates the header and reads the body. */
export function CancelRequestDecoderDecode(buf: Uint8Array): CancelRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 2) throw new Error("invalid template_id");
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const symbol = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = CancelRequestSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid CancelRequestSide');
  const order_qty_v = readF64BE(buf, pos);
  const order_qty = buf[pos.value++] === 1 ? order_qty_v : null;
  const value: CancelRequest = {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    symbol: symbol,
    side: side,
    order_qty: order_qty,
  };
  return value;
}

/** SBE message CancelReplaceRequest */
export interface CancelReplaceRequest {
  cl_ord_id: string;
  orig_cl_ord_id: string;
  symbol: string;
  side: CancelReplaceRequestSide;
  order_qty: number;
  price: number | null;
}

/** SBE body encoder for CancelReplaceRequest (fields only, no header). */
export function CancelReplaceRequestBodyEncoderEncode(value: CancelReplaceRequest, buf: number[]): void {
  writeString(buf, value.cl_ord_id);
  writeString(buf, value.orig_cl_ord_id);
  writeString(buf, value.symbol);
  buf.push(CancelReplaceRequestSideToValue(value.side));
  writeF64BE(buf, value.order_qty);
  writeF64BE(buf, value.price ?? 0);
  buf.push(value.price != null ? 1 : 0);
}

/** SBE body decoder for CancelReplaceRequest: reads fields at `pos`. */
export function CancelReplaceRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CancelReplaceRequest {
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const symbol = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = CancelReplaceRequestSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid CancelReplaceRequestSide');
  const order_qty = readF64BE(buf, pos);
  const price_v = readF64BE(buf, pos);
  const price = buf[pos.value++] === 1 ? price_v : null;
  const value: CancelReplaceRequest = {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    symbol: symbol,
    side: side,
    order_qty: order_qty,
    price: price,
  };
  return value;
}

/** SBE encoder for CancelReplaceRequest — returns the complete message. */
export function CancelReplaceRequestEncoderEncode(cl_ord_id: string, orig_cl_ord_id: string, symbol: string, side: CancelReplaceRequestSide, order_qty: number, price: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 3);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);
  writeString(buf, cl_ord_id);
  writeString(buf, orig_cl_ord_id);
  writeString(buf, symbol);
  buf.push(CancelReplaceRequestSideToValue(side));
  writeF64BE(buf, order_qty);
  writeF64BE(buf, price ?? 0);
  buf.push(price != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for CancelReplaceRequest: validates the header and reads the body. */
export function CancelReplaceRequestDecoderDecode(buf: Uint8Array): CancelReplaceRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 3) throw new Error("invalid template_id");
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const symbol = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  const side = CancelReplaceRequestSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid CancelReplaceRequestSide');
  const order_qty = readF64BE(buf, pos);
  const price_v = readF64BE(buf, pos);
  const price = buf[pos.value++] === 1 ? price_v : null;
  const value: CancelReplaceRequest = {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    symbol: symbol,
    side: side,
    order_qty: order_qty,
    price: price,
  };
  return value;
}

/** SBE message ExecutionReport */
export interface ExecutionReport {
  cl_ord_id: string;
  order_id: string;
  exec_id: string;
  exec_type: ExecutionReportExecType;
  ord_status: ExecutionReportOrdStatus;
  side: ExecutionReportSide;
  last_qty: number | null;
  last_price: number | null;
  leaves_qty: number;
  cum_qty: number;
  avg_price: number;
  symbol: string;
  transact_time: bigint;
}

/** SBE body encoder for ExecutionReport (fields only, no header). */
export function ExecutionReportBodyEncoderEncode(value: ExecutionReport, buf: number[]): void {
  writeString(buf, value.cl_ord_id);
  writeString(buf, value.order_id);
  writeString(buf, value.exec_id);
  buf.push(ExecutionReportExecTypeToValue(value.exec_type));
  buf.push(ExecutionReportOrdStatusToValue(value.ord_status));
  buf.push(ExecutionReportSideToValue(value.side));
  writeF64BE(buf, value.last_qty ?? 0);
  buf.push(value.last_qty != null ? 1 : 0);
  writeF64BE(buf, value.last_price ?? 0);
  buf.push(value.last_price != null ? 1 : 0);
  writeF64BE(buf, value.leaves_qty);
  writeF64BE(buf, value.cum_qty);
  writeF64BE(buf, value.avg_price);
  writeString(buf, value.symbol);
  writeU64BE(buf, BigInt.asUintN(64, value.transact_time));
}

/** SBE body decoder for ExecutionReport: reads fields at `pos`. */
export function ExecutionReportBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): ExecutionReport {
  const cl_ord_id = readString(buf, pos);
  const order_id = readString(buf, pos);
  const exec_id = readString(buf, pos);
  const exec_typeRaw = readU8(buf, pos);
  const exec_type = ExecutionReportExecTypeFromValue(exec_typeRaw);
  if (exec_type == null) throw new Error('invalid ExecutionReportExecType');
  const ord_statusRaw = readU8(buf, pos);
  const ord_status = ExecutionReportOrdStatusFromValue(ord_statusRaw);
  if (ord_status == null) throw new Error('invalid ExecutionReportOrdStatus');
  const sideRaw = readU8(buf, pos);
  const side = ExecutionReportSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid ExecutionReportSide');
  const last_qty_v = readF64BE(buf, pos);
  const last_qty = buf[pos.value++] === 1 ? last_qty_v : null;
  const last_price_v = readF64BE(buf, pos);
  const last_price = buf[pos.value++] === 1 ? last_price_v : null;
  const leaves_qty = readF64BE(buf, pos);
  const cum_qty = readF64BE(buf, pos);
  const avg_price = readF64BE(buf, pos);
  const symbol = readString(buf, pos);
  const transact_time = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: ExecutionReport = {
    cl_ord_id: cl_ord_id,
    order_id: order_id,
    exec_id: exec_id,
    exec_type: exec_type,
    ord_status: ord_status,
    side: side,
    last_qty: last_qty,
    last_price: last_price,
    leaves_qty: leaves_qty,
    cum_qty: cum_qty,
    avg_price: avg_price,
    symbol: symbol,
    transact_time: transact_time,
  };
  return value;
}

/** SBE encoder for ExecutionReport — returns the complete message. */
export function ExecutionReportEncoderEncode(cl_ord_id: string, order_id: string, exec_id: string, exec_type: ExecutionReportExecType, ord_status: ExecutionReportOrdStatus, side: ExecutionReportSide, last_qty: number | null, last_price: number | null, leaves_qty: number, cum_qty: number, avg_price: number, symbol: string, transact_time: bigint): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 4);
  writeU16BE(buf, 0);
  writeU16BE(buf, 51);
  writeString(buf, cl_ord_id);
  writeString(buf, order_id);
  writeString(buf, exec_id);
  buf.push(ExecutionReportExecTypeToValue(exec_type));
  buf.push(ExecutionReportOrdStatusToValue(ord_status));
  buf.push(ExecutionReportSideToValue(side));
  writeF64BE(buf, last_qty ?? 0);
  buf.push(last_qty != null ? 1 : 0);
  writeF64BE(buf, last_price ?? 0);
  buf.push(last_price != null ? 1 : 0);
  writeF64BE(buf, leaves_qty);
  writeF64BE(buf, cum_qty);
  writeF64BE(buf, avg_price);
  writeString(buf, symbol);
  writeU64BE(buf, BigInt.asUintN(64, transact_time));
  return new Uint8Array(buf);
}

/** SBE decoder for ExecutionReport: validates the header and reads the body. */
export function ExecutionReportDecoderDecode(buf: Uint8Array): ExecutionReport {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 4) throw new Error("invalid template_id");
  const cl_ord_id = readString(buf, pos);
  const order_id = readString(buf, pos);
  const exec_id = readString(buf, pos);
  const exec_typeRaw = readU8(buf, pos);
  const exec_type = ExecutionReportExecTypeFromValue(exec_typeRaw);
  if (exec_type == null) throw new Error('invalid ExecutionReportExecType');
  const ord_statusRaw = readU8(buf, pos);
  const ord_status = ExecutionReportOrdStatusFromValue(ord_statusRaw);
  if (ord_status == null) throw new Error('invalid ExecutionReportOrdStatus');
  const sideRaw = readU8(buf, pos);
  const side = ExecutionReportSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid ExecutionReportSide');
  const last_qty_v = readF64BE(buf, pos);
  const last_qty = buf[pos.value++] === 1 ? last_qty_v : null;
  const last_price_v = readF64BE(buf, pos);
  const last_price = buf[pos.value++] === 1 ? last_price_v : null;
  const leaves_qty = readF64BE(buf, pos);
  const cum_qty = readF64BE(buf, pos);
  const avg_price = readF64BE(buf, pos);
  const symbol = readString(buf, pos);
  const transact_time = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: ExecutionReport = {
    cl_ord_id: cl_ord_id,
    order_id: order_id,
    exec_id: exec_id,
    exec_type: exec_type,
    ord_status: ord_status,
    side: side,
    last_qty: last_qty,
    last_price: last_price,
    leaves_qty: leaves_qty,
    cum_qty: cum_qty,
    avg_price: avg_price,
    symbol: symbol,
    transact_time: transact_time,
  };
  return value;
}

/** SBE message CancelReject */
export interface CancelReject {
  cl_ord_id: string;
  orig_cl_ord_id: string;
  reject_reason: CancelRejectRejectReason;
  symbol: string;
}

/** SBE body encoder for CancelReject (fields only, no header). */
export function CancelRejectBodyEncoderEncode(value: CancelReject, buf: number[]): void {
  writeString(buf, value.cl_ord_id);
  writeString(buf, value.orig_cl_ord_id);
  buf.push(CancelRejectRejectReasonToValue(value.reject_reason));
  writeString(buf, value.symbol);
}

/** SBE body decoder for CancelReject: reads fields at `pos`. */
export function CancelRejectBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CancelReject {
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const reject_reasonRaw = readU8(buf, pos);
  const reject_reason = CancelRejectRejectReasonFromValue(reject_reasonRaw);
  if (reject_reason == null) throw new Error('invalid CancelRejectRejectReason');
  const symbol = readString(buf, pos);
  const value: CancelReject = {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    reject_reason: reject_reason,
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for CancelReject — returns the complete message. */
export function CancelRejectEncoderEncode(cl_ord_id: string, orig_cl_ord_id: string, reject_reason: CancelRejectRejectReason, symbol: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 5);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, cl_ord_id);
  writeString(buf, orig_cl_ord_id);
  buf.push(CancelRejectRejectReasonToValue(reject_reason));
  writeString(buf, symbol);
  return new Uint8Array(buf);
}

/** SBE decoder for CancelReject: validates the header and reads the body. */
export function CancelRejectDecoderDecode(buf: Uint8Array): CancelReject {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 5) throw new Error("invalid template_id");
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const reject_reasonRaw = readU8(buf, pos);
  const reject_reason = CancelRejectRejectReasonFromValue(reject_reasonRaw);
  if (reject_reason == null) throw new Error('invalid CancelRejectRejectReason');
  const symbol = readString(buf, pos);
  const value: CancelReject = {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    reject_reason: reject_reason,
    symbol: symbol,
  };
  return value;
}

/** SBE message MarketDataSnapshot */
export interface MarketDataSnapshot {
  symbol: string;
  exchange: string;
  bids: PriceLevel[];
  asks: PriceLevel[];
  timestamp: bigint;
  sequence: bigint | null;
  is_snapshot: number | null;
}

/** SBE body encoder for MarketDataSnapshot (fields only, no header). */
export function MarketDataSnapshotBodyEncoderEncode(value: MarketDataSnapshot, buf: number[]): void {
  writeString(buf, value.symbol);
  writeString(buf, value.exchange);
  writeU32BE(buf, value.bids.length);
  for (const item of value.bids) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU32BE(buf, value.asks.length);
  for (const item of value.asks) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  writeU64BE(buf, value.sequence ?? 0n);
  buf.push(value.sequence != null ? 1 : 0);
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for MarketDataSnapshot: reads fields at `pos`. */
export function MarketDataSnapshotBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MarketDataSnapshot {
  const symbol = readString(buf, pos);
  const exchange = readString(buf, pos);
  const bidsCount = readU32BE(buf, pos);
  const bids: PriceLevel[] = [];
  for (let i = 0; i < bidsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    bids.push(item);
  }
  const asksCount = readU32BE(buf, pos);
  const asks: PriceLevel[] = [];
  for (let i = 0; i < asksCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    asks.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MarketDataSnapshot = {
    symbol: symbol,
    exchange: exchange,
    bids: bids,
    asks: asks,
    timestamp: timestamp,
    sequence: sequence,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for MarketDataSnapshot — returns the complete message. */
export function MarketDataSnapshotEncoderEncode(symbol: string, exchange: string, bids: PriceLevel[], asks: PriceLevel[], timestamp: bigint, sequence: bigint | null, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 6);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);
  writeString(buf, symbol);
  writeString(buf, exchange);
  writeU32BE(buf, bids.length);
  for (const item of bids) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU32BE(buf, asks.length);
  for (const item of asks) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  writeU64BE(buf, sequence ?? 0n);
  buf.push(sequence != null ? 1 : 0);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for MarketDataSnapshot: validates the header and reads the body. */
export function MarketDataSnapshotDecoderDecode(buf: Uint8Array): MarketDataSnapshot {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 6) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const exchange = readString(buf, pos);
  const bidsCount = readU32BE(buf, pos);
  const bids: PriceLevel[] = [];
  for (let i = 0; i < bidsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    bids.push(item);
  }
  const asksCount = readU32BE(buf, pos);
  const asks: PriceLevel[] = [];
  for (let i = 0; i < asksCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    asks.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MarketDataSnapshot = {
    symbol: symbol,
    exchange: exchange,
    bids: bids,
    asks: asks,
    timestamp: timestamp,
    sequence: sequence,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message MarketDataIncrementalRefresh */
export interface MarketDataIncrementalRefresh {
  symbol: string;
  updates: MarketDataUpdate[];
  timestamp: bigint;
  sequence: bigint | null;
}

/** SBE body encoder for MarketDataIncrementalRefresh (fields only, no header). */
export function MarketDataIncrementalRefreshBodyEncoderEncode(value: MarketDataIncrementalRefresh, buf: number[]): void {
  writeString(buf, value.symbol);
  writeU32BE(buf, value.updates.length);
  for (const item of value.updates) {
    const itemBuf: number[] = [];
    MarketDataUpdateEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  writeU64BE(buf, value.sequence ?? 0n);
  buf.push(value.sequence != null ? 1 : 0);
}

/** SBE body decoder for MarketDataIncrementalRefresh: reads fields at `pos`. */
export function MarketDataIncrementalRefreshBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MarketDataIncrementalRefresh {
  const symbol = readString(buf, pos);
  const updatesCount = readU32BE(buf, pos);
  const updates: MarketDataUpdate[] = [];
  for (let i = 0; i < updatesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = MarketDataUpdateDecoderDecode(buf, pos);
    pos.value = itemEnd;
    updates.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const value: MarketDataIncrementalRefresh = {
    symbol: symbol,
    updates: updates,
    timestamp: timestamp,
    sequence: sequence,
  };
  return value;
}

/** SBE encoder for MarketDataIncrementalRefresh — returns the complete message. */
export function MarketDataIncrementalRefreshEncoderEncode(symbol: string, updates: MarketDataUpdate[], timestamp: bigint, sequence: bigint | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 7);
  writeU16BE(buf, 0);
  writeU16BE(buf, 16);
  writeString(buf, symbol);
  writeU32BE(buf, updates.length);
  for (const item of updates) {
    const itemBuf: number[] = [];
    MarketDataUpdateEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  writeU64BE(buf, sequence ?? 0n);
  buf.push(sequence != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for MarketDataIncrementalRefresh: validates the header and reads the body. */
export function MarketDataIncrementalRefreshDecoderDecode(buf: Uint8Array): MarketDataIncrementalRefresh {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 7) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const updatesCount = readU32BE(buf, pos);
  const updates: MarketDataUpdate[] = [];
  for (let i = 0; i < updatesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = MarketDataUpdateDecoderDecode(buf, pos);
    pos.value = itemEnd;
    updates.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const value: MarketDataIncrementalRefresh = {
    symbol: symbol,
    updates: updates,
    timestamp: timestamp,
    sequence: sequence,
  };
  return value;
}

/** SBE message OrderBookRequest */
export interface OrderBookRequest {
  symbol: string;
  depth: number | null;
  at_time: bigint | null;
}

/** SBE body encoder for OrderBookRequest (fields only, no header). */
export function OrderBookRequestBodyEncoderEncode(value: OrderBookRequest, buf: number[]): void {
  writeString(buf, value.symbol);
  writeU32BE(buf, value.depth ?? 0);
  buf.push(value.depth != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.at_time ?? 0n));
  buf.push(value.at_time != null ? 1 : 0);
}

/** SBE body decoder for OrderBookRequest: reads fields at `pos`. */
export function OrderBookRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OrderBookRequest {
  const symbol = readString(buf, pos);
  const depth_v = readU32BE(buf, pos);
  const depth = buf[pos.value++] === 1 ? depth_v : null;
  const at_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const at_time = buf[pos.value++] === 1 ? at_time_v : null;
  const value: OrderBookRequest = {
    symbol: symbol,
    depth: depth,
    at_time: at_time,
  };
  return value;
}

/** SBE encoder for OrderBookRequest — returns the complete message. */
export function OrderBookRequestEncoderEncode(symbol: string, depth: number | null, at_time: bigint | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 8);
  writeU16BE(buf, 0);
  writeU16BE(buf, 12);
  writeString(buf, symbol);
  writeU32BE(buf, depth ?? 0);
  buf.push(depth != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, at_time ?? 0n));
  buf.push(at_time != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for OrderBookRequest: validates the header and reads the body. */
export function OrderBookRequestDecoderDecode(buf: Uint8Array): OrderBookRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 8) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const depth_v = readU32BE(buf, pos);
  const depth = buf[pos.value++] === 1 ? depth_v : null;
  const at_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const at_time = buf[pos.value++] === 1 ? at_time_v : null;
  const value: OrderBookRequest = {
    symbol: symbol,
    depth: depth,
    at_time: at_time,
  };
  return value;
}

/** SBE message OrderBookSnapshot */
export interface OrderBookSnapshot {
  symbol: string;
  exchange: string;
  bids: PriceLevel[];
  asks: PriceLevel[];
  timestamp: bigint;
  sequence: bigint | null;
  is_snapshot: number | null;
}

/** SBE body encoder for OrderBookSnapshot (fields only, no header). */
export function OrderBookSnapshotBodyEncoderEncode(value: OrderBookSnapshot, buf: number[]): void {
  writeString(buf, value.symbol);
  writeString(buf, value.exchange);
  writeU32BE(buf, value.bids.length);
  for (const item of value.bids) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU32BE(buf, value.asks.length);
  for (const item of value.asks) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  writeU64BE(buf, value.sequence ?? 0n);
  buf.push(value.sequence != null ? 1 : 0);
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for OrderBookSnapshot: reads fields at `pos`. */
export function OrderBookSnapshotBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OrderBookSnapshot {
  const symbol = readString(buf, pos);
  const exchange = readString(buf, pos);
  const bidsCount = readU32BE(buf, pos);
  const bids: PriceLevel[] = [];
  for (let i = 0; i < bidsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    bids.push(item);
  }
  const asksCount = readU32BE(buf, pos);
  const asks: PriceLevel[] = [];
  for (let i = 0; i < asksCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    asks.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: OrderBookSnapshot = {
    symbol: symbol,
    exchange: exchange,
    bids: bids,
    asks: asks,
    timestamp: timestamp,
    sequence: sequence,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for OrderBookSnapshot — returns the complete message. */
export function OrderBookSnapshotEncoderEncode(symbol: string, exchange: string, bids: PriceLevel[], asks: PriceLevel[], timestamp: bigint, sequence: bigint | null, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 9);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);
  writeString(buf, symbol);
  writeString(buf, exchange);
  writeU32BE(buf, bids.length);
  for (const item of bids) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU32BE(buf, asks.length);
  for (const item of asks) {
    const itemBuf: number[] = [];
    PriceLevelEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  writeU64BE(buf, sequence ?? 0n);
  buf.push(sequence != null ? 1 : 0);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for OrderBookSnapshot: validates the header and reads the body. */
export function OrderBookSnapshotDecoderDecode(buf: Uint8Array): OrderBookSnapshot {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 9) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const exchange = readString(buf, pos);
  const bidsCount = readU32BE(buf, pos);
  const bids: PriceLevel[] = [];
  for (let i = 0; i < bidsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    bids.push(item);
  }
  const asksCount = readU32BE(buf, pos);
  const asks: PriceLevel[] = [];
  for (let i = 0; i < asksCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PriceLevelDecoderDecode(buf, pos);
    pos.value = itemEnd;
    asks.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: OrderBookSnapshot = {
    symbol: symbol,
    exchange: exchange,
    bids: bids,
    asks: asks,
    timestamp: timestamp,
    sequence: sequence,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message OrderBookDelta */
export interface OrderBookDelta {
  symbol: string;
  updates: MarketDataUpdate[];
  timestamp: bigint;
  sequence: bigint | null;
}

/** SBE body encoder for OrderBookDelta (fields only, no header). */
export function OrderBookDeltaBodyEncoderEncode(value: OrderBookDelta, buf: number[]): void {
  writeString(buf, value.symbol);
  writeU32BE(buf, value.updates.length);
  for (const item of value.updates) {
    const itemBuf: number[] = [];
    MarketDataUpdateEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  writeU64BE(buf, value.sequence ?? 0n);
  buf.push(value.sequence != null ? 1 : 0);
}

/** SBE body decoder for OrderBookDelta: reads fields at `pos`. */
export function OrderBookDeltaBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OrderBookDelta {
  const symbol = readString(buf, pos);
  const updatesCount = readU32BE(buf, pos);
  const updates: MarketDataUpdate[] = [];
  for (let i = 0; i < updatesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = MarketDataUpdateDecoderDecode(buf, pos);
    pos.value = itemEnd;
    updates.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const value: OrderBookDelta = {
    symbol: symbol,
    updates: updates,
    timestamp: timestamp,
    sequence: sequence,
  };
  return value;
}

/** SBE encoder for OrderBookDelta — returns the complete message. */
export function OrderBookDeltaEncoderEncode(symbol: string, updates: MarketDataUpdate[], timestamp: bigint, sequence: bigint | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 10);
  writeU16BE(buf, 0);
  writeU16BE(buf, 16);
  writeString(buf, symbol);
  writeU32BE(buf, updates.length);
  for (const item of updates) {
    const itemBuf: number[] = [];
    MarketDataUpdateEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  writeU64BE(buf, sequence ?? 0n);
  buf.push(sequence != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for OrderBookDelta: validates the header and reads the body. */
export function OrderBookDeltaDecoderDecode(buf: Uint8Array): OrderBookDelta {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 10) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const updatesCount = readU32BE(buf, pos);
  const updates: MarketDataUpdate[] = [];
  for (let i = 0; i < updatesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = MarketDataUpdateDecoderDecode(buf, pos);
    pos.value = itemEnd;
    updates.push(item);
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const sequence_v = readU64BE(buf, pos);
  const sequence = buf[pos.value++] === 1 ? sequence_v : null;
  const value: OrderBookDelta = {
    symbol: symbol,
    updates: updates,
    timestamp: timestamp,
    sequence: sequence,
  };
  return value;
}

/** SBE message AggregateTradeEvent */
export interface AggregateTradeEvent {
  trade: AggregateTrade;
}

/** SBE body encoder for AggregateTradeEvent (fields only, no header). */
export function AggregateTradeEventBodyEncoderEncode(value: AggregateTradeEvent, buf: number[]): void {
  AggregateTradeEncoderEncode(value.trade, buf);
}

/** SBE body decoder for AggregateTradeEvent: reads fields at `pos`. */
export function AggregateTradeEventBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): AggregateTradeEvent {
  const trade = AggregateTradeDecoderDecode(buf, pos);
  const value: AggregateTradeEvent = {
    trade: trade,
  };
  return value;
}

/** SBE encoder for AggregateTradeEvent — returns the complete message. */
export function AggregateTradeEventEncoderEncode(trade: AggregateTrade): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 11);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  AggregateTradeEncoderEncode(trade, buf);
  return new Uint8Array(buf);
}

/** SBE decoder for AggregateTradeEvent: validates the header and reads the body. */
export function AggregateTradeEventDecoderDecode(buf: Uint8Array): AggregateTradeEvent {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 11) throw new Error("invalid template_id");
  const trade = AggregateTradeDecoderDecode(buf, pos);
  const value: AggregateTradeEvent = {
    trade: trade,
  };
  return value;
}

/** SBE message AggregateTradeRequest */
export interface AggregateTradeRequest {
  symbol: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: number | null;
  cursor: string | null;
}

/** SBE body encoder for AggregateTradeRequest (fields only, no header). */
export function AggregateTradeRequestBodyEncoderEncode(value: AggregateTradeRequest, buf: number[]): void {
  writeString(buf, value.symbol);
  writeU64BE(buf, BigInt.asUintN(64, value.start_time ?? 0n));
  buf.push(value.start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.end_time ?? 0n));
  buf.push(value.end_time != null ? 1 : 0);
  writeU32BE(buf, value.limit ?? 0);
  buf.push(value.limit != null ? 1 : 0);
  if (value.cursor != null) { buf.push(1); writeString(buf, value.cursor); } else { buf.push(0); }
}

/** SBE body decoder for AggregateTradeRequest: reads fields at `pos`. */
export function AggregateTradeRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): AggregateTradeRequest {
  let cursor: string | null = null;
  const symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: AggregateTradeRequest = {
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE encoder for AggregateTradeRequest — returns the complete message. */
export function AggregateTradeRequestEncoderEncode(symbol: string, start_time: bigint | null, end_time: bigint | null, limit: number | null, cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 12);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);
  writeString(buf, symbol);
  writeU64BE(buf, BigInt.asUintN(64, start_time ?? 0n));
  buf.push(start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, end_time ?? 0n));
  buf.push(end_time != null ? 1 : 0);
  writeU32BE(buf, limit ?? 0);
  buf.push(limit != null ? 1 : 0);
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for AggregateTradeRequest: validates the header and reads the body. */
export function AggregateTradeRequestDecoderDecode(buf: Uint8Array): AggregateTradeRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 12) throw new Error("invalid template_id");
  let cursor: string | null = null;
  const symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: AggregateTradeRequest = {
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE message AggregateTradeBatch */
export interface AggregateTradeBatch {
  symbol: string;
  trades: AggregateTrade[];
  has_more: number;
  next_cursor: string | null;
}

/** SBE body encoder for AggregateTradeBatch (fields only, no header). */
export function AggregateTradeBatchBodyEncoderEncode(value: AggregateTradeBatch, buf: number[]): void {
  writeString(buf, value.symbol);
  writeU32BE(buf, value.trades.length);
  for (const item of value.trades) {
    const itemBuf: number[] = [];
    AggregateTradeEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** SBE body decoder for AggregateTradeBatch: reads fields at `pos`. */
export function AggregateTradeBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): AggregateTradeBatch {
  let next_cursor: string | null = null;
  const symbol = readString(buf, pos);
  const tradesCount = readU32BE(buf, pos);
  const trades: AggregateTrade[] = [];
  for (let i = 0; i < tradesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = AggregateTradeDecoderDecode(buf, pos);
    pos.value = itemEnd;
    trades.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: AggregateTradeBatch = {
    symbol: symbol,
    trades: trades,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE encoder for AggregateTradeBatch — returns the complete message. */
export function AggregateTradeBatchEncoderEncode(symbol: string, trades: AggregateTrade[], has_more: number, next_cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 13);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, symbol);
  writeU32BE(buf, trades.length);
  for (const item of trades) {
    const itemBuf: number[] = [];
    AggregateTradeEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for AggregateTradeBatch: validates the header and reads the body. */
export function AggregateTradeBatchDecoderDecode(buf: Uint8Array): AggregateTradeBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 13) throw new Error("invalid template_id");
  let next_cursor: string | null = null;
  const symbol = readString(buf, pos);
  const tradesCount = readU32BE(buf, pos);
  const trades: AggregateTrade[] = [];
  for (let i = 0; i < tradesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = AggregateTradeDecoderDecode(buf, pos);
    pos.value = itemEnd;
    trades.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: AggregateTradeBatch = {
    symbol: symbol,
    trades: trades,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE message MiniTicker */
export interface MiniTicker {
  symbol: string;
  last_price: number;
  volume: number;
  timestamp: bigint;
  is_snapshot: number | null;
}

/** SBE body encoder for MiniTicker (fields only, no header). */
export function MiniTickerBodyEncoderEncode(value: MiniTicker, buf: number[]): void {
  writeString(buf, value.symbol);
  writeF64BE(buf, value.last_price);
  writeF64BE(buf, value.volume);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for MiniTicker: reads fields at `pos`. */
export function MiniTickerBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MiniTicker {
  const symbol = readString(buf, pos);
  const last_price = readF64BE(buf, pos);
  const volume = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MiniTicker = {
    symbol: symbol,
    last_price: last_price,
    volume: volume,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for MiniTicker — returns the complete message. */
export function MiniTickerEncoderEncode(symbol: string, last_price: number, volume: number, timestamp: bigint, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 14);
  writeU16BE(buf, 0);
  writeU16BE(buf, 25);
  writeString(buf, symbol);
  writeF64BE(buf, last_price);
  writeF64BE(buf, volume);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for MiniTicker: validates the header and reads the body. */
export function MiniTickerDecoderDecode(buf: Uint8Array): MiniTicker {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 14) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const last_price = readF64BE(buf, pos);
  const volume = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MiniTicker = {
    symbol: symbol,
    last_price: last_price,
    volume: volume,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message AllMidsRequest */
export interface AllMidsRequest {
}

/** SBE body encoder for AllMidsRequest (fields only, no header). */
export function AllMidsRequestBodyEncoderEncode(value: AllMidsRequest, buf: number[]): void {
}

/** SBE body decoder for AllMidsRequest: reads fields at `pos`. */
export function AllMidsRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): AllMidsRequest {
  const value: AllMidsRequest = {
  };
  return value;
}

/** SBE encoder for AllMidsRequest — returns the complete message. */
export function AllMidsRequestEncoderEncode(): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 15);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  return new Uint8Array(buf);
}

/** SBE decoder for AllMidsRequest: validates the header and reads the body. */
export function AllMidsRequestDecoderDecode(buf: Uint8Array): AllMidsRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 15) throw new Error("invalid template_id");
  const value: AllMidsRequest = {
  };
  return value;
}

/** SBE message AllMidsBatch */
export interface AllMidsBatch {
  tickers: MiniTicker[];
}

/** SBE body encoder for AllMidsBatch (fields only, no header). */
export function AllMidsBatchBodyEncoderEncode(value: AllMidsBatch, buf: number[]): void {
  writeU32BE(buf, value.tickers.length);
  for (const item of value.tickers) {
    const itemBuf: number[] = [];
    MiniTickerBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
}

/** SBE body decoder for AllMidsBatch: reads fields at `pos`. */
export function AllMidsBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): AllMidsBatch {
  const tickersCount = readU32BE(buf, pos);
  const tickers: MiniTicker[] = [];
  for (let i = 0; i < tickersCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = MiniTickerBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    tickers.push(item);
  }
  const value: AllMidsBatch = {
    tickers: tickers,
  };
  return value;
}

/** SBE encoder for AllMidsBatch — returns the complete message. */
export function AllMidsBatchEncoderEncode(tickers: MiniTicker[]): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 16);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeU32BE(buf, tickers.length);
  for (const item of tickers) {
    const itemBuf: number[] = [];
    MiniTickerBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  return new Uint8Array(buf);
}

/** SBE decoder for AllMidsBatch: validates the header and reads the body. */
export function AllMidsBatchDecoderDecode(buf: Uint8Array): AllMidsBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 16) throw new Error("invalid template_id");
  const tickersCount = readU32BE(buf, pos);
  const tickers: MiniTicker[] = [];
  for (let i = 0; i < tickersCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = MiniTickerBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    tickers.push(item);
  }
  const value: AllMidsBatch = {
    tickers: tickers,
  };
  return value;
}

/** SBE message MarkPriceUpdate */
export interface MarkPriceUpdate {
  symbol: string;
  mark_price: number;
  index_price: number | null;
  funding_rate: number | null;
  timestamp: bigint;
  is_snapshot: number | null;
}

/** SBE body encoder for MarkPriceUpdate (fields only, no header). */
export function MarkPriceUpdateBodyEncoderEncode(value: MarkPriceUpdate, buf: number[]): void {
  writeString(buf, value.symbol);
  writeF64BE(buf, value.mark_price);
  writeF64BE(buf, value.index_price ?? 0);
  buf.push(value.index_price != null ? 1 : 0);
  writeF64BE(buf, value.funding_rate ?? 0);
  buf.push(value.funding_rate != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for MarkPriceUpdate: reads fields at `pos`. */
export function MarkPriceUpdateBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MarkPriceUpdate {
  const symbol = readString(buf, pos);
  const mark_price = readF64BE(buf, pos);
  const index_price_v = readF64BE(buf, pos);
  const index_price = buf[pos.value++] === 1 ? index_price_v : null;
  const funding_rate_v = readF64BE(buf, pos);
  const funding_rate = buf[pos.value++] === 1 ? funding_rate_v : null;
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MarkPriceUpdate = {
    symbol: symbol,
    mark_price: mark_price,
    index_price: index_price,
    funding_rate: funding_rate,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for MarkPriceUpdate — returns the complete message. */
export function MarkPriceUpdateEncoderEncode(symbol: string, mark_price: number, index_price: number | null, funding_rate: number | null, timestamp: bigint, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 17);
  writeU16BE(buf, 0);
  writeU16BE(buf, 33);
  writeString(buf, symbol);
  writeF64BE(buf, mark_price);
  writeF64BE(buf, index_price ?? 0);
  buf.push(index_price != null ? 1 : 0);
  writeF64BE(buf, funding_rate ?? 0);
  buf.push(funding_rate != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for MarkPriceUpdate: validates the header and reads the body. */
export function MarkPriceUpdateDecoderDecode(buf: Uint8Array): MarkPriceUpdate {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 17) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const mark_price = readF64BE(buf, pos);
  const index_price_v = readF64BE(buf, pos);
  const index_price = buf[pos.value++] === 1 ? index_price_v : null;
  const funding_rate_v = readF64BE(buf, pos);
  const funding_rate = buf[pos.value++] === 1 ? funding_rate_v : null;
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MarkPriceUpdate = {
    symbol: symbol,
    mark_price: mark_price,
    index_price: index_price,
    funding_rate: funding_rate,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message MarkPriceRequest */
export interface MarkPriceRequest {
  symbol: string;
}

/** SBE body encoder for MarkPriceRequest (fields only, no header). */
export function MarkPriceRequestBodyEncoderEncode(value: MarkPriceRequest, buf: number[]): void {
  writeString(buf, value.symbol);
}

/** SBE body decoder for MarkPriceRequest: reads fields at `pos`. */
export function MarkPriceRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MarkPriceRequest {
  const symbol = readString(buf, pos);
  const value: MarkPriceRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for MarkPriceRequest — returns the complete message. */
export function MarkPriceRequestEncoderEncode(symbol: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 18);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeString(buf, symbol);
  return new Uint8Array(buf);
}

/** SBE decoder for MarkPriceRequest: validates the header and reads the body. */
export function MarkPriceRequestDecoderDecode(buf: Uint8Array): MarkPriceRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 18) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const value: MarkPriceRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE message LiquidationTradeEvent */
export interface LiquidationTradeEvent {
  trade: LiquidationTrade;
}

/** SBE body encoder for LiquidationTradeEvent (fields only, no header). */
export function LiquidationTradeEventBodyEncoderEncode(value: LiquidationTradeEvent, buf: number[]): void {
  LiquidationTradeEncoderEncode(value.trade, buf);
}

/** SBE body decoder for LiquidationTradeEvent: reads fields at `pos`. */
export function LiquidationTradeEventBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): LiquidationTradeEvent {
  const trade = LiquidationTradeDecoderDecode(buf, pos);
  const value: LiquidationTradeEvent = {
    trade: trade,
  };
  return value;
}

/** SBE encoder for LiquidationTradeEvent — returns the complete message. */
export function LiquidationTradeEventEncoderEncode(trade: LiquidationTrade): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 19);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  LiquidationTradeEncoderEncode(trade, buf);
  return new Uint8Array(buf);
}

/** SBE decoder for LiquidationTradeEvent: validates the header and reads the body. */
export function LiquidationTradeEventDecoderDecode(buf: Uint8Array): LiquidationTradeEvent {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 19) throw new Error("invalid template_id");
  const trade = LiquidationTradeDecoderDecode(buf, pos);
  const value: LiquidationTradeEvent = {
    trade: trade,
  };
  return value;
}

/** SBE message CandleBarEvent */
export interface CandleBarEvent {
  bar: CandleBar;
}

/** SBE body encoder for CandleBarEvent (fields only, no header). */
export function CandleBarEventBodyEncoderEncode(value: CandleBarEvent, buf: number[]): void {
  CandleBarEncoderEncode(value.bar, buf);
}

/** SBE body decoder for CandleBarEvent: reads fields at `pos`. */
export function CandleBarEventBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CandleBarEvent {
  const bar = CandleBarDecoderDecode(buf, pos);
  const value: CandleBarEvent = {
    bar: bar,
  };
  return value;
}

/** SBE encoder for CandleBarEvent — returns the complete message. */
export function CandleBarEventEncoderEncode(bar: CandleBar): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 20);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  CandleBarEncoderEncode(bar, buf);
  return new Uint8Array(buf);
}

/** SBE decoder for CandleBarEvent: validates the header and reads the body. */
export function CandleBarEventDecoderDecode(buf: Uint8Array): CandleBarEvent {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 20) throw new Error("invalid template_id");
  const bar = CandleBarDecoderDecode(buf, pos);
  const value: CandleBarEvent = {
    bar: bar,
  };
  return value;
}

/** SBE message CandleBarRequest */
export interface CandleBarRequest {
  symbol: string;
  interval: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: number | null;
  cursor: string | null;
}

/** SBE body encoder for CandleBarRequest (fields only, no header). */
export function CandleBarRequestBodyEncoderEncode(value: CandleBarRequest, buf: number[]): void {
  writeString(buf, value.symbol);
  writeString(buf, value.interval);
  writeU64BE(buf, BigInt.asUintN(64, value.start_time ?? 0n));
  buf.push(value.start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.end_time ?? 0n));
  buf.push(value.end_time != null ? 1 : 0);
  writeU32BE(buf, value.limit ?? 0);
  buf.push(value.limit != null ? 1 : 0);
  if (value.cursor != null) { buf.push(1); writeString(buf, value.cursor); } else { buf.push(0); }
}

/** SBE body decoder for CandleBarRequest: reads fields at `pos`. */
export function CandleBarRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CandleBarRequest {
  let cursor: string | null = null;
  const symbol = readString(buf, pos);
  const interval = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: CandleBarRequest = {
    symbol: symbol,
    interval: interval,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE encoder for CandleBarRequest — returns the complete message. */
export function CandleBarRequestEncoderEncode(symbol: string, interval: string, start_time: bigint | null, end_time: bigint | null, limit: number | null, cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 21);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);
  writeString(buf, symbol);
  writeString(buf, interval);
  writeU64BE(buf, BigInt.asUintN(64, start_time ?? 0n));
  buf.push(start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, end_time ?? 0n));
  buf.push(end_time != null ? 1 : 0);
  writeU32BE(buf, limit ?? 0);
  buf.push(limit != null ? 1 : 0);
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for CandleBarRequest: validates the header and reads the body. */
export function CandleBarRequestDecoderDecode(buf: Uint8Array): CandleBarRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 21) throw new Error("invalid template_id");
  let cursor: string | null = null;
  const symbol = readString(buf, pos);
  const interval = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: CandleBarRequest = {
    symbol: symbol,
    interval: interval,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE message CandleBarBatch */
export interface CandleBarBatch {
  symbol: string;
  interval: string;
  bars: CandleBar[];
  has_more: number;
  next_cursor: string | null;
}

/** SBE body encoder for CandleBarBatch (fields only, no header). */
export function CandleBarBatchBodyEncoderEncode(value: CandleBarBatch, buf: number[]): void {
  writeString(buf, value.symbol);
  writeString(buf, value.interval);
  writeU32BE(buf, value.bars.length);
  for (const item of value.bars) {
    const itemBuf: number[] = [];
    CandleBarEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** SBE body decoder for CandleBarBatch: reads fields at `pos`. */
export function CandleBarBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CandleBarBatch {
  let next_cursor: string | null = null;
  const symbol = readString(buf, pos);
  const interval = readString(buf, pos);
  const barsCount = readU32BE(buf, pos);
  const bars: CandleBar[] = [];
  for (let i = 0; i < barsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = CandleBarDecoderDecode(buf, pos);
    pos.value = itemEnd;
    bars.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: CandleBarBatch = {
    symbol: symbol,
    interval: interval,
    bars: bars,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE encoder for CandleBarBatch — returns the complete message. */
export function CandleBarBatchEncoderEncode(symbol: string, interval: string, bars: CandleBar[], has_more: number, next_cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 22);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, symbol);
  writeString(buf, interval);
  writeU32BE(buf, bars.length);
  for (const item of bars) {
    const itemBuf: number[] = [];
    CandleBarEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for CandleBarBatch: validates the header and reads the body. */
export function CandleBarBatchDecoderDecode(buf: Uint8Array): CandleBarBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 22) throw new Error("invalid template_id");
  let next_cursor: string | null = null;
  const symbol = readString(buf, pos);
  const interval = readString(buf, pos);
  const barsCount = readU32BE(buf, pos);
  const bars: CandleBar[] = [];
  for (let i = 0; i < barsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = CandleBarDecoderDecode(buf, pos);
    pos.value = itemEnd;
    bars.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: CandleBarBatch = {
    symbol: symbol,
    interval: interval,
    bars: bars,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE message PublicTradeEvent */
export interface PublicTradeEvent {
  trade: PublicTrade;
}

/** SBE body encoder for PublicTradeEvent (fields only, no header). */
export function PublicTradeEventBodyEncoderEncode(value: PublicTradeEvent, buf: number[]): void {
  PublicTradeEncoderEncode(value.trade, buf);
}

/** SBE body decoder for PublicTradeEvent: reads fields at `pos`. */
export function PublicTradeEventBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): PublicTradeEvent {
  const trade = PublicTradeDecoderDecode(buf, pos);
  const value: PublicTradeEvent = {
    trade: trade,
  };
  return value;
}

/** SBE encoder for PublicTradeEvent — returns the complete message. */
export function PublicTradeEventEncoderEncode(trade: PublicTrade): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 23);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  PublicTradeEncoderEncode(trade, buf);
  return new Uint8Array(buf);
}

/** SBE decoder for PublicTradeEvent: validates the header and reads the body. */
export function PublicTradeEventDecoderDecode(buf: Uint8Array): PublicTradeEvent {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 23) throw new Error("invalid template_id");
  const trade = PublicTradeDecoderDecode(buf, pos);
  const value: PublicTradeEvent = {
    trade: trade,
  };
  return value;
}

/** SBE message TradeHistoryRequest */
export interface TradeHistoryRequest {
  symbol: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: number | null;
  cursor: string | null;
}

/** SBE body encoder for TradeHistoryRequest (fields only, no header). */
export function TradeHistoryRequestBodyEncoderEncode(value: TradeHistoryRequest, buf: number[]): void {
  writeString(buf, value.symbol);
  writeU64BE(buf, BigInt.asUintN(64, value.start_time ?? 0n));
  buf.push(value.start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.end_time ?? 0n));
  buf.push(value.end_time != null ? 1 : 0);
  writeU32BE(buf, value.limit ?? 0);
  buf.push(value.limit != null ? 1 : 0);
  if (value.cursor != null) { buf.push(1); writeString(buf, value.cursor); } else { buf.push(0); }
}

/** SBE body decoder for TradeHistoryRequest: reads fields at `pos`. */
export function TradeHistoryRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): TradeHistoryRequest {
  let cursor: string | null = null;
  const symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: TradeHistoryRequest = {
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE encoder for TradeHistoryRequest — returns the complete message. */
export function TradeHistoryRequestEncoderEncode(symbol: string, start_time: bigint | null, end_time: bigint | null, limit: number | null, cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 24);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);
  writeString(buf, symbol);
  writeU64BE(buf, BigInt.asUintN(64, start_time ?? 0n));
  buf.push(start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, end_time ?? 0n));
  buf.push(end_time != null ? 1 : 0);
  writeU32BE(buf, limit ?? 0);
  buf.push(limit != null ? 1 : 0);
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for TradeHistoryRequest: validates the header and reads the body. */
export function TradeHistoryRequestDecoderDecode(buf: Uint8Array): TradeHistoryRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 24) throw new Error("invalid template_id");
  let cursor: string | null = null;
  const symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: TradeHistoryRequest = {
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE message PublicTradeBatch */
export interface PublicTradeBatch {
  symbol: string;
  trades: PublicTrade[];
  has_more: number;
  next_cursor: string | null;
}

/** SBE body encoder for PublicTradeBatch (fields only, no header). */
export function PublicTradeBatchBodyEncoderEncode(value: PublicTradeBatch, buf: number[]): void {
  writeString(buf, value.symbol);
  writeU32BE(buf, value.trades.length);
  for (const item of value.trades) {
    const itemBuf: number[] = [];
    PublicTradeEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** SBE body decoder for PublicTradeBatch: reads fields at `pos`. */
export function PublicTradeBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): PublicTradeBatch {
  let next_cursor: string | null = null;
  const symbol = readString(buf, pos);
  const tradesCount = readU32BE(buf, pos);
  const trades: PublicTrade[] = [];
  for (let i = 0; i < tradesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PublicTradeDecoderDecode(buf, pos);
    pos.value = itemEnd;
    trades.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: PublicTradeBatch = {
    symbol: symbol,
    trades: trades,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE encoder for PublicTradeBatch — returns the complete message. */
export function PublicTradeBatchEncoderEncode(symbol: string, trades: PublicTrade[], has_more: number, next_cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 25);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, symbol);
  writeU32BE(buf, trades.length);
  for (const item of trades) {
    const itemBuf: number[] = [];
    PublicTradeEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for PublicTradeBatch: validates the header and reads the body. */
export function PublicTradeBatchDecoderDecode(buf: Uint8Array): PublicTradeBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 25) throw new Error("invalid template_id");
  let next_cursor: string | null = null;
  const symbol = readString(buf, pos);
  const tradesCount = readU32BE(buf, pos);
  const trades: PublicTrade[] = [];
  for (let i = 0; i < tradesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PublicTradeDecoderDecode(buf, pos);
    pos.value = itemEnd;
    trades.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: PublicTradeBatch = {
    symbol: symbol,
    trades: trades,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE message BestBidOffer */
export interface BestBidOffer {
  symbol: string;
  bid_price: number | null;
  bid_qty: number | null;
  ask_price: number | null;
  ask_qty: number | null;
  timestamp: bigint;
  is_snapshot: number | null;
}

/** SBE body encoder for BestBidOffer (fields only, no header). */
export function BestBidOfferBodyEncoderEncode(value: BestBidOffer, buf: number[]): void {
  writeString(buf, value.symbol);
  writeF64BE(buf, value.bid_price ?? 0);
  buf.push(value.bid_price != null ? 1 : 0);
  writeF64BE(buf, value.bid_qty ?? 0);
  buf.push(value.bid_qty != null ? 1 : 0);
  writeF64BE(buf, value.ask_price ?? 0);
  buf.push(value.ask_price != null ? 1 : 0);
  writeF64BE(buf, value.ask_qty ?? 0);
  buf.push(value.ask_qty != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for BestBidOffer: reads fields at `pos`. */
export function BestBidOfferBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): BestBidOffer {
  const symbol = readString(buf, pos);
  const bid_price_v = readF64BE(buf, pos);
  const bid_price = buf[pos.value++] === 1 ? bid_price_v : null;
  const bid_qty_v = readF64BE(buf, pos);
  const bid_qty = buf[pos.value++] === 1 ? bid_qty_v : null;
  const ask_price_v = readF64BE(buf, pos);
  const ask_price = buf[pos.value++] === 1 ? ask_price_v : null;
  const ask_qty_v = readF64BE(buf, pos);
  const ask_qty = buf[pos.value++] === 1 ? ask_qty_v : null;
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: BestBidOffer = {
    symbol: symbol,
    bid_price: bid_price,
    bid_qty: bid_qty,
    ask_price: ask_price,
    ask_qty: ask_qty,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for BestBidOffer — returns the complete message. */
export function BestBidOfferEncoderEncode(symbol: string, bid_price: number | null, bid_qty: number | null, ask_price: number | null, ask_qty: number | null, timestamp: bigint, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 26);
  writeU16BE(buf, 0);
  writeU16BE(buf, 41);
  writeString(buf, symbol);
  writeF64BE(buf, bid_price ?? 0);
  buf.push(bid_price != null ? 1 : 0);
  writeF64BE(buf, bid_qty ?? 0);
  buf.push(bid_qty != null ? 1 : 0);
  writeF64BE(buf, ask_price ?? 0);
  buf.push(ask_price != null ? 1 : 0);
  writeF64BE(buf, ask_qty ?? 0);
  buf.push(ask_qty != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for BestBidOffer: validates the header and reads the body. */
export function BestBidOfferDecoderDecode(buf: Uint8Array): BestBidOffer {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 26) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const bid_price_v = readF64BE(buf, pos);
  const bid_price = buf[pos.value++] === 1 ? bid_price_v : null;
  const bid_qty_v = readF64BE(buf, pos);
  const bid_qty = buf[pos.value++] === 1 ? bid_qty_v : null;
  const ask_price_v = readF64BE(buf, pos);
  const ask_price = buf[pos.value++] === 1 ? ask_price_v : null;
  const ask_qty_v = readF64BE(buf, pos);
  const ask_qty = buf[pos.value++] === 1 ? ask_qty_v : null;
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: BestBidOffer = {
    symbol: symbol,
    bid_price: bid_price,
    bid_qty: bid_qty,
    ask_price: ask_price,
    ask_qty: ask_qty,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message SymbolTicker */
export interface SymbolTicker {
  symbol: string;
  last_price: number;
  price_change: number;
  price_change_pct: number;
  volume: number;
  high: number;
  low: number;
  open: number;
  timestamp: bigint;
  is_snapshot: number | null;
}

/** SBE body encoder for SymbolTicker (fields only, no header). */
export function SymbolTickerBodyEncoderEncode(value: SymbolTicker, buf: number[]): void {
  writeString(buf, value.symbol);
  writeF64BE(buf, value.last_price);
  writeF64BE(buf, value.price_change);
  writeF64BE(buf, value.price_change_pct);
  writeF64BE(buf, value.volume);
  writeF64BE(buf, value.high);
  writeF64BE(buf, value.low);
  writeF64BE(buf, value.open);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for SymbolTicker: reads fields at `pos`. */
export function SymbolTickerBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): SymbolTicker {
  const symbol = readString(buf, pos);
  const last_price = readF64BE(buf, pos);
  const price_change = readF64BE(buf, pos);
  const price_change_pct = readF64BE(buf, pos);
  const volume = readF64BE(buf, pos);
  const high = readF64BE(buf, pos);
  const low = readF64BE(buf, pos);
  const open = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: SymbolTicker = {
    symbol: symbol,
    last_price: last_price,
    price_change: price_change,
    price_change_pct: price_change_pct,
    volume: volume,
    high: high,
    low: low,
    open: open,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for SymbolTicker — returns the complete message. */
export function SymbolTickerEncoderEncode(symbol: string, last_price: number, price_change: number, price_change_pct: number, volume: number, high: number, low: number, open: number, timestamp: bigint, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 27);
  writeU16BE(buf, 0);
  writeU16BE(buf, 65);
  writeString(buf, symbol);
  writeF64BE(buf, last_price);
  writeF64BE(buf, price_change);
  writeF64BE(buf, price_change_pct);
  writeF64BE(buf, volume);
  writeF64BE(buf, high);
  writeF64BE(buf, low);
  writeF64BE(buf, open);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for SymbolTicker: validates the header and reads the body. */
export function SymbolTickerDecoderDecode(buf: Uint8Array): SymbolTicker {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 27) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const last_price = readF64BE(buf, pos);
  const price_change = readF64BE(buf, pos);
  const price_change_pct = readF64BE(buf, pos);
  const volume = readF64BE(buf, pos);
  const high = readF64BE(buf, pos);
  const low = readF64BE(buf, pos);
  const open = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: SymbolTicker = {
    symbol: symbol,
    last_price: last_price,
    price_change: price_change,
    price_change_pct: price_change_pct,
    volume: volume,
    high: high,
    low: low,
    open: open,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message TickerRequest */
export interface TickerRequest {
  symbol: string;
}

/** SBE body encoder for TickerRequest (fields only, no header). */
export function TickerRequestBodyEncoderEncode(value: TickerRequest, buf: number[]): void {
  writeString(buf, value.symbol);
}

/** SBE body decoder for TickerRequest: reads fields at `pos`. */
export function TickerRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): TickerRequest {
  const symbol = readString(buf, pos);
  const value: TickerRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for TickerRequest — returns the complete message. */
export function TickerRequestEncoderEncode(symbol: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 28);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeString(buf, symbol);
  return new Uint8Array(buf);
}

/** SBE decoder for TickerRequest: validates the header and reads the body. */
export function TickerRequestDecoderDecode(buf: Uint8Array): TickerRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 28) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const value: TickerRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE message InstrumentCatalogRequest */
export interface InstrumentCatalogRequest {
}

/** SBE body encoder for InstrumentCatalogRequest (fields only, no header). */
export function InstrumentCatalogRequestBodyEncoderEncode(value: InstrumentCatalogRequest, buf: number[]): void {
}

/** SBE body decoder for InstrumentCatalogRequest: reads fields at `pos`. */
export function InstrumentCatalogRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): InstrumentCatalogRequest {
  const value: InstrumentCatalogRequest = {
  };
  return value;
}

/** SBE encoder for InstrumentCatalogRequest — returns the complete message. */
export function InstrumentCatalogRequestEncoderEncode(): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 29);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  return new Uint8Array(buf);
}

/** SBE decoder for InstrumentCatalogRequest: validates the header and reads the body. */
export function InstrumentCatalogRequestDecoderDecode(buf: Uint8Array): InstrumentCatalogRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 29) throw new Error("invalid template_id");
  const value: InstrumentCatalogRequest = {
  };
  return value;
}

/** SBE message InstrumentCatalogResponse */
export interface InstrumentCatalogResponse {
  instruments: InstrumentMetadata[];
}

/** SBE body encoder for InstrumentCatalogResponse (fields only, no header). */
export function InstrumentCatalogResponseBodyEncoderEncode(value: InstrumentCatalogResponse, buf: number[]): void {
  writeU32BE(buf, value.instruments.length);
  for (const item of value.instruments) {
    const itemBuf: number[] = [];
    InstrumentMetadataEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
}

/** SBE body decoder for InstrumentCatalogResponse: reads fields at `pos`. */
export function InstrumentCatalogResponseBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): InstrumentCatalogResponse {
  const instrumentsCount = readU32BE(buf, pos);
  const instruments: InstrumentMetadata[] = [];
  for (let i = 0; i < instrumentsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = InstrumentMetadataDecoderDecode(buf, pos);
    pos.value = itemEnd;
    instruments.push(item);
  }
  const value: InstrumentCatalogResponse = {
    instruments: instruments,
  };
  return value;
}

/** SBE encoder for InstrumentCatalogResponse — returns the complete message. */
export function InstrumentCatalogResponseEncoderEncode(instruments: InstrumentMetadata[]): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 30);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeU32BE(buf, instruments.length);
  for (const item of instruments) {
    const itemBuf: number[] = [];
    InstrumentMetadataEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  return new Uint8Array(buf);
}

/** SBE decoder for InstrumentCatalogResponse: validates the header and reads the body. */
export function InstrumentCatalogResponseDecoderDecode(buf: Uint8Array): InstrumentCatalogResponse {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 30) throw new Error("invalid template_id");
  const instrumentsCount = readU32BE(buf, pos);
  const instruments: InstrumentMetadata[] = [];
  for (let i = 0; i < instrumentsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = InstrumentMetadataDecoderDecode(buf, pos);
    pos.value = itemEnd;
    instruments.push(item);
  }
  const value: InstrumentCatalogResponse = {
    instruments: instruments,
  };
  return value;
}

/** SBE message AccountSummary */
export interface AccountSummary {
  account: string;
  balance: number;
  buying_power: number;
  currency: string;
}

/** SBE body encoder for AccountSummary (fields only, no header). */
export function AccountSummaryBodyEncoderEncode(value: AccountSummary, buf: number[]): void {
  writeString(buf, value.account);
  writeF64BE(buf, value.balance);
  writeF64BE(buf, value.buying_power);
  writeString(buf, value.currency);
}

/** SBE body decoder for AccountSummary: reads fields at `pos`. */
export function AccountSummaryBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): AccountSummary {
  const account = readString(buf, pos);
  const balance = readF64BE(buf, pos);
  const buying_power = readF64BE(buf, pos);
  const currency = readString(buf, pos);
  const value: AccountSummary = {
    account: account,
    balance: balance,
    buying_power: buying_power,
    currency: currency,
  };
  return value;
}

/** SBE encoder for AccountSummary — returns the complete message. */
export function AccountSummaryEncoderEncode(account: string, balance: number, buying_power: number, currency: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 31);
  writeU16BE(buf, 0);
  writeU16BE(buf, 16);
  writeString(buf, account);
  writeF64BE(buf, balance);
  writeF64BE(buf, buying_power);
  writeString(buf, currency);
  return new Uint8Array(buf);
}

/** SBE decoder for AccountSummary: validates the header and reads the body. */
export function AccountSummaryDecoderDecode(buf: Uint8Array): AccountSummary {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 31) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const balance = readF64BE(buf, pos);
  const buying_power = readF64BE(buf, pos);
  const currency = readString(buf, pos);
  const value: AccountSummary = {
    account: account,
    balance: balance,
    buying_power: buying_power,
    currency: currency,
  };
  return value;
}

/** SBE message MarginSummary */
export interface MarginSummary {
  account: string;
  balance: number;
  buying_power: number;
  equity: number;
  margin_used: number;
  available: number;
  currency: string;
}

/** SBE body encoder for MarginSummary (fields only, no header). */
export function MarginSummaryBodyEncoderEncode(value: MarginSummary, buf: number[]): void {
  writeString(buf, value.account);
  writeF64BE(buf, value.balance);
  writeF64BE(buf, value.buying_power);
  writeF64BE(buf, value.equity);
  writeF64BE(buf, value.margin_used);
  writeF64BE(buf, value.available);
  writeString(buf, value.currency);
}

/** SBE body decoder for MarginSummary: reads fields at `pos`. */
export function MarginSummaryBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MarginSummary {
  const account = readString(buf, pos);
  const balance = readF64BE(buf, pos);
  const buying_power = readF64BE(buf, pos);
  const equity = readF64BE(buf, pos);
  const margin_used = readF64BE(buf, pos);
  const available = readF64BE(buf, pos);
  const currency = readString(buf, pos);
  const value: MarginSummary = {
    account: account,
    balance: balance,
    buying_power: buying_power,
    equity: equity,
    margin_used: margin_used,
    available: available,
    currency: currency,
  };
  return value;
}

/** SBE encoder for MarginSummary — returns the complete message. */
export function MarginSummaryEncoderEncode(account: string, balance: number, buying_power: number, equity: number, margin_used: number, available: number, currency: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 32);
  writeU16BE(buf, 0);
  writeU16BE(buf, 40);
  writeString(buf, account);
  writeF64BE(buf, balance);
  writeF64BE(buf, buying_power);
  writeF64BE(buf, equity);
  writeF64BE(buf, margin_used);
  writeF64BE(buf, available);
  writeString(buf, currency);
  return new Uint8Array(buf);
}

/** SBE decoder for MarginSummary: validates the header and reads the body. */
export function MarginSummaryDecoderDecode(buf: Uint8Array): MarginSummary {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 32) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const balance = readF64BE(buf, pos);
  const buying_power = readF64BE(buf, pos);
  const equity = readF64BE(buf, pos);
  const margin_used = readF64BE(buf, pos);
  const available = readF64BE(buf, pos);
  const currency = readString(buf, pos);
  const value: MarginSummary = {
    account: account,
    balance: balance,
    buying_power: buying_power,
    equity: equity,
    margin_used: margin_used,
    available: available,
    currency: currency,
  };
  return value;
}

/** SBE message BalanceSnapshot */
export interface BalanceSnapshot {
  account: string;
  balances: BalanceEntry[];
  is_snapshot: number | null;
}

/** SBE body encoder for BalanceSnapshot (fields only, no header). */
export function BalanceSnapshotBodyEncoderEncode(value: BalanceSnapshot, buf: number[]): void {
  writeString(buf, value.account);
  writeU32BE(buf, value.balances.length);
  for (const item of value.balances) {
    const itemBuf: number[] = [];
    BalanceEntryEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for BalanceSnapshot: reads fields at `pos`. */
export function BalanceSnapshotBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): BalanceSnapshot {
  const account = readString(buf, pos);
  const balancesCount = readU32BE(buf, pos);
  const balances: BalanceEntry[] = [];
  for (let i = 0; i < balancesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = BalanceEntryDecoderDecode(buf, pos);
    pos.value = itemEnd;
    balances.push(item);
  }
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: BalanceSnapshot = {
    account: account,
    balances: balances,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for BalanceSnapshot — returns the complete message. */
export function BalanceSnapshotEncoderEncode(account: string, balances: BalanceEntry[], is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 33);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeU32BE(buf, balances.length);
  for (const item of balances) {
    const itemBuf: number[] = [];
    BalanceEntryEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for BalanceSnapshot: validates the header and reads the body. */
export function BalanceSnapshotDecoderDecode(buf: Uint8Array): BalanceSnapshot {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 33) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const balancesCount = readU32BE(buf, pos);
  const balances: BalanceEntry[] = [];
  for (let i = 0; i < balancesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = BalanceEntryDecoderDecode(buf, pos);
    pos.value = itemEnd;
    balances.push(item);
  }
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: BalanceSnapshot = {
    account: account,
    balances: balances,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message BalanceUpdate */
export interface BalanceUpdate {
  account: string;
  asset: string;
  delta: number;
  total: number;
  available: number;
  reason: BalanceUpdateReason;
}

/** SBE body encoder for BalanceUpdate (fields only, no header). */
export function BalanceUpdateBodyEncoderEncode(value: BalanceUpdate, buf: number[]): void {
  writeString(buf, value.account);
  writeString(buf, value.asset);
  writeF64BE(buf, value.delta);
  writeF64BE(buf, value.total);
  writeF64BE(buf, value.available);
  buf.push(BalanceUpdateReasonToValue(value.reason));
}

/** SBE body decoder for BalanceUpdate: reads fields at `pos`. */
export function BalanceUpdateBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): BalanceUpdate {
  const account = readString(buf, pos);
  const asset = readString(buf, pos);
  const delta = readF64BE(buf, pos);
  const total = readF64BE(buf, pos);
  const available = readF64BE(buf, pos);
  const reasonRaw = readU8(buf, pos);
  const reason = BalanceUpdateReasonFromValue(reasonRaw);
  if (reason == null) throw new Error('invalid BalanceUpdateReason');
  const value: BalanceUpdate = {
    account: account,
    asset: asset,
    delta: delta,
    total: total,
    available: available,
    reason: reason,
  };
  return value;
}

/** SBE encoder for BalanceUpdate — returns the complete message. */
export function BalanceUpdateEncoderEncode(account: string, asset: string, delta: number, total: number, available: number, reason: BalanceUpdateReason): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 34);
  writeU16BE(buf, 0);
  writeU16BE(buf, 25);
  writeString(buf, account);
  writeString(buf, asset);
  writeF64BE(buf, delta);
  writeF64BE(buf, total);
  writeF64BE(buf, available);
  buf.push(BalanceUpdateReasonToValue(reason));
  return new Uint8Array(buf);
}

/** SBE decoder for BalanceUpdate: validates the header and reads the body. */
export function BalanceUpdateDecoderDecode(buf: Uint8Array): BalanceUpdate {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 34) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const asset = readString(buf, pos);
  const delta = readF64BE(buf, pos);
  const total = readF64BE(buf, pos);
  const available = readF64BE(buf, pos);
  const reasonRaw = readU8(buf, pos);
  const reason = BalanceUpdateReasonFromValue(reasonRaw);
  if (reason == null) throw new Error('invalid BalanceUpdateReason');
  const value: BalanceUpdate = {
    account: account,
    asset: asset,
    delta: delta,
    total: total,
    available: available,
    reason: reason,
  };
  return value;
}

/** SBE message PositionSnapshot */
export interface PositionSnapshot {
  account: string;
  positions: PositionEntry[];
  is_snapshot: number | null;
}

/** SBE body encoder for PositionSnapshot (fields only, no header). */
export function PositionSnapshotBodyEncoderEncode(value: PositionSnapshot, buf: number[]): void {
  writeString(buf, value.account);
  writeU32BE(buf, value.positions.length);
  for (const item of value.positions) {
    const itemBuf: number[] = [];
    PositionEntryEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for PositionSnapshot: reads fields at `pos`. */
export function PositionSnapshotBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): PositionSnapshot {
  const account = readString(buf, pos);
  const positionsCount = readU32BE(buf, pos);
  const positions: PositionEntry[] = [];
  for (let i = 0; i < positionsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PositionEntryDecoderDecode(buf, pos);
    pos.value = itemEnd;
    positions.push(item);
  }
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: PositionSnapshot = {
    account: account,
    positions: positions,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for PositionSnapshot — returns the complete message. */
export function PositionSnapshotEncoderEncode(account: string, positions: PositionEntry[], is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 35);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeU32BE(buf, positions.length);
  for (const item of positions) {
    const itemBuf: number[] = [];
    PositionEntryEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for PositionSnapshot: validates the header and reads the body. */
export function PositionSnapshotDecoderDecode(buf: Uint8Array): PositionSnapshot {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 35) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const positionsCount = readU32BE(buf, pos);
  const positions: PositionEntry[] = [];
  for (let i = 0; i < positionsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = PositionEntryDecoderDecode(buf, pos);
    pos.value = itemEnd;
    positions.push(item);
  }
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: PositionSnapshot = {
    account: account,
    positions: positions,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message PositionUpdate */
export interface PositionUpdate {
  account: string;
  symbol: string;
  qty: number;
  entry_price: number;
  unrealized_pnl: number;
}

/** SBE body encoder for PositionUpdate (fields only, no header). */
export function PositionUpdateBodyEncoderEncode(value: PositionUpdate, buf: number[]): void {
  writeString(buf, value.account);
  writeString(buf, value.symbol);
  writeF64BE(buf, value.qty);
  writeF64BE(buf, value.entry_price);
  writeF64BE(buf, value.unrealized_pnl);
}

/** SBE body decoder for PositionUpdate: reads fields at `pos`. */
export function PositionUpdateBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): PositionUpdate {
  const account = readString(buf, pos);
  const symbol = readString(buf, pos);
  const qty = readF64BE(buf, pos);
  const entry_price = readF64BE(buf, pos);
  const unrealized_pnl = readF64BE(buf, pos);
  const value: PositionUpdate = {
    account: account,
    symbol: symbol,
    qty: qty,
    entry_price: entry_price,
    unrealized_pnl: unrealized_pnl,
  };
  return value;
}

/** SBE encoder for PositionUpdate — returns the complete message. */
export function PositionUpdateEncoderEncode(account: string, symbol: string, qty: number, entry_price: number, unrealized_pnl: number): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 36);
  writeU16BE(buf, 0);
  writeU16BE(buf, 24);
  writeString(buf, account);
  writeString(buf, symbol);
  writeF64BE(buf, qty);
  writeF64BE(buf, entry_price);
  writeF64BE(buf, unrealized_pnl);
  return new Uint8Array(buf);
}

/** SBE decoder for PositionUpdate: validates the header and reads the body. */
export function PositionUpdateDecoderDecode(buf: Uint8Array): PositionUpdate {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 36) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const symbol = readString(buf, pos);
  const qty = readF64BE(buf, pos);
  const entry_price = readF64BE(buf, pos);
  const unrealized_pnl = readF64BE(buf, pos);
  const value: PositionUpdate = {
    account: account,
    symbol: symbol,
    qty: qty,
    entry_price: entry_price,
    unrealized_pnl: unrealized_pnl,
  };
  return value;
}

/** SBE message MarginUpdate */
export interface MarginUpdate {
  account: string;
  summary: MarginSummary;
  is_snapshot: number | null;
}

/** SBE body encoder for MarginUpdate (fields only, no header). */
export function MarginUpdateBodyEncoderEncode(value: MarginUpdate, buf: number[]): void {
  writeString(buf, value.account);
  MarginSummaryBodyEncoderEncode(value.summary, buf);
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for MarginUpdate: reads fields at `pos`. */
export function MarginUpdateBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MarginUpdate {
  const account = readString(buf, pos);
  const summary = MarginSummaryBodyDecoderDecode(buf, pos);
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MarginUpdate = {
    account: account,
    summary: summary,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for MarginUpdate — returns the complete message. */
export function MarginUpdateEncoderEncode(account: string, summary: MarginSummary, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 37);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  MarginSummaryBodyEncoderEncode(summary, buf);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for MarginUpdate: validates the header and reads the body. */
export function MarginUpdateDecoderDecode(buf: Uint8Array): MarginUpdate {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 37) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const summary = MarginSummaryBodyDecoderDecode(buf, pos);
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: MarginUpdate = {
    account: account,
    summary: summary,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message UserLiquidation */
export interface UserLiquidation {
  account: string;
  symbol: string;
  qty: number;
  price: number;
  timestamp: bigint;
}

/** SBE body encoder for UserLiquidation (fields only, no header). */
export function UserLiquidationBodyEncoderEncode(value: UserLiquidation, buf: number[]): void {
  writeString(buf, value.account);
  writeString(buf, value.symbol);
  writeF64BE(buf, value.qty);
  writeF64BE(buf, value.price);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
}

/** SBE body decoder for UserLiquidation: reads fields at `pos`. */
export function UserLiquidationBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): UserLiquidation {
  const account = readString(buf, pos);
  const symbol = readString(buf, pos);
  const qty = readF64BE(buf, pos);
  const price = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: UserLiquidation = {
    account: account,
    symbol: symbol,
    qty: qty,
    price: price,
    timestamp: timestamp,
  };
  return value;
}

/** SBE encoder for UserLiquidation — returns the complete message. */
export function UserLiquidationEncoderEncode(account: string, symbol: string, qty: number, price: number, timestamp: bigint): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 38);
  writeU16BE(buf, 0);
  writeU16BE(buf, 24);
  writeString(buf, account);
  writeString(buf, symbol);
  writeF64BE(buf, qty);
  writeF64BE(buf, price);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  return new Uint8Array(buf);
}

/** SBE decoder for UserLiquidation: validates the header and reads the body. */
export function UserLiquidationDecoderDecode(buf: Uint8Array): UserLiquidation {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 38) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const symbol = readString(buf, pos);
  const qty = readF64BE(buf, pos);
  const price = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: UserLiquidation = {
    account: account,
    symbol: symbol,
    qty: qty,
    price: price,
    timestamp: timestamp,
  };
  return value;
}

/** SBE message OrderListStatus */
export interface OrderListStatus {
  account: string;
  list_id: string;
  status: OrderListStatusStatus;
  symbol: string | null;
}

/** SBE body encoder for OrderListStatus (fields only, no header). */
export function OrderListStatusBodyEncoderEncode(value: OrderListStatus, buf: number[]): void {
  writeString(buf, value.account);
  writeString(buf, value.list_id);
  buf.push(OrderListStatusStatusToValue(value.status));
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
}

/** SBE body decoder for OrderListStatus: reads fields at `pos`. */
export function OrderListStatusBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OrderListStatus {
  let symbol: string | null = null;
  const account = readString(buf, pos);
  const list_id = readString(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = OrderListStatusStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid OrderListStatusStatus');
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const value: OrderListStatus = {
    account: account,
    list_id: list_id,
    status: status,
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for OrderListStatus — returns the complete message. */
export function OrderListStatusEncoderEncode(account: string, list_id: string, status: OrderListStatusStatus, symbol: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 39);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeString(buf, list_id);
  buf.push(OrderListStatusStatusToValue(status));
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for OrderListStatus: validates the header and reads the body. */
export function OrderListStatusDecoderDecode(buf: Uint8Array): OrderListStatus {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 39) throw new Error("invalid template_id");
  let symbol: string | null = null;
  const account = readString(buf, pos);
  const list_id = readString(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = OrderListStatusStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid OrderListStatusStatus');
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const value: OrderListStatus = {
    account: account,
    list_id: list_id,
    status: status,
    symbol: symbol,
  };
  return value;
}

/** SBE message FillHistoryRequest */
export interface FillHistoryRequest {
  account: string;
  symbol: string | null;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: number | null;
  cursor: string | null;
}

/** SBE body encoder for FillHistoryRequest (fields only, no header). */
export function FillHistoryRequestBodyEncoderEncode(value: FillHistoryRequest, buf: number[]): void {
  writeString(buf, value.account);
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
  writeU64BE(buf, BigInt.asUintN(64, value.start_time ?? 0n));
  buf.push(value.start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.end_time ?? 0n));
  buf.push(value.end_time != null ? 1 : 0);
  writeU32BE(buf, value.limit ?? 0);
  buf.push(value.limit != null ? 1 : 0);
  if (value.cursor != null) { buf.push(1); writeString(buf, value.cursor); } else { buf.push(0); }
}

/** SBE body decoder for FillHistoryRequest: reads fields at `pos`. */
export function FillHistoryRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): FillHistoryRequest {
  let symbol: string | null = null;
  let cursor: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: FillHistoryRequest = {
    account: account,
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE encoder for FillHistoryRequest — returns the complete message. */
export function FillHistoryRequestEncoderEncode(account: string, symbol: string | null, start_time: bigint | null, end_time: bigint | null, limit: number | null, cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 40);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  writeU64BE(buf, BigInt.asUintN(64, start_time ?? 0n));
  buf.push(start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, end_time ?? 0n));
  buf.push(end_time != null ? 1 : 0);
  writeU32BE(buf, limit ?? 0);
  buf.push(limit != null ? 1 : 0);
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for FillHistoryRequest: validates the header and reads the body. */
export function FillHistoryRequestDecoderDecode(buf: Uint8Array): FillHistoryRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 40) throw new Error("invalid template_id");
  let symbol: string | null = null;
  let cursor: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: FillHistoryRequest = {
    account: account,
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE message FillHistoryBatch */
export interface FillHistoryBatch {
  account: string;
  fills: ExecutionReport[];
  has_more: number;
  next_cursor: string | null;
}

/** SBE body encoder for FillHistoryBatch (fields only, no header). */
export function FillHistoryBatchBodyEncoderEncode(value: FillHistoryBatch, buf: number[]): void {
  writeString(buf, value.account);
  writeU32BE(buf, value.fills.length);
  for (const item of value.fills) {
    const itemBuf: number[] = [];
    ExecutionReportBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** SBE body decoder for FillHistoryBatch: reads fields at `pos`. */
export function FillHistoryBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): FillHistoryBatch {
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const fillsCount = readU32BE(buf, pos);
  const fills: ExecutionReport[] = [];
  for (let i = 0; i < fillsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = ExecutionReportBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    fills.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: FillHistoryBatch = {
    account: account,
    fills: fills,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE encoder for FillHistoryBatch — returns the complete message. */
export function FillHistoryBatchEncoderEncode(account: string, fills: ExecutionReport[], has_more: number, next_cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 41);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeU32BE(buf, fills.length);
  for (const item of fills) {
    const itemBuf: number[] = [];
    ExecutionReportBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for FillHistoryBatch: validates the header and reads the body. */
export function FillHistoryBatchDecoderDecode(buf: Uint8Array): FillHistoryBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 41) throw new Error("invalid template_id");
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const fillsCount = readU32BE(buf, pos);
  const fills: ExecutionReport[] = [];
  for (let i = 0; i < fillsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = ExecutionReportBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    fills.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: FillHistoryBatch = {
    account: account,
    fills: fills,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE message FundingPayment */
export interface FundingPayment {
  account: string;
  symbol: string | null;
  amount: number;
  rate: number;
  timestamp: bigint;
}

/** SBE body encoder for FundingPayment (fields only, no header). */
export function FundingPaymentBodyEncoderEncode(value: FundingPayment, buf: number[]): void {
  writeString(buf, value.account);
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
  writeF64BE(buf, value.amount);
  writeF64BE(buf, value.rate);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
}

/** SBE body decoder for FundingPayment: reads fields at `pos`. */
export function FundingPaymentBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): FundingPayment {
  let symbol: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const amount = readF64BE(buf, pos);
  const rate = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: FundingPayment = {
    account: account,
    symbol: symbol,
    amount: amount,
    rate: rate,
    timestamp: timestamp,
  };
  return value;
}

/** SBE encoder for FundingPayment — returns the complete message. */
export function FundingPaymentEncoderEncode(account: string, symbol: string | null, amount: number, rate: number, timestamp: bigint): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 42);
  writeU16BE(buf, 0);
  writeU16BE(buf, 24);
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  writeF64BE(buf, amount);
  writeF64BE(buf, rate);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  return new Uint8Array(buf);
}

/** SBE decoder for FundingPayment: validates the header and reads the body. */
export function FundingPaymentDecoderDecode(buf: Uint8Array): FundingPayment {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 42) throw new Error("invalid template_id");
  let symbol: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const amount = readF64BE(buf, pos);
  const rate = readF64BE(buf, pos);
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: FundingPayment = {
    account: account,
    symbol: symbol,
    amount: amount,
    rate: rate,
    timestamp: timestamp,
  };
  return value;
}

/** SBE message FundingHistoryRequest */
export interface FundingHistoryRequest {
  account: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: number | null;
  cursor: string | null;
}

/** SBE body encoder for FundingHistoryRequest (fields only, no header). */
export function FundingHistoryRequestBodyEncoderEncode(value: FundingHistoryRequest, buf: number[]): void {
  writeString(buf, value.account);
  writeU64BE(buf, BigInt.asUintN(64, value.start_time ?? 0n));
  buf.push(value.start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.end_time ?? 0n));
  buf.push(value.end_time != null ? 1 : 0);
  writeU32BE(buf, value.limit ?? 0);
  buf.push(value.limit != null ? 1 : 0);
  if (value.cursor != null) { buf.push(1); writeString(buf, value.cursor); } else { buf.push(0); }
}

/** SBE body decoder for FundingHistoryRequest: reads fields at `pos`. */
export function FundingHistoryRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): FundingHistoryRequest {
  let cursor: string | null = null;
  const account = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: FundingHistoryRequest = {
    account: account,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE encoder for FundingHistoryRequest — returns the complete message. */
export function FundingHistoryRequestEncoderEncode(account: string, start_time: bigint | null, end_time: bigint | null, limit: number | null, cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 43);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);
  writeString(buf, account);
  writeU64BE(buf, BigInt.asUintN(64, start_time ?? 0n));
  buf.push(start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, end_time ?? 0n));
  buf.push(end_time != null ? 1 : 0);
  writeU32BE(buf, limit ?? 0);
  buf.push(limit != null ? 1 : 0);
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for FundingHistoryRequest: validates the header and reads the body. */
export function FundingHistoryRequestDecoderDecode(buf: Uint8Array): FundingHistoryRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 43) throw new Error("invalid template_id");
  let cursor: string | null = null;
  const account = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: FundingHistoryRequest = {
    account: account,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE message FundingHistoryBatch */
export interface FundingHistoryBatch {
  account: string;
  payments: FundingPayment[];
  has_more: number;
  next_cursor: string | null;
}

/** SBE body encoder for FundingHistoryBatch (fields only, no header). */
export function FundingHistoryBatchBodyEncoderEncode(value: FundingHistoryBatch, buf: number[]): void {
  writeString(buf, value.account);
  writeU32BE(buf, value.payments.length);
  for (const item of value.payments) {
    const itemBuf: number[] = [];
    FundingPaymentBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** SBE body decoder for FundingHistoryBatch: reads fields at `pos`. */
export function FundingHistoryBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): FundingHistoryBatch {
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const paymentsCount = readU32BE(buf, pos);
  const payments: FundingPayment[] = [];
  for (let i = 0; i < paymentsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = FundingPaymentBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    payments.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: FundingHistoryBatch = {
    account: account,
    payments: payments,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE encoder for FundingHistoryBatch — returns the complete message. */
export function FundingHistoryBatchEncoderEncode(account: string, payments: FundingPayment[], has_more: number, next_cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 44);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeU32BE(buf, payments.length);
  for (const item of payments) {
    const itemBuf: number[] = [];
    FundingPaymentBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for FundingHistoryBatch: validates the header and reads the body. */
export function FundingHistoryBatchDecoderDecode(buf: Uint8Array): FundingHistoryBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 44) throw new Error("invalid template_id");
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const paymentsCount = readU32BE(buf, pos);
  const payments: FundingPayment[] = [];
  for (let i = 0; i < paymentsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = FundingPaymentBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    payments.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: FundingHistoryBatch = {
    account: account,
    payments: payments,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE message LedgerUpdate */
export interface LedgerUpdate {
  account: string;
  asset: string;
  delta: number;
  kind: LedgerUpdateKind;
  timestamp: bigint;
  reference_id: string | null;
}

/** SBE body encoder for LedgerUpdate (fields only, no header). */
export function LedgerUpdateBodyEncoderEncode(value: LedgerUpdate, buf: number[]): void {
  writeString(buf, value.account);
  writeString(buf, value.asset);
  writeF64BE(buf, value.delta);
  buf.push(LedgerUpdateKindToValue(value.kind));
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  if (value.reference_id != null) { buf.push(1); writeString(buf, value.reference_id); } else { buf.push(0); }
}

/** SBE body decoder for LedgerUpdate: reads fields at `pos`. */
export function LedgerUpdateBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): LedgerUpdate {
  let reference_id: string | null = null;
  const account = readString(buf, pos);
  const asset = readString(buf, pos);
  const delta = readF64BE(buf, pos);
  const kindRaw = readU8(buf, pos);
  const kind = LedgerUpdateKindFromValue(kindRaw);
  if (kind == null) throw new Error('invalid LedgerUpdateKind');
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  if (readU8(buf, pos) === 1) reference_id = readString(buf, pos);
  const value: LedgerUpdate = {
    account: account,
    asset: asset,
    delta: delta,
    kind: kind,
    timestamp: timestamp,
    reference_id: reference_id,
  };
  return value;
}

/** SBE encoder for LedgerUpdate — returns the complete message. */
export function LedgerUpdateEncoderEncode(account: string, asset: string, delta: number, kind: LedgerUpdateKind, timestamp: bigint, reference_id: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 45);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);
  writeString(buf, account);
  writeString(buf, asset);
  writeF64BE(buf, delta);
  buf.push(LedgerUpdateKindToValue(kind));
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  if (reference_id != null) { buf.push(1); writeString(buf, reference_id); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for LedgerUpdate: validates the header and reads the body. */
export function LedgerUpdateDecoderDecode(buf: Uint8Array): LedgerUpdate {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 45) throw new Error("invalid template_id");
  let reference_id: string | null = null;
  const account = readString(buf, pos);
  const asset = readString(buf, pos);
  const delta = readF64BE(buf, pos);
  const kindRaw = readU8(buf, pos);
  const kind = LedgerUpdateKindFromValue(kindRaw);
  if (kind == null) throw new Error('invalid LedgerUpdateKind');
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  if (readU8(buf, pos) === 1) reference_id = readString(buf, pos);
  const value: LedgerUpdate = {
    account: account,
    asset: asset,
    delta: delta,
    kind: kind,
    timestamp: timestamp,
    reference_id: reference_id,
  };
  return value;
}

/** SBE message LedgerHistoryRequest */
export interface LedgerHistoryRequest {
  account: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: number | null;
  cursor: string | null;
}

/** SBE body encoder for LedgerHistoryRequest (fields only, no header). */
export function LedgerHistoryRequestBodyEncoderEncode(value: LedgerHistoryRequest, buf: number[]): void {
  writeString(buf, value.account);
  writeU64BE(buf, BigInt.asUintN(64, value.start_time ?? 0n));
  buf.push(value.start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.end_time ?? 0n));
  buf.push(value.end_time != null ? 1 : 0);
  writeU32BE(buf, value.limit ?? 0);
  buf.push(value.limit != null ? 1 : 0);
  if (value.cursor != null) { buf.push(1); writeString(buf, value.cursor); } else { buf.push(0); }
}

/** SBE body decoder for LedgerHistoryRequest: reads fields at `pos`. */
export function LedgerHistoryRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): LedgerHistoryRequest {
  let cursor: string | null = null;
  const account = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: LedgerHistoryRequest = {
    account: account,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE encoder for LedgerHistoryRequest — returns the complete message. */
export function LedgerHistoryRequestEncoderEncode(account: string, start_time: bigint | null, end_time: bigint | null, limit: number | null, cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 46);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);
  writeString(buf, account);
  writeU64BE(buf, BigInt.asUintN(64, start_time ?? 0n));
  buf.push(start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, end_time ?? 0n));
  buf.push(end_time != null ? 1 : 0);
  writeU32BE(buf, limit ?? 0);
  buf.push(limit != null ? 1 : 0);
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for LedgerHistoryRequest: validates the header and reads the body. */
export function LedgerHistoryRequestDecoderDecode(buf: Uint8Array): LedgerHistoryRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 46) throw new Error("invalid template_id");
  let cursor: string | null = null;
  const account = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: LedgerHistoryRequest = {
    account: account,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE message LedgerHistoryBatch */
export interface LedgerHistoryBatch {
  account: string;
  entries: LedgerUpdate[];
  has_more: number;
  next_cursor: string | null;
}

/** SBE body encoder for LedgerHistoryBatch (fields only, no header). */
export function LedgerHistoryBatchBodyEncoderEncode(value: LedgerHistoryBatch, buf: number[]): void {
  writeString(buf, value.account);
  writeU32BE(buf, value.entries.length);
  for (const item of value.entries) {
    const itemBuf: number[] = [];
    LedgerUpdateBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** SBE body decoder for LedgerHistoryBatch: reads fields at `pos`. */
export function LedgerHistoryBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): LedgerHistoryBatch {
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const entriesCount = readU32BE(buf, pos);
  const entries: LedgerUpdate[] = [];
  for (let i = 0; i < entriesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = LedgerUpdateBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    entries.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: LedgerHistoryBatch = {
    account: account,
    entries: entries,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE encoder for LedgerHistoryBatch — returns the complete message. */
export function LedgerHistoryBatchEncoderEncode(account: string, entries: LedgerUpdate[], has_more: number, next_cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 47);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeU32BE(buf, entries.length);
  for (const item of entries) {
    const itemBuf: number[] = [];
    LedgerUpdateBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for LedgerHistoryBatch: validates the header and reads the body. */
export function LedgerHistoryBatchDecoderDecode(buf: Uint8Array): LedgerHistoryBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 47) throw new Error("invalid template_id");
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const entriesCount = readU32BE(buf, pos);
  const entries: LedgerUpdate[] = [];
  for (let i = 0; i < entriesCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = LedgerUpdateBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    entries.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: LedgerHistoryBatch = {
    account: account,
    entries: entries,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE message OpenOrdersRequest */
export interface OpenOrdersRequest {
  account: string;
  symbol: string | null;
}

/** SBE body encoder for OpenOrdersRequest (fields only, no header). */
export function OpenOrdersRequestBodyEncoderEncode(value: OpenOrdersRequest, buf: number[]): void {
  writeString(buf, value.account);
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
}

/** SBE body decoder for OpenOrdersRequest: reads fields at `pos`. */
export function OpenOrdersRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OpenOrdersRequest {
  let symbol: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const value: OpenOrdersRequest = {
    account: account,
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for OpenOrdersRequest — returns the complete message. */
export function OpenOrdersRequestEncoderEncode(account: string, symbol: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 48);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for OpenOrdersRequest: validates the header and reads the body. */
export function OpenOrdersRequestDecoderDecode(buf: Uint8Array): OpenOrdersRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 48) throw new Error("invalid template_id");
  let symbol: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const value: OpenOrdersRequest = {
    account: account,
    symbol: symbol,
  };
  return value;
}

/** SBE message OpenOrdersSnapshot */
export interface OpenOrdersSnapshot {
  account: string;
  orders: ExecutionReport[];
  is_snapshot: number | null;
}

/** SBE body encoder for OpenOrdersSnapshot (fields only, no header). */
export function OpenOrdersSnapshotBodyEncoderEncode(value: OpenOrdersSnapshot, buf: number[]): void {
  writeString(buf, value.account);
  writeU32BE(buf, value.orders.length);
  for (const item of value.orders) {
    const itemBuf: number[] = [];
    ExecutionReportBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for OpenOrdersSnapshot: reads fields at `pos`. */
export function OpenOrdersSnapshotBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OpenOrdersSnapshot {
  const account = readString(buf, pos);
  const ordersCount = readU32BE(buf, pos);
  const orders: ExecutionReport[] = [];
  for (let i = 0; i < ordersCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = ExecutionReportBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    orders.push(item);
  }
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: OpenOrdersSnapshot = {
    account: account,
    orders: orders,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for OpenOrdersSnapshot — returns the complete message. */
export function OpenOrdersSnapshotEncoderEncode(account: string, orders: ExecutionReport[], is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 49);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeU32BE(buf, orders.length);
  for (const item of orders) {
    const itemBuf: number[] = [];
    ExecutionReportBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for OpenOrdersSnapshot: validates the header and reads the body. */
export function OpenOrdersSnapshotDecoderDecode(buf: Uint8Array): OpenOrdersSnapshot {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 49) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const ordersCount = readU32BE(buf, pos);
  const orders: ExecutionReport[] = [];
  for (let i = 0; i < ordersCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = ExecutionReportBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    orders.push(item);
  }
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: OpenOrdersSnapshot = {
    account: account,
    orders: orders,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message OrderHistoryRequest */
export interface OrderHistoryRequest {
  account: string;
  symbol: string | null;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: number | null;
  cursor: string | null;
}

/** SBE body encoder for OrderHistoryRequest (fields only, no header). */
export function OrderHistoryRequestBodyEncoderEncode(value: OrderHistoryRequest, buf: number[]): void {
  writeString(buf, value.account);
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
  writeU64BE(buf, BigInt.asUintN(64, value.start_time ?? 0n));
  buf.push(value.start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.end_time ?? 0n));
  buf.push(value.end_time != null ? 1 : 0);
  writeU32BE(buf, value.limit ?? 0);
  buf.push(value.limit != null ? 1 : 0);
  if (value.cursor != null) { buf.push(1); writeString(buf, value.cursor); } else { buf.push(0); }
}

/** SBE body decoder for OrderHistoryRequest: reads fields at `pos`. */
export function OrderHistoryRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OrderHistoryRequest {
  let symbol: string | null = null;
  let cursor: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: OrderHistoryRequest = {
    account: account,
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE encoder for OrderHistoryRequest — returns the complete message. */
export function OrderHistoryRequestEncoderEncode(account: string, symbol: string | null, start_time: bigint | null, end_time: bigint | null, limit: number | null, cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 50);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  writeU64BE(buf, BigInt.asUintN(64, start_time ?? 0n));
  buf.push(start_time != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, end_time ?? 0n));
  buf.push(end_time != null ? 1 : 0);
  writeU32BE(buf, limit ?? 0);
  buf.push(limit != null ? 1 : 0);
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for OrderHistoryRequest: validates the header and reads the body. */
export function OrderHistoryRequestDecoderDecode(buf: Uint8Array): OrderHistoryRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 50) throw new Error("invalid template_id");
  let symbol: string | null = null;
  let cursor: string | null = null;
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const start_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const start_time = buf[pos.value++] === 1 ? start_time_v : null;
  const end_time_v = BigInt.asIntN(64, readU64BE(buf, pos));
  const end_time = buf[pos.value++] === 1 ? end_time_v : null;
  const limit_v = readU32BE(buf, pos);
  const limit = buf[pos.value++] === 1 ? limit_v : null;
  if (readU8(buf, pos) === 1) cursor = readString(buf, pos);
  const value: OrderHistoryRequest = {
    account: account,
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
  };
  return value;
}

/** SBE message OrderHistoryBatch */
export interface OrderHistoryBatch {
  account: string;
  orders: ExecutionReport[];
  has_more: number;
  next_cursor: string | null;
}

/** SBE body encoder for OrderHistoryBatch (fields only, no header). */
export function OrderHistoryBatchBodyEncoderEncode(value: OrderHistoryBatch, buf: number[]): void {
  writeString(buf, value.account);
  writeU32BE(buf, value.orders.length);
  for (const item of value.orders) {
    const itemBuf: number[] = [];
    ExecutionReportBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(value.has_more);
  if (value.next_cursor != null) { buf.push(1); writeString(buf, value.next_cursor); } else { buf.push(0); }
}

/** SBE body decoder for OrderHistoryBatch: reads fields at `pos`. */
export function OrderHistoryBatchBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): OrderHistoryBatch {
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const ordersCount = readU32BE(buf, pos);
  const orders: ExecutionReport[] = [];
  for (let i = 0; i < ordersCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = ExecutionReportBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    orders.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: OrderHistoryBatch = {
    account: account,
    orders: orders,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE encoder for OrderHistoryBatch — returns the complete message. */
export function OrderHistoryBatchEncoderEncode(account: string, orders: ExecutionReport[], has_more: number, next_cursor: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 51);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, account);
  writeU32BE(buf, orders.length);
  for (const item of orders) {
    const itemBuf: number[] = [];
    ExecutionReportBodyEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for OrderHistoryBatch: validates the header and reads the body. */
export function OrderHistoryBatchDecoderDecode(buf: Uint8Array): OrderHistoryBatch {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 51) throw new Error("invalid template_id");
  let next_cursor: string | null = null;
  const account = readString(buf, pos);
  const ordersCount = readU32BE(buf, pos);
  const orders: ExecutionReport[] = [];
  for (let i = 0; i < ordersCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = ExecutionReportBodyDecoderDecode(buf, pos);
    pos.value = itemEnd;
    orders.push(item);
  }
  const has_more = readU8(buf, pos);
  if (readU8(buf, pos) === 1) next_cursor = readString(buf, pos);
  const value: OrderHistoryBatch = {
    account: account,
    orders: orders,
    has_more: has_more,
    next_cursor: next_cursor,
  };
  return value;
}

/** SBE message CapabilitiesRequest */
export interface CapabilitiesRequest {
}

/** SBE body encoder for CapabilitiesRequest (fields only, no header). */
export function CapabilitiesRequestBodyEncoderEncode(value: CapabilitiesRequest, buf: number[]): void {
}

/** SBE body decoder for CapabilitiesRequest: reads fields at `pos`. */
export function CapabilitiesRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CapabilitiesRequest {
  const value: CapabilitiesRequest = {
  };
  return value;
}

/** SBE encoder for CapabilitiesRequest — returns the complete message. */
export function CapabilitiesRequestEncoderEncode(): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 52);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  return new Uint8Array(buf);
}

/** SBE decoder for CapabilitiesRequest: validates the header and reads the body. */
export function CapabilitiesRequestDecoderDecode(buf: Uint8Array): CapabilitiesRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 52) throw new Error("invalid template_id");
  const value: CapabilitiesRequest = {
  };
  return value;
}

/** SBE message CapabilitiesResponse */
export interface CapabilitiesResponse {
  schema_ids: number[];
  paths: CapabilityPath[];
  symbols: string[];
  intervals: string[];
}

/** SBE body encoder for CapabilitiesResponse (fields only, no header). */
export function CapabilitiesResponseBodyEncoderEncode(value: CapabilitiesResponse, buf: number[]): void {
  writeU32BE(buf, value.schema_ids.length);
  for (const item of value.schema_ids) {
  buf.push(item);
  }
  writeU32BE(buf, value.paths.length);
  for (const item of value.paths) {
    const itemBuf: number[] = [];
    CapabilityPathEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU32BE(buf, value.symbols.length);
  for (const item of value.symbols) {
    writeStringU32(buf, item);
  }
  writeU32BE(buf, value.intervals.length);
  for (const item of value.intervals) {
    writeStringU32(buf, item);
  }
}

/** SBE body decoder for CapabilitiesResponse: reads fields at `pos`. */
export function CapabilitiesResponseBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): CapabilitiesResponse {
  const schema_idsCount = readU32BE(buf, pos);
  const schema_ids: number[] = [];
  for (let i = 0; i < schema_idsCount; i++) {
    schema_ids.push(readU8(buf, pos));
  }
  const pathsCount = readU32BE(buf, pos);
  const paths: CapabilityPath[] = [];
  for (let i = 0; i < pathsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = CapabilityPathDecoderDecode(buf, pos);
    pos.value = itemEnd;
    paths.push(item);
  }
  const symbolsCount = readU32BE(buf, pos);
  const symbols: string[] = [];
  for (let i = 0; i < symbolsCount; i++) {
    symbols.push(readStringU32(buf, pos));
  }
  const intervalsCount = readU32BE(buf, pos);
  const intervals: string[] = [];
  for (let i = 0; i < intervalsCount; i++) {
    intervals.push(readStringU32(buf, pos));
  }
  const value: CapabilitiesResponse = {
    schema_ids: schema_ids,
    paths: paths,
    symbols: symbols,
    intervals: intervals,
  };
  return value;
}

/** SBE encoder for CapabilitiesResponse — returns the complete message. */
export function CapabilitiesResponseEncoderEncode(schema_ids: number[], paths: CapabilityPath[], symbols: string[], intervals: string[]): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 53);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeU32BE(buf, schema_ids.length);
  for (const item of schema_ids) {
  buf.push(item);
  }
  writeU32BE(buf, paths.length);
  for (const item of paths) {
    const itemBuf: number[] = [];
    CapabilityPathEncoderEncode(item, itemBuf);
    writeU32BE(buf, itemBuf.length);
    for (const b of itemBuf) buf.push(b);
  }
  writeU32BE(buf, symbols.length);
  for (const item of symbols) {
    writeStringU32(buf, item);
  }
  writeU32BE(buf, intervals.length);
  for (const item of intervals) {
    writeStringU32(buf, item);
  }
  return new Uint8Array(buf);
}

/** SBE decoder for CapabilitiesResponse: validates the header and reads the body. */
export function CapabilitiesResponseDecoderDecode(buf: Uint8Array): CapabilitiesResponse {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 53) throw new Error("invalid template_id");
  const schema_idsCount = readU32BE(buf, pos);
  const schema_ids: number[] = [];
  for (let i = 0; i < schema_idsCount; i++) {
    schema_ids.push(readU8(buf, pos));
  }
  const pathsCount = readU32BE(buf, pos);
  const paths: CapabilityPath[] = [];
  for (let i = 0; i < pathsCount; i++) {
    const itemLen = readU32BE(buf, pos);
    const itemEnd = pos.value + itemLen;
    const item = CapabilityPathDecoderDecode(buf, pos);
    pos.value = itemEnd;
    paths.push(item);
  }
  const symbolsCount = readU32BE(buf, pos);
  const symbols: string[] = [];
  for (let i = 0; i < symbolsCount; i++) {
    symbols.push(readStringU32(buf, pos));
  }
  const intervalsCount = readU32BE(buf, pos);
  const intervals: string[] = [];
  for (let i = 0; i < intervalsCount; i++) {
    intervals.push(readStringU32(buf, pos));
  }
  const value: CapabilitiesResponse = {
    schema_ids: schema_ids,
    paths: paths,
    symbols: symbols,
    intervals: intervals,
  };
  return value;
}

/** SBE message PositionRequest */
export interface PositionRequest {
  account: string;
}

/** SBE body encoder for PositionRequest (fields only, no header). */
export function PositionRequestBodyEncoderEncode(value: PositionRequest, buf: number[]): void {
  writeString(buf, value.account);
}

/** SBE body decoder for PositionRequest: reads fields at `pos`. */
export function PositionRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): PositionRequest {
  const account = readString(buf, pos);
  const value: PositionRequest = {
    account: account,
  };
  return value;
}

/** SBE encoder for PositionRequest — returns the complete message. */
export function PositionRequestEncoderEncode(account: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 54);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeString(buf, account);
  return new Uint8Array(buf);
}

/** SBE decoder for PositionRequest: validates the header and reads the body. */
export function PositionRequestDecoderDecode(buf: Uint8Array): PositionRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 54) throw new Error("invalid template_id");
  const account = readString(buf, pos);
  const value: PositionRequest = {
    account: account,
  };
  return value;
}

/** SBE message TradingSessionStatusRequest */
export interface TradingSessionStatusRequest {
  symbol: string | null;
}

/** SBE body encoder for TradingSessionStatusRequest (fields only, no header). */
export function TradingSessionStatusRequestBodyEncoderEncode(value: TradingSessionStatusRequest, buf: number[]): void {
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
}

/** SBE body decoder for TradingSessionStatusRequest: reads fields at `pos`. */
export function TradingSessionStatusRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): TradingSessionStatusRequest {
  let symbol: string | null = null;
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const value: TradingSessionStatusRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for TradingSessionStatusRequest — returns the complete message. */
export function TradingSessionStatusRequestEncoderEncode(symbol: string | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 55);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  return new Uint8Array(buf);
}

/** SBE decoder for TradingSessionStatusRequest: validates the header and reads the body. */
export function TradingSessionStatusRequestDecoderDecode(buf: Uint8Array): TradingSessionStatusRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 55) throw new Error("invalid template_id");
  let symbol: string | null = null;
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const value: TradingSessionStatusRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE message TradingSessionStatus */
export interface TradingSessionStatus {
  symbol: string | null;
  trading_session_id: string;
  status: TradingSessionStatusStatus;
  halt_reason: TradingSessionStatusHaltReason | null;
  timestamp: bigint;
  is_snapshot: number | null;
}

/** SBE body encoder for TradingSessionStatus (fields only, no header). */
export function TradingSessionStatusBodyEncoderEncode(value: TradingSessionStatus, buf: number[]): void {
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
  writeString(buf, value.trading_session_id);
  buf.push(TradingSessionStatusStatusToValue(value.status));
  buf.push(value.halt_reason != null ? TradingSessionStatusHaltReasonToValue(value.halt_reason) : 0);
  buf.push(value.halt_reason != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for TradingSessionStatus: reads fields at `pos`. */
export function TradingSessionStatusBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): TradingSessionStatus {
  let symbol: string | null = null;
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const trading_session_id = readString(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = TradingSessionStatusStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid TradingSessionStatusStatus');
  const halt_reasonRaw = readU8(buf, pos);
  let halt_reason: TradingSessionStatusHaltReason | null = null;
  if (buf[pos.value++] === 1) {
    const v = TradingSessionStatusHaltReasonFromValue(halt_reasonRaw);
    if (v == null) throw new Error('invalid TradingSessionStatusHaltReason');
    halt_reason = v;
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: TradingSessionStatus = {
    symbol: symbol,
    trading_session_id: trading_session_id,
    status: status,
    halt_reason: halt_reason,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for TradingSessionStatus — returns the complete message. */
export function TradingSessionStatusEncoderEncode(symbol: string | null, trading_session_id: string, status: TradingSessionStatusStatus, halt_reason: TradingSessionStatusHaltReason | null, timestamp: bigint, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 56);
  writeU16BE(buf, 0);
  writeU16BE(buf, 11);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  writeString(buf, trading_session_id);
  buf.push(TradingSessionStatusStatusToValue(status));
  buf.push(halt_reason != null ? TradingSessionStatusHaltReasonToValue(halt_reason) : 0);
  buf.push(halt_reason != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for TradingSessionStatus: validates the header and reads the body. */
export function TradingSessionStatusDecoderDecode(buf: Uint8Array): TradingSessionStatus {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 56) throw new Error("invalid template_id");
  let symbol: string | null = null;
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const trading_session_id = readString(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = TradingSessionStatusStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid TradingSessionStatusStatus');
  const halt_reasonRaw = readU8(buf, pos);
  let halt_reason: TradingSessionStatusHaltReason | null = null;
  if (buf[pos.value++] === 1) {
    const v = TradingSessionStatusHaltReasonFromValue(halt_reasonRaw);
    if (v == null) throw new Error('invalid TradingSessionStatusHaltReason');
    halt_reason = v;
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: TradingSessionStatus = {
    symbol: symbol,
    trading_session_id: trading_session_id,
    status: status,
    halt_reason: halt_reason,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message SecurityStatusRequest */
export interface SecurityStatusRequest {
  symbol: string;
}

/** SBE body encoder for SecurityStatusRequest (fields only, no header). */
export function SecurityStatusRequestBodyEncoderEncode(value: SecurityStatusRequest, buf: number[]): void {
  writeString(buf, value.symbol);
}

/** SBE body decoder for SecurityStatusRequest: reads fields at `pos`. */
export function SecurityStatusRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): SecurityStatusRequest {
  const symbol = readString(buf, pos);
  const value: SecurityStatusRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for SecurityStatusRequest — returns the complete message. */
export function SecurityStatusRequestEncoderEncode(symbol: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 57);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeString(buf, symbol);
  return new Uint8Array(buf);
}

/** SBE decoder for SecurityStatusRequest: validates the header and reads the body. */
export function SecurityStatusRequestDecoderDecode(buf: Uint8Array): SecurityStatusRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 57) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const value: SecurityStatusRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE message SecurityStatus */
export interface SecurityStatus {
  symbol: string;
  status: SecurityStatusStatus;
  halt_reason: SecurityStatusHaltReason | null;
  timestamp: bigint;
  is_snapshot: number | null;
}

/** SBE body encoder for SecurityStatus (fields only, no header). */
export function SecurityStatusBodyEncoderEncode(value: SecurityStatus, buf: number[]): void {
  writeString(buf, value.symbol);
  buf.push(SecurityStatusStatusToValue(value.status));
  buf.push(value.halt_reason != null ? SecurityStatusHaltReasonToValue(value.halt_reason) : 0);
  buf.push(value.halt_reason != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for SecurityStatus: reads fields at `pos`. */
export function SecurityStatusBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): SecurityStatus {
  const symbol = readString(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = SecurityStatusStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid SecurityStatusStatus');
  const halt_reasonRaw = readU8(buf, pos);
  let halt_reason: SecurityStatusHaltReason | null = null;
  if (buf[pos.value++] === 1) {
    const v = SecurityStatusHaltReasonFromValue(halt_reasonRaw);
    if (v == null) throw new Error('invalid SecurityStatusHaltReason');
    halt_reason = v;
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: SecurityStatus = {
    symbol: symbol,
    status: status,
    halt_reason: halt_reason,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for SecurityStatus — returns the complete message. */
export function SecurityStatusEncoderEncode(symbol: string, status: SecurityStatusStatus, halt_reason: SecurityStatusHaltReason | null, timestamp: bigint, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 58);
  writeU16BE(buf, 0);
  writeU16BE(buf, 11);
  writeString(buf, symbol);
  buf.push(SecurityStatusStatusToValue(status));
  buf.push(halt_reason != null ? SecurityStatusHaltReasonToValue(halt_reason) : 0);
  buf.push(halt_reason != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for SecurityStatus: validates the header and reads the body. */
export function SecurityStatusDecoderDecode(buf: Uint8Array): SecurityStatus {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 58) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = SecurityStatusStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid SecurityStatusStatus');
  const halt_reasonRaw = readU8(buf, pos);
  let halt_reason: SecurityStatusHaltReason | null = null;
  if (buf[pos.value++] === 1) {
    const v = SecurityStatusHaltReasonFromValue(halt_reasonRaw);
    if (v == null) throw new Error('invalid SecurityStatusHaltReason');
    halt_reason = v;
  }
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: SecurityStatus = {
    symbol: symbol,
    status: status,
    halt_reason: halt_reason,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE message MassCancelRequest */
export interface MassCancelRequest {
  cl_ord_id: string;
  account: string;
  symbol: string | null;
  side: MassCancelRequestSide | null;
}

/** SBE body encoder for MassCancelRequest (fields only, no header). */
export function MassCancelRequestBodyEncoderEncode(value: MassCancelRequest, buf: number[]): void {
  writeString(buf, value.cl_ord_id);
  writeString(buf, value.account);
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
  buf.push(value.side != null ? MassCancelRequestSideToValue(value.side) : 0);
  buf.push(value.side != null ? 1 : 0);
}

/** SBE body decoder for MassCancelRequest: reads fields at `pos`. */
export function MassCancelRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MassCancelRequest {
  let symbol: string | null = null;
  const cl_ord_id = readString(buf, pos);
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  let side: MassCancelRequestSide | null = null;
  if (buf[pos.value++] === 1) {
    const v = MassCancelRequestSideFromValue(sideRaw);
    if (v == null) throw new Error('invalid MassCancelRequestSide');
    side = v;
  }
  const value: MassCancelRequest = {
    cl_ord_id: cl_ord_id,
    account: account,
    symbol: symbol,
    side: side,
  };
  return value;
}

/** SBE encoder for MassCancelRequest — returns the complete message. */
export function MassCancelRequestEncoderEncode(cl_ord_id: string, account: string, symbol: string | null, side: MassCancelRequestSide | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 59);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);
  writeString(buf, cl_ord_id);
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  buf.push(side != null ? MassCancelRequestSideToValue(side) : 0);
  buf.push(side != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for MassCancelRequest: validates the header and reads the body. */
export function MassCancelRequestDecoderDecode(buf: Uint8Array): MassCancelRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 59) throw new Error("invalid template_id");
  let symbol: string | null = null;
  const cl_ord_id = readString(buf, pos);
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const sideRaw = readU8(buf, pos);
  let side: MassCancelRequestSide | null = null;
  if (buf[pos.value++] === 1) {
    const v = MassCancelRequestSideFromValue(sideRaw);
    if (v == null) throw new Error('invalid MassCancelRequestSide');
    side = v;
  }
  const value: MassCancelRequest = {
    cl_ord_id: cl_ord_id,
    account: account,
    symbol: symbol,
    side: side,
  };
  return value;
}

/** SBE message MassCancelReport */
export interface MassCancelReport {
  cl_ord_id: string;
  account: string;
  symbol: string | null;
  total_affected: number;
  status: MassCancelReportStatus;
  timestamp: bigint;
}

/** SBE body encoder for MassCancelReport (fields only, no header). */
export function MassCancelReportBodyEncoderEncode(value: MassCancelReport, buf: number[]): void {
  writeString(buf, value.cl_ord_id);
  writeString(buf, value.account);
  if (value.symbol != null) { buf.push(1); writeString(buf, value.symbol); } else { buf.push(0); }
  writeU32BE(buf, value.total_affected);
  buf.push(MassCancelReportStatusToValue(value.status));
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
}

/** SBE body decoder for MassCancelReport: reads fields at `pos`. */
export function MassCancelReportBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): MassCancelReport {
  let symbol: string | null = null;
  const cl_ord_id = readString(buf, pos);
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const total_affected = readU32BE(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = MassCancelReportStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid MassCancelReportStatus');
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: MassCancelReport = {
    cl_ord_id: cl_ord_id,
    account: account,
    symbol: symbol,
    total_affected: total_affected,
    status: status,
    timestamp: timestamp,
  };
  return value;
}

/** SBE encoder for MassCancelReport — returns the complete message. */
export function MassCancelReportEncoderEncode(cl_ord_id: string, account: string, symbol: string | null, total_affected: number, status: MassCancelReportStatus, timestamp: bigint): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 60);
  writeU16BE(buf, 0);
  writeU16BE(buf, 13);
  writeString(buf, cl_ord_id);
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  writeU32BE(buf, total_affected);
  buf.push(MassCancelReportStatusToValue(status));
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  return new Uint8Array(buf);
}

/** SBE decoder for MassCancelReport: validates the header and reads the body. */
export function MassCancelReportDecoderDecode(buf: Uint8Array): MassCancelReport {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 60) throw new Error("invalid template_id");
  let symbol: string | null = null;
  const cl_ord_id = readString(buf, pos);
  const account = readString(buf, pos);
  if (readU8(buf, pos) === 1) symbol = readString(buf, pos);
  const total_affected = readU32BE(buf, pos);
  const statusRaw = readU8(buf, pos);
  const status = MassCancelReportStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid MassCancelReportStatus');
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const value: MassCancelReport = {
    cl_ord_id: cl_ord_id,
    account: account,
    symbol: symbol,
    total_affected: total_affected,
    status: status,
    timestamp: timestamp,
  };
  return value;
}

/** SBE message PriceLimitsRequest */
export interface PriceLimitsRequest {
  symbol: string;
}

/** SBE body encoder for PriceLimitsRequest (fields only, no header). */
export function PriceLimitsRequestBodyEncoderEncode(value: PriceLimitsRequest, buf: number[]): void {
  writeString(buf, value.symbol);
}

/** SBE body decoder for PriceLimitsRequest: reads fields at `pos`. */
export function PriceLimitsRequestBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): PriceLimitsRequest {
  const symbol = readString(buf, pos);
  const value: PriceLimitsRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE encoder for PriceLimitsRequest — returns the complete message. */
export function PriceLimitsRequestEncoderEncode(symbol: string): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 61);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);
  writeString(buf, symbol);
  return new Uint8Array(buf);
}

/** SBE decoder for PriceLimitsRequest: validates the header and reads the body. */
export function PriceLimitsRequestDecoderDecode(buf: Uint8Array): PriceLimitsRequest {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 61) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const value: PriceLimitsRequest = {
    symbol: symbol,
  };
  return value;
}

/** SBE message PriceLimits */
export interface PriceLimits {
  symbol: string;
  lower_limit: number | null;
  upper_limit: number | null;
  reference_price: number | null;
  timestamp: bigint;
  is_snapshot: number | null;
}

/** SBE body encoder for PriceLimits (fields only, no header). */
export function PriceLimitsBodyEncoderEncode(value: PriceLimits, buf: number[]): void {
  writeString(buf, value.symbol);
  writeF64BE(buf, value.lower_limit ?? 0);
  buf.push(value.lower_limit != null ? 1 : 0);
  writeF64BE(buf, value.upper_limit ?? 0);
  buf.push(value.upper_limit != null ? 1 : 0);
  writeF64BE(buf, value.reference_price ?? 0);
  buf.push(value.reference_price != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, value.timestamp));
  buf.push(value.is_snapshot ?? 0);
  buf.push(value.is_snapshot != null ? 1 : 0);
}

/** SBE body decoder for PriceLimits: reads fields at `pos`. */
export function PriceLimitsBodyDecoderDecode(buf: Uint8Array, pos: { value: number }): PriceLimits {
  const symbol = readString(buf, pos);
  const lower_limit_v = readF64BE(buf, pos);
  const lower_limit = buf[pos.value++] === 1 ? lower_limit_v : null;
  const upper_limit_v = readF64BE(buf, pos);
  const upper_limit = buf[pos.value++] === 1 ? upper_limit_v : null;
  const reference_price_v = readF64BE(buf, pos);
  const reference_price = buf[pos.value++] === 1 ? reference_price_v : null;
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: PriceLimits = {
    symbol: symbol,
    lower_limit: lower_limit,
    upper_limit: upper_limit,
    reference_price: reference_price,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

/** SBE encoder for PriceLimits — returns the complete message. */
export function PriceLimitsEncoderEncode(symbol: string, lower_limit: number | null, upper_limit: number | null, reference_price: number | null, timestamp: bigint, is_snapshot: number | null): Uint8Array {
  const buf: number[] = [];
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 62);
  writeU16BE(buf, 0);
  writeU16BE(buf, 33);
  writeString(buf, symbol);
  writeF64BE(buf, lower_limit ?? 0);
  buf.push(lower_limit != null ? 1 : 0);
  writeF64BE(buf, upper_limit ?? 0);
  buf.push(upper_limit != null ? 1 : 0);
  writeF64BE(buf, reference_price ?? 0);
  buf.push(reference_price != null ? 1 : 0);
  writeU64BE(buf, BigInt.asUintN(64, timestamp));
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);
  return new Uint8Array(buf);
}

/** SBE decoder for PriceLimits: validates the header and reads the body. */
export function PriceLimitsDecoderDecode(buf: Uint8Array): PriceLimits {
  if (buf.length < 8) throw new Error("buffer too short for SBE header");
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos);
  readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error("invalid schema_id");
  if (tmplId !== 62) throw new Error("invalid template_id");
  const symbol = readString(buf, pos);
  const lower_limit_v = readF64BE(buf, pos);
  const lower_limit = buf[pos.value++] === 1 ? lower_limit_v : null;
  const upper_limit_v = readF64BE(buf, pos);
  const upper_limit = buf[pos.value++] === 1 ? upper_limit_v : null;
  const reference_price_v = readF64BE(buf, pos);
  const reference_price = buf[pos.value++] === 1 ? reference_price_v : null;
  const timestamp = BigInt.asIntN(64, readU64BE(buf, pos));
  const is_snapshot_v = readU8(buf, pos);
  const is_snapshot = buf[pos.value++] === 1 ? is_snapshot_v : null;
  const value: PriceLimits = {
    symbol: symbol,
    lower_limit: lower_limit,
    upper_limit: upper_limit,
    reference_price: reference_price,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
  };
  return value;
}

