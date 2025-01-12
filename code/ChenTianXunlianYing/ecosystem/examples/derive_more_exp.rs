use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, From, Add)]
struct MyInt(i32);

#[derive(Debug, PartialEq, From, Into, Add)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display("int: {_0}")]
    Int(i32),
    Uint(u32),
    #[display("nothing")]
    Nothing,
}

fn main() {
    let a = Point2D {x:1, y:1};
    let b = Point2D {x:2, y:2};
    println!("{:?}", a + b);
}