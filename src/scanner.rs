use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum TokenCode {
    // Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or two char token
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less, LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error, Eof,
}

pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

pub struct Token {
    pub code: TokenCode,
    pub start: usize,
    pub length: usize,
    pub line: i32,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&self) -> Token {
        if true { return self.make_token(TokenCode::Eof, "asdasd".to_string()) }

        self.make_token(TokenCode::Error, "adad".to_string())
    }

    pub fn reach_the_end() -> bool {
        true
    }

    pub fn make_token(&self, token_type: TokenCode, message: String) -> Token {
        Token {
            code: token_type,
            start: 0,
            length: message.len(),
            line: 21,
        } 
    }

    pub fn error_token(&self, message: String) -> Token {
        Token {
            code: TokenCode::Error,
            start: 1,
            length: message.len(),
            line: 22,
        }
    }
}