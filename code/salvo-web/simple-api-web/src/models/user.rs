use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Default, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub state: u8,
    pub time: NaiveDateTime
}

impl User {
    pub fn is_activate(&self) -> bool {
        self.state == 0
    }

    pub fn set_delete(&mut self) {
        self.state = 1;
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