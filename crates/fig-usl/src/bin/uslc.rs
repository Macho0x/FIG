use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// FIG FTL (Fig Tree Language) Schema Language Compiler
#[derive(Parser)]
#[command(name = "ftlc", version, about = "FIG FTL (Fig Tree Language) Schema Language Compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile an FTL schema to target language
    Compile {
        /// The .usl file to compile
        file: PathBuf,
        /// Target language ("rust" for serde structs, "sbe" for SBE binary codec)
        #[arg(long)]
        lang: String,
        /// Output directory for generated code
        #[arg(long)]
        out: PathBuf,
    },
    /// Validate an FTL schema file
    Validate {
        /// The .usl file to validate
        file: PathBuf,
    },
    /// Parse an FTL schema and output the AST
    Parse {
        /// The .usl file to parse
        file: PathBuf,
        /// Output format: "json" or "debug"
        #[arg(long, default_value = "json")]
        format: String,
    },
}

fn read_file(path: &PathBuf) -> anyhow::Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { file, lang, out } => {
            let input = read_file(&file)?;
            let schema = fig_usl::Parser::parse(&input)
                .with_context(|| format!("Failed to parse {}", file.display()))?;

            let code = match lang.as_str() {
                "rust" => fig_usl::RustCodegen::generate(&schema),
                "sbe" => fig_usl::sbe_codegen::generate_sbe(&schema),
                other => anyhow::bail!(
                    "Unsupported language '{}'. Supported: 'rust' (serde structs), 'sbe' (SBE codec).",
                    other
                ),
            };

            // Ensure output directory exists
            std::fs::create_dir_all(&out)
                .with_context(|| format!("Failed to create output directory: {}", out.display()))?;

            let output_path = out.join(match lang.as_str() {
                "sbe" => "sbe_generated.rs",
                _ => "generated.rs",
            });
            std::fs::write(&output_path, &code)
                .with_context(|| format!("Failed to write output file: {}", output_path.display()))?;

            println!("Generated {} code: {}", lang, output_path.display());
            println!("  Schema: {} {}", schema.name, schema.version);
            println!("  Messages: {}", schema.messages.len());
            println!("  Type definitions: {}", schema.type_defs.len());
        }

        Commands::Validate { file } => {
            let input = read_file(&file)?;
            match fig_usl::Parser::parse(&input) {
                Ok(schema) => {
                    let msg_count = schema.messages.len();
                    println!("Valid: {}", file.display());
                    println!("  Schema: {} {}", schema.name, schema.version);
                    println!("  Messages: {}", msg_count);
                    println!("  Type definitions: {}", schema.type_defs.len());
                }
                Err(e) => {
                    eprintln!("Error in {}: {}", file.display(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Parse { file, format } => {
            let input = read_file(&file)?;
            let schema = fig_usl::Parser::parse(&input)
                .with_context(|| format!("Failed to parse {}", file.display()))?;

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&schema)?;
                    println!("{}", json);
                }
                "debug" => {
                    println!("{:#?}", schema);
                }
                other => {
                    anyhow::bail!(
                        "Unknown format '{}'. Use 'json' or 'debug'.",
                        other
                    );
                }
            }
        }
    }

    Ok(())
}
