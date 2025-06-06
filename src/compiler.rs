use crate::parser::scanner::*;
use crate::parser::Parser;
use crate::value::Function;
use crate::value::FunctionType;
use crate::vm::InterpretResult;

pub fn compile<T: std::io::Read>(source_code: T) -> Option<(Function, InterpretResult)> {
    /* Default app function, "main" so to speak. */
    let function = Function::default();

    let mut parser = Parser::new(function, FunctionType::Script);

    #[cfg(feature = "debug-scan")]
    dbg!(&parser.token_stream);
    parser.advance();

    while parser.current.unwrap().code != TokenCode::Eof {
        parser.declaration();
    }

    match parser.end_compiler() {
        Some(f) => Some((f, InterpretResult::Ok)),
        None => None
    }
}
