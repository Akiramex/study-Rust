extern crate glob;
use glob::glob;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

fn write_an_address() -> Result<()> {
    let person = Person {
        name: "Akira".to_owned(),
        age: 10,
        phones: vec!["123".to_owned(), "456".to_owned()]
    };

    let j = serde_json::to_string_pretty(&person)?;

    let mut file = File::create("data.json").expect("open File Error");

    file.write_all(j.as_bytes()).expect("write Error");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let hlw = "Hello World".to_string();
        let bytes = hlw.as_bytes();
        let len = bytes.len();

        println!("{len}")
    }

    #[test]
    fn find_file() {
        for entry in glob("*").unwrap() {
            match entry {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    #[test]
    fn function() {
        write_an_address().unwrap();
    }
}