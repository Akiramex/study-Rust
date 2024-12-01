mod encode;
mod decode;

use std::collections::{HashMap, HashSet};
use std::ops::Deref;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

/*
    如何解析 Frame
    - simple string: "+OK\r\n"
    - error: "-Error message\r\n"
    - integer: ":[<+|->]<value>\r\n"
    - bulk error: "!<lenth>\r\n<error>\r\n"
    - bulk string: "$-1\r\n"
    - null: "_\r\n"
    - null array: "*-1\r\n"
    - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
    - boolean: "#<t|f>\r\n"
    - double: "[<+|->]<integral>[.<fractional>]<E|e>[sign]<exponent>\r\n"
    - big number: "([+|-]<number>\r\n"
    - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    - set: "~"<number-of-elements>\r\n<element-1>...<element-n>
*/
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

struct SimpleString(String);
impl Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SimpleString {
    pub fn new(s :impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

struct SimpleError(String);
impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct BulkString(Vec<u8>);
impl Deref for BulkString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct RespNull;
struct RespNullArray;
struct RespNullBulkString;
struct RespArray(Vec<RespFrame>);
impl Deref for RespArray {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct RespMap(HashMap<String, RespFrame>);
impl Deref for RespMap {
    type Target = HashMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct RespSet(HashSet<RespFrame>);
impl Deref for RespSet {
    type Target = HashSet<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}