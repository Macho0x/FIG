# Java bindings

Native FIG client via JNI over [`fig.h`](../../crates/fig-ffi/include/fig.h).

```bash
cargo build -p fig-ffi
export LD_LIBRARY_PATH=$PWD/target/debug
javac -h . bindings/java/FigNative.java
# Implement generated fig_FigNative.h stubs calling fig.h, then:
java -Djava.library.path=target/debug fig.FigNative
```

[`FigNative.java`](FigNative.java) mirrors the C# P/Invoke surface. FSL types: `ftlc --lang java`.
