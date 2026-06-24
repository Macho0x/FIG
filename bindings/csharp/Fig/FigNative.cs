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

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            FigNative.fig_client_close(_handle);
            _handle = IntPtr.Zero;
        }
    }
}
