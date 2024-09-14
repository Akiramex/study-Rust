use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeacherReponse {
    pub id: i32,
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}