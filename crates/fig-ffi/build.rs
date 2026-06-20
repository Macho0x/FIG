use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        include_guard: Some("FIG_FFI_H".to_string()),
        ..Default::default()
    };
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("cbindgen")
        .write_to_file(crate_dir.join("include/fig.h"));
}
