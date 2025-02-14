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

        if self.panic_mode {
            self.syncronize();
        }
    }

    pub fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if (self.match_token(TokenCode::Equal)) {
            self.expression();
            self.emit_byte(OpCode::SetGlobal(global));
        } else {
            self.emit_byte(OpCode::GetGlobal(global));
        }

        self.consume(
            TokenCode::SemiColon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(global);
    }

    pub fn parse_variable(&mut self, error_msg: &str) -> usize {
        self.consume(TokenCode::Identifier, error_msg);

        self.identifier_constant()
    }

    // REVIEW be wary of previous and current token requisite order
    /// Get var's name and emit it's Value (String) to constants vec. 
    /// 
    pub fn identifier_constant(&mut self) -> usize {
        // Gets chars from token and set it as var name
        let value = self.scanner.as_ref().unwrap().chars[self.previous.as_ref().unwrap().start
            ..self.previous.unwrap().start + self.previous.as_ref().unwrap().length]
            .to_vec();

        self.chunk
            .as_mut()
            .unwrap()
            .write_constant(Value::String(value))
    }

    pub fn define_variable(&mut self, var_index: usize) {
        self.emit_byte(OpCode::DefineGlobal(var_index));
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
                    TokenCode::Class
                    | TokenCode::Fun
                    | TokenCode::Var
                    | TokenCode::For
                    | TokenCode::If
                    | TokenCode::While
                    | TokenCode::Print
                    | TokenCode::Return => return,
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

        let can_assign = precedence <= Precedence::Assignment;
        (prefix_rule)(self, can_assign);

        while precedence <= get_rule(&self.current.as_ref().unwrap().code).precedence {
            self.advance();

            let infix_rule = get_rule(&self.previous.as_ref().unwrap().code).infix;
            (infix_rule)(self, can_assign)
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
