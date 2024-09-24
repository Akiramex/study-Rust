use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Default, Clone, Serialize, sqlx::FromRow)]
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

impl From<User> for ResponseUser {
    fn from(value: User) -> Self {
        ResponseUser {
            id: value.id,
            name: value.name.clone(),
            time: value.time
        }
    }
}