package fig;

/** JWT encode/verify round-trip — no live FIG server. */
public final class JwtSmoke {
    public static void main(String[] args) {
        String token = FigNative.jwtEncode("alice", 4_000_000_000L, "fig-smoke-secret");
        if (token == null || token.isEmpty()) {
            throw new IllegalStateException("empty JWT");
        }
        FigNative.jwtVerifyBearer(token, "fig-smoke-secret");
        System.out.println("fig-java smoke OK");
    }
}
