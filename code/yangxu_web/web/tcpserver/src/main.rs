use std::net::TcpListener;
use std::io::{Write, Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buf = [0;1024];

        stream.read_exact(&mut buf).unwrap();
        stream.write_all(&buf).unwrap();
    }
}