use std::{
    collections::HashMap, iter::Peekable, slice::Iter, str::Lines, sync::LazyLock,
};

use crate::value::Type;

/// Token Stream created from Scanning Asterisk code.
///
pub type TokenStream<'a> = Iter<'a, Token>;

pub static TYPE_KEYS: [&str; 5] = ["Int", "Float", "String", "Bool", "Void"];

#[derive(Debug)]
struct TokenIterator<'a> {
    s: &'a str,
    pos: usize,
    pub comment_mode: bool,
}

impl<'a> TokenIterator<'a> {
    fn new(s: &'a str) -> Self {
        TokenIterator { s, pos: 0, comment_mode: false }
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
        /*
            If token starts with &, verify for TYPES, the differences between the result is that when the keyword is not present,
            it means & is a reference to something, and not a type itself, so we emit a TokenCode::Ampersand to tell parser it is a reference value instead of a reference type.
        */
        } else if self.s.as_bytes()[self.pos] == b'&' {
            /* && handling */
            if self.s.as_bytes()[self.pos + 1] == b'&' {
                self.pos += 1;

                return Some(&self.s[start..=self.pos]);
            }

            /* While we don't found a supported type, iterates */
            while 
                !self.s.as_bytes()[self.pos].is_ascii_whitespace() 
                && self.pos < self.s.len() -1 
            {
                self.pos += 1;

                if TYPE_KEYS.contains(&&self.s[start + 1..self.pos]) {
                    return Some(&self.s[start..self.pos]);
                }
            }

            /* Isolate '&' and pass iteration to the next chars, Ex. &"name" => [TokenCode::Ampersand, TokenCode::String ] */
            self.pos = start + 1;
        } else {
            /* Advance until a whitespace is found */
            while self.pos < self.s.len() && !self.s.as_bytes()[self.pos].is_ascii_whitespace() {
                /*
                    This makes the validation for tokens which ends with ';' or other single characters.
                    It prevents the current token to be increased when in [pos],
                    making the scanner correctly pass the ';' token, for example, to the next iteration.
                    this would cause, for example let a = 32; to pos be in whitespace in the final of loop,
                    invalidating the ';' semicolon token, this way, when a token has ; on final,
                    as the Some(..) return on function's final are not inclusive, it return the correct stripped token,
                    the condition start != self.pos validate that ';' correct match, without this, when pure ';' token
                    are scanned, it would restart the while loop, causing a infinite loop.
                */

                /* (CONDITION) handling */
                if self.s.as_bytes()[self.pos] == b'(' {
                    self.pos += 1;
                    break;
                }

                if self.s.as_bytes()[self.pos] == b')' && start != self.pos {
                    break;
                }

                /* '/' or '/ *' handling */
                if self.s.as_bytes()[self.pos] == b'/' {
                    /* Isolate / to the next iteration */
                    if start != self.pos {
                        return Some(&self.s[start..self.pos]);
                    /* Skip entire rest of line when // is found */
                    } else if self.pos + 1 < self.s.len() && self.s.as_bytes()[self.pos + 1] == b'/' {
                        self.pos = self.s.len();
                        break;
                    /* Isolated / handling */
                    } else {
                        self.pos += 1;
                        if self.s.as_bytes()[self.pos] == b'*' { self.pos += 1 }

                        return Some(&self.s[start..self.pos]);
                    }
                }

                /* '* /' and '*' handling, casting it to the next iteration */
                if self.s.as_bytes()[self.pos] == b'*' && start != self.pos {
                    if self.s.as_bytes()[self.pos] == b'/' {
                        self.pos += 1;
                    }

                    break
                }

                if self.s.as_bytes()[self.pos] == b'*' {
                    self.pos += 1;

                    if self.s.as_bytes()[self.pos] == b'/' {
                        self.pos += 1;
                    }

                    break
                }

                if self.s.as_bytes()[self.pos] == b';' && start != self.pos {
                    break;
                }

                /* Handling for isolating ; on start of word */
                if self.s.as_bytes()[self.pos] == b';' {
                    self.pos += 1;
                    return Some(&self.s[start..self.pos])
                }

                /* Send : token to the next iteration */
                if self.s.as_bytes()[self.pos] == b':' && start != self.pos {
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
    comment_mode: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(lines: &'a mut Lines<'a>) -> Self {
        Scanner {
            token_stream: vec![],
            lines,
            tokens: None,
            current_token: None,
            comment_mode: false,
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

        /* Get new line iterator over the line's tokens on each scan_l() call */
        self.tokens = Some(TokenIterator::new(line.unwrap()).peekable());

        /* While tokens are available on source code line TokenIterator, iterates. */
        while self.tokens.as_mut().unwrap().peek().is_some() {
            self.advance_token();
            /* Do not parse on comment mode */
            if !self.comment_mode && self.current_token.as_ref().unwrap() != "*/" { self.parse_tokens(); }
        }

        self.line += 1;

        /* Iterates over lines recursivelly */
        self.scan_l()
    }

    fn parse_tokens(&mut self) {
        /*
            This bind is necessary because inner functions can advance tokens pointer and current_token.
            An example is the self.string() call which tries to match a " to craft token.
            Also, to be used as KEYWORDS key.
        */
        let token = &self.current_token.clone().unwrap()[..];

        if token.is_empty() || token.starts_with("//") {
            return;
        }

        match token {
            /* Handle keywords and generate identifier token if no was found between the keywords */
            token if self.is_typedef(token) => self.make_token(
                (KEYWORDS.get(token))
                    .unwrap_or(&TokenCode::Error("Invalid Type Token."))
                    .clone(),
            ),
            token if self.is_alphabetic(token) => {
                /*  Identifier is kinda a fallback for when no keyword is match */
                self.make_token(
                    (KEYWORDS.get(token))
                        .unwrap_or(&TokenCode::Identifier)
                        .clone(),
                )
            }
            /* Numeric values handling (Int, Float) */
            token if self.is_numeric(token) => self.make_token(
                (KEYWORDS.get("number"))
                    .unwrap_or(&TokenCode::Error("Invalid numeric token."))
                    .clone(),
            ),
            /* String handling */
            token if token.starts_with("\"") => self.string(token),
            _ => self.make_token((KEYWORDS.get(token)).unwrap_or_default().clone()),
        };
    }

    /// Advance token by advancing tokens Iter pointer (calls next()).
    /// This keeps tokens and current sync.
    ///
    fn advance_token(&mut self) {
        if self.tokens.as_mut().unwrap().peek().is_none() {
            panic!("Error advancing token: EOF reached.")
        }

        self.current_token = Some(self.tokens.as_mut().unwrap().next().unwrap().to_owned());

        if self.current_token.as_ref().unwrap() == "/*" { self.comment_mode = true };
        if self.current_token.as_ref().unwrap() == "*/" { self.comment_mode = false };
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

    /// Check for a type and it's ref equivalent
    ///
    fn is_typedef(&self, token: &str) -> bool {
        if TYPE_KEYS.contains(&token) || (token.starts_with("&") && token.len() > 1) {
            return true;
        }

        false
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

#[derive(Debug, PartialEq, Clone)]
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
    String,
    Number,
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
    Nil,
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

pub const KEYWORDS: LazyLock<HashMap<&'static str, TokenCode>> = LazyLock::new(|| {
    let mut map: HashMap<&'static str, TokenCode> = HashMap::new();

    // Keywords
    map.insert("and", TokenCode::And);
    map.insert("&&", TokenCode::And);
    map.insert("class", TokenCode::Class);
    map.insert("case", TokenCode::Case);
    map.insert("const", TokenCode::Const);
    map.insert("continue", TokenCode::Continue);
    map.insert("default", TokenCode::Default);
    map.insert("else", TokenCode::Else);
    map.insert("false", TokenCode::False);
    map.insert("for", TokenCode::For);
    map.insert("fn", TokenCode::Fun);
    map.insert("if", TokenCode::If);
    map.insert("mut", TokenCode::Modifier);
    map.insert("number", TokenCode::Number);
    map.insert("nil", TokenCode::Nil);
    map.insert("or", TokenCode::Or);
    map.insert("||", TokenCode::Or);
    map.insert("print", TokenCode::Print);
    map.insert("return", TokenCode::Return);
    map.insert("switch", TokenCode::Switch);
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
    map.insert(":", TokenCode::Colon);
    map.insert(",", TokenCode::Comma);
    map.insert(".", TokenCode::Dot);
    map.insert("+", TokenCode::Plus);
    map.insert("-", TokenCode::Minus);
    map.insert("*", TokenCode::Star);
    map.insert("/", TokenCode::Slash);
    map.insert("&", TokenCode::Ampersand);
    map.insert("//", TokenCode::Comment);
    map.insert("!", TokenCode::Bang);
    map.insert("!=", TokenCode::BangEqual);
    map.insert("=", TokenCode::Equal);
    map.insert("==", TokenCode::EqualEqual);
    map.insert("<", TokenCode::Less);
    map.insert("<=", TokenCode::LessEqual);
    map.insert(">", TokenCode::Greater);
    map.insert(">=", TokenCode::GreaterEqual);

    /// Generate Primitive and it's Ref Type
    ///
    macro_rules! gen_types_n_refs {
        ($($variant:ident),* $(,)?) => {
            use std::rc::Rc;
            {
                $(
                    map.insert(stringify!($variant), TokenCode::TypeDef(Type::$variant));
                    map.insert(concat!("&", stringify!($variant)), TokenCode::TypeDef(Type::Ref(Rc::new(Type::$variant))));
                )*
            }
        }
    }

    gen_types_n_refs!(Int, Float, String, Bool, Void);

    // General compiler track
    map.insert("EOF", TokenCode::Eof);

    map
});