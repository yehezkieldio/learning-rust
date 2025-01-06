mod compresor;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aes-encrypt")]
#[command(about = "Compress file(s) into archive with various compression methods")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short = 'c', long, default_value = "deflate")]
        compression_method: String,
    },
    Analyze {
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Commands::Compress {
            input,
            output,
            compression_method,
        } => {
            let input = input.to_str().ok_or("Invalid input path").unwrap();
            let output = output.to_str().ok_or("Invalid output path").unwrap();

            let compression_method = match compression_method.as_str() {
                "deflate" => zip::CompressionMethod::Deflated,
                "stored" => zip::CompressionMethod::Stored,
                "bzip2" => zip::CompressionMethod::Bzip2,
                "zstd" => zip::CompressionMethod::Zstd,
                _ => {
                    eprintln!("Invalid compression method: {}", compression_method);
                    std::process::exit(1);
                }
            };

            if let Err(e) = compresor::compress(input, output, compression_method) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Analyze { input } => {
            let input = input.to_str().ok_or("Invalid input path").unwrap();

            if let Err(e) = compresor::analyze_zip_file(input) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
