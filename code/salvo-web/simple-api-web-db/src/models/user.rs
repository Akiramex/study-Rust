use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub status: i32,
    pub time: Option<NaiveDateTime>
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub password: String,
}

// 临时写在这里
#[derive(Deserialize)]
pub struct LoginParams {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Default)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub password: Option<String>,
    pub status: Option<i32>,
}

#[derive(Serialize)]
pub struct ResponseUser {
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>
}

impl LoginParams {
    pub fn check_params(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(Error::Generic("账号不得为空".to_owned()));
        }
        if self.password.trim().is_empty() {
            return Err(Error::Generic("密码不得为空".to_owned()))
        }
        Ok(())
    }
}

impl From<User> for ResponseUser {
    fn from(value: User) -> Self {
        ResponseUser {
            id: value.id,
            name: value.name.clone(),
            time: value.time
        }
    }
}