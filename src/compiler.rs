use crate::chunk::Chunk;
use crate::parser::scanner::*;
use crate::parser::Parser;
use crate::types::hash_table::HashTable;
use crate::value::Function;
use crate::value::FunctionType;
use crate::vm::InterpretResult;

pub fn compile(strings: &mut HashTable<String, String>, source_code: String) -> Option<(Function, InterpretResult)> {
    let mut source_lines = source_code.lines();
    let mut scanner = Scanner::new(&mut source_lines);

    let function = Function::default();

    let mut parser = Parser::new(scanner.scan(), function, FunctionType::Script);
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
