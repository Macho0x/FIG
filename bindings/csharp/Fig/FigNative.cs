using System;
using System.Runtime.InteropServices;

namespace Fig;

/// <summary>
/// Thin P/Invoke wrapper over fig.h — link libfig_ffi at build time.
/// </summary>
public static class FigNative
{
    private const string Lib = "fig_ffi";

    [StructLayout(LayoutKind.Sequential)]
    public struct FigBuffer
    {
        public IntPtr Data;
        public UIntPtr Len;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct FigFrameList
    {
        public IntPtr Frames;
        public UIntPtr Count;
    }

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_version();

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_buffer_free(FigBuffer buf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_frame_list_free(FigFrameList list);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_frame_encode_request_ex(
        ushort channelId,
        uint streamSeq,
        byte schemaId,
        string? channelPath,
        string? method,
        string? contentType,
        byte[]? payload,
        UIntPtr payloadLen,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_frame_encode_request_auth(
        ushort channelId,
        uint streamSeq,
        byte schemaId,
        string? channelPath,
        string? method,
        string? contentType,
        string? authToken,
        byte[]? payload,
        UIntPtr payloadLen,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_frame_encode_subscribe(
        ushort channelId,
        uint streamSeq,
        string routingKey,
        string channelPath,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_frame_encode_subscribe_auth(
        ushort channelId,
        uint streamSeq,
        string routingKey,
        string channelPath,
        string? authToken,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_frame_encode_ping(out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern ulong fig_client_channel_stream_id(ushort channelId, byte isServer);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_connect(
        string addr,
        string? serverName,
        out IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_client_close(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_ping(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_request_and_recv(
        IntPtr handle,
        byte[] frameBytes,
        UIntPtr frameLen,
        out FigFrameList outList);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_subscribe(
        IntPtr handle,
        byte[] frameBytes,
        UIntPtr frameLen,
        out FigFrameList snapshotOut,
        out IntPtr subOut);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_sub_next(
        IntPtr sub,
        uint timeoutMs,
        out FigBuffer frameOut);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_client_sub_close(IntPtr sub);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_jwt_encode(
        string sub,
        ulong exp,
        string secret,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_jwt_decode_sub(
        string token,
        string secret,
        out IntPtr outSub);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_jwt_verify_bearer(string token, string secret);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_string_free(IntPtr s);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_cbor_encode_mass_cancel_request(
        string clOrdId,
        string account,
        string? symbol,
        byte side,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_cbor_decode_mass_cancel_request(
        byte[] data,
        UIntPtr len,
        out IntPtr outClOrdId,
        out IntPtr outAccount,
        out IntPtr outSymbol,
        out byte outSide);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_cbor_decode_trading_session_status(
        byte[] data,
        UIntPtr len,
        out byte outStatus,
        out IntPtr outSessionId);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_cbor_decode_security_status(
        byte[] data,
        UIntPtr len,
        out IntPtr outSymbol,
        out byte outStatus);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_cbor_decode_mass_cancel_report(
        byte[] data,
        UIntPtr len,
        out IntPtr outClOrdId,
        out uint outTotalAffected,
        out byte outStatus);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_request_session_status(
        IntPtr handle,
        ushort channelId,
        out byte outStatus,
        out IntPtr outSessionId);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_request_security_status(
        IntPtr handle,
        string symbol,
        ushort channelId,
        out IntPtr outSymbol,
        out byte outStatus);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_mass_cancel(
        IntPtr handle,
        string clOrdId,
        string account,
        string? symbol,
        byte side,
        ushort channelId,
        out IntPtr outClOrdId,
        out uint outTotalAffected,
        out byte outStatus);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_sbe_encode_new_order_single(
        string clOrdId,
        string symbol,
        byte sideBuy,
        double orderQty,
        double price,
        sbyte postOnly,
        sbyte reduceOnly,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_funding_new(UIntPtr capacity);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_funding_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_funding_apply(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern UIntPtr fig_funding_len(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_funding_latest_amount(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_agg_trades_new(UIntPtr capacity);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_agg_trades_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_agg_trades_apply(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern UIntPtr fig_agg_trades_len(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_agg_trades_latest_price(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_ledger_new(UIntPtr capacity);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_ledger_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_ledger_apply(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern UIntPtr fig_ledger_len(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_liquidation_new(UIntPtr capacity);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_liquidation_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_liquidation_apply_user(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_liquidation_apply_public(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern UIntPtr fig_liquidation_user_count(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_client_post_batch(
        IntPtr handle,
        string account,
        byte[] payload,
        UIntPtr payloadLen,
        ushort channelId,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_order_book_new();

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_order_book_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_order_book_apply_snapshot(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_order_book_apply_delta(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_order_book_best_bid(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_mids_new();

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_mids_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_mids_apply_ticker(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_mids_apply_batch(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_mids_mid(IntPtr handle, string symbol);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_bbo_new();

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_bbo_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_bbo_apply(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_bbo_implied_mid(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_bbo_best_bid(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_bbo_best_ask(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_trade_tape_new(UIntPtr capacity);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_trade_tape_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_trade_tape_push(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern UIntPtr fig_trade_tape_len(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_trade_tape_latest_price(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_mark_price_new();

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_mark_price_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_mark_price_apply(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_mark_price_get(IntPtr handle, string symbol);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_orders_new(UIntPtr execCapacity);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_orders_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_orders_apply_snapshot(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_orders_apply_execution(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern UIntPtr fig_orders_open_count(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern UIntPtr fig_orders_execution_count(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_candle_new();

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_candle_free(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_candle_apply(IntPtr handle, IntPtr payload, UIntPtr len);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern double fig_candle_close(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_candle_is_final(IntPtr handle);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_candle_symbol(IntPtr handle, out IntPtr outText);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_candle_interval(IntPtr handle, out IntPtr outText);

    public static byte[] EncodeRequest(
        ushort channelId, uint streamSeq, byte schemaId, string? channelPath, string? method, byte[]? payload)
    {
        if (fig_frame_encode_request_ex(
                channelId, streamSeq, schemaId, channelPath, method, null, payload,
                (UIntPtr)(payload?.Length ?? 0), out var buf) != 0)
        {
            throw new InvalidOperationException("fig_frame_encode_request_ex failed");
        }
        return CopyBuffer(buf);
    }

    public static byte[] EncodeRequestAuth(
        ushort channelId, uint streamSeq, byte schemaId, string? channelPath, string? method,
        string? authToken, byte[]? payload)
    {
        if (fig_frame_encode_request_auth(
                channelId, streamSeq, schemaId, channelPath, method, null, authToken, payload,
                (UIntPtr)(payload?.Length ?? 0), out var buf) != 0)
        {
            throw new InvalidOperationException("fig_frame_encode_request_auth failed");
        }
        return CopyBuffer(buf);
    }

    public static byte[] EncodeSubscribe(ushort channelId, uint streamSeq, string routingKey, string channelPath)
    {
        if (fig_frame_encode_subscribe(channelId, streamSeq, routingKey, channelPath, out var buf) != 0)
        {
            throw new InvalidOperationException("fig_frame_encode_subscribe failed");
        }
        return CopyBuffer(buf);
    }

    public static byte[] EncodeSubscribeAuth(
        ushort channelId, uint streamSeq, string routingKey, string channelPath, string? authToken)
    {
        if (fig_frame_encode_subscribe_auth(
                channelId, streamSeq, routingKey, channelPath, authToken, out var buf) != 0)
        {
            throw new InvalidOperationException("fig_frame_encode_subscribe_auth failed");
        }
        return CopyBuffer(buf);
    }

    public static ulong ChannelStreamId(ushort channelId, bool isServer)
    {
        return fig_client_channel_stream_id(channelId, (byte)(isServer ? 1 : 0));
    }

    public static string JwtEncode(string sub, ulong exp, string secret)
    {
        if (fig_jwt_encode(sub, exp, secret, out var buf) != 0)
        {
            throw new InvalidOperationException("fig_jwt_encode failed");
        }
        return System.Text.Encoding.UTF8.GetString(CopyBuffer(buf));
    }

    public static string JwtDecodeSub(string token, string secret)
    {
        if (fig_jwt_decode_sub(token, secret, out var subPtr) != 0)
        {
            throw new InvalidOperationException("fig_jwt_decode_sub failed");
        }
        try
        {
            return Marshal.PtrToStringUTF8(subPtr) ?? string.Empty;
        }
        finally
        {
            fig_string_free(subPtr);
        }
    }

    public static void JwtVerifyBearer(string token, string secret)
    {
        if (fig_jwt_verify_bearer(token, secret) != 0)
        {
            throw new InvalidOperationException("fig_jwt_verify_bearer failed");
        }
    }

    public static byte[] SbeEncodeNewOrderSingle(
        string clOrdId,
        string symbol,
        bool sideBuy,
        double orderQty,
        double price)
    {
        if (fig_sbe_encode_new_order_single(
                clOrdId,
                symbol,
                (byte)(sideBuy ? 1 : 0),
                orderQty,
                price,
                -1,
                -1,
                out var buf) != 0)
        {
            throw new InvalidOperationException("fig_sbe_encode_new_order_single failed");
        }
        return CopyBuffer(buf);
    }

    internal static string TakeString(IntPtr p)
    {
        if (p == IntPtr.Zero)
        {
            return "";
        }
        var s = Marshal.PtrToStringUTF8(p) ?? "";
        fig_string_free(p);
        return s;
    }

    public static byte[] CborEncodeMassCancelRequest(string clOrdId, string account, string? symbol, byte side)
    {
        if (fig_cbor_encode_mass_cancel_request(clOrdId, account, symbol, side, out var buf) != 0)
        {
            throw new InvalidOperationException("fig_cbor_encode_mass_cancel_request failed");
        }
        return CopyBuffer(buf);
    }

    public static (string ClOrdId, string Account, string Symbol, byte Side) CborDecodeMassCancelRequest(byte[] data)
    {
        if (fig_cbor_decode_mass_cancel_request(
                data, (UIntPtr)data.Length, out var cl, out var account, out var symbol, out var side) != 0)
        {
            throw new InvalidOperationException("fig_cbor_decode_mass_cancel_request failed");
        }
        return (TakeString(cl), TakeString(account), TakeString(symbol), side);
    }

    public static (byte Status, string SessionId) CborDecodeTradingSessionStatus(byte[] data)
    {
        if (fig_cbor_decode_trading_session_status(data, (UIntPtr)data.Length, out var status, out var session) != 0)
        {
            throw new InvalidOperationException("fig_cbor_decode_trading_session_status failed");
        }
        return (status, TakeString(session));
    }

    public static (string Symbol, byte Status) CborDecodeSecurityStatus(byte[] data)
    {
        if (fig_cbor_decode_security_status(data, (UIntPtr)data.Length, out var symbol, out var status) != 0)
        {
            throw new InvalidOperationException("fig_cbor_decode_security_status failed");
        }
        return (TakeString(symbol), status);
    }

    public static (string ClOrdId, uint TotalAffected, byte Status) CborDecodeMassCancelReport(byte[] data)
    {
        if (fig_cbor_decode_mass_cancel_report(
                data, (UIntPtr)data.Length, out var cl, out var total, out var status) != 0)
        {
            throw new InvalidOperationException("fig_cbor_decode_mass_cancel_report failed");
        }
        return (TakeString(cl), total, status);
    }

    public static string Version => Marshal.PtrToStringUTF8(fig_version()) ?? "unknown";

    public static byte[] CopyBuffer(FigBuffer buf)
    {
        if (buf.Data == IntPtr.Zero || buf.Len == UIntPtr.Zero)
        {
            fig_buffer_free(buf);
            return Array.Empty<byte>();
        }
        var len = (int)buf.Len;
        var managed = new byte[len];
        Marshal.Copy(buf.Data, managed, 0, len);
        fig_buffer_free(buf);
        return managed;
    }

    public static byte[][] CopyFrameList(FigFrameList list)
    {
        if (list.Frames == IntPtr.Zero || list.Count == UIntPtr.Zero)
        {
            fig_frame_list_free(list);
            return Array.Empty<byte[]>();
        }
        var count = (int)list.Count;
        var stride = Marshal.SizeOf<FigBuffer>();
        var frames = new byte[count][];
        for (var i = 0; i < count; i++)
        {
            var ptr = list.Frames + i * stride;
            var buf = Marshal.PtrToStructure<FigBuffer>(ptr);
            frames[i] = CopyBuffer(buf);
        }
        fig_frame_list_free(list);
        return frames;
    }
}

/// <summary>Connected FIG client over TREE.</summary>
public sealed class FigClient : IDisposable
{
    private IntPtr _handle;

    public FigClient(string addr, string? serverName = null)
    {
        if (FigNative.fig_client_connect(addr, serverName, out _handle) != 0)
        {
            throw new InvalidOperationException("fig_client_connect failed");
        }
    }

    public void Ping()
    {
        if (FigNative.fig_client_ping(_handle) != 0)
        {
            throw new InvalidOperationException("fig_client_ping failed");
        }
    }

    public byte[][] RequestAndRecv(byte[] frameBytes)
    {
        if (FigNative.fig_client_request_and_recv(
                _handle,
                frameBytes,
                (UIntPtr)frameBytes.Length,
                out var list) != 0)
        {
            throw new InvalidOperationException("fig_client_request_and_recv failed");
        }
        return FigNative.CopyFrameList(list);
    }

    public FigSubscription Subscribe(byte[] frameBytes)
    {
        if (FigNative.fig_client_subscribe(
                _handle,
                frameBytes,
                (UIntPtr)frameBytes.Length,
                out var list,
                out var sub) != 0)
        {
            throw new InvalidOperationException("fig_client_subscribe failed");
        }
        return new FigSubscription(sub, FigNative.CopyFrameList(list));
    }

    public (byte Status, string SessionId) RequestSessionStatus(ushort channelId = 1)
    {
        if (FigNative.fig_client_request_session_status(_handle, channelId, out var status, out var session) != 0)
        {
            throw new InvalidOperationException("fig_client_request_session_status failed");
        }
        return (status, FigNative.TakeString(session));
    }

    public (string Symbol, byte Status) RequestSecurityStatus(string symbol, ushort channelId = 1)
    {
        if (FigNative.fig_client_request_security_status(_handle, symbol, channelId, out var outSymbol, out var status) != 0)
        {
            throw new InvalidOperationException("fig_client_request_security_status failed");
        }
        return (FigNative.TakeString(outSymbol), status);
    }

    public (string ClOrdId, uint TotalAffected, byte Status) MassCancel(
        string clOrdId, string account, string? symbol, byte side, ushort channelId = 1)
    {
        if (FigNative.fig_client_mass_cancel(
                _handle, clOrdId, account, symbol, side, channelId, out var cl, out var total, out var status) != 0)
        {
            throw new InvalidOperationException("fig_client_mass_cancel failed");
        }
        return (FigNative.TakeString(cl), total, status);
    }

    public byte[] PostBatch(string account, byte[] payload, ushort channelId = 1)
    {
        if (FigNative.fig_client_post_batch(
                _handle, account, payload, (UIntPtr)payload.Length, channelId, out var buf) != 0)
        {
            throw new InvalidOperationException("fig_client_post_batch failed");
        }
        return FigNative.CopyBuffer(buf);
    }

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_client_close(_handle);
            _handle = IntPtr.Zero;
        }
    }
}

/// <summary>Held-open SUBSCRIBE recv stream.</summary>
public sealed class FigSubscription : IDisposable
{
    private IntPtr _handle;

    internal FigSubscription(IntPtr handle, byte[][] snapshot)
    {
        _handle = handle;
        Snapshot = snapshot;
    }

    public byte[][] Snapshot { get; }

    /// <summary>Next live frame. timeoutMs 0 waits forever. Null on EOF.</summary>
    public byte[]? Next(uint timeoutMs = 0)
    {
        if (_handle == IntPtr.Zero)
        {
            throw new ObjectDisposedException(nameof(FigSubscription));
        }
        var rc = FigNative.fig_client_sub_next(_handle, timeoutMs, out var buf);
        if (rc == 1)
        {
            throw new TimeoutException("fig_client_sub_next timeout");
        }
        if (rc == 2)
        {
            return null;
        }
        if (rc != 0)
        {
            throw new InvalidOperationException("fig_client_sub_next failed");
        }
        return FigNative.CopyBuffer(buf);
    }

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_client_sub_close(_handle);
            _handle = IntPtr.Zero;
        }
    }
}
