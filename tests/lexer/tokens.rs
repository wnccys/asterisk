#[cfg(test)]
mod tokens {
    use asterisk::{parser::lexer::{Lexer, Token}, primitives::types::Type};

    #[test]
    pub fn word_matching() {
        /* 
            Omitted: Nil and Error 
            Comment (//, /* */) are present on source, but it is skipped naturally by lexer
        */
        let tokens: [Token; _] = [
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Dot,
            Token::Minus,
            Token::Plus,
            Token::Colon,
            Token::SemiColon,
            Token::Slash,
            Token::Star,
            Token::Ampersand,
            Token::Bang,
            Token::BangEqual,
            Token::Equal,
            Token::EqualEqual,
            Token::Arrow,
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
            Token::Identifier(String::from("ident")),
            Token::String("str".as_bytes().to_vec()),
            Token::Float(1.0),
            Token::Integer(1),
            Token::And,
            Token::Class,
            Token::Case,
            Token::Const,
            Token::Continue,
            Token::Default,
            Token::Else,
            Token::False,
            Token::For,
            Token::Fun,
            Token::If,
            Token::Modifier,
            Token::TypeDef(Type::String),
            Token::TypeDef(Type::Int),
            Token::TypeDef(Type::Float),
            Token::TypeDef(Type::Bool),
            Token::Or,
            Token::Print,
            Token::Return,
            Token::Switch,
            Token::Super,
            Token::This,
            Token::True,
            Token::Var,
            Token::While,
            Token::Eof,
        ];

        let source = r"
            ( ) { } , . - + : ; / * &
            ! != = == => > >= < <=
            ident 'str' 1.0 1
            && class case const continue
            default else false for fn if
            mut String Int Float Bool || print return switch
            super this true let while // /* */ \0
        ";

        let mut lex = Lexer::new(std::io::Cursor::new(source));

        for t in tokens.into_iter() {
            let l_tok = lex.next();
            assert_eq!(t, l_tok);
        }
    }
}