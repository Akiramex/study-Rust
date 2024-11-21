use clap::Parser;
use std::path::PathBuf;

use super::verity_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(httpServeOpts),
}

#[derive(Debug, Parser)]
pub struct httpServeOpts {
    #[arg(short, long, value_parser = verity_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
