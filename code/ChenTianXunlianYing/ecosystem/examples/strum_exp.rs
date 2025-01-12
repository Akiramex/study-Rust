use std::str::FromStr;

use strum::{EnumCount, EnumString, IntoStaticStr};


#[derive(Debug, EnumString, EnumCount, IntoStaticStr)]
enum MyEnum {
    A,
    B,
    C,
}

fn main() {
    let my_enum = MyEnum::from_str("A").unwrap();
    println!("{:?}", my_enum);

    println!("{:?}", MyEnum::COUNT);

    let str :&'static str= my_enum.into();
    println!("{:?}", str);
}