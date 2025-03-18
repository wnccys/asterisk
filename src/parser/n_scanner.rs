use std::slice::Iter;

pub type TokenStream<'a> = Iter<'a, Token<'a>>;

#[derive(Debug, Default)]
#[allow(unused)]
/// Parse chars to Tokens.
/// 
pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

impl Scanner {
    pub fn scan<'a>(source_code: &'a Vec<char>) -> TokenStream<'a> {
        for token in source_code.iter().collect::<String>().split(' ') {
            println!("{token}");
        }

        todo!()
    }

    /// Craft Token from TokenCode.
    /// 
    pub fn make_token<'a>(&self, token_code: TokenCode, source_code: &'a Vec<char>) -> Token<'a> {
        Token {
            code: token_code,
            lexeme: &source_code[self.current..self.start],
            line: self.line,
        }
    }

    /// Print message returning error token.
    /// 
    pub fn error_token<'a>(&self, message: &str, source_code: &'a Vec<char>) -> Token<'a> {
        println!("{}", message);

        Token {
            code: TokenCode::Error,
            lexeme: &source_code[self.current..self.start],
            line: self.line,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Token<'a> {
    pub code: TokenCode,
    pub lexeme: &'a [char],
    // pub start: usize,
    // pub length: usize,
    pub line: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(unused)]
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
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Const,
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
    VarMut,
    While,

    Error,
    Eof,
    Comment,
}