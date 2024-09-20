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

impl User {
    pub fn is_activate(&self) -> bool {
        self.status == 0
    }

    pub fn set_delete(&mut self) {
        self.status = 1;
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct ResponseUser {
    pub name: String,
    pub password: String,
}

impl From<User> for ResponseUser {
    fn from(value: User) -> Self {
        ResponseUser {
            name: value.name.clone(),
            password: value.password.clone(),
        }
    }
}