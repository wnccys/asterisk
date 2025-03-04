use crate::vm::Vm;

#[derive(Debug, PartialEq, Copy, Clone)]
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

    Error,
    Eof,
    Comment,
}

#[derive(Debug, Default)]
pub struct Scanner {
    pub chars: Vec<char>,
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub code: TokenCode,
    pub start: usize,
    pub length: usize,
    pub line: i32,
}

impl Scanner {
    pub fn scan_token(&mut self) -> Token {
        // NOTE self.start is 0 index based but current is generally always 1 advanced,
        // so current-1 is the exact position on chars,
        // and self.start..self.start+self.length is a correct range
        // so there's no need to subtract 1 because the index is advanced by 1 already
        self.start = self.current;
        if self.reach_source_end() {
            return self.make_token(TokenCode::Eof);
        }

        self.current += 1;
        self.handle_lines();

        if self.chars[self.current - 1].is_alphabetic() {
            return self.alphanumeric();
        }

        if self.chars[self.current - 1].is_ascii_digit() {
            return self.number();
        }

        match self.chars[self.current - 1] {
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
            '/' => {
                if self.chars[self.current] == '/' {
                    self.current += 1;
                    self.skip_comment()
                } else {
                    self.make_token(TokenCode::Slash)
                }
            }
            '!' => {
                if self.chars[self.current] == '=' {
                    self.current += 1;
                    self.make_token(TokenCode::BangEqual)
                } else {
                    self.make_token(TokenCode::Bang)
                }
            }
            '=' => {
                if self.chars[self.current] == '=' {
                    self.current += 1;
                    self.make_token(TokenCode::EqualEqual)
                } else {
                    self.make_token(TokenCode::Equal)
                }
            }
            '<' => {
                if self.chars[self.current] == '=' {
                    self.current += 1;
                    self.make_token(TokenCode::LessEqual)
                } else {
                    self.make_token(TokenCode::Less)
                }
            }
            '>' => {
                if self.chars[self.current] == '=' {
                    self.current += 1;
                    self.make_token(TokenCode::GreaterEqual)
                } else {
                    self.make_token(TokenCode::Greater)
                }
            }
            '"' => self.string(),
            _ => self.error_token(&format!("not implemented yet. {:?}", self.chars[self.current - 1])),
        }
    }

    /// Check if source self.chars\[self.current - 1\] doesn't match space char, '\n', or '\r'
    /// advancing current and adding lines until new chars are found.
    fn handle_lines(&mut self) {
        while !self.reach_source_end() && match self.chars[self.current - 1] {
            x if x == ' ' || x == '\n' || x == '\r' => true,
            _ => false,
        } {
            if self.chars[self.current - 1] == ' '
            {
                self.current += 1;
                self.start += 1;
            }

            if self.chars[self.current - 1] == '\r'  {
                self.current += 1;
                self.start += 1;
                self.line += 1;

                if !self.reach_source_end() && self.chars[self.current - 1] == '\n' {
                    self.current += 1;
                    self.start += 1;
                }
            }

            if self.chars[self.current - 1] == '\n'  {
                self.current += 1;
                self.start += 1;
                self.line += 1;
            }
        }
    }

    fn alphanumeric(&mut self) -> Token {
        while !self.reach_source_end() && self.chars[self.current].is_alphanumeric() {
            self.current += 1;
        }

        let identifier_token = self.identifier();
        self.make_token(identifier_token)
    }

    fn identifier(&mut self) -> TokenCode {
        match self.chars[self.start] {
            'a' => self.check_keyword(1, "nd", TokenCode::And),
            'c' => self.check_keyword(1, "lass", TokenCode::Class),
            'e' => self.check_keyword(1, "lse", TokenCode::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.chars[self.start + 1] {
                        'a' => self.check_keyword(2, "lse", TokenCode::False),
                        'o' => self.check_keyword(2, "r", TokenCode::For),
                        'n' => self.check_keyword(1, "n", TokenCode::Fun),
                        _ => panic!("invalid identifier."),
                    }
                } else {
                    TokenCode::Identifier
                }
            }
            'i' => self.check_keyword(1, "f", TokenCode::If),
            'n' => self.check_keyword(1, "il", TokenCode::Nil),
            'o' => self.check_keyword(1, "r", TokenCode::Or),
            'p' => self.check_keyword(1, "rint", TokenCode::Print),
            'r' => self.check_keyword(1, "eturn", TokenCode::Return),
            's' => self.check_keyword(1, "uper", TokenCode::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.chars[self.start + 1] {
                        'h' => self.check_keyword(2, "is", TokenCode::This),
                        'r' => self.check_keyword(2, "ue", TokenCode::True),
                        _ => panic!("invalid identifier."),
                    }
                } else {
                    TokenCode::Identifier
                }
            }
            'l' => self.check_keyword(1, "et", TokenCode::Var),
            'w' => self.check_keyword(1, "hile", TokenCode::While),
            _ => TokenCode::Identifier,
        }
    }

    fn check_keyword(
        &mut self,
        matcher_start: usize,
        matcher: &str,
        token_code: TokenCode,
    ) -> TokenCode {
        let mut matched_chars: usize = 0;

        while matched_chars < matcher.len()
            && self.chars.len() > 1
            && matcher.chars().nth(matched_chars).unwrap()
                == self.chars[self.start + matcher_start + matched_chars]
        {
            matched_chars += 1;
        }

        if matched_chars == matcher.len() {
            return token_code;
        }

        TokenCode::Identifier
    }

    fn number(&mut self) -> Token {
        while !self.reach_source_end() && self.chars[self.current].is_ascii_digit() {
            self.current += 1;
        }

        if !self.reach_source_end() && self.chars[self.current] == '.' {
            self.current += 1;
            while !self.reach_source_end() && self.chars[self.current].is_ascii_digit() {
                self.current += 1
            }
        }

        self.make_token(TokenCode::Number)
    }

    fn string(&mut self) -> Token {
        while !self.reach_source_end() && self.chars[self.current] != '"' {
            if self.chars[self.current] == '\n' {
                self.line += 1;
            };
            self.current += 1;

            // Safelly verifies if ${} expression is present
            if !self.reach_source_end() && self.chars[self.current] == '$' {
                self.current += 1;

                if !self.reach_source_end() && self.chars[self.current] == '{' {
                    self.evaluate_expression();
                }
            }
        }

        if self.reach_source_end() && self.chars[self.current - 1] != '"' {
            return self.error_token("unterminated string.");
        };

        self.current += 1;
        self.make_token(TokenCode::String)
    }

    /// Invokes a new VM which parses the code between "{ }"
    /// 
    fn evaluate_expression(&mut self) {
        let mut inner_current = self.current + 1;
        let mut vm = Vm::default();
        let mut expression = Vec::with_capacity(4);

        while self.chars.len() > inner_current && self.chars[inner_current] != '}' {
            expression.push(self.chars[inner_current]);
            inner_current += 1;
        }

        if self.chars.len() > inner_current && self.chars[inner_current] == '}' {
            let source = expression;
            vm.interpret(source);
        }

        self.current += inner_current - self.current - 1;
    }

    fn skip_comment(&mut self) -> Token {
        while !self.reach_source_end() && self.chars[self.current] != '\n' {
            self.current += 1;
        }

        self.make_token(TokenCode::Comment)
    }

    fn reach_source_end(&self) -> bool {
        self.current == self.chars.len()
    }

    pub fn make_token(&self, token_code: TokenCode) -> Token {
        Token {
            code: token_code,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    pub fn error_token(&self, message: &str) -> Token {
        println!("{}", message);
        Token {
            code: TokenCode::Error,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }
}
