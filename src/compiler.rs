#![allow(dead_code, unused)]
use crate::chunk::{Chunk, OpCode};
use crate::parser::ruler::*;
use crate::parser::scanner::*;
use crate::types::Table;
use crate::utils::print::{disassemble_chunk, print_stack};
use crate::value::Value;
use crate::vm::{InterpretResult, Vm};

#[derive(Debug)]
pub struct Parser<'a> {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    pub chunk: Option<Chunk>,
    pub scanner: Option<Scanner>,
    pub had_error: bool,
    pub panic_mode: bool,
    pub strings: Option<&'a mut Table>,
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
            strings: None,
        }
    }
}

pub fn compile(strings: &mut Table, chars: Vec<char>) -> (Chunk, InterpretResult) {
    let mut parser = Parser {
        scanner: Some(Scanner {
            chars,
            ..Default::default()
        }),
        strings: Some(strings),
        ..Default::default()
    };

    parser.advance();

    // parser.consume(TokenCode::Eof, "expected end of expression.");
    while parser.current.unwrap().code != TokenCode::Eof {
        parser.declaration();
    }

    parser.end_compiler();
    if parser.had_error {
        return (parser.chunk.unwrap(), InterpretResult::RuntimeError);
    }

    (parser.chunk.unwrap(), InterpretResult::Ok)
}

impl<'a> Parser<'a> {
    pub fn declaration(&mut self) {
        if (self.match_token(TokenCode::Var)) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.panic_mode { self.syncronize(); }
    }

    pub fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if (self.match_token(TokenCode::Equal)) {
            self.expression();
        } else {
            self.emit_byte(OpCode::False);
        }

        self.consume(TokenCode::SemiColon, "Expect ';' after variable declaration.");

        self.define_variable(global);
    }

    pub fn define_variable(&mut self, global: Value) {
    }

    pub fn parse_variable(&mut self, error_msg: &str) {
        self.consume(TokenCode::Identifier, error_msg);
        
        return self.identifier_constant();
    }

    pub fn identifier_constant(&mut self) {
        return;
    }

    pub fn statement(&mut self) {
        if self.match_token(TokenCode::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    pub fn syncronize(&mut self) {
        self.panic_mode = false;

        while self.current.unwrap().code != TokenCode::Eof {
            if (self.previous.unwrap().code == TokenCode::SemiColon) {
                match (self.current.unwrap().code) {
                    TokenCode::Class |
                    TokenCode::Fun |
                    TokenCode::Var |
                    TokenCode::For |
                    TokenCode::If | 
                    TokenCode::While |
                    TokenCode::Print |
                    TokenCode::Return => return,
                    _ => (),
                }

            }

            self.advance();
        }
    }

    pub fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenCode::SemiColon, "Expect ';' after value.");
        self.emit_byte(OpCode::Print);
    }

    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenCode::SemiColon, "Expect ';' after expression.");
        self.emit_byte(OpCode::Pop);
    }

    pub fn match_token(&mut self, token: TokenCode) -> bool {
        if !self.check(token) {
            return false;
        }
        self.advance();
        true
    }

    pub fn check(&self, token: TokenCode) -> bool {
        self.current.unwrap().code == token
    }

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

    pub fn emit_constant(&mut self, value: Value) {
        let const_index = self
            .chunk
            .as_mut()
            .unwrap()
            .write_constant(value.to_owned());

        self.emit_byte(OpCode::Constant(const_index));
    }

    fn end_compiler(&mut self) {
        // self.emit_byte(OpCode::Return);

        if !self.had_error {
            disassemble_chunk(self.chunk.as_ref().unwrap(), "code".to_string());
        }
    }

    pub fn error(&self, msg: &str) {
        if self.panic_mode {
            return;
        }

        let token = self.current.unwrap();
        match token.code {
            TokenCode::Eof => println!(" at end."),
            TokenCode::Error => (),
            _ => println!(" at {} | position: {}", token.line + 1, token.start),
        }

        println!("{}", msg);
    }
}
