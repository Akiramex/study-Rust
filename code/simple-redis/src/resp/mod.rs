mod decode;
mod encode;

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use thiserror::Error;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length: {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,
    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
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
impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string())
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
impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string()).into()
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
impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString(s.as_bytes().to_vec())
    }
}
impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec())
    }
}
impl From<&[u8]> for RespFrame {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec()).into()
    }
}
impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec())
    }
}
impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec()).into()
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
