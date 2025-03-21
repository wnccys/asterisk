use std::{collections::HashMap, slice::Iter, sync::LazyLock};

/// Token Stream created from Scanning Asterisk code.
/// 
pub type TokenStream<'a> = Iter<'a, Token<'a>>;

#[derive(Debug)]
#[allow(unused)]
/// Parse chars to Tokens.
/// 
pub struct Scanner<'a> {
    /// Transformed into Iterator after function execution
    pub token_stream: Vec<Token<'a>>,
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
            token_stream: vec![],
            // Current lexeme start
            lex_start: 0,
            // Current lexeme end
            lex_end: 0,
            line: 1,
            source_code,
        }
    }

    /// Scan Asterisk tokens crafting TokenStream from them.
    /// 
    pub fn scan(&mut self) -> TokenStream {
        for line in self.source_code
            .iter()
            .collect::<String>()
            .lines() 
            {
                let end_semi_c = line.ends_with(";");

                for _token in line.split(" ") {
                    let mut token = _token.to_owned();
                    if token.is_empty() { continue }

                    // TODO set lex_st/lex_ed handling

                    if end_semi_c {
                        token = token.replace(";", "");
                    }

                    // Shadow token to &str
                    let token = &token[..];

                    match token {
                        token if self.is_alphabetic(token) => self.make_token(*(KEYWORDS.get(token)).unwrap_or_else(|| &TokenCode::Identifier )),
                        token if self.is_numeric(token) => self.make_token(*(KEYWORDS.get(token)).unwrap_or_else(|| &TokenCode::Identifier )),
                        token if token.starts_with("\"") => self.string(token),
                        _ => self.make_token(*(KEYWORDS.get(token)).unwrap_or_default()),
                    };

                    // match &token[..] {
                    //     token if self.is_alphabetic(token) => self.make_token(*(KEYWORDS.get(token)).unwrap_or_else(|| &TokenCode::Identifier )),
                    //     token if self.is_numeric(token) => self.make_token(*(KEYWORDS.get(token)).unwrap_or_else(|| &TokenCode::Identifier )),
                    //     token if token.starts_with("\"") => self.string(token),
                    //     "(" => self.make_token(TokenCode::LeftParen),
                    //     ")" => self.make_token(TokenCode::RightParen),
                    //     "{" => self.make_token(TokenCode::LeftBrace),
                    //     "}" => self.make_token(TokenCode::RightBrace),
                    //     ";" => self.make_token(TokenCode::SemiColon),
                    //     "," => self.make_token(TokenCode::Comma),
                    //     "." => self.make_token(TokenCode::Dot),
                    //     "+" => self.make_token(TokenCode::Plus),
                    //     "-" => self.make_token(TokenCode::Minus),
                    //     "*" => self.make_token(TokenCode::Star),
                    //     "/" => self.make_token(TokenCode::Slash),
                    //     "//" => self.skip_comment(&token[..]),
                    //     "!" => self.make_token(TokenCode::Bang),
                    //     "!=" => self.make_token(TokenCode::BangEqual),
                    //     "=" => self.make_token(TokenCode::Equal),
                    //     "==" => self.make_token(TokenCode::EqualEqual),
                    //     "<" => self.make_token(TokenCode::Less),
                    //     "<=" => self.make_token(TokenCode::LessEqual),
                    //     ">" => self.make_token(TokenCode::Greater),
                    //     ">=" => self.make_token(TokenCode::GreaterEqual),
                    //     _ => self.make_token(TokenCode::Error("Invalid token."))
                    // };

                    if end_semi_c {
                        self.make_token(TokenCode::SemiColon);
                    }
                };
            }

        self.line += 1;
        todo!();
    }

    /// Match Number values for construct integer values and float values with ".".
    /// 
    fn is_numeric(&self, token: &str) -> bool {
        for char in token.chars() {
            if !char.is_ascii_digit() && char != '.' {
                return false;
            }
        }

        return true;
     }

    /// Identifiers can't contain numbers, so it must be alphabetic-only.
    /// 
    fn is_alphabetic(&self, token: &str) -> bool {
        for char in token.chars() {
            if !char.is_alphanumeric() {
                return false;
            }
        }

        return true;
     }

    //  fn check_keywords(&mut self, token: &str) -> Token {
    //     KEYWORDS.get(token).unwrap_or(self.make_token(TokenCode::Identifier))
    //  }

     fn skip_comment(&mut self, token: &str) {
        todo!()
     }

    fn string(&self, token: &str) {
        todo!()
     }

    /// Craft Token from TokenCode handling TokenCode::Error internally.
    /// 
    pub fn make_token(&mut self, token_code: TokenCode) {
        if let TokenCode::Error(msg) = token_code {
            println!("{}", msg);

            self.token_stream.push(Token {
                code: TokenCode::Error(msg),
                lexeme: &self.source_code[self.lex_start..self.lex_end],
                line: self.line,
            });

            return;
        }

        self.token_stream.push(Token {
            code: token_code,
            lexeme: &self.source_code[self.lex_start..self.lex_end],
            line: self.line,
        })
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

    Error(&'static str),
    Eof,
    Comment,
}

impl Default for &TokenCode {
    fn default() -> Self {
        &TokenCode::Error("Invalid token.")
    }
}

static KEYWORDS: LazyLock<HashMap<&'static str, TokenCode>> = LazyLock::new(|| {
    let mut map: HashMap<&'static str, TokenCode> = HashMap::new();
    map.insert("and", TokenCode::And);
    map.insert("and", TokenCode::And);
    map.insert("const", TokenCode::Const);
    map.insert("class", TokenCode::Class);
    map.insert("else", TokenCode::Else);
    map.insert("false", TokenCode::False);
    map.insert("for", TokenCode::For);
    map.insert("fn", TokenCode::Fun);
    map.insert("if", TokenCode::If);
    map.insert("nil", TokenCode::Nil);
    map.insert("or", TokenCode::Or);
    map.insert("print", TokenCode::Print);
    map.insert("return", TokenCode::Return);
    map.insert("super", TokenCode::Super);
    map.insert("this", TokenCode::This);
    map.insert("true", TokenCode::True);
    map.insert("let", TokenCode::Var); // "let" was checked as "l" + "et"
    map.insert("while", TokenCode::While);
    map
});