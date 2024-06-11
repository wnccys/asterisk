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

    pub fn scan_token(&mut self, source: &String) -> Token {
        self.start = self.current;
        if self.reach_source_end(source) { return self.make_token(TokenCode::Eof) }
        
        let chars: Vec<char> = source
                                .chars()
                                .collect();
        self.current += 1;

        match chars[self.current-1] {
            '(' => self.make_token(TokenCode::LeftParen),
            ')' => self.make_token(TokenCode::RightParen),
            '{' => self.make_token(TokenCode::LeftBrace),
            '}' => self.make_token(TokenCode::RightBrace),
            ';' => self.make_token(TokenCode::SemiColon),
            ',' => self.make_token(TokenCode::Comma),
            '.' => self.make_token(TokenCode::Dot),
            '+' => self.make_token(TokenCode::Plus),
            '-' => self.make_token(TokenCode::Minus),
            '*' => self.make_token(TokenCode::Star),
            '/' => self.make_token(TokenCode::Slash),
            '!' => if !self.reach_source_end(source) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::BangEqual) } 
                    else { self.make_token(TokenCode::Bang) },
            '=' => if !self.reach_source_end(source) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::EqualEqual) } 
                    else { self.make_token(TokenCode::Equal) }
            '<' => if !self.reach_source_end(source) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::LessEqual) } 
                    else { self.make_token(TokenCode::Less) }
            '>' => if !self.reach_source_end(source) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::GreaterEqual) } 
                    else { self.make_token(TokenCode::Greater) }
            _ => self.error_token(), 
        }
    }

    fn reach_source_end(&self, source: &String) -> bool {
        self.current == source.len()
    }

    pub fn make_token(&self, token_code: TokenCode) -> Token {
        Token {
            code: token_code,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        } 
    }

    // TODO set proper token info
    pub fn error_token(&self) -> Token {
        Token {
            code: TokenCode::Error,
            start: todo!(),
            length: todo!(),
            line: 22,
        }
    }
}