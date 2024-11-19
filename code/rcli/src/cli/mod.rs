mod base64;
mod csv;
mod genpass;

use self::{csv::CsvOpts, genpass::GenPassOpts};

use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
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
}

// 用户输入检查
fn verity_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verity_input_file("-"), Ok("-".into()));
        assert_eq!(verity_input_file("*"), Err("File does not exist"));
        assert_eq!(verity_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verity_input_file("no-exist"), Err("File does not exist"));
    }
}
