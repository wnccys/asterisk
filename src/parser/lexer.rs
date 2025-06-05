use std::{
    collections::HashMap, io::Bytes, iter::Peekable, slice::Iter, str::Lines, sync::LazyLock
};

use crate::value::Type;

#[derive(Debug)]
pub struct Lexer<R: std::io::Read> {
    source: Peekable::<Bytes::<R>>,
    current: Token
}

impl<R: std::io::Read> Lexer<R> {
    fn new(source: R) -> Self {
        Lexer {
            source: source.bytes().peekable(),
            current: Token::Eof
        }
    }

    fn next(&mut self) -> Token {
        if let Token::Eof = self.current {
            self.do_next()
        } else {

        }
    }

    fn peek() -> Token {}

    fn do_next(&mut self) -> Token {}
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

    Error(&'static str),
    Eof,
    Comment,
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
