use crate::parser::scanner::*;
use crate::parser::Parser;
use crate::value::Function;
use crate::value::FunctionType;
use crate::vm::InterpretResult;
use crate::vm::Stack;

pub fn compile(source_code: String, stack_ref: &mut Stack) -> Option<(Function, InterpretResult)> {
    let mut source_lines = source_code.lines();
    let mut scanner = Scanner::new(&mut source_lines);
    let mut token_stream = scanner.scan();

    /* Default app function, "main" so to speak. */
    let function = Function::default();

    let mut parser = Parser::new(&mut token_stream, function, FunctionType::Script, stack_ref);
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
