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

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr fig_version();

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void fig_buffer_free(FigBuffer buf);

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
    public static extern int fig_frame_encode_subscribe(
        ushort channelId,
        uint streamSeq,
        string routingKey,
        string channelPath,
        out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int fig_frame_encode_ping(out FigBuffer outBuf);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern ulong fig_client_channel_stream_id(ushort channelId, byte isServer);

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
}
