#![allow(unused)]

mod error;
mod prelude;
mod uitls;

use std::fmt::format;
use std::fs::read_dir;
use crate::error::Error;
use crate::prelude::*;

fn main() -> Result<()> {
    for entry in read_dir("./").unwrap().filter_map(|e| e.ok()) {
        let entry = entry
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| {
                Error::Generic(format!("Invalid path {entry:?}"))
            })?;
        println!("{entry}")
    }


    for entry in read_dir("./").unwrap().filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}")
    }
    Ok(())
}
