//! Workspace codegen tasks (FSL → Rust/SBE generated artifacts).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use fig_fsl::load_merged_trading_schema;

#[derive(Parser)]
#[command(name = "xtask", about = "FIG workspace tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Regenerate FSL outputs consumed by fig-core
    Codegen {
        /// Fail if generated files differ from committed artifacts
        #[arg(long)]
        check: bool,
    },
    /// Verify gateway REST/WS catalogs map all §17 high-priority paths
    CheckGateway,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Codegen { check } => run_codegen(check),
        Commands::CheckGateway => run_check_gateway(),
    }
}

fn run_codegen(check: bool) -> Result<()> {
    let root = workspace_root();
    let parsed = load_merged_trading_schema();
    let out_dir = root.join("crates/fig-core/src/generated");
    fs::create_dir_all(&out_dir)?;

    write_if_changed(
        &out_dir.join("messages.rs"),
        &fig_fsl::RustCodegen::generate(&parsed),
        check,
    )?;
    write_if_changed_rust(
        &out_dir.join("sbe_generated.rs"),
        &fig_fsl::sbe_codegen::generate_sbe(&parsed),
        check,
    )?;

    let sbe_targets = [
        (
            root.join("bindings/go/fig/sbe_generated.go"),
            fig_fsl::SbeTargetLang::Go,
        ),
        (
            root.join("bindings/cpp/include/fig/sbe_generated.hpp"),
            fig_fsl::SbeTargetLang::Cpp,
        ),
        (
            root.join("bindings/csharp/Fig/SbeGenerated.cs"),
            fig_fsl::SbeTargetLang::Csharp,
        ),
        (
            root.join("bindings/typescript/sbe_generated.ts"),
            fig_fsl::SbeTargetLang::TypeScript,
        ),
        (
            root.join("bindings/zig/sbe_generated.zig"),
            fig_fsl::SbeTargetLang::Zig,
        ),
    ];
    for (path, lang) in sbe_targets {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_if_changed(&path, &fig_fsl::generate_sbe_target(&parsed, lang), check)?;
    }

    let cpp_pure = root.join("bindings/cpp/pure");
    let zig_pure = root.join("bindings/zig/pure");
    fs::create_dir_all(&cpp_pure)?;
    fs::create_dir_all(&zig_pure)?;
    write_if_changed(
        &cpp_pure.join("fig_protocol.hpp"),
        &fig_fsl::generate_protocol(fig_fsl::ProtocolTargetLang::Cpp),
        check,
    )?;
    write_if_changed(
        &zig_pure.join("protocol.zig"),
        &fig_fsl::generate_protocol(fig_fsl::ProtocolTargetLang::Zig),
        check,
    )?;

    if check {
        println!("codegen check OK");
    } else {
        println!("codegen refreshed in {}", out_dir.display());
        println!("SBE target bindings refreshed under bindings/{{go,cpp,csharp,typescript,zig}}");
        println!("protocol libraries refreshed in bindings/{{cpp,zig}}/pure");
    }
    Ok(())
}

fn run_check_gateway() -> Result<()> {
    let status = Command::new("cargo")
        .args(["test", "-p", "fig-gateways", "--lib", "catalog"])
        .status()?;
    if !status.success() {
        bail!("gateway catalog completeness check failed");
    }
    println!("gateway catalog completeness OK");
    Ok(())
}

fn write_if_changed(path: &Path, content: &str, check: bool) -> Result<()> {
    if path.exists() {
        let existing = fs::read_to_string(path)?;
        if existing == content {
            return Ok(());
        }
        if check {
            bail!(
                "generated file drift: {} (run `cargo xtask codegen`)",
                path.display()
            );
        }
    } else if check {
        bail!(
            "missing generated file: {} (run `cargo xtask codegen`)",
            path.display()
        );
    }
    fs::write(path, content).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

/// Like [`write_if_changed`] but normalizes Rust sources with `rustfmt` before compare/write.
fn write_if_changed_rust(path: &Path, content: &str, check: bool) -> Result<()> {
    let formatted = rustfmt_string(path, content)?;
    write_if_changed(path, &formatted, check)
}

fn rustfmt_string(path: &Path, content: &str) -> Result<String> {
    let tmp = std::env::temp_dir().join(format!(
        "fig-codegen-{}",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("out.rs")
    ));
    fs::write(&tmp, content)?;
    let status = Command::new("rustfmt").arg(&tmp).status()?;
    if !status.success() {
        bail!("rustfmt failed for {}", path.display());
    }
    Ok(fs::read_to_string(&tmp)?)
}

fn workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.pop();
    dir
}

/// Convenience alias used from CI scripts.
pub fn codegen_check() -> Result<()> {
    let status = Command::new("cargo")
        .args(["run", "-p", "xtask", "--", "codegen", "--check"])
        .status()?;
    if !status.success() {
        bail!("codegen check failed");
    }
    Ok(())
}
