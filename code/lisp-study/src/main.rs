mod object;
mod env;
mod lexer;
mod parser;

use std::{rc::Rc, cell::RefCell, io::Write};
use linefeed::{Interface, ReadResult};

fn main() {
    let reader = Interface::new("application").unwrap();
    let mut env = Rc::new(RefCell::new(env::Env::new()));

    reader.set_prompt("> ").unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        println!("get input {:?}", input); 
    }
    println!("Good Bye!");
}

fn my_interface() {
    use std::io;

    let mut buf = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();
        if input.eq("exit") {
            break;
        }
        println!("get input {:?}", input);
        buf.clear();
    }
    println!("Good Bye!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        #[derive(Debug, Default)]
        struct Test {
            a: Option<Box<Test>>
        }

        let a = Test::default();
        println!("{:?}", a)
    }
}