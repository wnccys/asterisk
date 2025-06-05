use std::{
    collections::HashMap, io::Bytes, iter::Peekable, sync::LazyLock
};

use crate::value::Type;

#[derive(Debug)]
pub struct Lexer<R: std::io::Read> {
    source: Peekable::<Bytes::<R>>,
    current: Token,
    line: u16
}

impl<R: std::io::Read> Lexer<R> {
    fn new(source: R) -> Self {
        Lexer {
            source: source.bytes().peekable(),
            current: Token::Eof,
            line: 0
        }
    }

    fn next(&mut self) -> Token {
        let byt = self.read_byte();

        match byt {
            b' ' | b'\t' | b'\r'  => self.next(),
            b'\n' => { self.line+=1; self.next() },
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b',' => Token::Comma,
            b'.' => Token::Dot,
            b'-' => Token::Minus,
            b'+' => Token::Plus,
            b':' => Token::Colon,
            b';' => Token::SemiColon,
            b'/' => Token::Slash,
            b'*' => Token::Star,
            b'&' => self.check_ahead(b'&', Token::Ampersand, Token::And),
            b'!' => self.check_ahead(b'=', Token::Bang, Token::BangEqual),
            b'=' => self.check_ahead(b'=', Token::Equal, Token::EqualEqual),
            b'>' => self.check_ahead(b'=', Token::Greater, Token::GreaterEqual),
            b'<' => self.check_ahead(b'<', Token::Less, Token::LessEqual),
            b'0'..=b'9' => self.number(byt),
            b'A'..=b'Z' | b'a'..=b'z' => self.keyword(byt),
            b'\0' => Token::Eof,
            _ => panic!("invalid token {}", byt as char)
        }
    }

    fn peek_byte(&mut self) -> &u8 {
        match self.source.peek() {
            Some(r) => r.as_ref().unwrap(),
            None => &b'\0',
        }
    }

    fn read_byte(&mut self) -> u8 {
        match self.source.next() {
            Some(Ok(ch)) => ch,
            Some(_) => panic!("error reading byte on 0 line {}", self.line),
            None => b'\0'
        }
    }

    fn check_ahead(&mut self, ahead: u8, short: Token, long: Token) -> Token {
        if *self.peek_byte() == ahead {
            long
        } else {
            short
        }
    }

    fn number(&mut self, num: u8) -> Token {
    }

    fn keyword(&mut self, char: u8) -> Token {
        let mut word = String::new();

        match &word as &str {
            "and" => Token::And,
            "or" => Token::Or,
            "class" => Token::Class,
            "case" => Token::Case,
            "const" => Token::Const,
            "continue" => Token::Continue,
            "default" => Token::Default,
            "else" => Token::Else,
            "false" => Token::False,
            "for" => Token::For,
            "fn" => Token::Fun,
            "if" => Token::If,
            "mut" => Token::Modifier,
            "print" => Token::Print,
            "return" => Token::Return,
            "switch" => Token::Switch,
            "super" => Token::Super,
            "this" => Token::This,
            "let" => Token::Var,
            "while" => Token::While,
            _ => panic!("invalid keyword {word}")
        }
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    Void(()),
    ShortStr(),
    MidStr(),
    LongStr(),
    Float(f64),
    Integer(i64),
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

    Comment,
    Error,
    Eof,
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
