use std::io::{Write, Read};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    let buf = "Hello world";
    stream.write(buf.as_bytes()).unwrap();
    
    let mut buf = [0; 11];
    stream.read(&mut buf).unwrap();

    println!("Response from server:{:?}", str::from_utf8(&buf).unwrap());
}
