#[macro_use] extern crate log;
use std::io;
use std::fmt;

#[derive(Show, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::String for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::Add => write!(f, "Add"),
            Op::Sub => write!(f, "Sub"),
            Op::Mul => write!(f, "Mul"),
            Op::Div => write!(f, "Div"),
        }
    }
}

#[derive(Show, PartialEq)]
enum Token {
    BinOp(Op),
    Number(i64),
    EOL,
    EOF,
}

impl fmt::String for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::BinOp(ref op) => write!(f, "BinOp: {}", op),
            Token::Number(n) => write!(f, "Number: {}", n),
            Token::EOF => write!(f, "EOF"),
            Token::EOL => write!(f, "EOL"),
        }        
    }
}

struct Lexer<R: io::Reader> {
    curr_char: io::IoResult<char>,
    reader: R,
}

impl<R: io::Reader> Lexer<R> {
    fn new(r: R) -> Lexer<R> {
        let mut lexer = Lexer {
            curr_char: Result::Ok(' '),
            reader: r,
        };
        lexer.get_char();
        return lexer;
    }

    fn get_token(&mut self) -> Result<Token, &'static str> {
        match self.curr_char {
            Ok(' ') => { self.get_char(); self.get_token() },
            Ok('+') => { self.get_char(); Ok(Token::BinOp(Op::Add)) },
            Ok('-') => { self.get_char(); Ok(Token::BinOp(Op::Sub)) },
            Ok('*') => { self.get_char(); Ok(Token::BinOp(Op::Mul)) },
            Ok('/') => { self.get_char(); Ok(Token::BinOp(Op::Div)) },
            Ok('\n') => { self.get_char(); Ok(Token::EOL) },
            Ok(c) if c.is_digit(10) => Ok(Token::Number(self.get_number())),
            Ok(c) => { self.get_char(); Err("Unknown symbol") },
            Err(..) => Ok(Token::EOF),
        }
    }

    fn get_number(&mut self) -> i64 {
        let mut number = 0i64;
        loop {
            match self.curr_char {
                Ok(c) if c.is_digit(10) => { 
                    number = 10 * number + c as i64 - '0' as i64
                },
                _ => { return number },
            }
            self.get_char();
        }
    }

    fn get_char(&mut self) {
        self.curr_char = match self.reader.read_byte() {
            Ok(c) => Ok(c as char),
            Err(e) => Err(e),
        };
        debug!("curr_char: {}", self.curr_char.as_ref().unwrap());
    }
}

fn main() {
    let mut lexer = Lexer::new(io::stdin());
    let mut stack: Vec<i64> = Vec::new();
    loop {
        let token = lexer.get_token();
        println!("{}", token.as_ref().unwrap());
        match token {
            Ok(Token::Number(n)) => stack.push(n),
            Ok(Token::BinOp(op)) => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match op {
                    Op::Add => stack.push(lhs + rhs),
                    Op::Sub => stack.push(lhs - rhs),
                    Op::Mul => stack.push(lhs * rhs),
                    Op::Div => stack.push(lhs / rhs),
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

struct MockReader {
    ch: char
}

impl Reader for MockReader {
    fn read_byte(&mut self) -> io::IoResult<u8> {
        Ok('+' as u8)
    }
    fn read(&mut self, buf: &mut [u8]) -> io::IoResult<usize> {
        Ok(0)
    }
}

#[test]
fn test_add_token() {
    let mut lexer = Lexer::new(MockReader{ch: '+'});
    let token = lexer.get_token();
    assert_eq!(token, Ok(Token::BinOp(Op::Add)));
}
