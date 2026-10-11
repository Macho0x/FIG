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

    public static native int figCborEncodeMassCancelRequest(
            String clOrdId, String account, String symbol, byte side, byte[][] out);

    public static native int figCborDecodeMassCancelRequest(
            byte[] data, String[] outClOrdId, String[] outAccount, String[] outSymbol, int[] outSide);

    public static native int figCborDecodeTradingSessionStatus(
            byte[] data, int[] outStatus, String[] outSessionId);

    public static native int figCborDecodeSecurityStatus(byte[] data, String[] outSymbol, int[] outStatus);

    public static native int figCborDecodeMassCancelReport(
            byte[] data, String[] outClOrdId, int[] outTotalAffected, int[] outStatus);

    public static native int figClientRequestSessionStatus(
            long handle, short channelId, int[] outStatus, String[] outSessionId);

    public static native int figClientRequestSecurityStatus(
            long handle, String symbol, short channelId, String[] outSymbol, int[] outStatus);

    public static native int figClientMassCancel(
            long handle,
            String clOrdId,
            String account,
            String symbol,
            byte side,
            short channelId,
            String[] outClOrdId,
            int[] outTotalAffected,
            int[] outStatus);

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

    /** Decoded mass-cancel request. side 0 means the side filter was absent. symbol may be null. */
    public static final class MassCancelRequest {
        public final String clOrdId;
        public final String account;
        public final String symbol;
        public final int side;

        public MassCancelRequest(String clOrdId, String account, String symbol, int side) {
            this.clOrdId = clOrdId;
            this.account = account;
            this.symbol = symbol;
            this.side = side;
        }
    }

    public static final class TradingSessionStatus {
        public final int status;
        public final String sessionId;

        public TradingSessionStatus(int status, String sessionId) {
            this.status = status;
            this.sessionId = sessionId;
        }
    }

    public static final class SecurityStatus {
        public final String symbol;
        public final int status;

        public SecurityStatus(String symbol, int status) {
            this.symbol = symbol;
            this.status = status;
        }
    }

    public static final class MassCancelReport {
        public final String clOrdId;
        public final int totalAffected;
        public final int status;

        public MassCancelReport(String clOrdId, int totalAffected, int status) {
            this.clOrdId = clOrdId;
            this.totalAffected = totalAffected;
            this.status = status;
        }
    }

    /** side 0 omits the filter. symbol null omits the symbol filter. */
    public static byte[] cborEncodeMassCancelRequest(String clOrdId, String account, String symbol, byte side) {
        byte[][] out = new byte[1][];
        if (figCborEncodeMassCancelRequest(clOrdId, account, symbol, side, out) != 0) {
            throw new IllegalStateException("fig_cbor_encode_mass_cancel_request failed");
        }
        return out[0];
    }

    public static MassCancelRequest cborDecodeMassCancelRequest(byte[] data) {
        String[] cl = new String[1];
        String[] account = new String[1];
        String[] symbol = new String[1];
        int[] side = new int[1];
        if (figCborDecodeMassCancelRequest(data, cl, account, symbol, side) != 0) {
            throw new IllegalStateException("fig_cbor_decode_mass_cancel_request failed");
        }
        return new MassCancelRequest(cl[0], account[0], symbol[0], side[0]);
    }

    public static TradingSessionStatus cborDecodeTradingSessionStatus(byte[] data) {
        int[] status = new int[1];
        String[] session = new String[1];
        if (figCborDecodeTradingSessionStatus(data, status, session) != 0) {
            throw new IllegalStateException("fig_cbor_decode_trading_session_status failed");
        }
        return new TradingSessionStatus(status[0], session[0]);
    }

    public static SecurityStatus cborDecodeSecurityStatus(byte[] data) {
        String[] symbol = new String[1];
        int[] status = new int[1];
        if (figCborDecodeSecurityStatus(data, symbol, status) != 0) {
            throw new IllegalStateException("fig_cbor_decode_security_status failed");
        }
        return new SecurityStatus(symbol[0], status[0]);
    }

    public static MassCancelReport cborDecodeMassCancelReport(byte[] data) {
        String[] cl = new String[1];
        int[] total = new int[1];
        int[] status = new int[1];
        if (figCborDecodeMassCancelReport(data, cl, total, status) != 0) {
            throw new IllegalStateException("fig_cbor_decode_mass_cancel_report failed");
        }
        return new MassCancelReport(cl[0], total[0], status[0]);
    }

    public static TradingSessionStatus requestSessionStatus(long handle, short channelId) {
        int[] status = new int[1];
        String[] session = new String[1];
        if (figClientRequestSessionStatus(handle, channelId, status, session) != 0) {
            throw new IllegalStateException("fig_client_request_session_status failed");
        }
        return new TradingSessionStatus(status[0], session[0]);
    }

    public static SecurityStatus requestSecurityStatus(long handle, String symbol, short channelId) {
        String[] outSymbol = new String[1];
        int[] status = new int[1];
        if (figClientRequestSecurityStatus(handle, symbol, channelId, outSymbol, status) != 0) {
            throw new IllegalStateException("fig_client_request_security_status failed");
        }
        return new SecurityStatus(outSymbol[0], status[0]);
    }

    public static MassCancelReport massCancel(
            long handle, String clOrdId, String account, String symbol, byte side, short channelId) {
        String[] cl = new String[1];
        int[] total = new int[1];
        int[] status = new int[1];
        if (figClientMassCancel(handle, clOrdId, account, symbol, side, channelId, cl, total, status) != 0) {
            throw new IllegalStateException("fig_client_mass_cancel failed");
        }
        return new MassCancelReport(cl[0], total[0], status[0]);
    }
}
