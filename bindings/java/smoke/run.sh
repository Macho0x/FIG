#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$root"
cargo build -p fig-ffi --release

: "${JAVA_HOME:?JAVA_HOME is required for Java JNI smoke}"
inc=(-I"$JAVA_HOME/include")
if [[ -d "$JAVA_HOME/include/linux" ]]; then
  inc+=(-I"$JAVA_HOME/include/linux")
elif [[ -d "$JAVA_HOME/include/darwin" ]]; then
  inc+=(-I"$JAVA_HOME/include/darwin")
fi

out_dir="$(mktemp -d)"
trap 'rm -rf "$out_dir"' EXIT

gcc -shared -fPIC "${inc[@]}" -I crates/fig-ffi/include \
  bindings/java/native/fig_jni.c \
  target/release/libfig_ffi.a \
  -lpthread -ldl -lm \
  -o "$out_dir/libfig_ffi.so"

javac -d "$out_dir" bindings/java/FigNative.java bindings/java/smoke/JwtSmoke.java
java -Djava.library.path="$out_dir" -cp "$out_dir" fig.JwtSmoke
