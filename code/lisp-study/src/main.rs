mod object;
mod env;
mod lexer;
mod parser;
mod eval;

use std::{rc::Rc, cell::RefCell};
use object::Object;
use linefeed::{Interface, ReadResult};

const PROMPT: &str = "> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let reader = Interface::new("lisp_study").unwrap();
    let mut env = Rc::new(RefCell::new(env::Env::new()));

    reader.set_prompt(PROMPT).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let val = eval::eval(input.as_ref(), &mut env)?;
        match val {
            Object::Void => {}
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            Object::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
            }
            _ => println!("{}", val),
        }
    }
    
    println!("Good Bye!");
    Ok(())
}
