/**
 * TypeScript bindings over fig.h — Node/Bun via `node:ffi` (native libfig_ffi).
 * Browser clients should use fig-gateway REST/WebSocket instead.
 */

import { dlopen, FFIType, ptr, type Pointer } from "node:ffi";

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
  fig_version: { parameters: [], result: FFIType.cstring },
  fig_buffer_free: { parameters: [FFIType.pointer], result: FFIType.void },
  fig_frame_list_free: { parameters: [FFIType.pointer], result: FFIType.void },
  fig_client_connect: {
    parameters: [FFIType.cstring, FFIType.cstring, FFIType.pointer],
    result: FFIType.i32,
  },
  fig_client_close: { parameters: [FFIType.pointer], result: FFIType.void },
  fig_client_ping: { parameters: [FFIType.pointer], result: FFIType.i32 },
  fig_client_request_and_recv: {
    parameters: [FFIType.pointer, FFIType.pointer, FFIType.u64, FFIType.pointer],
    result: FFIType.i32,
  },
  fig_frame_encode_request_auth: {
    parameters: [
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
    result: FFIType.i32,
  },
  fig_frame_encode_subscribe_auth: {
    parameters: [
      FFIType.u16,
      FFIType.u32,
      FFIType.cstring,
      FFIType.cstring,
      FFIType.cstring,
      FFIType.pointer,
    ],
    result: FFIType.i32,
  },
});

export function version(): string {
  return fig.symbols.fig_version() as string;
}

export class FigClient {
  private handle: Pointer;

  constructor(addr: string, serverName: string | null = "localhost") {
    const out = ptr(null);
    const rc = fig.symbols.fig_client_connect(addr, serverName, out) as number;
    if (rc !== 0) {
      throw new Error(`fig_client_connect failed: ${rc}`);
    }
    this.handle = out.deref() as Pointer;
  }

  ping(): void {
    const rc = fig.symbols.fig_client_ping(this.handle) as number;
    if (rc !== 0) {
      throw new Error(`fig_client_ping failed: ${rc}`);
    }
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_client_close(this.handle);
      this.handle = ptr(null);
    }
  }
}

export { libPath };
