use std::{fmt, path::PathBuf, str::FromStr};

use clap::Parser;

use super::{verity_file, verity_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verity a signed message")]
    Verity(TextVerityOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "Encrypt a message")]
    Encrypt(TextEncrypt),
    #[command(about = "Decrypt a encrypt message")]
    Decrypt(TextDecrypt),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verity_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verity_file)]
    pub key: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerityOpts {
    #[arg(short, long, value_parser = verity_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verity_file)]
    pub key: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long)]
    pub sig: String,
}
#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verity_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncrypt {
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecrypt {
    #[arg(short, long)]
    pub key: String,
}

fn parse_format(input: &str) -> Result<TextSignFormat, anyhow::Error> {
    input.parse()
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            v => anyhow::bail!("Invalid format: {v}"),
        }
    }
}
