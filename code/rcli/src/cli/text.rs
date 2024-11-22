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
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a encrypt message")]
    Decrypt(TextDecryptOpts),
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
    #[arg(short, long, value_parser = parse_generate_format, default_value = "blake3")]
    pub format: TextGenerateFormat,

    #[arg(short, long, value_parser = verity_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verity_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verity_file)]
    pub key: String,

    #[arg(long, value_parser = parse_encrypt_format, default_value = "chacha20ploy1305")]
    pub format: TextEncryptFormat,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verity_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verity_file)]
    pub key: String,

    #[arg(long, value_parser = parse_encrypt_format, default_value = "chacha20ploy1305")]
    pub format: TextEncryptFormat,
}

fn parse_format(input: &str) -> Result<TextSignFormat, anyhow::Error> {
    input.parse()
}

fn parse_encrypt_format(input: &str) -> Result<TextEncryptFormat, anyhow::Error> {
    input.parse()
}

fn parse_generate_format(input: &str) -> Result<TextGenerateFormat, anyhow::Error> {
    input.parse()
}

#[derive(Debug, Clone, Copy)]
pub enum TextEncryptFormat {
    ChaCha20,
    Aes,
}

impl From<TextEncryptFormat> for &'static str {
    fn from(format: TextEncryptFormat) -> Self {
        match format {
            TextEncryptFormat::ChaCha20 => "chacha20ploy1305",
            TextEncryptFormat::Aes => "aes",
        }
    }
}

impl fmt::Display for TextEncryptFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl FromStr for TextEncryptFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "chacha20ploy1305" => Ok(TextEncryptFormat::ChaCha20),
            "aes" => Ok(TextEncryptFormat::Aes),
            v => anyhow::bail!("Invalid format: {v}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextGenerateFormat {
    Blake3,
    Ed25519,
    ChaCha20,
    Aes
}

impl From<TextGenerateFormat> for &'static str {
    fn from(format: TextGenerateFormat) -> Self {
        match format {
            TextGenerateFormat::Blake3 => "blake3",
            TextGenerateFormat::Ed25519 => "ed25519",
            TextGenerateFormat::ChaCha20 => "chacha20ploy1305",
            TextGenerateFormat::Aes => "aes",
        }
    }
}

impl fmt::Display for TextGenerateFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl FromStr for TextGenerateFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextGenerateFormat::Blake3),
            "ed25519" => Ok(TextGenerateFormat::Ed25519),
            "chacha20ploy1305" => Ok(TextGenerateFormat::ChaCha20),
            "aes" => Ok(TextGenerateFormat::Aes),
            v => anyhow::bail!("Invalid format: {v}"),
        }
    }
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