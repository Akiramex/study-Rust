pub mod user;

use serde::Serialize;

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