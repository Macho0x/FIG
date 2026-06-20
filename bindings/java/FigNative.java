package fig;

/**
 * JNI-style native bindings over fig.h — load libfig_ffi before use.
 *
 * <pre>{@code
 * System.loadLibrary("fig_ffi");
 * long handle = FigNative.connect("127.0.0.1:8443", "localhost");
 * FigNative.ping(handle);
 * FigNative.close(handle);
 * }</pre>
 */
public final class FigNative {
    static {
        System.loadLibrary("fig_ffi");
    }

    private FigNative() {}

    public static native int figClientConnect(String addr, String serverName, long[] outHandle);

    public static native void figClientClose(long handle);

    public static native int figClientPing(long handle);

    public static native int figPayloadCompress(byte[] data, byte[][] out);

    public static native int figFrameEncodeSubscribeAuth(
            short channelId,
            int streamSeq,
            String routingKey,
            String channelPath,
            String authToken,
            byte[][] out);

    public static long connect(String addr, String serverName) {
        long[] out = new long[1];
        if (figClientConnect(addr, serverName, out) != 0) {
            throw new IllegalStateException("fig_client_connect failed");
        }
        return out[0];
    }

    public static void ping(long handle) {
        if (figClientPing(handle) != 0) {
            throw new IllegalStateException("fig_client_ping failed");
        }
    }

    public static void close(long handle) {
        figClientClose(handle);
    }
}
