use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// FIG FSL (Fig Schema Language) compiler
#[derive(Parser)]
#[command(
    name = "ftlc",
    version,
    about = "FIG FSL (Fig Schema Language) compiler"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile an FSL schema to target language
    Compile {
        /// The .fsl file to compile
        file: PathBuf,
        /// Target language: rust, sbe, go, proto, sbe-xml, cpp, csharp, python, typescript, ocaml, zig, json-schema, fix-yaml
        #[arg(long)]
        lang: String,
        /// Output directory for generated code
        #[arg(long)]
        out: PathBuf,
    },
    /// Validate an FSL schema file
    Validate {
        /// The .fsl file to validate
        file: PathBuf,
    },
    /// Parse an FSL schema and output the AST
    Parse {
        /// The .fsl file to parse
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
            let schema = fig_fsl::Parser::parse(&input)
                .with_context(|| format!("Failed to parse {}", file.display()))?;

            let code = match lang.as_str() {
                "rust" => fig_fsl::RustCodegen::generate(&schema),
                "sbe" => fig_fsl::sbe_codegen::generate_sbe(&schema),
                "go" => fig_fsl::GoCodegen::generate(&schema),
                "proto" => fig_fsl::ProtoCodegen::generate(&schema),
                "sbe-xml" => fig_fsl::SbeXmlCodegen::generate(&schema),
                "cpp" => fig_fsl::CppCodegen::generate(&schema),
                "csharp" => fig_fsl::CsharpCodegen::generate(&schema),
                "python" => fig_fsl::PythonCodegen::generate(&schema),
                "typescript" | "ts" => fig_fsl::TypeScriptCodegen::generate(&schema),
                "ocaml" => fig_fsl::OcamlCodegen::generate(&schema),
                "zig" => fig_fsl::ZigCodegen::generate(&schema),
                "json-schema" | "jsonschema" => fig_fsl::JsonSchemaCodegen::generate(&schema),
                "fix-yaml" | "fix" => fig_fsl::FixYamlCodegen::generate(&schema),
                other => anyhow::bail!(
                    "Unsupported language '{}'. Supported: rust, sbe, go, proto, sbe-xml, cpp, csharp, python, typescript, ocaml, zig, json-schema, fix-yaml.",
                    other
                ),
            };

            // Ensure output directory exists
            std::fs::create_dir_all(&out)
                .with_context(|| format!("Failed to create output directory: {}", out.display()))?;

            let output_filename: String = match lang.as_str() {
                "sbe" => "sbe_generated.rs".to_string(),
                "go" => "generated.go".to_string(),
                "proto" => format!("{}.proto", schema.name.replace('.', "_")),
                "sbe-xml" => format!("{}.sbe.xml", schema.name.replace('.', "_")),
                "cpp" => "generated.hpp".to_string(),
                "csharp" => "Generated.cs".to_string(),
                "python" => "generated.py".to_string(),
                "typescript" | "ts" => "generated.ts".to_string(),
                "ocaml" => "generated.ml".to_string(),
                "zig" => "generated.zig".to_string(),
                "json-schema" | "jsonschema" => {
                    format!("{}.schema.json", schema.name.replace('.', "_"))
                }
                "fix-yaml" | "fix" => format!("{}.fix.yaml", schema.name.replace('.', "_")),
                _ => "generated.rs".to_string(),
            };
            let output_path = out.join(output_filename);
            std::fs::write(&output_path, &code).with_context(|| {
                format!("Failed to write output file: {}", output_path.display())
            })?;

            println!("Generated {} code: {}", lang, output_path.display());
            println!("  Schema: {} {}", schema.name, schema.version);
            println!("  Messages: {}", schema.messages.len());
            println!("  Type definitions: {}", schema.type_defs.len());
        }

        Commands::Validate { file } => {
            let input = read_file(&file)?;
            match fig_fsl::Parser::parse(&input) {
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
            let schema = fig_fsl::Parser::parse(&input)
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
                    anyhow::bail!("Unknown format '{}'. Use 'json' or 'debug'.", other);
                }
            }
        }
    }

    Ok(())
}
