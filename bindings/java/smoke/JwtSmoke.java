package fig;

/** JWT encode/verify round-trip — no live FIG server. */
public final class JwtSmoke {
    public static void main(String[] args) {
        String token = FigNative.jwtEncode("alice", 4_000_000_000L, "fig-smoke-secret");
        if (token == null || token.isEmpty()) {
            throw new IllegalStateException("empty JWT");
        }
        FigNative.jwtVerifyBearer(token, "fig-smoke-secret");
        byte[] mass = FigNative.cborEncodeMassCancelRequest("MC-001", "DEMO", "AAPL", (byte) 1);
        FigNative.MassCancelRequest decoded = FigNative.cborDecodeMassCancelRequest(mass);
        if (!"MC-001".equals(decoded.clOrdId)
                || !"DEMO".equals(decoded.account)
                || !"AAPL".equals(decoded.symbol)
                || decoded.side != 1) {
            throw new IllegalStateException("mass-cancel CBOR round trip failed");
        }
        System.out.println("fig-java smoke OK");
    }
}
