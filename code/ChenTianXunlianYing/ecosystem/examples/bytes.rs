use bytes::{BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(1024);

    buf.put(&b"hello world"[..]);

    let a = buf.split();

    println!("{:?}", a);

    println!("{:?}", buf);
}