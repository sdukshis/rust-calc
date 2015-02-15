use std::io;
use std::fmt;

#[derive(Show, PartialEq)]
pub enum Token {
    BinOp(&'static str),
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

pub struct Scanner<R: io::Reader> {
    curr_char: io::IoResult<char>,
    reader: R,
}

impl<R: io::Reader> Scanner<R> {
    pub fn new(r: R) -> Scanner<R> {
        let mut scanner = Scanner {
            curr_char: Result::Ok(' '),
            reader: r,
        };
        scanner.get_char();
        return scanner;
    }

    pub fn get_token(&mut self) -> Result<Token, &'static str> {
        match self.curr_char {
            Ok(' ') => { self.get_char(); self.get_token() },
            Ok('+') => { self.get_char(); Ok(Token::BinOp("+")) },
            Ok('-') => { self.get_char(); Ok(Token::BinOp("-")) },
            Ok('*') => { self.get_char(); Ok(Token::BinOp("*")) },
            Ok('/') => { self.get_char(); Ok(Token::BinOp("/")) },
            Ok('%') => { self.get_char(); Ok(Token::BinOp("%")) },
            Ok('\n') => { self.curr_char = Ok(' '); Ok(Token::EOL) },
            Ok(c) if c.is_digit(10) => Ok(Token::Number(self.get_number())),
            Ok(_) => { self.get_char(); Err("Unknown symbol") },
            Err(ref e) => { Ok(Token::EOF) },
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
            Ok(c) => {debug!("curr char: {}", c); Ok(c as char) },
            Err(e) => Err(e),
        };
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
fn add_token() {
    let mut scanner = Scanner::new(MockReader{ch: '+'});
    let token = scanner.get_token();
    assert_eq!(token, Ok(Token::BinOp("+")));
}