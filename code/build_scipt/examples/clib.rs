use thiserror::Error;
use anyhow::Result;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Nul error: {0}")]
    NulError(#[from] std::ffi::NulError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

fn prompt(s: &str) -> Result<String> {
    use std::io::Write;
    print!("{}", s);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}


extern {
    fn hello();
    fn greet(name: *const c_char);
}



fn main() -> Result<()> {
    unsafe { hello() }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}