use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use fig_conformance::run_vectors_file;

#[derive(Parser)]
#[command(name = "fig-conformance", about = "Run FIG conformance vectors")]
struct Args {
    #[arg(long, default_value = "tests/conformance/vectors/v1.json")]
    vectors: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    run_vectors_file(&args.vectors)?;
    println!("conformance OK: {}", args.vectors.display());
    Ok(())
}
