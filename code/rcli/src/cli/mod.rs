mod base64;
mod csv;
mod genpass;
mod text;
mod http;
use std::path::{Path, PathBuf};

use self::{csv::CsvOpts, genpass::GenPassOpts};

use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    text::{TextSignFormat, TextSubCommand},
    http::HttpSubCommand,
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Operator csv file")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

// 用户输入检查
fn verity_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verity_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist, or is not a directory")
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verity_file("-"), Ok("-".into()));
        assert_eq!(verity_file("*"), Err("File does not exist"));
        assert_eq!(verity_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verity_file("no-exist"), Err("File does not exist"));
    }

    #[test]
    fn test_verify_input_path() {
        assert_eq!(verity_path("*"), Err("Path does not exist, or is not a directory"));
        assert_eq!(verity_path("Cargo.toml"), Err("Path does not exist, or is not a directory"));
        assert_eq!(verity_path("assets"), Ok("assets".into()));
    }
}
