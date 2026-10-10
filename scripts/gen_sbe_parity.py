#!/usr/bin/env python3
"""Generate per-language SBE parity drivers covering every catalog message.

Reads:
  - crates/fig-conformance/src/lib.rs          (sample_* builders)
  - bindings/go/fig/sbe/sbe_generated.go       (encoder signatures, structs, enums)
  - tests/conformance/vectors/v1.json          (pinned hex + vector ids)

Writes:
  - bindings/go/fig/sbe/parity_test.go
  - bindings/cpp/smoke/sbe_parity.cpp
  - bindings/csharp/smoke/Program.cs           (keeps the existing FFI hex check)
  - bindings/typescript/smoke/sbe_parity.ts    (invoked from run.sh)
  - bindings/zig/sbe_parity.zig
"""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GO = (ROOT / "bindings/go/fig/sbe/sbe_generated.go").read_text()
LIB = (ROOT / "crates/fig-conformance/src/lib.rs").read_text()
VEC = json.loads((ROOT / "tests/conformance/vectors/v1.json").read_text())


# ── Go type universe ──────────────────────────────────────────────────────────

def parse_go_enums(src: str):
    enums = {}
    for m in re.finditer(
        r"type (\w+) uint8\n\nconst \(\n(.*?)\n\)", src, re.S
    ):
        name, body = m.group(1), m.group(2)
        variants = []
        for line in body.splitlines():
            mm = re.match(rf"\t{name}(\w+)\s+{name} = (\d+)", line)
            if mm:
                variants.append((mm.group(1), int(mm.group(2))))
        enums[name] = variants
    return enums


def parse_go_structs(src: str):
    structs = {}
    for m in re.finditer(r"type (\w+) struct \{\n(.*?)\n\}", src, re.S):
        name, body = m.group(1), m.group(2)
        fields = []
        for line in body.splitlines():
            mm = re.match(r"\s*(\w+)\s+(.+)$", line)
            if not mm:
                continue
            fields.append((mm.group(1), mm.group(2).strip()))
        structs[name] = fields
    return structs


def parse_go_encoders(src: str):
    encoders = {}
    for m in re.finditer(
        r"func \((\w+)Encoder\) Encode\(([^)]*)\) \(\[\]byte, error\)", src
    ):
        name, params = m.group(1), m.group(2).strip()
        if name.endswith("Body"):
            continue
        plist = []
        if params:
            for p in params.split(","):
                p = p.strip()
                n, t = p.split(" ", 1)
                plist.append((n, t.strip()))
        encoders[name] = plist
    return encoders


ENUMS = parse_go_enums(GO)
STRUCTS = parse_go_structs(GO)
ENCODERS = parse_go_encoders(GO)

VECTOR_BY_TYPE = {}
for v in VEC["vectors"]:
    if v["category"] == "sbe":
        VECTOR_BY_TYPE[v["message_type"]] = v

# CandleBar is an FSL `type`, not a `message`. The historical vector
# `sbe.candle_bar.snapshot` is produced by wrapping a CandleBar in
# CandleBarEventEncoder (see fig_core::sbe::encode_candle_bar).
ALIASES = {
    "CandleBar": ("CandleBarEvent", "Bar", "CandleBar"),
}
assert set(ENCODERS) | set(ALIASES) == set(VECTOR_BY_TYPE), (
    (set(ENCODERS) | set(ALIASES)) - set(VECTOR_BY_TYPE),
    set(VECTOR_BY_TYPE) - (set(ENCODERS) | set(ALIASES)),
)


# ── sample parser ─────────────────────────────────────────────────────────────

class Tok:
    def __init__(self, kind, value, pos):
        self.kind, self.value, self.pos = kind, value, pos

    def __repr__(self):
        return f"{self.kind}:{self.value!r}"


def tokenize(s: str):
    s = re.sub(r"//[^\n]*", "", s)
    s = s.replace(".to_string()", "")
    s = s.replace("fig_core::messages::", "")
    i, n, out = 0, len(s), []
    while i < n:
        c = s[i]
        if c.isspace():
            i += 1
            continue
        if c in "{},()[]:+!":
            out.append(Tok(c, c, i))
            i += 1
            continue
        if c == '"':
            j = i + 1
            while s[j] != '"':
                j += 1
            out.append(Tok("str", s[i + 1 : j], i))
            i = j + 1
            continue
        if c.isdigit() or (c == "." and i + 1 < n and s[i + 1].isdigit()):
            j = i
            while j < n and (s[j].isalnum() or s[j] in "._"):
                j += 1
            out.append(Tok("num", s[i:j], i))
            i = j
            continue
        if c.isalpha() or c == "_":
            j = i
            while j < n and (s[j].isalnum() or s[j] == "_"):
                j += 1
            ident = s[i:j]
            # skip `::Ident` path tail — keep the last component
            out.append(Tok("id", ident, i))
            i = j
            continue
        raise SystemExit(f"tokenize: unexpected {c!r} at {i} in {s[max(0,i-20):i+20]!r}")
    return out


class Parser:
    def __init__(self, toks):
        self.toks, self.i = toks, 0

    def peek(self):
        return self.toks[self.i] if self.i < len(self.toks) else None

    def take(self, kind=None):
        t = self.peek()
        if t is None:
            raise SystemExit("unexpected EOF")
        if kind and t.kind != kind:
            raise SystemExit(f"expected {kind} got {t}")
        self.i += 1
        return t

    def accept(self, kind, value=None):
        t = self.peek()
        if t and t.kind == kind and (value is None or t.value == value):
            self.i += 1
            return t
        return None

    def parse_expr(self):
        t = self.peek()
        if t is None:
            raise SystemExit("empty expr")
        if t.kind == "str":
            self.take()
            return ("str", t.value)
        if t.kind == "num":
            self.take()
            left = self._num(t.value)
            if self.accept("+"):
                right = self.parse_expr()
                assert right[0] == "int"
                return ("int", left[1] + right[1] if left[0] == "int" else left)
            return left
        if t.kind == "id":
            ident = self.take().value
            if ident in ("true", "false"):
                return ("bool", ident == "true")
            if ident == "None":
                return ("none",)
            if ident == "Some":
                self.take("(")
                inner = self.parse_expr()
                self.take(")")
                return ("some", inner)
            if ident in ("Price", "Quantity"):
                self.take("(")
                inner = self.parse_expr()
                self.take(")")
                return inner
            if ident == "vec":
                self.take("!")
                self.take("[")
                items = []
                if not self.accept("]"):
                    items.append(self.parse_expr())
                    while self.accept(","):
                        if self.peek() and self.peek().kind == "]":
                            break
                        items.append(self.parse_expr())
                    self.take("]")
                return ("list", items)
            if ident == "sample":
                # shouldn't happen
                pass
            # sample_foo()
            if ident.startswith("sample_") and self.peek() and self.peek().kind == "(":
                self.take("(")
                self.take(")")
                return ("sample", ident[len("sample_") :])
            # Type { ... } or Type::Variant or Type::from_value(n)
            if self.accept("{"):
                fields = []
                while not self.accept("}"):
                    if self.accept(","):
                        continue
                    key = self.take("id").value
                    self.take(":")
                    fields.append((key, self.parse_expr()))
                    self.accept(",")
                return ("struct", ident, fields)
            if self.peek() and self.peek().kind == "id" and self.peek().value == "from_value":
                # already consumed Type? no — `Side` then we need `::` which we stripped
                # after stripping fig_core::messages::, we still have Side::from_value
                pass
            # Ident::from_value / Ident::Variant — `::` was kept as two tokens? we drop `:`
            # tokenize treats `:` as token, so Side::from_value is id, :, :, id
            if self.accept(":") and self.accept(":"):
                method = self.take("id").value
                if method == "from_value":
                    self.take("(")
                    n = self.parse_expr()
                    self.take(")")
                    # .expect("variant")
                    if self.peek() and self.peek().kind == "id" and self.peek().value == "expect":
                        # `.` isn't a token — "expect" after `)` ?
                        pass
                    return ("enum_value", ident, n[1] if n[0] == "int" else n)
                return ("enum_name", ident, method)
            return ("ident", ident)
        raise SystemExit(f"parse: unexpected {t}")

    def _num(self, raw: str):
        raw = raw.replace("_", "")
        # 4u32 / 1700...i64
        m = re.match(r"^([0-9]+)(?:u32|u64|i64|usize)?$", raw)
        if m:
            return ("int", int(m.group(1)))
        if re.match(r"^[0-9]*\.[0-9]+$", raw):
            return ("float", float(raw))
        raise SystemExit(f"bad number {raw}")


def extract_fn_body(src: str, start: int) -> str:
    """`start` points at the `{` that opens the function. Return the inside."""
    assert src[start] == "{"
    depth, i = 0, start
    while i < len(src):
        c = src[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return src[start + 1 : i]
        elif c == '"':
            i += 1
            while src[i] != '"':
                i += 1
        i += 1
    raise SystemExit("unclosed function")


def parse_samples(src: str):
    samples = {}
    for m in re.finditer(r"pub fn sample_(\w+)\(\) -> (\w+) \{", src):
        snake, typ = m.group(1), m.group(2)
        body = extract_fn_body(src, m.end() - 1)
        body = re.sub(r"^\s*use .*?;\s*$", "", body, flags=re.M)
        body = re.sub(r'\.expect\("[^"]*"\)', "", body)
        toks = tokenize(body)
        p = Parser(toks)
        expr = p.parse_expr()
        if p.peek() is not None:
            raise SystemExit(f"trailing tokens in sample_{snake}: {p.peek()}")
        samples[typ] = expr
        samples[snake] = expr
    return samples


SAMPLES = parse_samples(LIB)

# NewOrderSingle has no sample_* — pin the original vector's payload.
SAMPLES["NewOrderSingle"] = (
    "struct",
    "NewOrderSingle",
    [
        ("cl_ord_id", ("str", "CONF-001")),
        ("side", ("enum_name", "Side", "Buy")),
        ("order_qty", ("float", 100.0)),
        ("price", ("some", ("float", 50.25))),
        ("stop_price", ("none",)),
        ("symbol", ("str", "AAPL")),
        ("order_type", ("enum_name", "OrderType", "Limit")),
        ("time_in_force", ("enum_name", "TimeInForce", "Day")),
        ("expire_time", ("none",)),
        ("account", ("none",)),
        ("strategy_id", ("none",)),
        ("security_id", ("none",)),
        ("id_source", ("none",)),
        ("security_exchange", ("none",)),
        ("post_only", ("none",)),
        ("reduce_only", ("none",)),
    ],
)


def resolve(expr):
    if expr[0] == "sample":
        # snake -> Type via SAMPLES[typ]
        snake = expr[1]
        for k, v in SAMPLES.items():
            if k.lower() == snake.replace("_", "").lower() or k == snake:
                return resolve(v)
        # sample_candle_bar -> CandleBar
        pascal = "".join(x.title() for x in snake.split("_"))
        return resolve(SAMPLES[pascal])
    if expr[0] == "struct":
        return ("struct", expr[1], [(n, resolve(v)) for n, v in expr[2]])
    if expr[0] == "list":
        return ("list", [resolve(x) for x in expr[1]])
    if expr[0] == "some":
        return ("some", resolve(expr[1]))
    return expr


for k in list(SAMPLES):
    SAMPLES[k] = resolve(SAMPLES[k])


def snake_to_pascal(s: str) -> str:
    return "".join(x[:1].upper() + x[1:] for x in s.split("_"))


def pascal_to_snake(s: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", s).lower()


def enum_variant(enum_ty: str, expr) -> str:
    """Return the variant *suffix* (Buy, New, Executing, ...)."""
    variants = ENUMS[enum_ty]
    by_name = {n: v for n, v in variants}
    by_val = {v: n for n, v in variants}
    if expr[0] == "enum_name":
        name = expr[2]
        if name in by_name:
            return name
        # try matching ignoring case
        for n in by_name:
            if n.lower() == name.lower():
                return n
        raise SystemExit(f"enum {enum_ty}: no variant {name} in {list(by_name)}")
    if expr[0] == "enum_value":
        return by_val[int(expr[2])]
    raise SystemExit(f"enum {enum_ty}: cannot map {expr}")


def is_enum(ty: str) -> bool:
    return ty in ENUMS


def is_struct(ty: str) -> bool:
    return ty in STRUCTS


def unwrap_ptr(ty: str):
    return ty[1:] if ty.startswith("*") else None


def unwrap_slice(ty: str):
    return ty[2:] if ty.startswith("[]") else None


def is_boolish(ty: str) -> bool:
    t = unwrap_ptr(ty) or ty
    return t in ("uint8", "byte", "u8")  # callers decide based on sample being bool


# ── emitters ──────────────────────────────────────────────────────────────────

def emit_go(ty: str, expr) -> str:
    if expr[0] == "none":
        return "nil"
    inner = unwrap_ptr(ty)
    if inner is not None:
        if expr[0] == "some":
            return f"ptr({emit_go(inner, expr[1])})"
        return f"ptr({emit_go(inner, expr)})"
    sl = unwrap_slice(ty)
    if sl is not None:
        assert expr[0] == "list", expr
        if not expr[1]:
            return "nil"
        items = ", ".join(emit_go(sl, x) for x in expr[1])
        return f"[]{sl}{{{items}}}"
    if is_enum(ty):
        return f"{ty}{enum_variant(ty, expr)}"
    if is_struct(ty):
        assert expr[0] == "struct", expr
        fmap = {n: v for n, v in expr[2]}
        parts = []
        for fname, fty in STRUCTS[ty]:
            key = pascal_to_snake(fname)
            if key not in fmap and fname.lower() not in {k.lower() for k in fmap}:
                # try original snake from sample
                raise SystemExit(f"{ty}.{fname}: missing in sample {list(fmap)}")
            val = fmap.get(key) or next(v for k, v in fmap.items() if k.replace("_", "") == fname.lower())
            parts.append(f"{fname}: {emit_go(fty, val)}")
        return f"{ty}{{{', '.join(parts)}}}"
    # scalars
    if expr[0] == "some":
        return emit_go(ty, expr[1])
    if expr[0] == "bool":
        bit = "1" if expr[1] else "0"
        if ty in ("uint8", "byte"):
            return f"uint8({bit})"
        return bit
    if expr[0] == "str":
        return json.dumps(expr[1])
    if expr[0] == "int":
        if ty in ("float64",):
            return f"{expr[1]}.0"
        if ty in ("int64",):
            return f"int64({expr[1]})"
        if ty in ("uint64",):
            return f"uint64({expr[1]})"
        if ty in ("uint32",):
            return f"uint32({expr[1]})"
        if ty in ("uint8", "byte"):
            return f"uint8({expr[1]})"
        return str(expr[1])
    if expr[0] == "float":
        v = expr[1]
        s = repr(v) if v != int(v) else f"{v}.0"
        # keep 0.0001 etc
        if "e" in str(v) or "E" in str(v):
            s = str(v)
        return str(v) if "." in str(v) else f"{v}.0"
    raise SystemExit(f"go: cannot emit {ty} from {expr}")


def emit_cpp(ty: str, expr) -> str:
    # ty is the Go type; map to C++
    if expr[0] == "none":
        return "std::nullopt"
    inner = unwrap_ptr(ty)
    if inner is not None:
        val = expr[1] if expr[0] == "some" else expr
        return emit_cpp(inner, val)
    sl = unwrap_slice(ty)
    if sl is not None:
        assert expr[0] == "list"
        cpp_sl = cpp_type(sl)
        if not expr[1]:
            return f"std::vector<{cpp_sl}>{{}}"
        items = ", ".join(emit_cpp(sl, x) for x in expr[1])
        return f"std::vector<{cpp_sl}>{{{items}}}"
    if is_enum(ty):
        return f"fig::sbe::{ty}::{enum_variant(ty, expr)}"
    if is_struct(ty):
        assert expr[0] == "struct"
        fmap = {n: v for n, v in expr[2]}
        parts = []
        for fname, fty in STRUCTS[ty]:
            key = pascal_to_snake(fname)
            val = fmap[key]
            parts.append(f".{key} = {emit_cpp(fty, val)}")
        return f"fig::sbe::{ty}{{{', '.join(parts)}}}"
    if expr[0] == "some":
        return emit_cpp(ty, expr[1])
    if expr[0] == "bool":
        return "uint8_t{1}" if expr[1] else "uint8_t{0}"
    if expr[0] == "str":
        return json.dumps(expr[1])
    if expr[0] == "int":
        if ty in ("int64",):
            return f"{expr[1]}LL"
        if ty in ("uint64",):
            return f"uint64_t{{{expr[1]}ULL}}"
        if ty in ("uint32",):
            return f"uint32_t{{{expr[1]}}}"
        if ty in ("uint8",):
            return f"uint8_t{{{expr[1]}}}"
        if ty == "float64":
            return f"{expr[1]}.0"
        return str(expr[1])
    if expr[0] == "float":
        return str(expr[1])
    raise SystemExit(f"cpp: cannot emit {ty} from {expr}")


def cpp_type(go_ty: str) -> str:
    inner = unwrap_ptr(go_ty)
    if inner:
        return f"std::optional<{cpp_type(inner)}>"
    sl = unwrap_slice(go_ty)
    if sl:
        return f"std::vector<{cpp_type(sl)}>"
    return {
        "string": "std::string",
        "float64": "double",
        "int64": "int64_t",
        "uint64": "uint64_t",
        "uint32": "uint32_t",
        "uint8": "uint8_t",
        "byte": "uint8_t",
    }.get(go_ty, f"fig::sbe::{go_ty}")


def emit_cs(ty: str, expr) -> str:
    if expr[0] == "none":
        return "null"
    inner = unwrap_ptr(ty)
    if inner is not None:
        val = expr[1] if expr[0] == "some" else expr
        return emit_cs(inner, val)
    sl = unwrap_slice(ty)
    if sl is not None:
        assert expr[0] == "list"
        cs_sl = cs_type(sl)
        if not expr[1]:
            return f"new List<{cs_sl}>()"
        items = ", ".join(emit_cs(sl, x) for x in expr[1])
        return f"new List<{cs_sl}> {{ {items} }}"
    if is_enum(ty):
        return f"Fig.Sbe.{ty}.{enum_variant(ty, expr)}"
    if is_struct(ty):
        assert expr[0] == "struct"
        fmap = {n: v for n, v in expr[2]}
        parts = []
        for fname, fty in STRUCTS[ty]:
            key = pascal_to_snake(fname)
            val = fmap[key]
            parts.append(f"{fname} = {emit_cs(fty, val)}")
        return f"new Fig.Sbe.{ty} {{ {', '.join(parts)} }}"
    if expr[0] == "some":
        return emit_cs(ty, expr[1])
    if expr[0] == "bool":
        return "(byte)1" if expr[1] else "(byte)0"
    if expr[0] == "str":
        return json.dumps(expr[1])
    if expr[0] == "int":
        if ty in ("int64",):
            return f"{expr[1]}L"
        if ty in ("uint64",):
            return f"{expr[1]}UL"
        if ty in ("uint32",):
            return f"{expr[1]}u"
        if ty == "float64":
            return f"{expr[1]}.0"
        return str(expr[1])
    if expr[0] == "float":
        return str(expr[1])
    raise SystemExit(f"cs: cannot emit {ty} from {expr}")


def cs_type(go_ty: str) -> str:
    inner = unwrap_ptr(go_ty)
    if inner:
        return cs_type(inner) + "?"
    sl = unwrap_slice(go_ty)
    if sl:
        return f"List<{cs_type(sl)}>"
    return {
        "string": "string",
        "float64": "double",
        "int64": "long",
        "uint64": "ulong",
        "uint32": "uint",
        "uint8": "byte",
        "byte": "byte",
    }.get(go_ty, f"Fig.Sbe.{go_ty}")


def emit_ts(ty: str, expr) -> str:
    if expr[0] == "none":
        return "null"
    inner = unwrap_ptr(ty)
    if inner is not None:
        val = expr[1] if expr[0] == "some" else expr
        return emit_ts(inner, val)
    sl = unwrap_slice(ty)
    if sl is not None:
        assert expr[0] == "list"
        if not expr[1]:
            return "[]"
        items = ", ".join(emit_ts(sl, x) for x in expr[1])
        return f"[{items}]"
    if is_enum(ty):
        return f"{ty}.{enum_variant(ty, expr)}"
    if is_struct(ty):
        assert expr[0] == "struct"
        fmap = {n: v for n, v in expr[2]}
        parts = []
        for fname, fty in STRUCTS[ty]:
            key = pascal_to_snake(fname)
            val = fmap[key]
            parts.append(f"{key}: {emit_ts(fty, val)}")
        return "{" + ", ".join(parts) + "}"
    if expr[0] == "some":
        return emit_ts(ty, expr[1])
    if expr[0] == "bool":
        return "1" if expr[1] else "0"
    if expr[0] == "str":
        return json.dumps(expr[1])
    if expr[0] == "int":
        if ty in ("int64", "uint64"):
            return f"{expr[1]}n"
        if ty == "float64":
            return f"{expr[1]}.0"
        return str(expr[1])
    if expr[0] == "float":
        return str(expr[1])
    raise SystemExit(f"ts: cannot emit {ty} from {expr}")


def emit_zig(ty: str, expr) -> str:
    if expr[0] == "none":
        return "null"
    inner = unwrap_ptr(ty)
    if inner is not None:
        val = expr[1] if expr[0] == "some" else expr
        return emit_zig(inner, val)
    sl = unwrap_slice(ty)
    if sl is not None:
        assert expr[0] == "list"
        if not expr[1]:
            return "&.{}"
        items = ", ".join(emit_zig(sl, x) for x in expr[1])
        return f"&.{{ {items} }}"
    if is_enum(ty):
        return f".{enum_variant(ty, expr)}"
    if is_struct(ty):
        assert expr[0] == "struct"
        fmap = {n: v for n, v in expr[2]}
        parts = []
        for fname, fty in STRUCTS[ty]:
            key = pascal_to_snake(fname)
            val = fmap[key]
            parts.append(f".{key} = {emit_zig(fty, val)}")
        return f"sbe.{ty}{{ {', '.join(parts)} }}"
    if expr[0] == "some":
        return emit_zig(ty, expr[1])
    if expr[0] == "bool":
        return "1" if expr[1] else "0"
    if expr[0] == "str":
        return json.dumps(expr[1])
    if expr[0] == "int":
        if ty == "float64":
            return f"{expr[1]}.0"
        return str(expr[1])
    if expr[0] == "float":
        return str(expr[1])
    raise SystemExit(f"zig: cannot emit {ty} from {expr}")


def struct_sample(msg: str):
    expr = SAMPLES[msg]
    assert expr[0] == "struct", (msg, expr[0])
    return {n: v for n, v in expr[2]}


def args_for(msg: str, emit):
    fmap = struct_sample(msg)
    params = ENCODERS[msg]
    if not params:
        return []
    out = []
    for pname, pty in params:
        key = pascal_to_snake(pname)
        if key not in fmap:
            raise SystemExit(f"{msg}.{pname} ({key}) missing in sample {list(fmap)}")
        out.append(emit(pty, fmap[key]))
    return out


def cases(emit):
    """Yield (vector_id, hex, encoder_name, arg_list) for every SBE vector."""
    out = []
    for msg in sorted(set(ENCODERS) | set(ALIASES)):
        v = VECTOR_BY_TYPE[msg]
        if msg in ALIASES:
            encoder, pname, sample_ty = ALIASES[msg]
            pty = dict(ENCODERS[encoder])[pname]
            args = [emit(pty, SAMPLES[sample_ty])]
            out.append((v["id"], v["expected_hex"], encoder, args))
        else:
            out.append((v["id"], v["expected_hex"], msg, args_for(msg, emit)))
    return out


def gen_go():
    lines = [
        "package figsbe",
        "",
        "// Code generated by scripts/gen_sbe_parity.py. DO NOT EDIT.",
        "//",
        "// Parity tests: the generated Go SBE code must produce exactly the bytes of",
        "// the shared conformance vectors (produced by the Rust/FFI encoder).",
        "",
        "import (",
        '\t"encoding/hex"',
        '\t"testing"',
        ")",
        "",
        "func ptr[T any](v T) *T { return &v }",
        "",
        "func TestAllSbeMessagesMatchConformanceVectors(t *testing.T) {",
        "\tcheck := func(t *testing.T, id, want string, got []byte, err error) {",
        "\t\tt.Helper()",
        "\t\tif err != nil {",
        '\t\t\tt.Fatalf("%s encode: %v", id, err)',
        "\t\t}",
        "\t\tif hex.EncodeToString(got) != want {",
        '\t\t\tt.Fatalf("%s hex mismatch\\n got %s\\nwant %s", id, hex.EncodeToString(got), want)',
        "\t\t}",
        "\t}",
        "",
    ]
    for vid, hexv, msg, args in cases(emit_go):
        call = ", ".join(args)
        lines.append(f'\tt.Run("{vid}", func(t *testing.T) {{')
        lines.append(f"\t\tgot, err := {msg}Encoder{{}}.Encode({call})")
        lines.append(f'\t\tcheck(t, "{vid}", "{hexv}", got, err)')
        lines.append("\t})")
    lines.append("}")
    lines.append("")
    # keep the existing round-trip coverage
    lines.append("""
func TestRoundTrips(t *testing.T) {
	price := 12.5
	raw, err := NewOrderSingleEncoder{}.Encode(
		"RT-001", NewOrderSingleSideBuy, 7, &price, nil, "MSFT",
		NewOrderSingleOrderTypeLimit, NewOrderSingleTimeInForceDay,
		nil, nil, nil, nil, nil, nil, nil, nil,
	)
	if err != nil {
		t.Fatalf("encode: %v", err)
	}
	got, consumed, err := NewOrderSingleDecoderDecode(raw)
	if err != nil {
		t.Fatalf("decode: %v", err)
	}
	if consumed != len(raw) {
		t.Fatalf("consumed %d of %d bytes", consumed, len(raw))
	}
	if got.ClOrdId != "RT-001" || got.Symbol != "MSFT" || got.OrderQty != 7 {
		t.Fatalf("round trip mismatch: %+v", got)
	}
	if got.Price == nil || *got.Price != price {
		t.Fatalf("price round trip mismatch: %v", got.Price)
	}
}
""")
    (ROOT / "bindings/go/fig/sbe/parity_test.go").write_text("\n".join(lines))
    print("wrote go parity_test.go", len(ENCODERS), "messages")


def gen_cpp():
    lines = [
        "// Code generated by scripts/gen_sbe_parity.py. DO NOT EDIT.",
        '// Per-language SBE parity: encode with the *generated* C++ code and compare',
        "// the bytes with the shared conformance vectors (produced by the Rust/FFI encoder).",
        '#include "fig/sbe_generated.hpp"',
        "",
        "#include <cstdio>",
        "#include <cstdint>",
        "#include <optional>",
        "#include <string>",
        "#include <vector>",
        "",
        "static std::string hex(const std::vector<uint8_t>& bytes) {",
        '    static const char* digits = "0123456789abcdef";',
        "    std::string out;",
        "    out.reserve(bytes.size() * 2);",
        "    for (uint8_t b : bytes) {",
        "        out.push_back(digits[b >> 4]);",
        "        out.push_back(digits[b & 0x0F]);",
        "    }",
        "    return out;",
        "}",
        "",
        "static int check(const char* label, const std::vector<uint8_t>& got, const std::string& want) {",
        "    if (hex(got) == want) {",
        '        std::printf("fig-cpp sbe parity %s OK\\n", label);',
        "        return 0;",
        "    }",
        '    std::printf("fig-cpp sbe parity %s FAILED\\n  got  %s\\n  want %s\\n", label,',
        "                hex(got).c_str(), want.c_str());",
        "    return 1;",
        "}",
        "",
        "int main() {",
        "    int bad = 0;",
        "",
    ]
    for vid, hexv, msg, args in cases(emit_cpp):
        call = ", ".join(args)
        lines.append("    bad += check(")
        lines.append(f'        "{vid}",')
        lines.append(f"        fig::sbe::{msg}Encoder::encode({call}),")
        lines.append(f'        "{hexv}");')
        lines.append("")
    lines += [
        "    // Round trip through the generated decoder",
        "    auto raw = fig::sbe::NewOrderSingleEncoder::encode(",
        '        "RT-001", fig::sbe::NewOrderSingleSide::Buy, 7.0, 12.5, std::nullopt, "MSFT",',
        "        fig::sbe::NewOrderSingleOrderType::Limit, fig::sbe::NewOrderSingleTimeInForce::Day,",
        "        std::nullopt, std::nullopt, std::nullopt, std::nullopt, std::nullopt, std::nullopt,",
        "        std::nullopt, std::nullopt);",
        "    auto value = fig::sbe::NewOrderSingleDecoder::decode(raw);",
        '    if (value.cl_ord_id != "RT-001" || value.symbol != "MSFT" ||',
        "        value.order_qty != 7.0 || !value.price || *value.price != 12.5) {",
        '        std::printf("fig-cpp sbe parity round_trip FAILED\\n");',
        "        bad += 1;",
        "    } else {",
        '        std::printf("fig-cpp sbe parity round_trip OK\\n");',
        "    }",
        "",
        "    return bad == 0 ? 0 : 1;",
        "}",
        "",
    ]
    (ROOT / "bindings/cpp/smoke/sbe_parity.cpp").write_text("\n".join(lines))
    print("wrote cpp sbe_parity.cpp")


def gen_cs():
    preamble = '''using System.Text.Json;
using Fig;

var bytes = FigNative.SbeEncodeNewOrderSingle("CONF-001", "AAPL", true, 100, 50.25);
if (bytes.Length == 0)
{
    throw new InvalidOperationException("empty SBE payload");
}

using var funding = new FundingState(4);

var root = Environment.GetEnvironmentVariable("FIG_REPO_ROOT")
    ?? Path.GetFullPath(Path.Combine(Directory.GetCurrentDirectory(), "..", "..", ".."));
var jsonPath = Path.Combine(root, "tests", "conformance", "vectors", "v1.json");
using var doc = JsonDocument.Parse(File.ReadAllText(jsonPath));
var got = Convert.ToHexString(bytes).ToLowerInvariant();

foreach (var v in doc.RootElement.GetProperty("vectors").EnumerateArray())
{
    if (v.GetProperty("category").GetString() != "sbe") continue;
    if (v.GetProperty("message_type").GetString() != "NewOrderSingle") continue;
    var expected = v.GetProperty("expected_hex").GetString()!;
    if (got != expected)
    {
        throw new InvalidOperationException($"SBE hex mismatch expected={expected} got={got}");
    }
}

Console.WriteLine($"fig-csharp smoke OK ({bytes.Length} bytes, SBE hex verified)");

// Code generated by scripts/gen_sbe_parity.py from here down. DO NOT EDIT.
static string Hex(byte[] b) => Convert.ToHexString(b).ToLowerInvariant();
static string Want(JsonDocument doc, string id) =>
    doc.RootElement.GetProperty("vectors").EnumerateArray()
        .First(v => v.GetProperty("id").GetString() == id)
        .GetProperty("expected_hex").GetString()!;
void Check(string id, byte[] encoded)
{
    if (Hex(encoded) != Want(doc, id))
        throw new InvalidOperationException($"generated C# {id} mismatch: {Hex(encoded)}");
}

'''
    chunks = [preamble]
    for vid, hexv, msg, args in cases(emit_cs):
        call = ", ".join(args)
        chunks.append(f'Check("{vid}", Fig.Sbe.{msg}Encoder.Encode({call}));\n')
    chunks.append('\nConsole.WriteLine("fig-csharp generated SBE parity OK");\n')
    (ROOT / "bindings/csharp/smoke/Program.cs").write_text("".join(chunks))
    print("wrote csharp Program.cs")


def gen_ts():
    # collect needed imports: encoder fns + enums referenced
    encoders = sorted({f"{m}EncoderEncode" for _, _, m, _ in cases(lambda *_: "x")})
    enums = sorted({ty for params in ENCODERS.values() for _, ty in params if ty in ENUMS})
    # also enums used inside structs — import all enums we emit
    used_enums = set(enums)
    # brute: every enum type
    used_enums |= set(ENUMS)
    imports = encoders + ["NewOrderSingleDecoderDecode"] + sorted(used_enums)
    lines = [
        "// Code generated by scripts/gen_sbe_parity.py. DO NOT EDIT.",
        'import { readFileSync } from "fs";',
        "import {",
        "  " + ",\n  ".join(imports) + ",",
        '} from "../sbe_generated.ts";',
        "",
        'const root = process.env.FIG_REPO_ROOT ?? process.cwd();',
        'const vectors = JSON.parse(readFileSync(root + "/tests/conformance/vectors/v1.json", "utf8")).vectors;',
        "const want = (id: string) => vectors.find((v: {id: string}) => v.id === id).expected_hex;",
        'const hex = (b: Uint8Array) => Buffer.from(b).toString("hex");',
        "",
    ]
    for vid, hexv, msg, args in cases(emit_ts):
        call = ", ".join(args)
        lines.append("{")
        lines.append(f"  const got = {msg}EncoderEncode({call});")
        lines.append(f'  if (hex(got) !== want("{vid}")) {{')
        lines.append(
            f'    throw new Error("{vid} parity failed\\n got " + hex(got) + "\\nwant " + want("{vid}"));'
        )
        lines.append("  }")
        lines.append("}")
    lines += [
        "",
        "const order = NewOrderSingleEncoderEncode(",
        '  "CONF-001", NewOrderSingleSide.Buy, 100, 50.25, null, "AAPL",',
        "  NewOrderSingleOrderType.Limit, NewOrderSingleTimeInForce.Day,",
        "  null, null, null, null, null, null, null, null,",
        ");",
        "const round = NewOrderSingleDecoderDecode(order);",
        'if (round.cl_ord_id !== "CONF-001" || round.symbol !== "AAPL" || round.order_qty !== 100 || round.price !== 50.25) {',
        '  throw new Error("round trip failed: " + JSON.stringify(round));',
        "}",
        'console.log("fig-ts sbe parity OK");',
        "",
    ]
    (ROOT / "bindings/typescript/smoke/sbe_parity.ts").write_text("\n".join(lines))
    print("wrote ts sbe_parity.ts")


def gen_zig():
    lines = [
        "// Code generated by scripts/gen_sbe_parity.py. DO NOT EDIT.",
        'const std = @import("std");',
        'const sbe = @import("sbe_generated.zig");',
        "",
        "fn hexAlloc(allocator: std.mem.Allocator, bytes: []const u8) ![]u8 {",
        '    const digits = "0123456789abcdef";',
        "    const out = try allocator.alloc(u8, bytes.len * 2);",
        "    for (bytes, 0..) |b, i| {",
        "        out[i * 2] = digits[b >> 4];",
        "        out[i * 2 + 1] = digits[b & 0x0f];",
        "    }",
        "    return out;",
        "}",
        "",
        "fn check(allocator: std.mem.Allocator, label: []const u8, got: []const u8, want: []const u8) !void {",
        "    const got_hex = try hexAlloc(allocator, got);",
        "    defer allocator.free(got_hex);",
        "    if (!std.mem.eql(u8, got_hex, want)) {",
        '        std.debug.print("zig sbe parity {s} FAILED\\n got  {s}\\n want {s}\\n", .{ label, got_hex, want });',
        "        return error.ParityMismatch;",
        "    }",
        "}",
        "",
        "pub fn main() !void {",
        "    const allocator = std.heap.page_allocator;",
        "",
    ]
    for vid, hexv, msg, args in cases(emit_zig):
        call = ", ".join(["allocator"] + args)
        lines.append("    {")
        lines.append(f"        const got = try sbe.{msg}Encoder.encode({call});")
        lines.append("        defer allocator.free(got);")
        lines.append(f'        try check(allocator, "{vid}", got, "{hexv}");')
        lines.append("    }")
    lines += [
        "",
        "    {",
        "        const raw = try sbe.NewOrderSingleEncoder.encode(",
        '            allocator, "RT-001", .Buy, 7.0, 12.5, null, "MSFT", .Limit, .Day,',
        "            null, null, null, null, null, null, null, null,",
        "        );",
        "        defer allocator.free(raw);",
        "        const decoded = try sbe.NewOrderSingleDecoder.decode(allocator, raw);",
        '        if (!std.mem.eql(u8, decoded.cl_ord_id, "RT-001") or',
        '            !std.mem.eql(u8, decoded.symbol, "MSFT") or',
        "            decoded.order_qty != 7.0 or decoded.price == null or decoded.price.? != 12.5)",
        "        {",
        '            std.debug.print("zig sbe parity round_trip FAILED\\n", .{});',
        "            return error.ParityMismatch;",
        "        }",
        "    }",
        "",
        '    std.debug.print("fig-zig sbe parity OK\\n", .{});',
        "}",
        "",
    ]
    (ROOT / "bindings/zig/sbe_parity.zig").write_text("\n".join(lines))
    print("wrote zig sbe_parity.zig")


def patch_ts_run_sh():
    p = ROOT / "bindings/typescript/smoke/run.sh"
    s = p.read_text()
    # replace the second bun -e block with an invocation of sbe_parity.ts
    start = s.index("# Per-language SBE parity")
    new = """# Per-language SBE parity: encode with the *generated* TypeScript code and compare
# against the shared conformance vectors (produced by the Rust/FFI encoder).
bun bindings/typescript/smoke/sbe_parity.ts
"""
    p.write_text(s[:start] + new)
    print("patched typescript/smoke/run.sh")


if __name__ == "__main__":
    print("encoders", len(ENCODERS), "vectors", len(VECTOR_BY_TYPE))
    missing = [m for m in ENCODERS if m not in SAMPLES]
    print("missing samples", missing)
    gen_go()
    gen_cpp()
    gen_cs()
    gen_ts()
    gen_zig()
    patch_ts_run_sh()
