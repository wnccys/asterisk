use crate::chunk::{Chunk, OpCode};
use crate::scanner::*;
use crate::vm::InterpretResult;

#[derive(Copy, Clone)]
struct Parser {
    current: Option<Token>,
    previous: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

pub fn compile(chars: &Vec<char>) -> (Chunk, InterpretResult) {
    let mut scanner = Scanner::new();
    let mut parser = Parser::new();
    let mut chunk = Chunk::new();

    parser.advance(chars, &mut scanner);
    parser.expression();
    parser.consume(TokenCode::Eof, chars, &mut scanner, "expected end of expression.");
    parser.end_compiler(&mut chunk);
    if parser.had_error { return (chunk, InterpretResult::RuntimeError) }

    (chunk, InterpretResult::Ok)
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, chars: &Vec<char>, scanner: &mut Scanner) {
        self.previous = self.current;

        loop {
            self.current = Some(scanner.scan_token(chars));

            if let Some(current) = self.current {
                if current.code != TokenCode::Error { break }
            }

            self.error("error advancing token.");
        }
    }

    pub fn expression(&mut self) {

    }

    pub fn consume(&mut self, token_code: TokenCode, chars: &Vec<char>, 
        scanner: &mut Scanner, msg: &str) 
    {
        if self.current.unwrap().code == token_code { self.advance(chars, scanner) }

        self.error(msg);
    }

    fn emit_byte(&self, chunk: &mut Chunk, code: OpCode) {
        chunk.write(code, self.current.unwrap().line);
    }

    fn emit_bytes(&self, byte1: OpCode, byte2: OpCode, 
        chunk: &mut Chunk) 
    {
        let chunk = chunk;
        self.emit_byte(chunk, byte1);

        self.emit_byte(chunk, byte2);
    }

    fn end_compiler(&mut self, chunk: &mut Chunk) {
        self.emit_byte(chunk, OpCode::OpReturn);
    }

    fn error(&self, msg: &str) {
        if self.panic_mode { return }

        let token = self.current.unwrap();
        print!("[{}] error", token.line);

        match token.code {
            TokenCode::Eof => println!(" at end."),
            TokenCode:: Error => (),
            _ => println!(" at {} {}", token.length, token.start),
        }

        println!("{}", msg);
    }
}