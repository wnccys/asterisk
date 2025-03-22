use std::{collections::HashMap, slice::Iter, sync::LazyLock};

/// Token Stream created from Scanning Asterisk code.
/// 
pub type TokenStream<'a> = Iter<'a, Token>;

#[derive(Debug)]
#[allow(unused)]
/// Parse chars to Tokens.
/// 
pub struct Scanner<'a> {
    /// Transformed into Iterator after function execution
    pub token_stream: Vec<Token>,
    pub source_code: &'a Vec<char>,
    current: Option<String>,
    pub line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a Vec<char>) -> Self {
        Scanner {
            token_stream: vec![],
            current: None,
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
				/* 
					Check for ";" at the end of line
					This is needed because last split token may have ; on it;
					This is also the why TokenCode::Comment is not in KEYWORDS,
					it is handled here, separatelly.
				*/
                let end_semi_c = line.ends_with(";");

                for _token in line.split(" ") {
                    let mut token = _token.to_owned();

                    if end_semi_c {
                        token = token.replace(";", "");
                    }

                    dbg!("START DEBUG ====");
                    /* Shadow token to &str so we can get it corresponding TokenCode as 'static in LazyLock */
                    let token = &token[..];
                    self.current = Some(token.to_owned());
                    println!("SELF CURRENT: {:?}", self.current);

                    match token {
						/* Handle keywords and generate identifier token if no was found between the keywords */
                        token if self.is_alphabetic(token) => self.make_token(*(KEYWORDS.get(token)).unwrap_or_else(|| &TokenCode::Identifier )),
						/* */
                        token if self.is_numeric(token) => self.make_token(*(KEYWORDS.get(token)).unwrap_or_else(|| &TokenCode::Error("Invalid numeric token."))),
						/* // comment handling */
						token if token.starts_with("\\\\") => self.make_token(*(KEYWORDS.get("\\\\")).unwrap_or_else(|| &TokenCode::Error("Invalid comment token.") )),
                        token if token.starts_with("\"") => self.string(token),
                        _ => self.make_token(*(KEYWORDS.get(token)).unwrap_or_default()),
                    };
                };

                if end_semi_c {
                    self.current = Some(String::from(";"));
                    self.make_token(TokenCode::SemiColon);
                }
            }

        self.line += 1;

        for i in 0..self.token_stream.len() {
            let token = &self.token_stream[i];

            if token.code == TokenCode::Comment { self.token_stream.remove(i); }
        }

        self.token_stream.iter()

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

    fn string(&mut self, token: &str) {
        if token.ends_with("\"") {
            self.make_token(TokenCode::String);
        };

        self.make_token(TokenCode::Error("Invalid string token."));
     }

    /// Craft Token from TokenCode handling TokenCode::Error internally.
    /// 
    pub fn make_token(&mut self, token_code: TokenCode) {
        dbg!(&self.current);

        let lexeme = self.current.take();

        dbg!(&lexeme);
        dbg!(&token_code);

        if let TokenCode::Error(msg) = token_code {
            println!("{}", msg);

            self.token_stream.push(Token {
                code: TokenCode::Error(msg),
                lexeme: lexeme.unwrap(),
                line: self.line,
            });

            return;
        }

        self.token_stream.push(Token {
            code: token_code,
            lexeme: lexeme.unwrap(),
            line: self.line,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Token {
    pub code: TokenCode,
    pub lexeme: String,
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
    /// Fallback for generic error on Token scanning.
    fn default() -> Self {
        &TokenCode::Error("Invalid token.")
    }
}

static KEYWORDS: LazyLock<HashMap<&'static str, TokenCode>> = LazyLock::new(|| {
    let mut map: HashMap<&'static str, TokenCode> = HashMap::new();

    // Keywords
    map.insert("and", TokenCode::And);
    map.insert("const", TokenCode::Const);
    map.insert("class", TokenCode::Class);
    map.insert("else", TokenCode::Else);
    map.insert("false", TokenCode::False);
    map.insert("for", TokenCode::For);
    map.insert("fn", TokenCode::Fun);
    map.insert("if", TokenCode::If);
    map.insert("mut", TokenCode::Modifier);
    map.insert("nil", TokenCode::Nil);
    map.insert("or", TokenCode::Or);
    map.insert("print", TokenCode::Print);
    map.insert("return", TokenCode::Return);
    map.insert("super", TokenCode::Super);
    map.insert("this", TokenCode::This);
    map.insert("true", TokenCode::True);
    map.insert("let", TokenCode::Var);
    map.insert("while", TokenCode::While);

    // Punctuation and operators
    map.insert("(", TokenCode::LeftParen);
    map.insert(")", TokenCode::RightParen);
    map.insert("{", TokenCode::LeftBrace);
    map.insert("}", TokenCode::RightBrace);
    // This one is automatically handled by scanner.
    // map.insert(";", TokenCode::SemiColon);
    map.insert(",", TokenCode::Comma);
    map.insert(".", TokenCode::Dot);
    map.insert("+", TokenCode::Plus);
    map.insert("-", TokenCode::Minus);
    map.insert("*", TokenCode::Star);
    map.insert("/", TokenCode::Slash);
    map.insert("//", TokenCode::Comment);
    map.insert("!", TokenCode::Bang);
    map.insert("!=", TokenCode::BangEqual);
    map.insert("=", TokenCode::Equal);
    map.insert("==", TokenCode::EqualEqual);
    map.insert("<", TokenCode::Less);
    map.insert("<=", TokenCode::LessEqual);
    map.insert(">", TokenCode::Greater);
    map.insert(">=", TokenCode::GreaterEqual);

    map
});