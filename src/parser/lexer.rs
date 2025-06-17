use std::{io::Bytes, iter::Peekable};

use crate::primitives::types::Type;

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

    // One or two char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    String(Vec<u8>),
    Float(f64),
    Integer(i64),
    Nil,
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
    Error(&'static str),
    Eof,
}

#[derive(Debug)]
pub struct Lexer<R: std::io::Read> {
    source: Peekable<Bytes<R>>,
    pub line: u32,
}

impl<R: std::io::Read> Lexer<R> {
    pub fn new(source: R) -> Self {
        Lexer {
            source: source.bytes().peekable(),
            line: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        let byt = self.read_byte();

        match byt {
            b' ' | b'\t' | b'\r' => self.next(),
            b'\n' => {
                self.line += 1;
                self.next()
            }
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
            b'/' => {
                if *self.peek_byte() == b'/' {
                    self.read_byte();
                    self.comment(false)
                } else if *self.peek_byte() == b'*' {
                    self.read_byte();
                    self.comment(true)
                } else {
                    Token::Slash
                }
            }
            b'0'..=b'9' => self.number(byt),
            b'\'' | b'"' => self.string(byt),
            b'A'..=b'Z' | b'a'..=b'z' => self.keyword(byt),
            b'\0' => Token::Eof,
            _ => Token::Error("Invalid Token"),
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
            None => b'\0',
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
        let n = self.peek_byte();

        match num {
            _ if *n == b'b' && num == b'0' => self.number_binary(),
            _ if *n == b'x' && num == b'0' => self.number_hex(),
            _ => {
                let mut result = u64::try_from(num - b'0').unwrap();

                loop {
                    if !(self.peek_byte().clone() as char).is_ascii_digit() {
                        break;
                    }

                    let n = self.next_byte().unwrap();

                    result = result
                        .checked_mul(10)
                        .expect("number overflow")
                        .checked_add((n - b'0') as u64)
                        .expect("cannot add {n} to {result}");
                }

                if *self.peek_byte() == b'.' {
                    self.read_byte();

                    return self.number_float(result);
                }

                Token::Integer(result as i64)
            }
        }
    }

    fn number_float(&mut self, first_half: u64) -> Token {
        let mut result = 0.0;
        let mut divisor = 1.0;

        loop {
            if !(self.peek_byte().clone() as char).is_ascii_digit() {
                break;
            }
            let ch = self.read_byte();

            result = result + ((ch - b'0') as f64) / (10.0 * divisor);
            divisor += 1.0;
        }

        Token::Float(first_half as f64 + result)
    }

    fn number_binary(&mut self) -> Token {
        let mut bnr: i64 = (self.read_byte() - b'0') as i64;
        if bnr != 1 && bnr != 0 { panic!("invalid binary number.") }

        loop {
            // binary number can be represented as 8chars_8chars_8chars.. in a maximum of 8 chunks (asterisk integer type is maximum 64bits long);
            for _ in 0..8 {
                // 2 is a escape for chars that are before 0 in ASCII table thus must be checked_sub.
                let ch = self.peek_byte().checked_sub(b'0').unwrap_or(2);
                if ch != 1 && ch != 0 { break; }
                self.read_byte();

                bnr <<= 1;
                bnr = bnr.checked_add(ch as i64).expect("binary overflow.");
            }

            dbg!(&bnr);
            dbg!(*self.peek_byte() as char);

            if *self.peek_byte() != b'_' { break; }
            self.read_byte();
        }
        dbg!("====", &bnr);

        Token::Integer(bnr)
    }

    fn number_hex(&mut self) -> Token {
        let mut hex: i64 = 0;

        match self.read_byte() - b'0' {
            b'A'..b'F' | b'0'..b'9' =>  {

            }
            _ => panic!("invalid hex value.")
        }

        let mut dnm: u8 = 0;

        loop {
            let ch = self.peek_byte();

            match ch {
                b'A'..b'F' | b'0'..b'9' => {

                }
                _ => break
            }
        }

        Token::Integer(hex)
    }

    fn string(&mut self, t: u8) -> Token {
        let mut str: Vec<u8> = Vec::new();

        loop {
            let mut ch = self.read_byte();

            match ch {
                // Skip escaped byte
                b'\\' => {
                    ch = self.read_byte();
                }
                _ if ch == t => { break }
                _ => ()
            }

            str.push(ch)
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
                _ => break,
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
            "Int" => Token::TypeDef(Type::Int),
            "Float" => Token::TypeDef(Type::Float),
            "Bool" => Token::TypeDef(Type::Bool),
            "String" => Token::TypeDef(Type::String),
            _ => Token::Identifier(word),
        }
    }

    fn comment(&mut self, multi: bool) -> Token {
        match multi {
            true => {
                while let Some(c) = self.next_byte() {
                    if c == b'\n' { self.line += 1 };

                    if c == b'*' {
                        let d = self.read_byte();

                        if d == b'/' {
                            break;
                        }
                    }
                }
            }
            false => {
                while let Some(c) = self.next_byte() {
                    if c == b'\n' {
                        self.line += 1;
                        break;
                    }
                }
            }
        }

        self.next()
    }

    /// FIXME
    pub fn curr_tok(&mut self) -> String {
        let mut word = String::new();

        loop {
            let ch = self.read_byte();
            if ch != b' ' {
                break;
            }

            /* \x1B[4m{}\x1B[0m */
            word.push_str(&format!("{}\u{0334}", ch as char) as &str);
        }

        word
    }

    fn error(&mut self) {

    }
}
