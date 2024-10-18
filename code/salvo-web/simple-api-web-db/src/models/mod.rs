pub mod user;

use serde::Serialize;
use crate::prelude::Error;

#[derive(Serialize, Debug)]
pub struct ResponseInfo<T> 
    where T: Serialize
{
    pub code: i32,
    pub msg: String,
    pub total: i32,
    pub data: T,
}

impl Default for ResponseInfo<String> {
    fn default() -> Self {
        ResponseInfo {
            code: 0,
            msg: String::new(),
            total: 0,
            data: String::new(),
        }
    }
}

impl From<Error> for ResponseInfo<String> {
    fn from(err: Error) -> Self {
        let code = match err {
            Error::Generic(_) => 1,
            Error::IO(_) => 2,
            Error::SalvoPE(_) => 3,
            Error::DB(_) => 4,
        };

        ResponseInfo {
            code,
            msg: err.to_string(),
            total: 0,
            data: String::new(),
        }
    }
}