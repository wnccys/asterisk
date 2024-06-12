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

    Error, Eof, Comment
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

    pub fn scan_token(&mut self, chars: &Vec<char>) -> Token {
        self.start = self.current;
        if self.reach_source_end(chars) { return self.make_token(TokenCode::Eof) }
        
        self.current += 1;
        if chars[self.current-1].is_digit(10) { return self.number(chars); }

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
            '/' => if !self.reach_source_end(chars) && chars[self.current] == '/' 
                    { self.current+=1; self.skip_comment(chars) } 
                    else { self.make_token(TokenCode::Slash) },
            '!' => if !self.reach_source_end(chars) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::BangEqual) } 
                    else { self.make_token(TokenCode::Bang) },
            '=' => if !self.reach_source_end(chars) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::EqualEqual) } 
                    else { self.make_token(TokenCode::Equal) }
            '<' => if !self.reach_source_end(chars) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::LessEqual) } 
                    else { self.make_token(TokenCode::Less) }
            '>' => if !self.reach_source_end(chars) && chars[self.current] == '='
                    { self.current+=1; self.make_token(TokenCode::GreaterEqual) } 
                    else { self.make_token(TokenCode::Greater) }
            '"' => self.string(chars),
            _ => self.error_token("not implemented yet."), 
        }
    }

    fn number(&mut self, chars: &Vec<char>) -> Token {
        while !self.reach_source_end(chars) && chars[self.current].is_digit(10) { self.current+=1 }

        if !self.reach_source_end(chars) && chars[self.current] == '.' {
            self.current+=1;
            if !self.reach_source_end(chars) && chars[self.current].is_digit(10) {
                while !self.reach_source_end(chars) && chars[self.current].is_digit(10) { self.current+=1 };
            }
        } 

        self.make_token(TokenCode::Number)
    }

    fn string(&mut self, chars: &Vec<char>) -> Token {
        while !self.reach_source_end(chars) && chars[self.current] != '"'
            { if chars[self.current] == '\n' { self.line+=1; }; 
            self.current+=1; }
            

        if self.reach_source_end(chars) && chars[self.current-1] != '"' 
            { return self.error_token("unterminated string.") };

        self.current+=1;
        self.make_token(TokenCode::String)
    }

    fn skip_comment(&mut self, chars: &Vec<char>) -> Token {
        while !self.reach_source_end(chars) && chars[self.current] != '\n' 
            { self.current+=1; } 
        
        self.make_token(TokenCode::Comment)
    }

    fn reach_source_end(&self, chars: &Vec<char>) -> bool {
        self.current == chars.len()
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
    pub fn error_token(&self, message: &str) -> Token {
        Token {
            code: TokenCode::Error,
            start: self.current - self.start,
            length: message.len(),
            line: self.line,
        }
    }
}