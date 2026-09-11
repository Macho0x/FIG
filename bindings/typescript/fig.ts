/**
 * TypeScript bindings over fig.h — Bun via `bun:ffi` (native libfig_ffi).
 */

import { dlopen, FFIType, ptr, type Pointer } from "bun:ffi";

export type FigBuffer = {
  data: Pointer;
  len: number;
};

export type FigFrameList = {
  frames: Pointer;
  count: number;
};

const libPath =
  process.env.FIG_FFI_LIB ??
  new URL("../../../target/debug/libfig_ffi.so", import.meta.url).pathname;

const fig = dlopen(libPath, {
  fig_version: { args: [], returns: FFIType.cstring },
  // FigBuffer / FigFrameList are two INTEGER-class fields; SysV passes them in two regs.
  fig_buffer_free: { args: [FFIType.pointer, FFIType.u64], returns: FFIType.void },
  fig_frame_list_free: { args: [FFIType.pointer, FFIType.u64], returns: FFIType.void },
  fig_string_free: { args: [FFIType.pointer], returns: FFIType.void },
  fig_client_connect: {
    args: [FFIType.cstring, FFIType.cstring, FFIType.pointer],
    returns: FFIType.i32,
  },
  fig_client_connect_0rtt: {
    args: [
      FFIType.cstring,
      FFIType.cstring,
      FFIType.pointer,
      FFIType.u64,
      FFIType.pointer,
    ],
    returns: FFIType.i32,
  },
  fig_client_close: { args: [FFIType.pointer], returns: FFIType.void },
  fig_client_ping: { args: [FFIType.pointer], returns: FFIType.void },
  fig_client_request_and_recv: {
    args: [FFIType.pointer, FFIType.pointer, FFIType.u64, FFIType.pointer],
    returns: FFIType.i32,
  },
  fig_client_subscribe: {
    args: [
      FFIType.pointer,
      FFIType.pointer,
      FFIType.u64,
      FFIType.pointer,
      FFIType.pointer,
    ],
    returns: FFIType.i32,
  },
  fig_client_sub_next: {
    args: [FFIType.pointer, FFIType.u32, FFIType.pointer],
    returns: FFIType.i32,
  },
  fig_client_sub_close: {
    args: [FFIType.pointer],
    returns: FFIType.void,
  },
  fig_frame_encode_request_auth: {
    args: [
      FFIType.u16,
      FFIType.u32,
      FFIType.u8,
      FFIType.cstring,
      FFIType.cstring,
      FFIType.cstring,
      FFIType.cstring,
      FFIType.pointer,
      FFIType.u64,
      FFIType.pointer,
    ],
    returns: FFIType.i32,
  },
  fig_frame_encode_subscribe_auth: {
    args: [
      FFIType.u16,
      FFIType.u32,
      FFIType.cstring,
      FFIType.cstring,
      FFIType.cstring,
      FFIType.pointer,
    ],
    returns: FFIType.i32,
  },
  fig_payload_compress: {
    args: [FFIType.pointer, FFIType.u64, FFIType.pointer],
    returns: FFIType.i32,
  },
  fig_payload_decompress: {
    args: [FFIType.pointer, FFIType.u64, FFIType.pointer],
    returns: FFIType.i32,
  },
  fig_frame_is_stream_item: {
    args: [FFIType.pointer, FFIType.u64],
    returns: FFIType.i32,
  },
  fig_frame_payload: {
    args: [
      FFIType.pointer,
      FFIType.u64,
      FFIType.pointer,
      FFIType.pointer,
      FFIType.pointer,
    ],
    returns: FFIType.i32,
  },
  fig_cbor_decode_execution_report_cl_ord_id: {
    args: [FFIType.pointer, FFIType.u64, FFIType.pointer],
    returns: FFIType.i32,
  },
  fig_jwt_encode: {
    args: [FFIType.cstring, FFIType.u64, FFIType.cstring, FFIType.pointer],
    returns: FFIType.i32,
  },
  fig_jwt_verify_bearer: {
    args: [FFIType.cstring, FFIType.cstring],
    returns: FFIType.i32,
  },
  fig_sbe_encode_new_order_single: {
    args: [
      FFIType.cstring,
      FFIType.cstring,
      FFIType.u8,
      FFIType.f64,
      FFIType.f64,
      FFIType.i8,
      FFIType.i8,
      FFIType.pointer,
    ],
    returns: FFIType.i32,
  },
  fig_funding_new: { args: [FFIType.u64], returns: FFIType.pointer },
  fig_funding_free: { args: [FFIType.pointer], returns: FFIType.void },
  fig_funding_apply: {
    args: [FFIType.pointer, FFIType.pointer, FFIType.u64],
    returns: FFIType.i32,
  },
  fig_funding_len: { args: [FFIType.pointer], returns: FFIType.u64 },
  fig_funding_latest_amount: { args: [FFIType.pointer], returns: FFIType.f64 },
  fig_agg_trades_new: { args: [FFIType.u64], returns: FFIType.pointer },
  fig_agg_trades_free: { args: [FFIType.pointer], returns: FFIType.void },
  fig_agg_trades_apply: {
    args: [FFIType.pointer, FFIType.pointer, FFIType.u64],
    returns: FFIType.i32,
  },
  fig_agg_trades_len: { args: [FFIType.pointer], returns: FFIType.u64 },
  fig_agg_trades_latest_price: { args: [FFIType.pointer], returns: FFIType.f64 },
  fig_ledger_new: { args: [FFIType.u64], returns: FFIType.pointer },
  fig_ledger_free: { args: [FFIType.pointer], returns: FFIType.void },
  fig_ledger_apply: {
    args: [FFIType.pointer, FFIType.pointer, FFIType.u64],
    returns: FFIType.i32,
  },
  fig_ledger_len: { args: [FFIType.pointer], returns: FFIType.u64 },
  fig_liquidation_new: { args: [FFIType.u64], returns: FFIType.pointer },
  fig_liquidation_free: { args: [FFIType.pointer], returns: FFIType.void },
  fig_liquidation_apply_user: {
    args: [FFIType.pointer, FFIType.pointer, FFIType.u64],
    returns: FFIType.i32,
  },
  fig_liquidation_apply_public: {
    args: [FFIType.pointer, FFIType.pointer, FFIType.u64],
    returns: FFIType.i32,
  },
  fig_liquidation_user_count: { args: [FFIType.pointer], returns: FFIType.u64 },
});

const libc = dlopen(
  process.platform === "darwin" ? "libSystem.B.dylib" : "libc.so.6",
  {
    memcpy: {
      args: [FFIType.pointer, FFIType.pointer, FFIType.u64],
      returns: FFIType.pointer,
    },
    strlen: { args: [FFIType.pointer], returns: FFIType.u64 },
  },
);

function outSlot(): Buffer {
  return Buffer.alloc(16);
}

function copyRaw(addr: number, len: number): Buffer {
  const out = Buffer.alloc(len);
  libc.symbols.memcpy(ptr(out), addr, len);
  return out;
}

function takeOwnedBytes(slot: Buffer): Buffer {
  const data = Number(slot.readBigUInt64LE(0));
  const len = Number(slot.readBigUInt64LE(8));
  if (!data || len === 0) {
    return Buffer.alloc(0);
  }
  const copied = copyRaw(data, len);
  fig.symbols.fig_buffer_free(data, len);
  return copied;
}

function takeFrameList(slot: Buffer): Buffer[] {
  const framesPtr = Number(slot.readBigUInt64LE(0));
  const count = Number(slot.readBigUInt64LE(8));
  if (!framesPtr || count === 0) {
    return [];
  }
  const table = copyRaw(framesPtr, count * 16);
  const out: Buffer[] = [];
  for (let i = 0; i < count; i++) {
    const data = Number(table.readBigUInt64LE(i * 16));
    const len = Number(table.readBigUInt64LE(i * 16 + 8));
    if (data && len > 0) {
      out.push(copyRaw(data, len));
    }
  }
  fig.symbols.fig_frame_list_free(framesPtr, count);
  return out;
}

function readHandle(slot: Buffer): Pointer {
  return Number(slot.readBigUInt64LE(0)) as unknown as Pointer;
}

export function version(): string {
  return fig.symbols.fig_version() as string;
}

/** Held-open SUBSCRIBE recv stream. timeoutMs 0 waits forever. */
export class FigSubscription {
  private handle: Pointer;

  constructor(handle: Pointer) {
    this.handle = handle;
  }

  /** Next live frame. `null` on EOF. */
  next(timeoutMs = 0): Buffer | null {
    const out = outSlot();
    const rc = fig.symbols.fig_client_sub_next(
      this.handle,
      timeoutMs,
      ptr(out),
    ) as number;
    if (rc === 1) {
      throw new Error("fig_client_sub_next timeout");
    }
    if (rc === 2) {
      return null;
    }
    if (rc !== 0) {
      throw new Error(`fig_client_sub_next failed: ${rc}`);
    }
    return takeOwnedBytes(out);
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_client_sub_close(this.handle);
      this.handle = ptr(null);
    }
  }
}

export class FigClient {
  private handle: Pointer;

  constructor(addr: string, serverName: string | null = "localhost", resumptionToken?: Buffer) {
    const out = Buffer.alloc(8);
    let rc: number;
    if (resumptionToken && resumptionToken.length > 0) {
      rc = fig.symbols.fig_client_connect_0rtt(
        addr,
        serverName,
        resumptionToken,
        resumptionToken.length,
        ptr(out),
      ) as number;
    } else {
      rc = fig.symbols.fig_client_connect(addr, serverName, ptr(out)) as number;
    }
    if (rc !== 0) {
      throw new Error(`fig_client_connect failed: ${rc}`);
    }
    this.handle = readHandle(out);
  }

  ping(): void {
    const rc = fig.symbols.fig_client_ping(this.handle) as number;
    if (rc !== 0) {
      throw new Error(`fig_client_ping failed: ${rc}`);
    }
  }

  request(
    channelPath: string,
    method: string,
    payload: Buffer = Buffer.alloc(0),
    opts: {
      authToken?: string | null;
      contentType?: string | null;
      channelId?: number;
      streamSeq?: number;
      schemaId?: number;
    } = {},
  ): Buffer[] {
    const frameBuf = outSlot();
    const rc = fig.symbols.fig_frame_encode_request_auth(
      opts.channelId ?? 1,
      opts.streamSeq ?? 1,
      opts.schemaId ?? 1,
      channelPath,
      method,
      opts.contentType ?? (payload.length ? "application/cbor" : null),
      opts.authToken ?? null,
      payload.length ? payload : ptr(null),
      payload.length,
      ptr(frameBuf),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_frame_encode_request_auth failed: ${rc}`);
    }
    return this.requestAndRecv(takeOwnedBytes(frameBuf));
  }

  /** SUBSCRIBE snapshot plus a live handle. Do not use requestAndRecv for SUBSCRIBE. */
  subscribe(
    channelPath: string,
    routingKey?: string,
    authToken?: string | null,
    channelId = 1,
    streamSeq = 1,
  ): { snapshot: Buffer[]; subscription: FigSubscription } {
    const frameBuf = outSlot();
    const rc = fig.symbols.fig_frame_encode_subscribe_auth(
      channelId,
      streamSeq,
      routingKey ?? channelPath,
      channelPath,
      authToken ?? null,
      ptr(frameBuf),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_frame_encode_subscribe_auth failed: ${rc}`);
    }
    return this.subscribeFrame(takeOwnedBytes(frameBuf));
  }

  subscribeFrame(frameBytes: Buffer): { snapshot: Buffer[]; subscription: FigSubscription } {
    const list = outSlot();
    const subOut = Buffer.alloc(8);
    const rc = fig.symbols.fig_client_subscribe(
      this.handle,
      frameBytes,
      frameBytes.length,
      ptr(list),
      ptr(subOut),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_client_subscribe failed: ${rc}`);
    }
    return {
      snapshot: takeFrameList(list),
      subscription: new FigSubscription(readHandle(subOut)),
    };
  }

  requestAndRecv(frameBytes: Buffer): Buffer[] {
    const list = outSlot();
    const rc = fig.symbols.fig_client_request_and_recv(
      this.handle,
      frameBytes,
      frameBytes.length,
      ptr(list),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_client_request_and_recv failed: ${rc}`);
    }
    return takeFrameList(list);
  }

  /** Decode STREAM_ITEM payload helpers for typed streams. */
  decodeExecutionClOrdId(frame: Buffer): string {
    const out = Buffer.alloc(8);
    const payload = this.framePayload(frame);
    const rc = fig.symbols.fig_cbor_decode_execution_report_cl_ord_id(
      payload,
      payload.length,
      ptr(out),
    ) as number;
    if (rc !== 0) {
      throw new Error(`decode execution report failed: ${rc}`);
    }
    const p = Number(readHandle(out));
    const n = Number(libc.symbols.strlen(p));
    const s = copyRaw(p, n).toString("utf8");
    fig.symbols.fig_string_free(p);
    return s;
  }

  framePayload(frame: Buffer): Buffer {
    const out = outSlot();
    const rc = fig.symbols.fig_frame_payload(
      frame,
      frame.length,
      ptr(out),
      ptr(null),
      ptr(null),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_frame_payload failed: ${rc}`);
    }
    return takeOwnedBytes(out);
  }

  isStreamItem(frame: Buffer): boolean {
    return (fig.symbols.fig_frame_is_stream_item(frame, frame.length) as number) === 1;
  }

  compressPayload(data: Buffer): Buffer {
    const out = outSlot();
    const rc = fig.symbols.fig_payload_compress(
      data,
      data.length,
      ptr(out),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_payload_compress failed: ${rc}`);
    }
    return takeOwnedBytes(out);
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_client_close(this.handle);
      this.handle = ptr(null);
    }
  }
}

export function jwtEncode(sub: string, exp: bigint, secret: string): string {
  const out = outSlot();
  const rc = fig.symbols.fig_jwt_encode(
    sub,
    exp,
    secret,
    ptr(out),
  ) as number;
  if (rc !== 0) {
    throw new Error(`fig_jwt_encode failed: ${rc}`);
  }
  return takeOwnedBytes(out).toString("utf8");
}

export function jwtVerifyBearer(token: string, secret: string): void {
  const rc = fig.symbols.fig_jwt_verify_bearer(token, secret) as number;
  if (rc !== 0) {
    throw new Error(`fig_jwt_verify_bearer failed: ${rc}`);
  }
}

export function sbeEncodeNewOrderSingle(
  clOrdId: string,
  symbol: string,
  sideBuy: boolean,
  qty: number,
  price: number,
): Buffer {
  const out = outSlot();
  const rc = fig.symbols.fig_sbe_encode_new_order_single(
    clOrdId,
    symbol,
    sideBuy ? 1 : 0,
    qty,
    price,
    -1,
    -1,
    ptr(out),
  ) as number;
  if (rc !== 0) {
    throw new Error(`fig_sbe_encode_new_order_single failed: ${rc}`);
  }
  return takeOwnedBytes(out);
}

/** RAII merge-state wrappers over fig.h stream merge handles. */

export class FundingState {
  private handle: Pointer;

  constructor(capacity = 128) {
    this.handle = fig.symbols.fig_funding_new(capacity) as Pointer;
    if (!this.handle) {
      throw new Error("fig_funding_new failed");
    }
  }

  apply(payload: Buffer): void {
    const rc = fig.symbols.fig_funding_apply(
      this.handle,
      payload,
      payload.length,
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_funding_apply failed: ${rc}`);
    }
  }

  len(): number {
    return Number(fig.symbols.fig_funding_len(this.handle));
  }

  latestAmount(): number {
    return fig.symbols.fig_funding_latest_amount(this.handle) as number;
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_funding_free(this.handle);
      this.handle = ptr(null);
    }
  }
}

export class AggTradesState {
  private handle: Pointer;

  constructor(capacity = 256) {
    this.handle = fig.symbols.fig_agg_trades_new(capacity) as Pointer;
    if (!this.handle) {
      throw new Error("fig_agg_trades_new failed");
    }
  }

  apply(payload: Buffer): void {
    const rc = fig.symbols.fig_agg_trades_apply(
      this.handle,
      payload,
      payload.length,
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_agg_trades_apply failed: ${rc}`);
    }
  }

  latestPrice(): number {
    return fig.symbols.fig_agg_trades_latest_price(this.handle) as number;
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_agg_trades_free(this.handle);
      this.handle = ptr(null);
    }
  }
}

export class LedgerState {
  private handle: Pointer;

  constructor(capacity = 128) {
    this.handle = fig.symbols.fig_ledger_new(capacity) as Pointer;
    if (!this.handle) {
      throw new Error("fig_ledger_new failed");
    }
  }

  apply(payload: Buffer): void {
    const rc = fig.symbols.fig_ledger_apply(
      this.handle,
      payload,
      payload.length,
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_ledger_apply failed: ${rc}`);
    }
  }

  len(): number {
    return Number(fig.symbols.fig_ledger_len(this.handle));
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_ledger_free(this.handle);
      this.handle = ptr(null);
    }
  }
}

export class LiquidationState {
  private handle: Pointer;

  constructor(capacity = 64) {
    this.handle = fig.symbols.fig_liquidation_new(capacity) as Pointer;
    if (!this.handle) {
      throw new Error("fig_liquidation_new failed");
    }
  }

  applyUser(payload: Buffer): void {
    const rc = fig.symbols.fig_liquidation_apply_user(
      this.handle,
      payload,
      payload.length,
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_liquidation_apply_user failed: ${rc}`);
    }
  }

  applyPublic(payload: Buffer): void {
    const rc = fig.symbols.fig_liquidation_apply_public(
      this.handle,
      payload,
      payload.length,
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_liquidation_apply_public failed: ${rc}`);
    }
  }

  userCount(): number {
    return Number(fig.symbols.fig_liquidation_user_count(this.handle));
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_liquidation_free(this.handle);
      this.handle = ptr(null);
    }
  }
}

export { libPath };
