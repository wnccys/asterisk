use crate::chunk::Chunk;
use crate::parser::scanner::*;
use crate::parser::Parser;
use crate::types::hash_table::HashTable;
use crate::vm::InterpretResult;

pub fn compile(strings: &mut HashTable<String, String>, source_code: String) -> (Chunk, InterpretResult) {
    let mut source_lines = source_code.lines();
    let mut scanner = Scanner::new(&mut source_lines);

    let mut parser = Parser::new(strings, scanner.scan());
    #[cfg(feature = "debug-scan")]
    dbg!(&parser.token_stream);
    parser.advance();

    while parser.current.unwrap().code != TokenCode::Eof {
        parser.declaration();
    }

    (parser.end_compiler(), InterpretResult::Ok)
}
