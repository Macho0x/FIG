/**
 * TypeScript bindings over fig.h — Node/Bun via `node:ffi` (native libfig_ffi).
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
  fig_string_free: { parameters: [FFIType.pointer], result: FFIType.void },
  fig_client_connect: {
    parameters: [FFIType.cstring, FFIType.cstring, FFIType.pointer],
    result: FFIType.i32,
  },
  fig_client_connect_0rtt: {
    parameters: [
      FFIType.cstring,
      FFIType.cstring,
      FFIType.pointer,
      FFIType.u64,
      FFIType.pointer,
    ],
    result: FFIType.i32,
  },
  fig_client_close: { parameters: [FFIType.pointer], result: FFIType.void },
  fig_client_ping: { parameters: [FFIType.pointer], result: FFIType.void },
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
  fig_payload_compress: {
    parameters: [FFIType.pointer, FFIType.u64, FFIType.pointer],
    result: FFIType.i32,
  },
  fig_payload_decompress: {
    parameters: [FFIType.pointer, FFIType.u64, FFIType.pointer],
    result: FFIType.i32,
  },
  fig_frame_is_stream_item: {
    parameters: [FFIType.pointer, FFIType.u64],
    result: FFIType.i32,
  },
  fig_frame_payload: {
    parameters: [
      FFIType.pointer,
      FFIType.u64,
      FFIType.pointer,
      FFIType.pointer,
      FFIType.pointer,
    ],
    result: FFIType.i32,
  },
  fig_cbor_decode_execution_report_cl_ord_id: {
    parameters: [FFIType.pointer, FFIType.u64, FFIType.pointer],
    result: FFIType.i32,
  },
});

function copyBuffer(buf: FigBuffer): Buffer {
  if (!buf.data || buf.len === 0) {
    fig.symbols.fig_buffer_free(buf);
    return Buffer.alloc(0);
  }
  const slice = buf.data.readBuffer(buf.len);
  fig.symbols.fig_buffer_free(buf);
  return Buffer.from(slice);
}

function emptyOutBuffer(): { buf: FigBuffer } {
  return { buf: { data: ptr(null), len: 0 } };
}

export function version(): string {
  return fig.symbols.fig_version() as string;
}

export class FigClient {
  private handle: Pointer;

  constructor(addr: string, serverName: string | null = "localhost", resumptionToken?: Buffer) {
    const out = ptr(null);
    let rc: number;
    if (resumptionToken && resumptionToken.length > 0) {
      rc = fig.symbols.fig_client_connect_0rtt(
        addr,
        serverName,
        resumptionToken,
        resumptionToken.length,
        out,
      ) as number;
    } else {
      rc = fig.symbols.fig_client_connect(addr, serverName, out) as number;
    }
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
    const { buf: frameBuf } = emptyOutBuffer();
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
    const encoded = copyBuffer(frameBuf);
    return this.requestAndRecv(encoded);
  }

  subscribe(
    channelPath: string,
    routingKey?: string,
    authToken?: string | null,
    channelId = 1,
    streamSeq = 1,
  ): Buffer[] {
    const { buf: frameBuf } = emptyOutBuffer();
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
    const encoded = copyBuffer(frameBuf);
    return this.requestAndRecv(encoded);
  }

  requestAndRecv(frameBytes: Buffer): Buffer[] {
    const list: FigFrameList = { frames: ptr(null), count: 0 };
    const rc = fig.symbols.fig_client_request_and_recv(
      this.handle,
      frameBytes,
      frameBytes.length,
      ptr(list),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_client_request_and_recv failed: ${rc}`);
    }
    if (!list.frames || list.count === 0) {
      fig.symbols.fig_frame_list_free(list);
      return [];
    }
    const stride = 16; // FigBuffer layout: ptr + len
    const out: Buffer[] = [];
    for (let i = 0; i < list.count; i++) {
      const framePtr = list.frames.add(i * stride);
      const data = framePtr.readPointer();
      const len = Number(framePtr.add(8).readBigUInt64LE());
      if (data && len > 0) {
        out.push(Buffer.from(data.readBuffer(len)));
      }
    }
    fig.symbols.fig_frame_list_free(list);
    return out;
  }

  /** Decode STREAM_ITEM payload helpers for typed streams. */
  decodeExecutionClOrdId(frame: Buffer): string {
    const out = ptr(null);
    const payload = this.framePayload(frame);
    const rc = fig.symbols.fig_cbor_decode_execution_report_cl_ord_id(
      payload,
      payload.length,
      out,
    ) as number;
    if (rc !== 0) {
      throw new Error(`decode execution report failed: ${rc}`);
    }
    const s = out.deref() as unknown as string;
    fig.symbols.fig_string_free(out.deref() as Pointer);
    return s;
  }

  framePayload(frame: Buffer): Buffer {
    const { buf: out } = emptyOutBuffer();
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
    return copyBuffer(out);
  }

  isStreamItem(frame: Buffer): boolean {
    return (fig.symbols.fig_frame_is_stream_item(frame, frame.length) as number) === 1;
  }

  compressPayload(data: Buffer): Buffer {
    const { buf: out } = emptyOutBuffer();
    const rc = fig.symbols.fig_payload_compress(
      data,
      data.length,
      ptr(out),
    ) as number;
    if (rc !== 0) {
      throw new Error(`fig_payload_compress failed: ${rc}`);
    }
    return copyBuffer(out);
  }

  close(): void {
    if (this.handle) {
      fig.symbols.fig_client_close(this.handle);
      this.handle = ptr(null);
    }
  }
}

export { libPath };
