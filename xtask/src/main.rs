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
    write_if_changed(
        &out_dir.join("sbe_generated.rs"),
        &fig_fsl::sbe_codegen::generate_sbe(&parsed),
        check,
    )?;

    if check {
        println!("codegen check OK");
    } else {
        println!("codegen refreshed in {}", out_dir.display());
    }
    Ok(())
}

fn run_check_gateway() -> Result<()> {
    let status = Command::new("cargo")
        .args([
            "test",
            "-p",
            "fig-gateways",
            "--lib",
            "catalog",
        ])
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
