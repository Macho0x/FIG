# Java bindings

JNI over [`fig.h`](../../crates/fig-ffi/include/fig.h). Stubs live in
[`native/fig_jni.c`](native/fig_jni.c); [`FigNative.java`](FigNative.java) is the
Java surface.

## Quick check (same as CI)

```bash
export JAVA_HOME    # required
bash bindings/java/smoke/run.sh
# prints: fig-java smoke OK
```

The smoke builds a shared JNI library from `fig_jni.c` + `libfig_ffi.a` and runs
[`smoke/JwtSmoke.java`](smoke/JwtSmoke.java) (JWT encode/verify, no live server).

Live subscribe: `FigNative.subscribe` / `subNext` / `subClose` wrap
`fig_client_subscribe` / `fig_client_sub_next`. The live e2e lives in
`fig-ffi` `client_integration`.

FSL types: `ftlc --lang java`.
