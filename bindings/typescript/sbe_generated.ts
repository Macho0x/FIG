// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' vv1.0.0
// Standard order entry and execution messages

export const SCHEMA_ID = 0x01 as const;

function writeU16BE(buf: number[], v: number): void { buf.push((v >> 8) & 0xff, v & 0xff); }
function writeU32BE(buf: number[], v: number): void { buf.push((v >> 24) & 0xff, (v >> 16) & 0xff, (v >> 8) & 0xff, v & 0xff); }
function writeU64BE(buf: number[], v: bigint): void { for (let i = 7; i >= 0; i--) buf.push(Number((v >> BigInt(i * 8)) & 0xffn)); }
function writeF64BE(buf: number[], v: number): void { const dv = new DataView(new ArrayBuffer(8)); dv.setFloat64(0, v, false); for (let i = 0; i < 8; i++) buf.push(dv.getUint8(i)); }
function writeI64BE(buf: number[], v: bigint): void { writeU64BE(buf, BigInt.asUintN(64, v)); }
function writeString(buf: number[], s: string): void { const bytes = new TextEncoder().encode(s); writeU16BE(buf, bytes.length); for (const b of bytes) buf.push(b); }
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

/** SBE encoder for NewOrderSingle */
export function NewOrderSingleEncoderEncode(cl_ord_id string, side NewOrderSingleSide, order_qty number, price number | null, stop_price number | null, symbol string, order_type NewOrderSingleOrderType, time_in_force NewOrderSingleTimeInForce, expire_time bigint | null, account string | null, strategy_id string | null, security_id string | null, id_source NewOrderSingleIdSource | null, security_exchange string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 1);
  writeU16BE(buf, 0);
  writeU16BE(buf, 36);

  // Fixed fields
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
  if (expire_time != null) { buf.push(1); writeI64BE(buf, expire_time); } else { buf.push(0); }
  if (account != null) { buf.push(1); writeString(buf, account); } else { buf.push(0); }
  if (strategy_id != null) { buf.push(1); writeString(buf, strategy_id); } else { buf.push(0); }
  if (security_id != null) { buf.push(1); writeString(buf, security_id); } else { buf.push(0); }
  buf.push(id_source != null ? NewOrderSingleIdSourceToValue(id_source) : 0);
  if (security_exchange != null) { buf.push(1); writeString(buf, security_exchange); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for NewOrderSingle */
export interface NewOrderSingleDecoder {
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
  encodedLen(): number;
}

export function NewOrderSingleDecoderDecode(buf: Uint8Array): NewOrderSingleDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 1) throw new Error('invalid template_id');
  const cl_ord_id = readString(buf, pos);
  const sideRaw = buf[pos.value++];
  const side = NewOrderSingleSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid NewOrderSingleSide');
  const order_qty = readF64BE(buf, pos);
  const priceRaw = readF64BE(buf, pos);
  const price = buf[pos.value++] === 1 ? priceRaw : null;
  const stop_priceRaw = readF64BE(buf, pos);
  const stop_price = buf[pos.value++] === 1 ? stop_priceRaw : null;
  const symbol = readString(buf, pos);
  const order_typeRaw = buf[pos.value++];
  const order_type = NewOrderSingleOrderTypeFromValue(order_typeRaw);
  if (order_type == null) throw new Error('invalid NewOrderSingleOrderType');
  const time_in_forceRaw = buf[pos.value++];
  const time_in_force = NewOrderSingleTimeInForceFromValue(time_in_forceRaw);
  if (time_in_force == null) throw new Error('invalid NewOrderSingleTimeInForce');
  let expire_time: bigint | null = null;
  if (buf[pos.value++] === 1) expire_time = readI64BE(buf, pos);
  let account: string | null;
  if (buf[pos.value++] === 1) account = readString(buf, pos); else account = null;
  let strategy_id: string | null;
  if (buf[pos.value++] === 1) strategy_id = readString(buf, pos); else strategy_id = null;
  let security_id: string | null;
  if (buf[pos.value++] === 1) security_id = readString(buf, pos); else security_id = null;
  const id_sourceRaw = buf[pos.value++];
  const id_source = NewOrderSingleIdSourceFromValue(id_sourceRaw);
  if (id_source == null) throw new Error('invalid NewOrderSingleIdSource');
  let security_exchange: string | null;
  if (buf[pos.value++] === 1) security_exchange = readString(buf, pos); else security_exchange = null;
  return {
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
    encodedLen: () => 0,
  };
}

/** SBE encoder for CancelRequest */
export function CancelRequestEncoderEncode(cl_ord_id string, orig_cl_ord_id string, symbol string, side CancelRequestSide, order_qty number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 2);
  writeU16BE(buf, 0);
  writeU16BE(buf, 9);

  // Fixed fields
  writeString(buf, cl_ord_id);
  writeString(buf, orig_cl_ord_id);
  writeString(buf, symbol);
  buf.push(CancelRequestSideToValue(side));
  writeF64BE(buf, order_qty ?? 0);
  buf.push(order_qty != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for CancelRequest */
export interface CancelRequestDecoder {
  cl_ord_id: string;
  orig_cl_ord_id: string;
  symbol: string;
  side: CancelRequestSide;
  order_qty: number | null;
  encodedLen(): number;
}

export function CancelRequestDecoderDecode(buf: Uint8Array): CancelRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 2) throw new Error('invalid template_id');
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const symbol = readString(buf, pos);
  const sideRaw = buf[pos.value++];
  const side = CancelRequestSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid CancelRequestSide');
  const order_qtyRaw = readF64BE(buf, pos);
  const order_qty = buf[pos.value++] === 1 ? order_qtyRaw : null;
  return {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    symbol: symbol,
    side: side,
    order_qty: order_qty,
    encodedLen: () => 0,
  };
}

/** SBE encoder for CancelReplaceRequest */
export function CancelReplaceRequestEncoderEncode(cl_ord_id string, orig_cl_ord_id string, symbol string, side CancelReplaceRequestSide, order_qty number, price number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 3);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);

  // Fixed fields
  writeString(buf, cl_ord_id);
  writeString(buf, orig_cl_ord_id);
  writeString(buf, symbol);
  buf.push(CancelReplaceRequestSideToValue(side));
  writeF64BE(buf, order_qty);
  writeF64BE(buf, price ?? 0);
  buf.push(price != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for CancelReplaceRequest */
export interface CancelReplaceRequestDecoder {
  cl_ord_id: string;
  orig_cl_ord_id: string;
  symbol: string;
  side: CancelReplaceRequestSide;
  order_qty: number;
  price: number | null;
  encodedLen(): number;
}

export function CancelReplaceRequestDecoderDecode(buf: Uint8Array): CancelReplaceRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 3) throw new Error('invalid template_id');
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const symbol = readString(buf, pos);
  const sideRaw = buf[pos.value++];
  const side = CancelReplaceRequestSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid CancelReplaceRequestSide');
  const order_qty = readF64BE(buf, pos);
  const priceRaw = readF64BE(buf, pos);
  const price = buf[pos.value++] === 1 ? priceRaw : null;
  return {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    symbol: symbol,
    side: side,
    order_qty: order_qty,
    price: price,
    encodedLen: () => 0,
  };
}

/** SBE encoder for ExecutionReport */
export function ExecutionReportEncoderEncode(cl_ord_id string, order_id string, exec_id string, exec_type ExecutionReportExecType, ord_status ExecutionReportOrdStatus, side ExecutionReportSide, last_qty number | null, last_price number | null, leaves_qty number, cum_qty number, avg_price number, symbol string, transact_time bigint): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 4);
  writeU16BE(buf, 0);
  writeU16BE(buf, 51);

  // Fixed fields
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
  writeI64BE(buf, transact_time);

  return new Uint8Array(buf);
}

/** SBE decoder for ExecutionReport */
export interface ExecutionReportDecoder {
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
  encodedLen(): number;
}

export function ExecutionReportDecoderDecode(buf: Uint8Array): ExecutionReportDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 4) throw new Error('invalid template_id');
  const cl_ord_id = readString(buf, pos);
  const order_id = readString(buf, pos);
  const exec_id = readString(buf, pos);
  const exec_typeRaw = buf[pos.value++];
  const exec_type = ExecutionReportExecTypeFromValue(exec_typeRaw);
  if (exec_type == null) throw new Error('invalid ExecutionReportExecType');
  const ord_statusRaw = buf[pos.value++];
  const ord_status = ExecutionReportOrdStatusFromValue(ord_statusRaw);
  if (ord_status == null) throw new Error('invalid ExecutionReportOrdStatus');
  const sideRaw = buf[pos.value++];
  const side = ExecutionReportSideFromValue(sideRaw);
  if (side == null) throw new Error('invalid ExecutionReportSide');
  const last_qtyRaw = readF64BE(buf, pos);
  const last_qty = buf[pos.value++] === 1 ? last_qtyRaw : null;
  const last_priceRaw = readF64BE(buf, pos);
  const last_price = buf[pos.value++] === 1 ? last_priceRaw : null;
  const leaves_qty = readF64BE(buf, pos);
  const cum_qty = readF64BE(buf, pos);
  const avg_price = readF64BE(buf, pos);
  const symbol = readString(buf, pos);
  const transact_time = readI64BE(buf, pos);
  return {
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
    encodedLen: () => 0,
  };
}

/** SBE encoder for CancelReject */
export function CancelRejectEncoderEncode(cl_ord_id string, orig_cl_ord_id string, reject_reason CancelRejectRejectReason, symbol string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 5);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, cl_ord_id);
  writeString(buf, orig_cl_ord_id);
  buf.push(CancelRejectRejectReasonToValue(reject_reason));
  writeString(buf, symbol);

  return new Uint8Array(buf);
}

/** SBE decoder for CancelReject */
export interface CancelRejectDecoder {
  cl_ord_id: string;
  orig_cl_ord_id: string;
  reject_reason: CancelRejectRejectReason;
  symbol: string;
  encodedLen(): number;
}

export function CancelRejectDecoderDecode(buf: Uint8Array): CancelRejectDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 5) throw new Error('invalid template_id');
  const cl_ord_id = readString(buf, pos);
  const orig_cl_ord_id = readString(buf, pos);
  const reject_reasonRaw = buf[pos.value++];
  const reject_reason = CancelRejectRejectReasonFromValue(reject_reasonRaw);
  if (reject_reason == null) throw new Error('invalid CancelRejectRejectReason');
  const symbol = readString(buf, pos);
  return {
    cl_ord_id: cl_ord_id,
    orig_cl_ord_id: orig_cl_ord_id,
    reject_reason: reject_reason,
    symbol: symbol,
    encodedLen: () => 0,
  };
}

/** SBE encoder for MarketDataSnapshot */
export function MarketDataSnapshotEncoderEncode(symbol string, exchange string, bids string[], asks string[], timestamp bigint, sequence bigint | null, is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 6);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);

  // Fixed fields
  writeString(buf, symbol);
  writeString(buf, exchange);
  writeU32BE(buf, bids.length);
  for (const item of bids) {
    PriceLevelEncode(item, buf);
  }
  writeU32BE(buf, asks.length);
  for (const item of asks) {
    PriceLevelEncode(item, buf);
  }
  writeI64BE(buf, timestamp);
  if (sequence != null) { buf.push(1); writeI64BE(buf, sequence); } else { buf.push(0); }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for MarketDataSnapshot */
export interface MarketDataSnapshotDecoder {
  symbol: string;
  exchange: string;
  bids: string[];
  asks: string[];
  timestamp: bigint;
  sequence: bigint | null;
  is_snapshot: number | null;
  encodedLen(): number;
}

export function MarketDataSnapshotDecoderDecode(buf: Uint8Array): MarketDataSnapshotDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 6) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const exchange = readString(buf, pos);
  const bidsCount = readU32BE(buf, pos);
  const bids: PriceLevel[] = [];
  for (let i = 0; i < bidsCount; i++) {
    const item = PriceLevelDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    bids.push(item);
  }
  const asksCount = readU32BE(buf, pos);
  const asks: PriceLevel[] = [];
  for (let i = 0; i < asksCount; i++) {
    const item = PriceLevelDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    asks.push(item);
  }
  const timestamp = readI64BE(buf, pos);
  let sequence: bigint | null = null;
  if (buf[pos.value++] === 1) sequence = readI64BE(buf, pos);
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    symbol: symbol,
    exchange: exchange,
    bids: bids,
    asks: asks,
    timestamp: timestamp,
    sequence: sequence,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for MarketDataIncrementalRefresh */
export function MarketDataIncrementalRefreshEncoderEncode(symbol string, updates string[], timestamp bigint, sequence bigint | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 7);
  writeU16BE(buf, 0);
  writeU16BE(buf, 16);

  // Fixed fields
  writeString(buf, symbol);
  writeU32BE(buf, updates.length);
  for (const item of updates) {
    MarketDataUpdateEncode(item, buf);
  }
  writeI64BE(buf, timestamp);
  if (sequence != null) { buf.push(1); writeI64BE(buf, sequence); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for MarketDataIncrementalRefresh */
export interface MarketDataIncrementalRefreshDecoder {
  symbol: string;
  updates: string[];
  timestamp: bigint;
  sequence: bigint | null;
  encodedLen(): number;
}

export function MarketDataIncrementalRefreshDecoderDecode(buf: Uint8Array): MarketDataIncrementalRefreshDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 7) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const updatesCount = readU32BE(buf, pos);
  const updates: MarketDataUpdate[] = [];
  for (let i = 0; i < updatesCount; i++) {
    const item = MarketDataUpdateDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    updates.push(item);
  }
  const timestamp = readI64BE(buf, pos);
  let sequence: bigint | null = null;
  if (buf[pos.value++] === 1) sequence = readI64BE(buf, pos);
  return {
    symbol: symbol,
    updates: updates,
    timestamp: timestamp,
    sequence: sequence,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OrderBookRequest */
export function OrderBookRequestEncoderEncode(symbol string, depth u32 | null, at_time bigint | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 8);
  writeU16BE(buf, 0);
  writeU16BE(buf, 12);

  // Fixed fields
  writeString(buf, symbol);
  // TODO: encode depth as u32
  if (at_time != null) { buf.push(1); writeI64BE(buf, at_time); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for OrderBookRequest */
export interface OrderBookRequestDecoder {
  symbol: string;
  depth: u32 | null;
  at_time: bigint | null;
  encodedLen(): number;
}

export function OrderBookRequestDecoderDecode(buf: Uint8Array): OrderBookRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 8) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
        // TODO: decode depth
  let at_time: bigint | null = null;
  if (buf[pos.value++] === 1) at_time = readI64BE(buf, pos);
  return {
    symbol: symbol,
    depth: depth,
    at_time: at_time,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OrderBookSnapshot */
export function OrderBookSnapshotEncoderEncode(symbol string, exchange string, bids string[], asks string[], timestamp bigint, sequence bigint | null, is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 9);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);

  // Fixed fields
  writeString(buf, symbol);
  writeString(buf, exchange);
  writeU32BE(buf, bids.length);
  for (const item of bids) {
    PriceLevelEncode(item, buf);
  }
  writeU32BE(buf, asks.length);
  for (const item of asks) {
    PriceLevelEncode(item, buf);
  }
  writeI64BE(buf, timestamp);
  if (sequence != null) { buf.push(1); writeI64BE(buf, sequence); } else { buf.push(0); }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for OrderBookSnapshot */
export interface OrderBookSnapshotDecoder {
  symbol: string;
  exchange: string;
  bids: string[];
  asks: string[];
  timestamp: bigint;
  sequence: bigint | null;
  is_snapshot: number | null;
  encodedLen(): number;
}

export function OrderBookSnapshotDecoderDecode(buf: Uint8Array): OrderBookSnapshotDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 9) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const exchange = readString(buf, pos);
  const bidsCount = readU32BE(buf, pos);
  const bids: PriceLevel[] = [];
  for (let i = 0; i < bidsCount; i++) {
    const item = PriceLevelDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    bids.push(item);
  }
  const asksCount = readU32BE(buf, pos);
  const asks: PriceLevel[] = [];
  for (let i = 0; i < asksCount; i++) {
    const item = PriceLevelDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    asks.push(item);
  }
  const timestamp = readI64BE(buf, pos);
  let sequence: bigint | null = null;
  if (buf[pos.value++] === 1) sequence = readI64BE(buf, pos);
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    symbol: symbol,
    exchange: exchange,
    bids: bids,
    asks: asks,
    timestamp: timestamp,
    sequence: sequence,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OrderBookDelta */
export function OrderBookDeltaEncoderEncode(symbol string, updates string[], timestamp bigint, sequence bigint | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 10);
  writeU16BE(buf, 0);
  writeU16BE(buf, 16);

  // Fixed fields
  writeString(buf, symbol);
  writeU32BE(buf, updates.length);
  for (const item of updates) {
    MarketDataUpdateEncode(item, buf);
  }
  writeI64BE(buf, timestamp);
  if (sequence != null) { buf.push(1); writeI64BE(buf, sequence); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for OrderBookDelta */
export interface OrderBookDeltaDecoder {
  symbol: string;
  updates: string[];
  timestamp: bigint;
  sequence: bigint | null;
  encodedLen(): number;
}

export function OrderBookDeltaDecoderDecode(buf: Uint8Array): OrderBookDeltaDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 10) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const updatesCount = readU32BE(buf, pos);
  const updates: MarketDataUpdate[] = [];
  for (let i = 0; i < updatesCount; i++) {
    const item = MarketDataUpdateDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    updates.push(item);
  }
  const timestamp = readI64BE(buf, pos);
  let sequence: bigint | null = null;
  if (buf[pos.value++] === 1) sequence = readI64BE(buf, pos);
  return {
    symbol: symbol,
    updates: updates,
    timestamp: timestamp,
    sequence: sequence,
    encodedLen: () => 0,
  };
}

/** SBE encoder for AggregateTradeEvent */
export function AggregateTradeEventEncoderEncode(trade string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 11);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, trade);

  return new Uint8Array(buf);
}

/** SBE decoder for AggregateTradeEvent */
export interface AggregateTradeEventDecoder {
  trade: string;
  encodedLen(): number;
}

export function AggregateTradeEventDecoderDecode(buf: Uint8Array): AggregateTradeEventDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 11) throw new Error('invalid template_id');
  const trade = readString(buf, pos);
  return {
    trade: trade,
    encodedLen: () => 0,
  };
}

/** SBE encoder for AggregateTradeRequest */
export function AggregateTradeRequestEncoderEncode(symbol string, start_time bigint | null, end_time bigint | null, limit u32 | null, cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 12);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);

  // Fixed fields
  writeString(buf, symbol);
  if (start_time != null) { buf.push(1); writeI64BE(buf, start_time); } else { buf.push(0); }
  if (end_time != null) { buf.push(1); writeI64BE(buf, end_time); } else { buf.push(0); }
  // TODO: encode limit as u32
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for AggregateTradeRequest */
export interface AggregateTradeRequestDecoder {
  symbol: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: u32 | null;
  cursor: string | null;
  encodedLen(): number;
}

export function AggregateTradeRequestDecoderDecode(buf: Uint8Array): AggregateTradeRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 12) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  let start_time: bigint | null = null;
  if (buf[pos.value++] === 1) start_time = readI64BE(buf, pos);
  let end_time: bigint | null = null;
  if (buf[pos.value++] === 1) end_time = readI64BE(buf, pos);
        // TODO: decode limit
  let cursor: string | null;
  if (buf[pos.value++] === 1) cursor = readString(buf, pos); else cursor = null;
  return {
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for AggregateTradeBatch */
export function AggregateTradeBatchEncoderEncode(symbol string, trades string[], has_more number, next_cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 13);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, symbol);
  writeU32BE(buf, trades.length);
  for (const item of trades) {
    AggregateTradeEncode(item, buf);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for AggregateTradeBatch */
export interface AggregateTradeBatchDecoder {
  symbol: string;
  trades: string[];
  has_more: number;
  next_cursor: string | null;
  encodedLen(): number;
}

export function AggregateTradeBatchDecoderDecode(buf: Uint8Array): AggregateTradeBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 13) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const tradesCount = readU32BE(buf, pos);
  const trades: AggregateTrade[] = [];
  for (let i = 0; i < tradesCount; i++) {
    const item = AggregateTradeDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    trades.push(item);
  }
  const has_more = buf[pos.value++];
  let next_cursor: string | null;
  if (buf[pos.value++] === 1) next_cursor = readString(buf, pos); else next_cursor = null;
  return {
    symbol: symbol,
    trades: trades,
    has_more: has_more,
    next_cursor: next_cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for MiniTicker */
export function MiniTickerEncoderEncode(symbol string, last_price number, volume number, timestamp bigint, is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 14);
  writeU16BE(buf, 0);
  writeU16BE(buf, 25);

  // Fixed fields
  writeString(buf, symbol);
  writeF64BE(buf, last_price);
  writeF64BE(buf, volume);
  writeI64BE(buf, timestamp);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for MiniTicker */
export interface MiniTickerDecoder {
  symbol: string;
  last_price: number;
  volume: number;
  timestamp: bigint;
  is_snapshot: number | null;
  encodedLen(): number;
}

export function MiniTickerDecoderDecode(buf: Uint8Array): MiniTickerDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 14) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const last_price = readF64BE(buf, pos);
  const volume = readF64BE(buf, pos);
  const timestamp = readI64BE(buf, pos);
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    symbol: symbol,
    last_price: last_price,
    volume: volume,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for AllMidsRequest */
export function AllMidsRequestEncoderEncode(): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 15);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields

  return new Uint8Array(buf);
}

/** SBE decoder for AllMidsRequest */
export interface AllMidsRequestDecoder {
  encodedLen(): number;
}

export function AllMidsRequestDecoderDecode(buf: Uint8Array): AllMidsRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 15) throw new Error('invalid template_id');
  return {

    encodedLen: () => 0,
  };
}

/** SBE encoder for AllMidsBatch */
export function AllMidsBatchEncoderEncode(tickers string[]): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 16);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeU32BE(buf, tickers.length);
  for (const item of tickers) {
    MiniTickerEncode(item, buf);
  }

  return new Uint8Array(buf);
}

/** SBE decoder for AllMidsBatch */
export interface AllMidsBatchDecoder {
  tickers: string[];
  encodedLen(): number;
}

export function AllMidsBatchDecoderDecode(buf: Uint8Array): AllMidsBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 16) throw new Error('invalid template_id');
  const tickersCount = readU32BE(buf, pos);
  const tickers: MiniTicker[] = [];
  for (let i = 0; i < tickersCount; i++) {
    const item = MiniTickerDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    tickers.push(item);
  }
  return {
    tickers: tickers,
    encodedLen: () => 0,
  };
}

/** SBE encoder for MarkPriceUpdate */
export function MarkPriceUpdateEncoderEncode(symbol string, mark_price number, index_price number | null, funding_rate number | null, timestamp bigint, is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 17);
  writeU16BE(buf, 0);
  writeU16BE(buf, 33);

  // Fixed fields
  writeString(buf, symbol);
  writeF64BE(buf, mark_price);
  writeF64BE(buf, index_price ?? 0);
  buf.push(index_price != null ? 1 : 0);
  writeF64BE(buf, funding_rate ?? 0);
  buf.push(funding_rate != null ? 1 : 0);
  writeI64BE(buf, timestamp);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for MarkPriceUpdate */
export interface MarkPriceUpdateDecoder {
  symbol: string;
  mark_price: number;
  index_price: number | null;
  funding_rate: number | null;
  timestamp: bigint;
  is_snapshot: number | null;
  encodedLen(): number;
}

export function MarkPriceUpdateDecoderDecode(buf: Uint8Array): MarkPriceUpdateDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 17) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const mark_price = readF64BE(buf, pos);
  const index_priceRaw = readF64BE(buf, pos);
  const index_price = buf[pos.value++] === 1 ? index_priceRaw : null;
  const funding_rateRaw = readF64BE(buf, pos);
  const funding_rate = buf[pos.value++] === 1 ? funding_rateRaw : null;
  const timestamp = readI64BE(buf, pos);
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    symbol: symbol,
    mark_price: mark_price,
    index_price: index_price,
    funding_rate: funding_rate,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for MarkPriceRequest */
export function MarkPriceRequestEncoderEncode(symbol string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 18);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, symbol);

  return new Uint8Array(buf);
}

/** SBE decoder for MarkPriceRequest */
export interface MarkPriceRequestDecoder {
  symbol: string;
  encodedLen(): number;
}

export function MarkPriceRequestDecoderDecode(buf: Uint8Array): MarkPriceRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 18) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  return {
    symbol: symbol,
    encodedLen: () => 0,
  };
}

/** SBE encoder for LiquidationTradeEvent */
export function LiquidationTradeEventEncoderEncode(trade string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 19);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, trade);

  return new Uint8Array(buf);
}

/** SBE decoder for LiquidationTradeEvent */
export interface LiquidationTradeEventDecoder {
  trade: string;
  encodedLen(): number;
}

export function LiquidationTradeEventDecoderDecode(buf: Uint8Array): LiquidationTradeEventDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 19) throw new Error('invalid template_id');
  const trade = readString(buf, pos);
  return {
    trade: trade,
    encodedLen: () => 0,
  };
}

/** SBE encoder for CandleBarEvent */
export function CandleBarEventEncoderEncode(bar string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 20);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, bar);

  return new Uint8Array(buf);
}

/** SBE decoder for CandleBarEvent */
export interface CandleBarEventDecoder {
  bar: string;
  encodedLen(): number;
}

export function CandleBarEventDecoderDecode(buf: Uint8Array): CandleBarEventDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 20) throw new Error('invalid template_id');
  const bar = readString(buf, pos);
  return {
    bar: bar,
    encodedLen: () => 0,
  };
}

/** SBE encoder for CandleBarRequest */
export function CandleBarRequestEncoderEncode(symbol string, interval string, start_time bigint | null, end_time bigint | null, limit u32 | null, cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 21);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);

  // Fixed fields
  writeString(buf, symbol);
  writeString(buf, interval);
  if (start_time != null) { buf.push(1); writeI64BE(buf, start_time); } else { buf.push(0); }
  if (end_time != null) { buf.push(1); writeI64BE(buf, end_time); } else { buf.push(0); }
  // TODO: encode limit as u32
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for CandleBarRequest */
export interface CandleBarRequestDecoder {
  symbol: string;
  interval: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: u32 | null;
  cursor: string | null;
  encodedLen(): number;
}

export function CandleBarRequestDecoderDecode(buf: Uint8Array): CandleBarRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 21) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const interval = readString(buf, pos);
  let start_time: bigint | null = null;
  if (buf[pos.value++] === 1) start_time = readI64BE(buf, pos);
  let end_time: bigint | null = null;
  if (buf[pos.value++] === 1) end_time = readI64BE(buf, pos);
        // TODO: decode limit
  let cursor: string | null;
  if (buf[pos.value++] === 1) cursor = readString(buf, pos); else cursor = null;
  return {
    symbol: symbol,
    interval: interval,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for CandleBarBatch */
export function CandleBarBatchEncoderEncode(symbol string, interval string, bars string[], has_more number, next_cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 22);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, symbol);
  writeString(buf, interval);
  writeU32BE(buf, bars.length);
  for (const item of bars) {
    CandleBarEncode(item, buf);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for CandleBarBatch */
export interface CandleBarBatchDecoder {
  symbol: string;
  interval: string;
  bars: string[];
  has_more: number;
  next_cursor: string | null;
  encodedLen(): number;
}

export function CandleBarBatchDecoderDecode(buf: Uint8Array): CandleBarBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 22) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const interval = readString(buf, pos);
  const barsCount = readU32BE(buf, pos);
  const bars: CandleBar[] = [];
  for (let i = 0; i < barsCount; i++) {
    const item = CandleBarDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    bars.push(item);
  }
  const has_more = buf[pos.value++];
  let next_cursor: string | null;
  if (buf[pos.value++] === 1) next_cursor = readString(buf, pos); else next_cursor = null;
  return {
    symbol: symbol,
    interval: interval,
    bars: bars,
    has_more: has_more,
    next_cursor: next_cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for PublicTradeEvent */
export function PublicTradeEventEncoderEncode(trade string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 23);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, trade);

  return new Uint8Array(buf);
}

/** SBE decoder for PublicTradeEvent */
export interface PublicTradeEventDecoder {
  trade: string;
  encodedLen(): number;
}

export function PublicTradeEventDecoderDecode(buf: Uint8Array): PublicTradeEventDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 23) throw new Error('invalid template_id');
  const trade = readString(buf, pos);
  return {
    trade: trade,
    encodedLen: () => 0,
  };
}

/** SBE encoder for TradeHistoryRequest */
export function TradeHistoryRequestEncoderEncode(symbol string, start_time bigint | null, end_time bigint | null, limit u32 | null, cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 24);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);

  // Fixed fields
  writeString(buf, symbol);
  if (start_time != null) { buf.push(1); writeI64BE(buf, start_time); } else { buf.push(0); }
  if (end_time != null) { buf.push(1); writeI64BE(buf, end_time); } else { buf.push(0); }
  // TODO: encode limit as u32
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for TradeHistoryRequest */
export interface TradeHistoryRequestDecoder {
  symbol: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: u32 | null;
  cursor: string | null;
  encodedLen(): number;
}

export function TradeHistoryRequestDecoderDecode(buf: Uint8Array): TradeHistoryRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 24) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  let start_time: bigint | null = null;
  if (buf[pos.value++] === 1) start_time = readI64BE(buf, pos);
  let end_time: bigint | null = null;
  if (buf[pos.value++] === 1) end_time = readI64BE(buf, pos);
        // TODO: decode limit
  let cursor: string | null;
  if (buf[pos.value++] === 1) cursor = readString(buf, pos); else cursor = null;
  return {
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for PublicTradeBatch */
export function PublicTradeBatchEncoderEncode(symbol string, trades string[], has_more number, next_cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 25);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, symbol);
  writeU32BE(buf, trades.length);
  for (const item of trades) {
    PublicTradeEncode(item, buf);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for PublicTradeBatch */
export interface PublicTradeBatchDecoder {
  symbol: string;
  trades: string[];
  has_more: number;
  next_cursor: string | null;
  encodedLen(): number;
}

export function PublicTradeBatchDecoderDecode(buf: Uint8Array): PublicTradeBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 25) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const tradesCount = readU32BE(buf, pos);
  const trades: PublicTrade[] = [];
  for (let i = 0; i < tradesCount; i++) {
    const item = PublicTradeDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    trades.push(item);
  }
  const has_more = buf[pos.value++];
  let next_cursor: string | null;
  if (buf[pos.value++] === 1) next_cursor = readString(buf, pos); else next_cursor = null;
  return {
    symbol: symbol,
    trades: trades,
    has_more: has_more,
    next_cursor: next_cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for BestBidOffer */
export function BestBidOfferEncoderEncode(symbol string, bid_price number | null, bid_qty number | null, ask_price number | null, ask_qty number | null, timestamp bigint, is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 26);
  writeU16BE(buf, 0);
  writeU16BE(buf, 41);

  // Fixed fields
  writeString(buf, symbol);
  writeF64BE(buf, bid_price ?? 0);
  buf.push(bid_price != null ? 1 : 0);
  writeF64BE(buf, bid_qty ?? 0);
  buf.push(bid_qty != null ? 1 : 0);
  writeF64BE(buf, ask_price ?? 0);
  buf.push(ask_price != null ? 1 : 0);
  writeF64BE(buf, ask_qty ?? 0);
  buf.push(ask_qty != null ? 1 : 0);
  writeI64BE(buf, timestamp);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for BestBidOffer */
export interface BestBidOfferDecoder {
  symbol: string;
  bid_price: number | null;
  bid_qty: number | null;
  ask_price: number | null;
  ask_qty: number | null;
  timestamp: bigint;
  is_snapshot: number | null;
  encodedLen(): number;
}

export function BestBidOfferDecoderDecode(buf: Uint8Array): BestBidOfferDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 26) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const bid_priceRaw = readF64BE(buf, pos);
  const bid_price = buf[pos.value++] === 1 ? bid_priceRaw : null;
  const bid_qtyRaw = readF64BE(buf, pos);
  const bid_qty = buf[pos.value++] === 1 ? bid_qtyRaw : null;
  const ask_priceRaw = readF64BE(buf, pos);
  const ask_price = buf[pos.value++] === 1 ? ask_priceRaw : null;
  const ask_qtyRaw = readF64BE(buf, pos);
  const ask_qty = buf[pos.value++] === 1 ? ask_qtyRaw : null;
  const timestamp = readI64BE(buf, pos);
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    symbol: symbol,
    bid_price: bid_price,
    bid_qty: bid_qty,
    ask_price: ask_price,
    ask_qty: ask_qty,
    timestamp: timestamp,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for SymbolTicker */
export function SymbolTickerEncoderEncode(symbol string, last_price number, price_change number, price_change_pct number, volume number, high number, low number, open number, timestamp bigint, is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 27);
  writeU16BE(buf, 0);
  writeU16BE(buf, 65);

  // Fixed fields
  writeString(buf, symbol);
  writeF64BE(buf, last_price);
  writeF64BE(buf, price_change);
  writeF64BE(buf, price_change_pct);
  writeF64BE(buf, volume);
  writeF64BE(buf, high);
  writeF64BE(buf, low);
  writeF64BE(buf, open);
  writeI64BE(buf, timestamp);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for SymbolTicker */
export interface SymbolTickerDecoder {
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
  encodedLen(): number;
}

export function SymbolTickerDecoderDecode(buf: Uint8Array): SymbolTickerDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 27) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  const last_price = readF64BE(buf, pos);
  const price_change = readF64BE(buf, pos);
  const price_change_pct = readF64BE(buf, pos);
  const volume = readF64BE(buf, pos);
  const high = readF64BE(buf, pos);
  const low = readF64BE(buf, pos);
  const open = readF64BE(buf, pos);
  const timestamp = readI64BE(buf, pos);
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
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
    encodedLen: () => 0,
  };
}

/** SBE encoder for TickerRequest */
export function TickerRequestEncoderEncode(symbol string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 28);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, symbol);

  return new Uint8Array(buf);
}

/** SBE decoder for TickerRequest */
export interface TickerRequestDecoder {
  symbol: string;
  encodedLen(): number;
}

export function TickerRequestDecoderDecode(buf: Uint8Array): TickerRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 28) throw new Error('invalid template_id');
  const symbol = readString(buf, pos);
  return {
    symbol: symbol,
    encodedLen: () => 0,
  };
}

/** SBE encoder for AccountSummary */
export function AccountSummaryEncoderEncode(account string, balance number, buying_power number, currency string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 29);
  writeU16BE(buf, 0);
  writeU16BE(buf, 16);

  // Fixed fields
  writeString(buf, account);
  writeF64BE(buf, balance);
  writeF64BE(buf, buying_power);
  writeString(buf, currency);

  return new Uint8Array(buf);
}

/** SBE decoder for AccountSummary */
export interface AccountSummaryDecoder {
  account: string;
  balance: number;
  buying_power: number;
  currency: string;
  encodedLen(): number;
}

export function AccountSummaryDecoderDecode(buf: Uint8Array): AccountSummaryDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 29) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const balance = readF64BE(buf, pos);
  const buying_power = readF64BE(buf, pos);
  const currency = readString(buf, pos);
  return {
    account: account,
    balance: balance,
    buying_power: buying_power,
    currency: currency,
    encodedLen: () => 0,
  };
}

/** SBE encoder for MarginSummary */
export function MarginSummaryEncoderEncode(account string, balance number, buying_power number, equity number, margin_used number, available number, currency string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 30);
  writeU16BE(buf, 0);
  writeU16BE(buf, 40);

  // Fixed fields
  writeString(buf, account);
  writeF64BE(buf, balance);
  writeF64BE(buf, buying_power);
  writeF64BE(buf, equity);
  writeF64BE(buf, margin_used);
  writeF64BE(buf, available);
  writeString(buf, currency);

  return new Uint8Array(buf);
}

/** SBE decoder for MarginSummary */
export interface MarginSummaryDecoder {
  account: string;
  balance: number;
  buying_power: number;
  equity: number;
  margin_used: number;
  available: number;
  currency: string;
  encodedLen(): number;
}

export function MarginSummaryDecoderDecode(buf: Uint8Array): MarginSummaryDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 30) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const balance = readF64BE(buf, pos);
  const buying_power = readF64BE(buf, pos);
  const equity = readF64BE(buf, pos);
  const margin_used = readF64BE(buf, pos);
  const available = readF64BE(buf, pos);
  const currency = readString(buf, pos);
  return {
    account: account,
    balance: balance,
    buying_power: buying_power,
    equity: equity,
    margin_used: margin_used,
    available: available,
    currency: currency,
    encodedLen: () => 0,
  };
}

/** SBE encoder for BalanceSnapshot */
export function BalanceSnapshotEncoderEncode(account string, balances string[], is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 31);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeU32BE(buf, balances.length);
  for (const item of balances) {
    BalanceEntryEncode(item, buf);
  }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for BalanceSnapshot */
export interface BalanceSnapshotDecoder {
  account: string;
  balances: string[];
  is_snapshot: number | null;
  encodedLen(): number;
}

export function BalanceSnapshotDecoderDecode(buf: Uint8Array): BalanceSnapshotDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 31) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const balancesCount = readU32BE(buf, pos);
  const balances: BalanceEntry[] = [];
  for (let i = 0; i < balancesCount; i++) {
    const item = BalanceEntryDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    balances.push(item);
  }
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    account: account,
    balances: balances,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for BalanceUpdate */
export function BalanceUpdateEncoderEncode(account string, asset string, delta number, total number, available number, reason BalanceUpdateReason): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 32);
  writeU16BE(buf, 0);
  writeU16BE(buf, 25);

  // Fixed fields
  writeString(buf, account);
  writeString(buf, asset);
  writeF64BE(buf, delta);
  writeF64BE(buf, total);
  writeF64BE(buf, available);
  buf.push(BalanceUpdateReasonToValue(reason));

  return new Uint8Array(buf);
}

/** SBE decoder for BalanceUpdate */
export interface BalanceUpdateDecoder {
  account: string;
  asset: string;
  delta: number;
  total: number;
  available: number;
  reason: BalanceUpdateReason;
  encodedLen(): number;
}

export function BalanceUpdateDecoderDecode(buf: Uint8Array): BalanceUpdateDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 32) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const asset = readString(buf, pos);
  const delta = readF64BE(buf, pos);
  const total = readF64BE(buf, pos);
  const available = readF64BE(buf, pos);
  const reasonRaw = buf[pos.value++];
  const reason = BalanceUpdateReasonFromValue(reasonRaw);
  if (reason == null) throw new Error('invalid BalanceUpdateReason');
  return {
    account: account,
    asset: asset,
    delta: delta,
    total: total,
    available: available,
    reason: reason,
    encodedLen: () => 0,
  };
}

/** SBE encoder for PositionSnapshot */
export function PositionSnapshotEncoderEncode(account string, positions string[], is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 33);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeU32BE(buf, positions.length);
  for (const item of positions) {
    PositionEntryEncode(item, buf);
  }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for PositionSnapshot */
export interface PositionSnapshotDecoder {
  account: string;
  positions: string[];
  is_snapshot: number | null;
  encodedLen(): number;
}

export function PositionSnapshotDecoderDecode(buf: Uint8Array): PositionSnapshotDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 33) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const positionsCount = readU32BE(buf, pos);
  const positions: PositionEntry[] = [];
  for (let i = 0; i < positionsCount; i++) {
    const item = PositionEntryDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    positions.push(item);
  }
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    account: account,
    positions: positions,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for PositionUpdate */
export function PositionUpdateEncoderEncode(account string, symbol string, qty number, entry_price number, unrealized_pnl number): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 34);
  writeU16BE(buf, 0);
  writeU16BE(buf, 24);

  // Fixed fields
  writeString(buf, account);
  writeString(buf, symbol);
  writeF64BE(buf, qty);
  writeF64BE(buf, entry_price);
  writeF64BE(buf, unrealized_pnl);

  return new Uint8Array(buf);
}

/** SBE decoder for PositionUpdate */
export interface PositionUpdateDecoder {
  account: string;
  symbol: string;
  qty: number;
  entry_price: number;
  unrealized_pnl: number;
  encodedLen(): number;
}

export function PositionUpdateDecoderDecode(buf: Uint8Array): PositionUpdateDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 34) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const symbol = readString(buf, pos);
  const qty = readF64BE(buf, pos);
  const entry_price = readF64BE(buf, pos);
  const unrealized_pnl = readF64BE(buf, pos);
  return {
    account: account,
    symbol: symbol,
    qty: qty,
    entry_price: entry_price,
    unrealized_pnl: unrealized_pnl,
    encodedLen: () => 0,
  };
}

/** SBE encoder for MarginUpdate */
export function MarginUpdateEncoderEncode(account string, summary string, is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 35);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeString(buf, summary);
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for MarginUpdate */
export interface MarginUpdateDecoder {
  account: string;
  summary: string;
  is_snapshot: number | null;
  encodedLen(): number;
}

export function MarginUpdateDecoderDecode(buf: Uint8Array): MarginUpdateDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 35) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const summary = readString(buf, pos);
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    account: account,
    summary: summary,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for UserLiquidation */
export function UserLiquidationEncoderEncode(account string, symbol string, qty number, price number, timestamp bigint): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 36);
  writeU16BE(buf, 0);
  writeU16BE(buf, 24);

  // Fixed fields
  writeString(buf, account);
  writeString(buf, symbol);
  writeF64BE(buf, qty);
  writeF64BE(buf, price);
  writeI64BE(buf, timestamp);

  return new Uint8Array(buf);
}

/** SBE decoder for UserLiquidation */
export interface UserLiquidationDecoder {
  account: string;
  symbol: string;
  qty: number;
  price: number;
  timestamp: bigint;
  encodedLen(): number;
}

export function UserLiquidationDecoderDecode(buf: Uint8Array): UserLiquidationDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 36) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const symbol = readString(buf, pos);
  const qty = readF64BE(buf, pos);
  const price = readF64BE(buf, pos);
  const timestamp = readI64BE(buf, pos);
  return {
    account: account,
    symbol: symbol,
    qty: qty,
    price: price,
    timestamp: timestamp,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OrderListStatus */
export function OrderListStatusEncoderEncode(account string, list_id string, status OrderListStatusStatus, symbol string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 37);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeString(buf, list_id);
  buf.push(OrderListStatusStatusToValue(status));
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for OrderListStatus */
export interface OrderListStatusDecoder {
  account: string;
  list_id: string;
  status: OrderListStatusStatus;
  symbol: string | null;
  encodedLen(): number;
}

export function OrderListStatusDecoderDecode(buf: Uint8Array): OrderListStatusDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 37) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const list_id = readString(buf, pos);
  const statusRaw = buf[pos.value++];
  const status = OrderListStatusStatusFromValue(statusRaw);
  if (status == null) throw new Error('invalid OrderListStatusStatus');
  let symbol: string | null;
  if (buf[pos.value++] === 1) symbol = readString(buf, pos); else symbol = null;
  return {
    account: account,
    list_id: list_id,
    status: status,
    symbol: symbol,
    encodedLen: () => 0,
  };
}

/** SBE encoder for FillHistoryRequest */
export function FillHistoryRequestEncoderEncode(account string, symbol string | null, start_time bigint | null, end_time bigint | null, limit u32 | null, cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 38);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);

  // Fixed fields
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  if (start_time != null) { buf.push(1); writeI64BE(buf, start_time); } else { buf.push(0); }
  if (end_time != null) { buf.push(1); writeI64BE(buf, end_time); } else { buf.push(0); }
  // TODO: encode limit as u32
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for FillHistoryRequest */
export interface FillHistoryRequestDecoder {
  account: string;
  symbol: string | null;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: u32 | null;
  cursor: string | null;
  encodedLen(): number;
}

export function FillHistoryRequestDecoderDecode(buf: Uint8Array): FillHistoryRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 38) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  let symbol: string | null;
  if (buf[pos.value++] === 1) symbol = readString(buf, pos); else symbol = null;
  let start_time: bigint | null = null;
  if (buf[pos.value++] === 1) start_time = readI64BE(buf, pos);
  let end_time: bigint | null = null;
  if (buf[pos.value++] === 1) end_time = readI64BE(buf, pos);
        // TODO: decode limit
  let cursor: string | null;
  if (buf[pos.value++] === 1) cursor = readString(buf, pos); else cursor = null;
  return {
    account: account,
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for FillHistoryBatch */
export function FillHistoryBatchEncoderEncode(account string, fills string[], has_more number, next_cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 39);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeU32BE(buf, fills.length);
  for (const item of fills) {
    ExecutionReportEncode(item, buf);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for FillHistoryBatch */
export interface FillHistoryBatchDecoder {
  account: string;
  fills: string[];
  has_more: number;
  next_cursor: string | null;
  encodedLen(): number;
}

export function FillHistoryBatchDecoderDecode(buf: Uint8Array): FillHistoryBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 39) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const fillsCount = readU32BE(buf, pos);
  const fills: ExecutionReport[] = [];
  for (let i = 0; i < fillsCount; i++) {
    const item = ExecutionReportDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    fills.push(item);
  }
  const has_more = buf[pos.value++];
  let next_cursor: string | null;
  if (buf[pos.value++] === 1) next_cursor = readString(buf, pos); else next_cursor = null;
  return {
    account: account,
    fills: fills,
    has_more: has_more,
    next_cursor: next_cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for FundingPayment */
export function FundingPaymentEncoderEncode(account string, symbol string | null, amount number, rate number, timestamp bigint): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 40);
  writeU16BE(buf, 0);
  writeU16BE(buf, 24);

  // Fixed fields
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  writeF64BE(buf, amount);
  writeF64BE(buf, rate);
  writeI64BE(buf, timestamp);

  return new Uint8Array(buf);
}

/** SBE decoder for FundingPayment */
export interface FundingPaymentDecoder {
  account: string;
  symbol: string | null;
  amount: number;
  rate: number;
  timestamp: bigint;
  encodedLen(): number;
}

export function FundingPaymentDecoderDecode(buf: Uint8Array): FundingPaymentDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 40) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  let symbol: string | null;
  if (buf[pos.value++] === 1) symbol = readString(buf, pos); else symbol = null;
  const amount = readF64BE(buf, pos);
  const rate = readF64BE(buf, pos);
  const timestamp = readI64BE(buf, pos);
  return {
    account: account,
    symbol: symbol,
    amount: amount,
    rate: rate,
    timestamp: timestamp,
    encodedLen: () => 0,
  };
}

/** SBE encoder for FundingHistoryRequest */
export function FundingHistoryRequestEncoderEncode(account string, start_time bigint | null, end_time bigint | null, limit u32 | null, cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 41);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);

  // Fixed fields
  writeString(buf, account);
  if (start_time != null) { buf.push(1); writeI64BE(buf, start_time); } else { buf.push(0); }
  if (end_time != null) { buf.push(1); writeI64BE(buf, end_time); } else { buf.push(0); }
  // TODO: encode limit as u32
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for FundingHistoryRequest */
export interface FundingHistoryRequestDecoder {
  account: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: u32 | null;
  cursor: string | null;
  encodedLen(): number;
}

export function FundingHistoryRequestDecoderDecode(buf: Uint8Array): FundingHistoryRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 41) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  let start_time: bigint | null = null;
  if (buf[pos.value++] === 1) start_time = readI64BE(buf, pos);
  let end_time: bigint | null = null;
  if (buf[pos.value++] === 1) end_time = readI64BE(buf, pos);
        // TODO: decode limit
  let cursor: string | null;
  if (buf[pos.value++] === 1) cursor = readString(buf, pos); else cursor = null;
  return {
    account: account,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for FundingHistoryBatch */
export function FundingHistoryBatchEncoderEncode(account string, payments string[], has_more number, next_cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 42);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeU32BE(buf, payments.length);
  for (const item of payments) {
    FundingPaymentEncode(item, buf);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for FundingHistoryBatch */
export interface FundingHistoryBatchDecoder {
  account: string;
  payments: string[];
  has_more: number;
  next_cursor: string | null;
  encodedLen(): number;
}

export function FundingHistoryBatchDecoderDecode(buf: Uint8Array): FundingHistoryBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 42) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const paymentsCount = readU32BE(buf, pos);
  const payments: FundingPayment[] = [];
  for (let i = 0; i < paymentsCount; i++) {
    const item = FundingPaymentDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    payments.push(item);
  }
  const has_more = buf[pos.value++];
  let next_cursor: string | null;
  if (buf[pos.value++] === 1) next_cursor = readString(buf, pos); else next_cursor = null;
  return {
    account: account,
    payments: payments,
    has_more: has_more,
    next_cursor: next_cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for LedgerUpdate */
export function LedgerUpdateEncoderEncode(account string, asset string, delta number, kind LedgerUpdateKind, timestamp bigint, reference_id string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 43);
  writeU16BE(buf, 0);
  writeU16BE(buf, 17);

  // Fixed fields
  writeString(buf, account);
  writeString(buf, asset);
  writeF64BE(buf, delta);
  buf.push(LedgerUpdateKindToValue(kind));
  writeI64BE(buf, timestamp);
  if (reference_id != null) { buf.push(1); writeString(buf, reference_id); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for LedgerUpdate */
export interface LedgerUpdateDecoder {
  account: string;
  asset: string;
  delta: number;
  kind: LedgerUpdateKind;
  timestamp: bigint;
  reference_id: string | null;
  encodedLen(): number;
}

export function LedgerUpdateDecoderDecode(buf: Uint8Array): LedgerUpdateDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 43) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const asset = readString(buf, pos);
  const delta = readF64BE(buf, pos);
  const kindRaw = buf[pos.value++];
  const kind = LedgerUpdateKindFromValue(kindRaw);
  if (kind == null) throw new Error('invalid LedgerUpdateKind');
  const timestamp = readI64BE(buf, pos);
  let reference_id: string | null;
  if (buf[pos.value++] === 1) reference_id = readString(buf, pos); else reference_id = null;
  return {
    account: account,
    asset: asset,
    delta: delta,
    kind: kind,
    timestamp: timestamp,
    reference_id: reference_id,
    encodedLen: () => 0,
  };
}

/** SBE encoder for LedgerHistoryRequest */
export function LedgerHistoryRequestEncoderEncode(account string, start_time bigint | null, end_time bigint | null, limit u32 | null, cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 44);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);

  // Fixed fields
  writeString(buf, account);
  if (start_time != null) { buf.push(1); writeI64BE(buf, start_time); } else { buf.push(0); }
  if (end_time != null) { buf.push(1); writeI64BE(buf, end_time); } else { buf.push(0); }
  // TODO: encode limit as u32
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for LedgerHistoryRequest */
export interface LedgerHistoryRequestDecoder {
  account: string;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: u32 | null;
  cursor: string | null;
  encodedLen(): number;
}

export function LedgerHistoryRequestDecoderDecode(buf: Uint8Array): LedgerHistoryRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 44) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  let start_time: bigint | null = null;
  if (buf[pos.value++] === 1) start_time = readI64BE(buf, pos);
  let end_time: bigint | null = null;
  if (buf[pos.value++] === 1) end_time = readI64BE(buf, pos);
        // TODO: decode limit
  let cursor: string | null;
  if (buf[pos.value++] === 1) cursor = readString(buf, pos); else cursor = null;
  return {
    account: account,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for LedgerHistoryBatch */
export function LedgerHistoryBatchEncoderEncode(account string, entries string[], has_more number, next_cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 45);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeU32BE(buf, entries.length);
  for (const item of entries) {
    LedgerUpdateEncode(item, buf);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for LedgerHistoryBatch */
export interface LedgerHistoryBatchDecoder {
  account: string;
  entries: string[];
  has_more: number;
  next_cursor: string | null;
  encodedLen(): number;
}

export function LedgerHistoryBatchDecoderDecode(buf: Uint8Array): LedgerHistoryBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 45) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const entriesCount = readU32BE(buf, pos);
  const entries: LedgerUpdate[] = [];
  for (let i = 0; i < entriesCount; i++) {
    const item = LedgerUpdateDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    entries.push(item);
  }
  const has_more = buf[pos.value++];
  let next_cursor: string | null;
  if (buf[pos.value++] === 1) next_cursor = readString(buf, pos); else next_cursor = null;
  return {
    account: account,
    entries: entries,
    has_more: has_more,
    next_cursor: next_cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OpenOrdersRequest */
export function OpenOrdersRequestEncoderEncode(account string, symbol string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 46);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for OpenOrdersRequest */
export interface OpenOrdersRequestDecoder {
  account: string;
  symbol: string | null;
  encodedLen(): number;
}

export function OpenOrdersRequestDecoderDecode(buf: Uint8Array): OpenOrdersRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 46) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  let symbol: string | null;
  if (buf[pos.value++] === 1) symbol = readString(buf, pos); else symbol = null;
  return {
    account: account,
    symbol: symbol,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OpenOrdersSnapshot */
export function OpenOrdersSnapshotEncoderEncode(account string, orders string[], is_snapshot number | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 47);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeU32BE(buf, orders.length);
  for (const item of orders) {
    ExecutionReportEncode(item, buf);
  }
  buf.push(is_snapshot ?? 0);
  buf.push(is_snapshot != null ? 1 : 0);

  return new Uint8Array(buf);
}

/** SBE decoder for OpenOrdersSnapshot */
export interface OpenOrdersSnapshotDecoder {
  account: string;
  orders: string[];
  is_snapshot: number | null;
  encodedLen(): number;
}

export function OpenOrdersSnapshotDecoderDecode(buf: Uint8Array): OpenOrdersSnapshotDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 47) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const ordersCount = readU32BE(buf, pos);
  const orders: ExecutionReport[] = [];
  for (let i = 0; i < ordersCount; i++) {
    const item = ExecutionReportDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    orders.push(item);
  }
  const v = buf[pos.value++];
  const is_snapshot = buf[pos.value++] === 1 ? v : null;
  return {
    account: account,
    orders: orders,
    is_snapshot: is_snapshot,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OrderHistoryRequest */
export function OrderHistoryRequestEncoderEncode(account string, symbol string | null, start_time bigint | null, end_time bigint | null, limit u32 | null, cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 48);
  writeU16BE(buf, 0);
  writeU16BE(buf, 20);

  // Fixed fields
  writeString(buf, account);
  if (symbol != null) { buf.push(1); writeString(buf, symbol); } else { buf.push(0); }
  if (start_time != null) { buf.push(1); writeI64BE(buf, start_time); } else { buf.push(0); }
  if (end_time != null) { buf.push(1); writeI64BE(buf, end_time); } else { buf.push(0); }
  // TODO: encode limit as u32
  if (cursor != null) { buf.push(1); writeString(buf, cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for OrderHistoryRequest */
export interface OrderHistoryRequestDecoder {
  account: string;
  symbol: string | null;
  start_time: bigint | null;
  end_time: bigint | null;
  limit: u32 | null;
  cursor: string | null;
  encodedLen(): number;
}

export function OrderHistoryRequestDecoderDecode(buf: Uint8Array): OrderHistoryRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 48) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  let symbol: string | null;
  if (buf[pos.value++] === 1) symbol = readString(buf, pos); else symbol = null;
  let start_time: bigint | null = null;
  if (buf[pos.value++] === 1) start_time = readI64BE(buf, pos);
  let end_time: bigint | null = null;
  if (buf[pos.value++] === 1) end_time = readI64BE(buf, pos);
        // TODO: decode limit
  let cursor: string | null;
  if (buf[pos.value++] === 1) cursor = readString(buf, pos); else cursor = null;
  return {
    account: account,
    symbol: symbol,
    start_time: start_time,
    end_time: end_time,
    limit: limit,
    cursor: cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for OrderHistoryBatch */
export function OrderHistoryBatchEncoderEncode(account string, orders string[], has_more number, next_cursor string | null): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 49);
  writeU16BE(buf, 0);
  writeU16BE(buf, 1);

  // Fixed fields
  writeString(buf, account);
  writeU32BE(buf, orders.length);
  for (const item of orders) {
    ExecutionReportEncode(item, buf);
  }
  buf.push(has_more);
  if (next_cursor != null) { buf.push(1); writeString(buf, next_cursor); } else { buf.push(0); }

  return new Uint8Array(buf);
}

/** SBE decoder for OrderHistoryBatch */
export interface OrderHistoryBatchDecoder {
  account: string;
  orders: string[];
  has_more: number;
  next_cursor: string | null;
  encodedLen(): number;
}

export function OrderHistoryBatchDecoderDecode(buf: Uint8Array): OrderHistoryBatchDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 49) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  const ordersCount = readU32BE(buf, pos);
  const orders: ExecutionReport[] = [];
  for (let i = 0; i < ordersCount; i++) {
    const item = ExecutionReportDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    orders.push(item);
  }
  const has_more = buf[pos.value++];
  let next_cursor: string | null;
  if (buf[pos.value++] === 1) next_cursor = readString(buf, pos); else next_cursor = null;
  return {
    account: account,
    orders: orders,
    has_more: has_more,
    next_cursor: next_cursor,
    encodedLen: () => 0,
  };
}

/** SBE encoder for CapabilitiesRequest */
export function CapabilitiesRequestEncoderEncode(): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 50);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields

  return new Uint8Array(buf);
}

/** SBE decoder for CapabilitiesRequest */
export interface CapabilitiesRequestDecoder {
  encodedLen(): number;
}

export function CapabilitiesRequestDecoderDecode(buf: Uint8Array): CapabilitiesRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 50) throw new Error('invalid template_id');
  return {

    encodedLen: () => 0,
  };
}

/** SBE encoder for CapabilitiesResponse */
export function CapabilitiesResponseEncoderEncode(schema_ids string[], paths string[], symbols string[], intervals string[]): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 51);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeU32BE(buf, schema_ids.length);
  for (const item of schema_ids) {
    u8Encode(item, buf);
  }
  writeU32BE(buf, paths.length);
  for (const item of paths) {
    CapabilityPathEncode(item, buf);
  }
  writeU32BE(buf, symbols.length);
  for (const item of symbols) {
    SymbolEncode(item, buf);
  }
  writeU32BE(buf, intervals.length);
  for (const item of intervals) {
    CandleIntervalEncode(item, buf);
  }

  return new Uint8Array(buf);
}

/** SBE decoder for CapabilitiesResponse */
export interface CapabilitiesResponseDecoder {
  schema_ids: string[];
  paths: string[];
  symbols: string[];
  intervals: string[];
  encodedLen(): number;
}

export function CapabilitiesResponseDecoderDecode(buf: Uint8Array): CapabilitiesResponseDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 51) throw new Error('invalid template_id');
  const schema_idsCount = readU32BE(buf, pos);
  const schema_ids: u8[] = [];
  for (let i = 0; i < schema_idsCount; i++) {
    const item = u8DecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    schema_ids.push(item);
  }
  const pathsCount = readU32BE(buf, pos);
  const paths: CapabilityPath[] = [];
  for (let i = 0; i < pathsCount; i++) {
    const item = CapabilityPathDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    paths.push(item);
  }
  const symbolsCount = readU32BE(buf, pos);
  const symbols: Symbol[] = [];
  for (let i = 0; i < symbolsCount; i++) {
    const item = SymbolDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    symbols.push(item);
  }
  const intervalsCount = readU32BE(buf, pos);
  const intervals: CandleInterval[] = [];
  for (let i = 0; i < intervalsCount; i++) {
    const item = CandleIntervalDecoderDecode(buf.subarray(pos.value));
    pos.value += item.encodedLen();
    intervals.push(item);
  }
  return {
    schema_ids: schema_ids,
    paths: paths,
    symbols: symbols,
    intervals: intervals,
    encodedLen: () => 0,
  };
}

/** SBE encoder for PositionRequest */
export function PositionRequestEncoderEncode(account string): Uint8Array {
  const buf: number[] = [];
  // SBE Message Header (8 bytes)
  writeU16BE(buf, SCHEMA_ID);
  writeU16BE(buf, 52);
  writeU16BE(buf, 0);
  writeU16BE(buf, 0);

  // Fixed fields
  writeString(buf, account);

  return new Uint8Array(buf);
}

/** SBE decoder for PositionRequest */
export interface PositionRequestDecoder {
  account: string;
  encodedLen(): number;
}

export function PositionRequestDecoderDecode(buf: Uint8Array): PositionRequestDecoder {
  if (buf.length < 8) throw new Error('buffer too short for SBE header');
  const pos = { value: 0 };
  const schemaId = readU16BE(buf, pos);
  const tmplId = readU16BE(buf, pos);
  readU16BE(buf, pos); readU16BE(buf, pos);
  if (schemaId !== SCHEMA_ID) throw new Error('invalid schema_id');
  if (tmplId !== 52) throw new Error('invalid template_id');
  const account = readString(buf, pos);
  return {
    account: account,
    encodedLen: () => 0,
  };
}

