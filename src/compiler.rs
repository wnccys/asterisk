use crate::chunk::Chunk;
use crate::parser::Parser;
use crate::parser::n_scanner::*;
use crate::types::hash_table::HashTable;
use crate::vm::InterpretResult;

pub fn compile(strings: &mut HashTable<String>, source_code: Vec<char>) -> (Chunk, InterpretResult) {
    let mut parser = Parser::new(
            strings, 
            Scanner::scan(&source_code)
        );
    parser.advance();

    while parser.current.unwrap().code != TokenCode::Eof {
        parser.declaration();
    }

    (parser.end_compiler(), InterpretResult::Ok)
}