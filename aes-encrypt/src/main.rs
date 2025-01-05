mod crypto;

use std::{fs, path::PathBuf};

use anyhow::Ok;
use clap::{Parser, Subcommand};
use rand::Rng;
use zeroize::Zeroize;

#[derive(Parser)]
#[command(name = "aes-encrypt")]
#[command(about = "Encrypt data using AES256-GCM")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        key_file: PathBuf,
    },
    Decrypt {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        key_file: PathBuf,
    },
    GenerateKey {
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn read_key(key_file: &std::path::Path) -> anyhow::Result<[u8; 32]> {
    fs::read(key_file)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid key length"))
}

fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Encrypt {
            input,
            output,
            key_file,
        } => {
            let mut key: [u8; 32] = read_key(&key_file)?;

            let data = fs::read(&input)?;
            let encryptor = crypto::Encryptor::new(&key);
            let encrypted = encryptor.encrypt(&data)?;
            fs::write(output, encrypted)?;

            println!("Data encrypted successfully");

            key.zeroize();
        }
        Commands::Decrypt {
            input,
            output,
            key_file,
        } => {
            let mut key: [u8; 32] = read_key(&key_file)?;

            let data = fs::read(&input)?;
            let encryptor = crypto::Encryptor::new(&key);
            let decrypted = encryptor.decrypt(&data)?;
            fs::write(output, decrypted)?;

            println!("Data decrypted successfully");

            key.zeroize();
        }
        Commands::GenerateKey { output } => {
            let mut key = [0u8; 32];

            rand::thread_rng().fill(&mut key);
            fs::write(output, key)?;

            println!("Key generated successfully");

            key.zeroize();
        }
    }

    Ok(())
}
