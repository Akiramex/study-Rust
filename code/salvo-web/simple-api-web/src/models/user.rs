#[derive(Debug, Default, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    state: u8,
}

impl User {
    pub fn is_activate(&self) -> bool {
        self.state == 0
    }
}