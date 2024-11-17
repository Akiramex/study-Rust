use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    law: String,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let header = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
