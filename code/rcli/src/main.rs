use anyhow::Ok;
// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Gun {
    #[serde(rename = "rownames")]
    id: i32,
    year: i32,
    violent: f64,
    murder: f64,
    robbery: f64,   
    prisoners: f64,
    afam: f64,
    cauc: f64,
    male: f64,
    population: f64,
    income: f64,
    density: f64,
    state: String,
    law: String
}

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Operator csv file")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, help = "Input file path", value_parser = verity_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json", help = "Output file path")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = false, help = "CSV has header or not")]
    header: bool,
}

fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            for result in reader.deserialize() {
                let record: Gun = result?;
                println!("{:?}", record)
            }
        }
    }

    Ok(())
}

// 用户输入检查
fn verity_input_file(filename: &str) -> Result<String, String> {
    if std::path::Path::new(filename).exists() {
        core::result::Result::Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}
