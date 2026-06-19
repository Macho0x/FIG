# ADR 0004: FSL as Single Source of Truth for Message Types

## Status

Accepted

## Context

FIG defines trading message types (enums, structs, field numbers) in multiple
places today:

- [`schemas/orders.fsl`](../../schemas/orders.fsl) — documented as canonical
- [`crates/fig-core/src/messages.rs`](../../crates/fig-core/src/messages.rs) — hand-maintained Rust types (temporary)
- [`crates/fig-fsl/src/target_codegen.rs`](../../crates/fig-fsl/src/target_codegen.rs) — flat struct codegen for Python, C++, C#, Go, etc.
- [`crates/fig-fsl/src/codegen.rs`](../../crates/fig-fsl/src/codegen.rs) + [`sbe_codegen.rs`](../../crates/fig-fsl/src/sbe_codegen.rs) — full Rust + SBE codegen from FSL

Editing Rust enums (e.g. `Side`, `OrderType`) directly forces manual updates
across FSL, SBE discriminants, gateway FIX mappings, conformance vectors, and
every language SDK. That workflow will drift.

Multi-language SDK parity ([TODO.md §16](../../TODO.md)) depends on one schema
change propagating to all targets from a single edit.

## Decision

**[`schemas/*.fsl`](../../schemas/) is the only authoritative definition** for
message types, enums, field numbers, and gateway mappings.

1. **Never edit generated or mirrored types by hand** — including
   `fig-core/src/messages.rs` once build-time codegen is wired (see
   [Transition](#transition) below). Until then, any hand edit to `messages.rs`
   must land in the same PR as the matching FSL change.

2. **Regenerate all targets from FSL** via `ftlc` after every schema change:

   ```bash
   cargo run -p fig-fsl --bin ftlc -- validate schemas/orders.fsl
   for lang in rust sbe python cpp csharp go typescript ocaml zig proto sbe-xml json-schema fix-yaml; do
     cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang "$lang" --out "generated/$lang"
   done
   ```

3. **Rust is a codegen target, not a special case.** `RustCodegen` and
   `--lang sbe` produce the same logical types as Python/C++/etc.; Rust runtime
   code consumes generated output rather than owning the schema.

4. **Wire compatibility is proven by conformance tests** ([TODO.md §16.1](../../TODO.md)),
   not by eyeballing generated code. Golden CBOR and SBE vectors are derived from
   the Rust reference implementation after regen.

5. **Schema versioning follows semver on the FSL `schema` declaration:**

   | Bump | When | Examples |
   |---|---|---|
   | **Patch** (`v1.0.0` → `v1.0.1`) | Docs/constraints only; no wire change | `max_len` doc, pattern tweak with no runtime effect |
   | **Minor** (`v1.0.0` → `v1.1.0`) | Additive, backward compatible | New optional field, new enum variant **appended**, new message type |
   | **Major** (`v1.0.0` → `v2.0.0`) | Breaking wire or API | Remove/rename field, reorder enum variants, change `@N` field numbers |

6. **SBE enum variants are positional.** Codegen assigns discriminants
   sequentially (`buy=1`, `sell=2`, …). **Append only** on minor bumps; never
   reorder or insert in the middle without a major version bump.

7. **CBOR uses serde string enums** (e.g. `"Buy"`, `"Limit"`). Renaming a
   variant is breaking for CBOR payloads unless both names are accepted during
   a migration window.

8. **Gateway mappings live in the same FSL file** (`gateway fix`, `gateway rest`).
   FIX tag/value maps and REST paths change with the schema in one commit.

## Consequences

### Positive

- One edit updates all languages; CI catches forgotten regen via `git diff`.
- Enum and field-number rules are explicit; SBE/CBOR breakage is predictable.
- FSL co-locates protocol types with FIX/REST gateway config.
- Clear path to multi-language SDK parity (TODO §16).

### Negative / tradeoffs

- Contributors must run `ftlc` (or CI fails) — cannot “just fix Rust.”
- Hand-maintained `messages.rs` must be retired to avoid two sources of truth.
- Full serializer parity per language (TODO §16.2) is still outstanding; until
  then, regen updates **shapes** but not all encode/decode paths in every lang.
- Breaking changes require coordinated rollout (schema version, SETTINGS, dual
  codec support during migration).

## Schema change checklist

Use this for every PR that touches `schemas/`:

- [ ] Edit `.fsl` only (not `messages.rs` or other hand-maintained mirrors)
- [ ] Bump `schema … vX.Y.Z` if wire or API semantics change
- [ ] Append SBE enum variants; do not reorder existing variants
- [ ] Assign new protobuf/FSL field numbers (`@N`); never reuse numbers
- [ ] Update `gateway fix` / `gateway rest` blocks if external mappings change
- [ ] Run `ftlc validate` on changed schemas
- [ ] Regenerate all `ftlc --lang` targets you ship (or full list above)
- [ ] Sync `fig-core/src/messages.rs` **only until** build-time codegen lands
- [ ] Update conformance / integration tests; add golden vectors if payloads change
- [ ] Document breaking changes in PR description and bump major version if needed

## Transition

Today `fig-core/src/messages.rs` is hand-maintained for bootstrap speed. The
target state:

1. `build.rs` or `cargo xtask codegen` runs `ftlc` for `rust` + `sbe` into
   `fig-core/src/generated/`.
2. `messages.rs` becomes a thin re-export or is removed.
3. CI job fails if generated output differs from committed artifacts.
4. Non-Rust SDKs consume the same FSL revision tagged with each FIG release.

Track remaining work in [TODO.md §16](../../TODO.md).

## References

- [SPEC.md §11 — FSL](../../SPEC.md)
- [CONTRIBUTING.md — Schema changes](../../CONTRIBUTING.md#schema-changes)
- [TODO.md §16 — Multi-Language SDK Parity](../../TODO.md)
