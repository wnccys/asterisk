use std::{
    collections::HashMap,
    iter::Peekable,
    slice::Iter,
    str::Lines,
    sync::LazyLock,
};

/// Token Stream created from Scanning Asterisk code.
///
pub type TokenStream<'a> = Iter<'a, Token>;

#[derive(Debug)]
struct TokenIterator<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> TokenIterator<'a> {
    fn new(s: &'a str) -> Self {
        TokenIterator { s, pos: 0 }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        /* Skip whitespaces before some token char are found */
        while self.pos < self.s.len() && self.s.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }

        /* Source end reached */
        if self.pos >= self.s.len() {
            return None;
        }

        let start = self.pos;

        /* Skip all whitespace parsing while between a string ("SKIPPED CHARS") */
        if self.s.as_bytes()[self.pos] == b'"' {
            self.pos += 1;

            /* While line end is not reached, skip chars by incrementing counter */
            while self.pos <= self.s.len() && self.s.as_bytes()[self.pos] != b'"' {
                self.pos += 1;
            }

            /* Advance " in final parsed string */
            if self.pos < self.s.len() {
                self.pos += 1;
            }
        } else {
            /* Advance until a whitespace is found */
            while self.pos < self.s.len() && !self.s.as_bytes()[self.pos].is_ascii_whitespace() {
                /*
                    This makes the validation for tokens which ends with ';'.
                    It prevents the current token to be increased when in pos,
                    making the scanner correctly pass the ';' token to the next iteration.
                    this would cause, for example let a = 32; to pos be in whitespace in the final of loop,
                    invalidating the ';' semicolon token, this way, when a token has ; on final,
                    as the Some(..) return on function's final are not inclusive, it return the correct stripped token,
                    the condition start != self.pos validate that ';' correct match, without this, when pure ';' token
                    are scanned, it would restart the while loop, causing a infinite loop.
                */
                if self.s.as_bytes()[self.pos] == b';' && start != self.pos {
                    break;
                }

                self.pos += 1;
            }
        }

        #[cfg(feature = "debug-scan")]
        dbg!(&self.s[start..self.pos]);
        Some(&self.s[start..self.pos])
    }
}

#[derive(Debug)]
#[allow(unused)]
/// Parse chars to Tokens.
///
pub struct Scanner<'a> {
    /// Transformed into Iterator after function execution
    pub token_stream: Vec<Token>,
    // Iterator over source code lines
    lines: &'a mut Lines<'a>,
    // Current line index
    pub line: i32,
    tokens: Option<Peekable<TokenIterator<'a>>>,
    current_token: Option<String>,
}

impl<'a> Scanner<'a> {
    pub fn new(lines: &'a mut Lines<'a>) -> Self {
        Scanner {
            token_stream: vec![],
            lines,
            tokens: None,
            current_token: None,
            line: 1,
        }
    }

    /// Scan Asterisk tokens crafting TokenStream from them.
    ///
    pub fn scan(&mut self) -> TokenStream {
        self.scan_l();

        self.token_stream.iter()
    }

    /// Scan Asterisk code lines parsing it's tokens recursivelly.
    ///
    fn scan_l(&mut self) {
        let line = self.lines.next();
        /* Recursion base case */
        if line.is_none() {
            return;
        }

        /* Skip line parsing when comment are found */
        if line.unwrap().starts_with("//") {
            self.scan_l();
            self.line += 1;
            return;
        }

        /* Get new line iterator over the line tokens on each scan_l() call  */
        self.tokens = Some(TokenIterator::new(line.unwrap()).peekable());

        /* While tokens are available, iterates. */
        while self.tokens.as_mut().unwrap().peek().is_some() {
            self.advance_token();
            self.parse_tokens();
        }

        self.line += 1;

        /* Iterates over line recursivelly */
        self.scan_l()
    }

    fn parse_tokens(&mut self) {
        /* ; on token final handling, also check if it is the semicolon token itself */
        if self.current_token.as_ref().unwrap().ends_with(";")
            && self.current_token.as_ref().unwrap().len() > 1
        {
            self.current_token = Some(self.current_token.take().unwrap().replace(";", ""))
        };

        /*
            This bind is necessary because inner functions can advance tokens pointer and current_token.
            An example is the self.string() call which tries to match a " to craft token.
            Also, to be used as KEYWORDS key.
        */
        let token = &self.current_token.clone().unwrap()[..];

        if token.is_empty() {
            return;
        }

        match token {
            /* Handle keywords and generate identifier token if no was found between the keywords */
            token if self.is_alphabetic(token) => {
                self.make_token(*(KEYWORDS.get(token)).unwrap_or_else(|| &TokenCode::Identifier))
            }
            /* Numeric values handling (Int, Float) */
            token if self.is_numeric(token) => self.make_token(
                *(KEYWORDS.get("number"))
                    .unwrap_or_else(|| &TokenCode::Error("Invalid numeric token.")),
            ),
            /* String handling */
            token if token.starts_with("\"") => self.string(token),
            _ => self.make_token(*(KEYWORDS.get(token)).unwrap_or_default()),
        };
    }

    /// Advance token by advancing tokens Iter pointer.
    /// This keeps tokens and current sync.
    ///
    fn advance_token(&mut self) {
        if self.tokens.as_mut().unwrap().peek().is_none() {
            panic!("Error advancing token: EOF reached.")
        }

        self.current_token = Some(self.tokens.as_mut().unwrap().next().unwrap().to_owned());
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
            if !char.is_alphabetic() {
                return false;
            }
        }

        return true;
    }

    /// Craft string tokens iterating until find or not (emit error) " string terminator.
    ///
    fn string(&mut self, token: &str) {
        if self.current_token.as_ref().unwrap().ends_with("\"") {
            /* Strip "" string starter and terminator */
            self.current_token = Some(token[1..(token.len() - 1)].to_owned());
            self.make_token(TokenCode::String);

            return;
        }

        self.make_token(TokenCode::Error("Invalid string token."));
    }

    /// Craft Token from TokenCode handling TokenCode::Error internally.
    ///
    pub fn make_token(&mut self, token_code: TokenCode) {
        let lexeme = self.current_token.take();

        #[cfg(feature = "debug-scan")]
        dbg!(&lexeme);

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
    map.insert("number", TokenCode::Number);
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
    map.insert(";", TokenCode::SemiColon);
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

    // General compiler track
    map.insert("EOF", TokenCode::Eof);

    map
});
