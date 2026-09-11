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

    public static native int figClientSubscribe(long handle, byte[] frame, Object[] snapshotOut, long[] subOut);

    public static native int figClientSubNext(long sub, int timeoutMs, byte[][] frameOut);

    public static native void figClientSubClose(long sub);

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

    public static long subscribe(long handle, byte[] frame, java.util.List<byte[]> snapshotOut) {
        Object[] snap = new Object[1];
        long[] sub = new long[1];
        if (figClientSubscribe(handle, frame, snap, sub) != 0) {
            throw new IllegalStateException("fig_client_subscribe failed");
        }
        if (snap[0] instanceof byte[][]) {
            for (byte[] f : (byte[][]) snap[0]) {
                snapshotOut.add(f);
            }
        }
        return sub[0];
    }

    public static byte[] subNext(long sub, int timeoutMs) {
        byte[][] out = new byte[1][];
        int rc = figClientSubNext(sub, timeoutMs, out);
        if (rc == 1) {
            throw new IllegalStateException("fig_client_sub_next timeout");
        }
        if (rc == 2) {
            return null;
        }
        if (rc != 0) {
            throw new IllegalStateException("fig_client_sub_next failed");
        }
        return out[0];
    }

    public static void subClose(long sub) {
        figClientSubClose(sub);
    }
}
