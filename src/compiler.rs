#![allow(dead_code, unused)]
use crate::chunk::{Chunk, OpCode};
use crate::parser::ruler::*;
use crate::parser::scanner::*;
use crate::utils::print::{disassemble_chunk, print_stack};
use crate::value::Value;
use crate::vm::InterpretResult;

#[derive(Debug)]
pub struct Parser<'a> {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    pub chunk: Option<Chunk>,
    pub scanner: Option<Scanner<'a>>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Self {
            current: None,
            previous: None,
            chunk: Some(Chunk::default()),
            scanner: Some(Scanner::default()),
            had_error: false,
            panic_mode: false,
        }
    }
}

pub fn compile(chars: &[char]) -> (Chunk, InterpretResult) {
    let mut parser = Parser {
        scanner: Some(Scanner {
            chars,
            ..Default::default()
        }),
        ..Default::default()
    };

    parser.advance();
    parser.expression();
    parser.consume(TokenCode::Eof, "expected end of expression.");
    parser.end_compiler();
    if parser.had_error {
        return (parser.chunk.unwrap(), InterpretResult::RuntimeError);
    }

    (parser.chunk.unwrap(), InterpretResult::Ok)
}

impl<'a> Parser<'a> {
    pub fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = Some(self.scanner.as_mut().unwrap().scan_token());
            dbg!(self.current);

            if let Some(current) = self.current {
                if current.code != TokenCode::Error {
                    break;
                }
            }

            self.error("error advancing token.");
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = get_rule(&self.previous.as_ref().unwrap().code).prefix;

        (prefix_rule)(self);

        while precedence <= get_rule(&self.current.as_ref().unwrap().code).precedence {
            self.advance();

            let infix_rule = get_rule(&self.previous.as_ref().unwrap().code).infix;
            (infix_rule)(self)
        }
    }

    pub fn consume(&mut self, token_code: TokenCode, msg: &str) {
        if self.current.unwrap().code == token_code {
            self.advance();
        } else {
            self.error(msg);
        }
    }

    pub fn emit_byte(&mut self, code: OpCode) {
        self.chunk
            .as_mut()
            .unwrap()
            .write(code, self.current.unwrap().line);
    }

    pub fn emit_constant(&mut self, value: &Value) {
        let const_index = self.chunk.as_mut().unwrap().write_constant(*value);

        self.emit_byte(OpCode::Constant(const_index));
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::Return);

        if !self.had_error {
            disassemble_chunk(self.chunk.as_ref().unwrap(), "code".to_string());
        }
    }

    pub fn error(&self, msg: &str) {
        if self.panic_mode {
            return;
        }

        let token = self.current.unwrap();
        print!("[{}] error", token.line);

        match token.code {
            TokenCode::Eof => println!(" at end."),
            TokenCode::Error => (),
            _ => println!(" at {} | line: {}", token.length, token.start),
        }

        println!("{}", msg);
    }
}
