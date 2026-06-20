//! FFI binding conformance against shared §16.1 / §17 vectors.

use std::path::PathBuf;

#[test]
fn ffi_binding_matches_conformance_vectors() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let vectors = manifest
        .join("../../tests/conformance/vectors/v1.json")
        .canonicalize()
        .expect("conformance vectors");
    fig_ffi::run_binding_conformance(&vectors).expect("binding conformance");
}
