use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // example of code_generate
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(&dest_path, 
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");

    // example of clib
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");   // outputs `libhello.a`
    println!("cargo:rerun-if-changed=src/hello.c");

    
}