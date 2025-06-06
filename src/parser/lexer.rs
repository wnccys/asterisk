use std::{io::Bytes, iter::Peekable};

use crate::value::Type;

#[derive(Debug)]
pub struct Lexer<R: std::io::Read> {
    source: Peekable::<Bytes::<R>>,
    line: u16
}

impl<R: std::io::Read> Lexer<R> {
    fn new(source: R) -> Self {
        Lexer {
            source: source.bytes().peekable(),
            line: 1
        }
    }

    fn next(&mut self) -> Token {
        let byt = self.read_byte();

        match byt {
            b' ' | b'\t' | b'\r'  => self.next(),
            b'\n' => { self.line+=1; self.next() },
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b',' => Token::Comma,
            b'.' => Token::Dot,
            b'-' => Token::Minus,
            b'+' => Token::Plus,
            b':' => Token::Colon,
            b';' => Token::SemiColon,
            b'*' => Token::Star,
            b'&' => self.check_ahead(b'&', Token::Ampersand, Token::And),
            b'!' => self.check_ahead(b'=', Token::Bang, Token::BangEqual),
            b'=' => self.check_ahead(b'=', Token::Equal, Token::EqualEqual),
            b'>' => self.check_ahead(b'=', Token::Greater, Token::GreaterEqual),
            b'<' => self.check_ahead(b'=', Token::Less, Token::LessEqual),
            b'/' =>  {
                if self.peek_byte() == &b'/' {
                    self.read_byte();
                    self.comment(false)
                } else if self.peek_byte() == &b'*' {
                    self.read_byte();
                    self.comment(true)
                } else {
                    Token::Slash
                }
            },
            b'0'..=b'9' => self.number(byt),
            b'\'' | b'"' => self.string(byt),
            b'A'..=b'Z' | b'a'..=b'z' => self.keyword(byt),
            b'\0' => Token::Eof,
            _ => panic!("invalid token {}", byt as char)
        }
    }

    fn peek_byte(&mut self) -> &u8 {
        match self.source.peek() {
            Some(Ok(r)) => r,
            Some(_) => panic!("error on byte peek"),
            None => &b'\0',
        }
    }
    
    fn next_byte(&mut self) -> Option<u8> {
        self.source.next().and_then(|r| Some(r.unwrap()))
    }

    fn read_byte(&mut self) -> u8 {
        match self.source.next() {
            Some(Ok(ch)) => ch,
            Some(_) => panic!("error reading byte on 0 line {}", self.line),
            None => b'\0'
        }
    }

    fn check_ahead(&mut self, ahead: u8, short: Token, long: Token) -> Token {
        if *self.peek_byte() == ahead {
            long
        } else {
            short
        }
    }

    fn number(&mut self, num: u8) -> Token {
        match self.read_byte() {
            t if t == b'b' => {
                self.number_binary()
            }
            t if t == b'x' => {
                self.number_hex()
            }
            t if t.is_ascii_digit() => {
                let mut result = u64::try_from(num).unwrap();

                loop {
                    if !(self.peek_byte().clone() as char).is_ascii_digit() { break; }

                    let n = self.next_byte().unwrap();

                    result = result
                                .checked_mul(10).expect("number overflow")
                                .checked_add((n - b'0') as u64).expect("cannot add {n} to {result}");
                };

                if self.peek_byte() == &b'.' {
                    self.read_byte();

                    return self.number_float(result);
                }

                Token::Integer(result as i64)
            }
            _ => panic!("invalid number")
        }
    }

    fn number_float(&mut self, first_half: u64) -> Token {
        let mut result = 0.0;
        let mut divisor= 1.0;

        loop {
            if !(self.peek_byte().clone() as char).is_ascii_digit() { break; }
            let ch = self.read_byte();

            result = result + ((ch - b'0') as f64)/(10.0 * divisor);
            divisor += 1.0;
        }

        Token::Float(first_half as f64 + result)
    }

    fn number_binary(&mut self) -> Token {
        todo!();
    }

    fn number_hex(&mut self) -> Token {
        todo!();
    }

    fn string(&mut self, t: u8) -> Token {
        let mut str: Vec<u8> = Vec::new();

        loop {
            let ch = self.read_byte();
            if ch == t { break; }

            str.push(ch);
        }

        Token::String(str)
    }

    fn keyword(&mut self, ch: u8) -> Token {
        let mut word = String::new();
        word.push(ch as char);

        loop {
            match *self.peek_byte() as char {
                t if t.is_alphanumeric() || t == '_' => {
                    self.read_byte();
                    word.push(t);
                }
                _ => break
            }
        }

        match &word as &str {
            "and" => Token::And,
            "or" => Token::Or,
            "class" => Token::Class,
            "case" => Token::Case,
            "const" => Token::Const,
            "continue" => Token::Continue,
            "default" => Token::Default,
            "else" => Token::Else,
            "false" => Token::False,
            "for" => Token::For,
            "fn" => Token::Fun,
            "if" => Token::If,
            "mut" => Token::Modifier,
            "print" => Token::Print,
            "return" => Token::Return,
            "switch" => Token::Switch,
            "super" => Token::Super,
            "this" => Token::This,
            "let" => Token::Var,
            "while" => Token::While,
            _ => Token::Identifier,
        }
    }

    fn comment(&mut self, multi: bool) -> Token {
        match multi {
            true => {
                while let Some(c) = self.next_byte() {
                    if c == b'*' {
                        d = self.read_byte();

                        if d == b'\\' {
                            break;
                        }
                    }
                }
            }
            false => {
                while let Some(c) = self.next_byte() {
                    if n == b'\n' { break }
                }
            }
        }


        self.next()
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    SemiColon,
    Slash,
    Star,
    Ampersand,
    // One or two char token
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String(Vec<u8>),
    Float(f64),
    Integer(i64),
    // Keywords
    And,
    Class,
    Case,
    Const,
    Continue,
    Default,
    Else,
    False,
    For,
    Fun,
    If,
    Modifier,
    TypeDef(Type),
    Or,
    Print,
    Return,
    Switch,
    Super,
    This,
    True,
    Var,
    While,

    Comment,
    Error,
    Eof,
}
