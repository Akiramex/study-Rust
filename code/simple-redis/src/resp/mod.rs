mod decode;
mod encode;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
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
#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct SimpleString(String);

impl Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct SimpleError(String);
impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct BulkString(Vec<u8>);
impl Deref for BulkString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct RespNull;


#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct RespNullArray;


#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct RespNullBulkString;


#[derive(Debug, PartialEq, PartialOrd)]
struct RespArray(Vec<RespFrame>);
impl Deref for RespArray {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(s.into())
    }
}


#[derive(Debug, PartialEq, PartialOrd)]
struct RespMap(BTreeMap<String, RespFrame>);
impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}


#[derive(Debug, PartialEq, PartialOrd)]
struct RespSet(Vec<RespFrame>);
impl Deref for RespSet {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}