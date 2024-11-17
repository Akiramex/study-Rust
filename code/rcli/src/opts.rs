use clap::Parser;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path", value_parser = verity_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json", help = "Output file path")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = false, help = "CSV has header or not")]
    pub header: bool,
}

// 用户输入检查
fn verity_input_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}
