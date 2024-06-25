use crate::chunk::{Chunk, OpCode};
use crate::utils::print::disassemble_chunk;
use crate::value::Value;
use crate::scanner::*;
use crate::vm::InterpretResult;
use crate::ruler::*;

#[derive(Debug)]
pub struct Parser<'a> {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    pub chars: Option<&'a Vec<char>>,
    pub chunk: Option<Chunk>,
    pub scanner: Option<Scanner>,
    // TODO possibly add scanner as a field;
    pub had_error: bool,
    pub panic_mode: bool,
}

pub fn compile(chars: &Vec<char>) -> (Chunk, InterpretResult) {
    let mut parser = Parser::new();

    parser.chunk = Some(Chunk::new());
    parser.scanner = Some(Scanner::new());
    parser.chars = Some(&chars);

    parser.advance();
    parser.expression();
    parser.consume(TokenCode::Eof, "expected end of expression.");
    parser.end_compiler();
    if parser.had_error { return (parser.chunk.unwrap(), InterpretResult::RuntimeError) }

    (parser.chunk.unwrap(), InterpretResult::Ok)
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            current: None,
            previous: None,
            chars: None,
            chunk: None,
            scanner: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self){
        self.previous = self.current;

        loop {
            self.current = Some(
                self.scanner
                    .as_mut()
                    .unwrap()
                    .scan_token(self.chars.as_ref().unwrap())
            );

            if let Some(current) = self.current {
                if current.code != TokenCode::Error { break }
            }

            self.error("error advancing token.");
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn consume(&mut self, token_code: TokenCode, msg: &str) 
    {
        if self.current.unwrap().code == token_code { self.advance(); }

        self.error(msg);
    }

    pub fn emit_byte(&mut self, code: OpCode) {
        self.chunk.as_mut().unwrap().write(code, self.current.unwrap().line);
    }

    pub fn emit_constant(&mut self, value: &i32) {
        let const_index = self
                                .chunk
                                .as_mut()
                                .unwrap()
                                .write_constant(Value::Int(*value));

        self.emit_byte(OpCode::OpConstant(const_index));
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::OpReturn);

        if !self.had_error {
            disassemble_chunk(self.chunk.as_ref().unwrap(), "code".to_string());
        }
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = get_rule(&self
                        .previous
                        .as_ref()
                        .unwrap()
                        .code).prefix;

        (prefix_rule)(self);

        while precedence <= get_rule(&self.current.as_ref().unwrap().code).precedence {
           self.advance(); 

           let infix_rule = get_rule(&self.previous.as_ref().unwrap().code).infix;
           (infix_rule)(self)
        }
    }

    pub fn error(&self, msg: &str) {
        if self.panic_mode { return }

        let token = self.current.unwrap();
        print!("[{}] error", token.line);

        match token.code {
            TokenCode::Eof => println!(" at end."),
            TokenCode::Error => (),
            _ => println!(" at {} {}", token.length, token.start),
        }

        println!("{}", msg);
    }
}