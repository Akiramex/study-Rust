use std::{fs::File, io::{Read, Write}};
use std::io::prelude::*;
use std::io::BufReader;

const FILE : &str = "foo.txt";

fn create_new_file_and_write() -> std::io::Result<()> {
    let mut file = File::create_new(FILE)?;

    file.write_all(b"Hello World")?;

    Ok(())
} 

fn open_file_and_read() -> std::io::Result<()> {
    let mut file = File::open(FILE)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    
    println!("{buf}");

    Ok(())
}

fn read_with_buffer() -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(File::open(FILE).unwrap());

    let mut line = String::new();
    let len = buf_reader.read_line(&mut line)?;
    // 换行符会占 2 bytes  --> /r/n
    println!("First line is {len} bytes long");
    println!("{line}");
    Ok(())
}

fn main() {
    let file = file!();
    println!("{file}");

    read_with_buffer();
}
