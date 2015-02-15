#[macro_use] extern crate log;
use std::old_io as io;
use std::fmt;
use scanner::{Token, Scanner};
mod scanner;


fn main() {
    let mut scanner = Scanner::new(io::stdin());
    let mut stack: Vec<i64> = Vec::new();
    loop {
        let token = scanner.get_token();
        debug!("{}", token.as_ref().unwrap());
        match token {
            Ok(Token::Number(n)) => stack.push(n),
            Ok(Token::BinOp(op)) => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match op {
                    "+" => stack.push(lhs + rhs),
                    "-" => stack.push(lhs - rhs),
                    "*" => stack.push(lhs * rhs),
                    "/" => stack.push(lhs / rhs),
                    "%" => stack.push(lhs % rhs),
                    _ => panic!("Uknown operator: {}", op),
                }
            }
            Ok(Token::EOL) => {
                match stack.len() {
                    1 => println!("{}", stack.pop().unwrap()),
                    0 => { },
                    _ => panic!("Invalid postfix expression!"),
                }
            }
            Ok(Token::EOF) => { break },
            Err(ref e) => { panic!("{}", e) },
        }
    }
}


