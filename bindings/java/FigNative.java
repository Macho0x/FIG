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

    public static native int figJwtEncode(String sub, long exp, String secret, byte[][] out);

    public static native int figJwtVerifyBearer(String token, String secret);

    public static native int figSbeEncodeNewOrderSingle(
            String clOrdId,
            String symbol,
            byte sideBuy,
            double orderQty,
            double price,
            byte[][] out);

    public static String jwtEncode(String sub, long exp, String secret) {
        byte[][] out = new byte[1][];
        if (figJwtEncode(sub, exp, secret, out) != 0) {
            throw new IllegalStateException("fig_jwt_encode failed");
        }
        return new String(out[0], java.nio.charset.StandardCharsets.UTF_8);
    }

    public static void jwtVerifyBearer(String token, String secret) {
        if (figJwtVerifyBearer(token, secret) != 0) {
            throw new IllegalStateException("fig_jwt_verify_bearer failed");
        }
    }

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
