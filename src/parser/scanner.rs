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

#[derive(Debug)]
pub struct Scanner {
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
        if chars[self.current-1].is_whitespace() {
            self.current+=1;
            self.start+=1;
        }
        if chars[self.current-1].is_alphabetic() 
            { return self.alphanumeric(chars) }
        if chars[self.current-1].is_digit(10) 
            { return self.number(chars); }

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

    fn alphanumeric(&mut self, chars: &Vec<char>) -> Token {
        while !self.reach_source_end(chars) && chars[self.current].is_alphanumeric()
        { self.current+=1; }

        let identifier_token  = self.identifier(chars);
        self.make_token(identifier_token)
    }

    fn identifier(&mut self, chars: &Vec<char>) -> TokenCode {
        match chars[self.start] {
            'a' => self.check_keyword(1, chars, "nd", TokenCode::And),
            'c' => self.check_keyword(1, chars, "lass", TokenCode::Class),
            'e' => self.check_keyword(1, chars, "lse", TokenCode::Else),
            'f' => if self.current-self.start > 1 {
                return match chars[self.start+1] {
                    'a' => self.check_keyword(2, chars, "lse", TokenCode::False),
                    'o' => self.check_keyword(2, chars, "r", TokenCode::For),
                    'u' => self.check_keyword(2, chars, "n", TokenCode::Fun), 
                    _ => panic!("invalid identifier."),
                }
            } else {
                TokenCode::Identifier
            }
            'i' => self.check_keyword(1, chars, "f", TokenCode::If),
            'n' => self.check_keyword(1, chars, "il", TokenCode::Nil),
            'o' => self.check_keyword(1, chars, "r" , TokenCode::Or),
            'p' => self.check_keyword(1, chars, "rint", TokenCode::Print),
            'r' => self.check_keyword(1, chars, "eturn", TokenCode::Return),
            's' => self.check_keyword(1, chars, "uper", TokenCode::Super),
            't' => if self.current-self.start > 1 {
                return match chars[self.start+1] {
                    'h' => self.check_keyword(2, chars, "is", TokenCode::This),
                    'r' => self.check_keyword(2, chars, "ue", TokenCode::True),
                    _ => panic!("invalid identifier."),
                }
            } else {
                TokenCode::Identifier
            }
            'v' => self.check_keyword(1, chars, "ar", TokenCode::Var),
            'w' => self.check_keyword(1, chars, "hile", TokenCode::While),
            _ => TokenCode::Identifier,
        }
    }

    fn check_keyword(&mut self, 
        matcher_start: usize, 
        chars: &Vec<char>, 
        matcher: &str, 
        token_code: TokenCode) 
        -> TokenCode
    {
        let mut matched_chars: usize = 0;

        while matched_chars < matcher.len() && 
        chars.len() > 1 &&
        matcher.chars().nth(matched_chars).unwrap() == chars[self.start+matcher_start+matched_chars] 
        {
            matched_chars +=1;
        }

        if matched_chars == matcher.len() {
            return token_code
        }

        TokenCode::Identifier
    }

    fn number(&mut self, chars: &Vec<char>) -> Token {
        while !self.reach_source_end(chars) && chars[self.current].is_digit(10) 
            { self.current+=1; }

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
        { 
            if chars[self.current] == '\n' { self.line+=1; }; 
            self.current+=1;

            // safelly verifies if ${} expression is present
            if !self.reach_source_end(chars) && chars[self.current] == '$' {
                self.current+=1;

                if !self.reach_source_end(chars) && chars[self.current] == '{' {
                    self.evaluate_expression(chars);
                }
            }
        }

        if self.reach_source_end(chars) && chars[self.current-1] != '"' 
            { return self.error_token("unterminated string.") };

        self.current+=1;
        self.make_token(TokenCode::String)
    }

    fn evaluate_expression(&mut self, chars: &Vec<char>) {
        let mut inner_current = self.current+1;
        let mut vm = Vm::new();
        let mut expression = Vec::with_capacity(4);

        while chars.len() > inner_current && chars[inner_current] != '}' {
            expression.push(chars[inner_current]);
            inner_current+=1;
        }

        if chars.len() > inner_current && chars[inner_current] == '}' {
            let source = expression.iter().collect();
            vm.interpret(&source);
        } 

        self.current += inner_current - self.current-1;
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

    pub fn error_token(&self, message: &str) -> Token {
        println!("{}", message);
        Token {
            code: TokenCode::Error,
            start: self.start,
            length: self.current-self.start,
            line: self.line,
        }
    }
}