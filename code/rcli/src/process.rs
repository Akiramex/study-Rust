use anyhow::Result;
use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};

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
    law: String
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Gun = result?;
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;

    Ok(())
}