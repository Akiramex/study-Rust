use std::process::{Child, Command};

#[derive(Debug, Default)]
pub struct ExProgram<'a> {
    pub path: &'a str,
    pub opt: Option<Vec<&'a str>>
}

impl<'a> ExProgram<'a> {

    pub fn run(&self) -> Child {
        Command::new(self.path)
        .spawn()
        .expect("Failed to start the exe program")
    }
}