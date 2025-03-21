use std::slice::Iter;

/// Token Stream created from Scanning Asterisk code.
/// 
pub type TokenStream<'a> = Iter<'a, Token<'a>>;

#[derive(Debug)]
#[allow(unused)]
/// Parse chars to Tokens.
/// 
pub struct Scanner<'a> {
    pub source_code: &'a Vec<char>,
    /// Lexeme start
    pub lex_start: usize,
    /// Lexeme end
    pub lex_end: usize,
    pub line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a Vec<char>) -> Self {
        Scanner {
            lex_start: 0,
            lex_end: 0,
            line: 1,
            source_code,
        }
    }

    pub fn scan(&mut self) -> TokenStream {
        for line in self.source_code.iter().collect::<String>().lines() {
                let end_semi_c = line.ends_with(";");

                for _token in line.split(" ") {
                    let mut token = _token.to_owned();

                    if end_semi_c {
                        token = token.replace(";", "");
                    }

                    println!("{:?}", token);

                    if end_semi_c {
                        self.make_token(TokenCode::SemiColon);
                    }
                };

                println!("EOL");
                self.line += 1;
        }

        todo!()
    }

    /// Craft Token from TokenCode.
    /// 
    pub fn make_token(&self, token_code: TokenCode) -> Token<'a> {
        Token {
            code: token_code,
            lexeme: &self.source_code[self.lex_start..self.lex_end],
            line: self.line,
        }
    }

    /// Print message returning error token.
    /// 
    pub fn error_token(&self, message: &str) -> Token {
        println!("{}", message);

        Token {
            code: TokenCode::Error,
            lexeme: &self.source_code[self.lex_start..self.lex_end],
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
    Modifier,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
    Comment,
}