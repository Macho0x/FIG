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

    public static native int figJwtDecodeSub(String token, String secret, String[] outSub);

    public static native String figVersion();

    public static native int figClientRequestAndRecv(long handle, byte[] frame, Object[] outFrames);

    public static native int figFrameEncodeRequestEx(
            short channelId,
            int streamSeq,
            byte schemaId,
            String channelPath,
            String method,
            byte[] payload,
            byte[][] out);

    public static native int figFrameEncodeRequestAuth(
            short channelId,
            int streamSeq,
            byte schemaId,
            String channelPath,
            String method,
            String authToken,
            byte[] payload,
            byte[][] out);

    public static native int figClientPostBatch(
            long handle, String account, byte[] payload, short channelId, byte[][] out);

    public static native long figOrderBookNew();
    public static native void figOrderBookFree(long handle);
    public static native int figOrderBookApplySnapshot(long handle, byte[] payload);
    public static native int figOrderBookApplyDelta(long handle, byte[] payload);
    public static native double figOrderBookBestBid(long handle);

    public static native long figMidsNew();
    public static native void figMidsFree(long handle);
    public static native int figMidsApplyTicker(long handle, byte[] payload);
    public static native int figMidsApplyBatch(long handle, byte[] payload);
    public static native double figMidsMid(long handle, String symbol);

    public static native long figBboNew();
    public static native void figBboFree(long handle);
    public static native int figBboApply(long handle, byte[] payload);
    public static native double figBboImpliedMid(long handle);
    public static native double figBboBestBid(long handle);
    public static native double figBboBestAsk(long handle);

    public static native long figTradeTapeNew(long capacity);
    public static native void figTradeTapeFree(long handle);
    public static native int figTradeTapePush(long handle, byte[] payload);
    public static native long figTradeTapeLen(long handle);
    public static native double figTradeTapeLatestPrice(long handle);

    public static native long figMarkPriceNew();
    public static native void figMarkPriceFree(long handle);
    public static native int figMarkPriceApply(long handle, byte[] payload);
    public static native double figMarkPriceGet(long handle, String symbol);

    public static native long figOrdersNew(long execCapacity);
    public static native void figOrdersFree(long handle);
    public static native int figOrdersApplySnapshot(long handle, byte[] payload);
    public static native int figOrdersApplyExecution(long handle, byte[] payload);
    public static native long figOrdersOpenCount(long handle);
    public static native long figOrdersExecutionCount(long handle);

    public static native long figFundingNew(long capacity);
    public static native void figFundingFree(long handle);
    public static native int figFundingApply(long handle, byte[] payload);
    public static native long figFundingLen(long handle);
    public static native double figFundingLatestAmount(long handle);

    public static native long figAggTradesNew(long capacity);
    public static native void figAggTradesFree(long handle);
    public static native int figAggTradesApply(long handle, byte[] payload);
    public static native long figAggTradesLen(long handle);
    public static native double figAggTradesLatestPrice(long handle);

    public static native long figLedgerNew(long capacity);
    public static native void figLedgerFree(long handle);
    public static native int figLedgerApply(long handle, byte[] payload);
    public static native long figLedgerLen(long handle);

    public static native long figLiquidationNew(long capacity);
    public static native void figLiquidationFree(long handle);
    public static native int figLiquidationApplyUser(long handle, byte[] payload);
    public static native int figLiquidationApplyPublic(long handle, byte[] payload);
    public static native long figLiquidationUserCount(long handle);

    public static native long figCandleNew();
    public static native void figCandleFree(long handle);
    public static native int figCandleApply(long handle, byte[] payload);
    public static native double figCandleClose(long handle);
    public static native int figCandleIsFinal(long handle);
    public static native int figCandleSymbol(long handle, String[] out);
    public static native int figCandleInterval(long handle, String[] out);

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

    public static String version() {
        return figVersion();
    }

    public static String jwtDecodeSub(String token, String secret) {
        String[] out = new String[1];
        if (figJwtDecodeSub(token, secret, out) != 0) {
            throw new IllegalStateException("fig_jwt_decode_sub failed");
        }
        return out[0];
    }

    public static byte[][] requestAndRecv(long handle, byte[] frame) {
        Object[] out = new Object[1];
        if (figClientRequestAndRecv(handle, frame, out) != 0) {
            throw new IllegalStateException("fig_client_request_and_recv failed");
        }
        if (out[0] instanceof byte[][]) {
            return (byte[][]) out[0];
        }
        return new byte[0][];
    }

    public static byte[] encodeRequest(
            short channelId, int streamSeq, byte schemaId, String channelPath, String method, byte[] payload) {
        byte[][] out = new byte[1][];
        if (figFrameEncodeRequestEx(channelId, streamSeq, schemaId, channelPath, method, payload, out) != 0) {
            throw new IllegalStateException("fig_frame_encode_request_ex failed");
        }
        return out[0];
    }

    public static byte[] encodeRequestAuth(
            short channelId,
            int streamSeq,
            byte schemaId,
            String channelPath,
            String method,
            String authToken,
            byte[] payload) {
        byte[][] out = new byte[1][];
        if (figFrameEncodeRequestAuth(
                channelId, streamSeq, schemaId, channelPath, method, authToken, payload, out) != 0) {
            throw new IllegalStateException("fig_frame_encode_request_auth failed");
        }
        return out[0];
    }

    public static byte[] postBatch(long handle, String account, byte[] payload, short channelId) {
        byte[][] out = new byte[1][];
        if (figClientPostBatch(handle, account, payload, channelId, out) != 0) {
            throw new IllegalStateException("fig_client_post_batch failed");
        }
        return out[0];
    }

    public static long orderBookNew() {
        long handle = figOrderBookNew();
        if (handle == 0) {
            throw new IllegalStateException("fig_order_book_new failed");
        }
        return handle;
    }

    public static void orderBookFree(long handle) {
        figOrderBookFree(handle);
    }

    public static String candleSymbol(long handle) {
        String[] out = new String[1];
        if (figCandleSymbol(handle, out) != 0) {
            return "";
        }
        return out[0] == null ? "" : out[0];
    }

    public static String candleInterval(long handle) {
        String[] out = new String[1];
        if (figCandleInterval(handle, out) != 0) {
            return "";
        }
        return out[0] == null ? "" : out[0];
    }
}
