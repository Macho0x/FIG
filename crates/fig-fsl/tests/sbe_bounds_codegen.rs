//! Regression tests: the SBE code generator must emit bounds-checked decoders.
//!
//! Before the hardening change the generated decoders indexed the input buffer
//! directly (`buf[pos]`, `&buf[pos..pos+len]`) after only an 8-byte header check,
//! and allocated `Vec::with_capacity(<wire count>)` unbounded. These tests pin
//! the safe-codegen contract so that cannot regress.

use fig_fsl::{generate_sbe, Parser};

const SCHEMA: &str = r#"
    schema test_bounds v1.0 {
        description: "bounds test"
        well_known_id: 0x01
        type Name: string(max_len: 32)
        type Level {
            price: i64
            qty: u32
        }
        message Greeting {
            channel_type: request_response
            name: Name @1
            note: Name @2 optional
            side: enum { buy, sell } @3
            levels: list<Level> @4
        }
    }
"#;

fn generated() -> String {
    let schema = Parser::parse(SCHEMA).expect("schema parses");
    generate_sbe(&schema)
}

#[test]
fn decoders_use_bounds_checked_helpers() {
    let code = generated();

    // The shared, checked cursor helpers must be emitted.
    for helper in [
        "fn sbe_take",
        "fn sbe_u8",
        "fn sbe_u16",
        "fn sbe_u32",
        "fn sbe_u64",
        "fn sbe_i64",
        "fn sbe_f64",
        "fn sbe_str",
    ] {
        assert!(code.contains(helper), "missing helper `{helper}`");
    }
}

#[test]
fn decoders_never_index_the_buffer_by_position() {
    let code = generated();

    // No decoder may index the input with an advancing position.
    for pattern in ["buf[pos", "buf[pos +", "buf[pos+"] {
        assert!(
            !code.contains(pattern),
            "generated decoder still uses unchecked indexing `{pattern}`"
        );
    }

    // The only remaining direct buffer indexing is the guarded 8-byte header
    // read (`buf[0]`..`buf[7]`), which is safe behind `buf.len() < 8`.
    assert!(!code.contains("&buf[pos"));
}

#[test]
fn list_allocations_are_capped_by_remaining_bytes() {
    let code = generated();

    assert!(
        code.contains("fn sbe_capacity"),
        "missing allocation-cap helper"
    );
    assert!(
        code.contains("Vec::with_capacity(sbe_capacity("),
        "list allocation is not capped against the remaining buffer"
    );
    // A raw, wire-controlled count must never be passed to with_capacity.
    assert!(!code.contains("Vec::with_capacity(levels_count)"));
}

#[test]
fn string_reads_are_length_validated() {
    let code = generated();

    assert!(
        code.contains("let name = sbe_str(buf, &mut pos)?"),
        "string fields must decode through the checked `sbe_str` helper"
    );
}
