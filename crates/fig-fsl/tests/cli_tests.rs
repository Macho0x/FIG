use std::process::Command;

/// Helper: get the path to the orders.usl schema file
fn orders_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{}/../../schemas/orders.usl", manifest_dir)
}

/// Helper: return a Command to run the ftlc binary.
/// When run via `cargo test`, cargo sets `CARGO_BIN_EXE_ftlc`.
fn ftlc_cmd() -> Command {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_ftlc") {
        Command::new(path)
    } else {
        // Fallback for manual runs
        let mut c = Command::new("cargo");
        c.arg("run")
            .arg("--quiet")
            .arg("-p")
            .arg("fig-fsl")
            .arg("--bin")
            .arg("ftlc")
            .arg("--");
        c
    }
}

#[test]
fn test_ftlc_validate_orders_usl() {
    let output = ftlc_cmd()
        .arg("validate")
        .arg(orders_path())
        .output()
        .expect("Failed to run uslc validate");

    assert!(
        output.status.success(),
        "uslc validate should succeed\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Valid:"),
        "Output should contain 'Valid:'\nstdout: {}",
        stdout
    );
    assert!(
        stdout.contains("trading.orders"),
        "Output should contain schema name\nstdout: {}",
        stdout
    );
}

#[test]
fn test_ftlc_validate_invalid_file() {
    let dir = std::env::temp_dir();
    let invalid_path = dir.join("__test_invalid.usl");
    std::fs::write(&invalid_path, "schema broken {").unwrap();

    let output = ftlc_cmd()
        .arg("validate")
        .arg(invalid_path.to_str().unwrap())
        .output()
        .expect("Failed to run uslc validate");

    let _ = std::fs::remove_file(&invalid_path);

    assert!(
        !output.status.success(),
        "uslc validate should fail for invalid schema\nstdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "Error output should be non-empty for invalid schema"
    );
}

#[test]
fn test_ftlc_compile_generates_file() {
    let out_dir = std::env::temp_dir().join("__test_uslc_out");
    let _ = std::fs::remove_dir_all(&out_dir);

    let output = ftlc_cmd()
        .arg("compile")
        .arg(orders_path())
        .arg("--lang")
        .arg("rust")
        .arg("--out")
        .arg(out_dir.to_str().unwrap())
        .output()
        .expect("Failed to run uslc compile");

    assert!(
        output.status.success(),
        "uslc compile should succeed\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let generated = out_dir.join("generated.rs");
    assert!(generated.exists(), "generated.rs should exist");

    let code = std::fs::read_to_string(&generated).unwrap();
    assert!(code.contains("pub struct NewOrderSingle"), "Should contain NewOrderSingle");
    assert!(code.contains("pub struct ExecutionReport"), "Should contain ExecutionReport");
    assert!(code.contains("pub const SCHEMA_ID: u8 = 0x01;"), "Should contain SCHEMA_ID");

    // Cleanup
    let _ = std::fs::remove_dir_all(&out_dir);
}

#[test]
fn test_ftlc_parse_json_output() {
    let output = ftlc_cmd()
        .arg("parse")
        .arg(orders_path())
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to run uslc parse");

    assert!(
        output.status.success(),
        "uslc parse should succeed\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Verify it's valid JSON by parsing it
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");

    assert_eq!(parsed["name"], "trading.orders");
    assert_eq!(parsed["version"], "v1.0.0");
    assert!(parsed["messages"].is_array());
    assert!(!parsed["messages"].as_array().unwrap().is_empty());
}

#[test]
fn test_ftlc_parse_debug_output() {
    let output = ftlc_cmd()
        .arg("parse")
        .arg(orders_path())
        .arg("--format")
        .arg("debug")
        .output()
        .expect("Failed to run uslc parse --debug");

    assert!(
        output.status.success(),
        "uslc parse --debug should succeed\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Debug output should contain the struct representation
    assert!(stdout.contains("Schema"), "Debug output should contain 'Schema'");
    assert!(
        stdout.contains("trading.orders"),
        "Debug output should contain schema name"
    );
}

#[test]
fn test_ftlc_compile_unsupported_lang() {
    let out_dir = std::env::temp_dir().join("__test_uslc_out_bad");
    let _ = std::fs::remove_dir_all(&out_dir);

    let output = ftlc_cmd()
        .arg("compile")
        .arg(orders_path())
        .arg("--lang")
        .arg("ruby")
        .arg("--out")
        .arg(out_dir.to_str().unwrap())
        .output()
        .expect("Failed to run uslc compile");

    let _ = std::fs::remove_dir_all(&out_dir);

    assert!(
        !output.status.success(),
        "uslc compile with unsupported lang should fail"
    );
}
