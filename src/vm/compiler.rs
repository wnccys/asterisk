use crate::parser::lexer::*;
use crate::parser::Parser;
use crate::primitives::functions::Function;
use crate::primitives::functions::FunctionType;

pub fn compile<T: std::io::Read>(source_code: T) -> Function {
    let lex = Lexer::new(source_code);
    /* Default app function, "main" so to speak. */
    let function = Function::default();

    let mut parser = Parser::new(function, FunctionType::Script, lex);

    parser.advance();

    while parser.current != Token::Eof {
        parser = parser.declaration();
    }

    parser.end_compiler()
}
