#![allow(dead_code, unused)]
use std::default;
use std::slice::Iter;

use crate::chunk::{Chunk, OpCode};
use crate::parser::{ruler::*, Local, Parser};
use crate::parser::n_scanner::*;
use crate::types::hash_table::HashTable;
use crate::utils::print::{disassemble_chunk, print_stack};
use crate::value::Value;
use crate::vm::{InterpretResult, Vm};

#[derive(Debug)]
pub struct Compiler<'a, 'b> {
    pub parser: Parser<'a>,
    pub token_stream: TokenStream<'b>,
    // pub locals: Vec<Local<'c>>,
    pub source_code: Vec<char>,
}

impl<'a, 'b> Compiler<'a, 'b> {
    pub fn new(strings: &'a mut HashTable<String>, token_stream: TokenStream<'b>, source_code: Vec<char>) -> Self {
        Compiler {
            parser: Parser {
                chunk: Chunk::default(),
                current: None,
                previous: None,
                had_error: false,
                panic_mode: false,
                strings,
                local_count: 0,
                locals: vec![],
                scope_depth: 0,
            },
            source_code,
            token_stream,
        }
    }


    pub fn compile(strings: &'a mut HashTable<String>, chars: Vec<char>) -> (Chunk, InterpretResult) {
        let mut compiler: Compiler<'_, '_> = 
            Compiler::new(
                strings, 
                Scanner::scan(&chars), 
                chars.clone()
            );

        compiler.parser.parse(compiler.token_stream);

        // while compiler.parser.current.as_ref().unwrap().code != TokenCode::Eof {
        //     compiler.parser.declaration();
        // }

        // compiler.parser.end_compiler();
        // if compiler.parser.had_error {
        //     return (compiler.parser.chunk.clone(), InterpretResult::RuntimeError);
        // }

        // (compiler.parser.chunk.clone(), InterpretResult::Ok)
        todo!()
    }
}

// #[derive(Debug)]
// pub struct Parser<'a> {
//     pub chunk: Chunk,
//     pub current: Option<Token<'a>>,
//     pub previous: Option<Token<'a>>,
//     had_error: bool,
//     panic_mode: bool,
//     /// String interning model
//     pub strings: &'a mut HashTable<String>,
// }

// /// Represents a block-scope
// /// 
// #[derive(Debug)]
// pub struct Local<'a> {
//     name: Token<'a>,
//     /// Scope depth of block where variable was defined.
//     /// 
//     depth: u16,
// }

impl<'a, 'b> Compiler<'a, 'b> {
    /// Declaration Flow Order
    /// → classDecl
    ///    | funDecl
    ///    | varDecl
    ///    | statement ;
    pub fn declaration(&mut self) {
        if (self.match_token(TokenCode::Var)) {
            self.var_declaration();
        } 
        else if self.match_token(TokenCode::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        }
        else {
            // Declaration Control Flow Fallback
            self.statement();
        }

        if self.parser.panic_mode {
            self.syncronize();
        }
    }

    /// Set new variable with SetGlobal or push a value to stack throught GetGlobal.
    /// 
    pub fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        // Checks if after consuming identifier '=' Token is present.
        if (self.match_token(TokenCode::Equal)) {
            self.expression();
        } else {
            // TODO Set handling for null values (not allowed in asterisk)
            self.emit_byte(OpCode::Nil);
        }

        self.consume(
            TokenCode::SemiColon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(global);
    }

    /// Consume identifier token and emit new constant (if global).
    /// 
    /// Local Variables are auto-declared so to speak, It follows a convention on var declaration
    /// and scope-flow, so there's no need to set them to constants vector, the Compiler object already take care
    /// of which indexes behaves to which variables by scope_depth and local_count when local vars are set.
    /// 
    pub fn parse_variable(&mut self, error_msg: &str) -> usize {
        self.consume(TokenCode::Identifier, error_msg);

        // Check if var is global
        if (self.parser.scope_depth == 0) {
            return self.identifier_constant();
        }

        self.declare_variable();
        return 0;
    }

    /// Get variable's name by analising previous Token lexeme and emit it's Identifier as String to constants vector.
    /// 
    pub fn identifier_constant(&mut self) -> usize {
        // Gets chars from token and set it as var name
        let value = todo!();
        // self.scanner
        //     .source_code[self.previous.as_ref().unwrap().start
        //         ..self.previous.unwrap().start + self.previous.as_ref().unwrap().length]
        //     .iter()
        //     .cloned()
        //     .collect::<String>();

        self.parser.chunk
            .write_constant(Value::String(value))
    }

    pub fn declare_variable(&mut self) {
        if (self.parser.scope_depth == 0) { return }

        self.add_local();
    }

    // TODO Add variable shadowing support
    /// Set previous Token as local variable, assign it to compiler.locals, increasing Compiler's local_count
    /// 
    fn add_local(&mut self) {
        let name = &self.parser.previous.unwrap_or(panic!("Could not get previous variable"));

        let mut local: Local = todo!(); // Local::new();
        local.name = *name.clone();
        local.depth = self.parser.scope_depth;

        self
        .parser.locals.push(local);

        self.parser.local_count += 1;
    }

    // TODO Remove call when variable just need to be peek'd.
    /// Emit DefineGlobal ByteCode with provided index.
    /// 
    fn define_variable(&mut self, var_index: usize) {
        if (self.parser.scope_depth > 0) { return }

        self.emit_byte(OpCode::DefineGlobal(var_index));
    }

    /// Currently this function is only called inside self.declaration().
    /// 
    /// Statement Flow Order 
    /// → exprStmt
    ///    | forStmt
    ///    | ifStmt
    ///    | printStmt
    ///    | returnStmt
    ///    | whileStmt
    ///    | block ;
    /// 
    pub fn statement(&mut self) {
        if self.match_token(TokenCode::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    pub fn syncronize(&mut self) {
        self.parser.panic_mode = false;

        while self.parser.current.as_ref().unwrap().code != TokenCode::Eof {
            if (self.parser.previous.as_ref().unwrap().code == TokenCode::SemiColon) {
                match (self.parser.current.as_ref().unwrap().code) {
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

    /// Parse further expression consuming semicolon on end.
    /// 
    /// Emit: OpCode::Print
    /// 
    pub fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenCode::SemiColon, "Expect ';' after value.");
        self.emit_byte(OpCode::Print);
    }

    /// Evaluate expression and consume ';' token.
    /// 
    /// Emit: OpCode::Pop
    /// 
    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenCode::SemiColon, "Expect ';' after expression.");
        self.emit_byte(OpCode::Pop);
    }

    /// Calls declaration() until LeftBrace or EOF are found, consuming RightBrace on end.
    /// 
    fn block(&mut self) {
        while (!self.check(TokenCode::RightBrace) && !self.check(TokenCode::Eof)) {
            self.declaration();
        }

        self.consume(TokenCode::RightBrace, "Expected '}' end-of-block.");
    }

    /// Check if current Token matches argument Token.
    /// 
    /// Advance parser current Token on match.
    /// 
    pub fn match_token(&mut self, token: TokenCode) -> bool {
        if !self.check(token) {
            return false;
        }
        self.advance();
        true
    }

    /// Compare current Token with param Token.
    /// 
    pub fn check(&self, token: TokenCode) -> bool {
        self.parser.current.unwrap().code == token
    }

    /// Scan new token and set it as self.current.
    /// 
    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current;

        loop {
            self.parser.current = Some(&Token { code: TokenCode::Nil, lexeme: &['m'], line: 2});// Some(self.parser.scanner.scan_token());
            #[cfg(feature = "debug")]
            dbg!(self.current);

            if let Some(current) = self.parser.current {
                if current.code != TokenCode::Error {
                    break;
                }
            }

            self.error("error advancing token.");
        }
    }

    /// Evaluate and emit or get values from Stack.
    /// 
    /// Expressions are Pratt-Parsed evaluated, each expression Bytecode are emitted throught 
    /// the prefix and infix rules which matches a Token and handle it's correct behavior.
    /// 
    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = get_rule(&self.parser.previous.as_ref().unwrap().code).prefix;

        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule(self, can_assign);

        while precedence <= get_rule(&self.parser.current.as_ref().unwrap().code).precedence {
            self.advance();

            let infix_rule = get_rule(&self.parser.previous.as_ref().unwrap().code).infix;
            (infix_rule)(self, can_assign)
        }
    }

    /// Match token_code with self.current and advance if true.
    /// 
    pub fn consume(&mut self, token_code: TokenCode, msg: &str) {
        if self.parser.current.unwrap().code == token_code {
            self.advance();
        } else {
            self.error(msg);
        }
    }

    /// Check for current identifier token variable name, walking backward in locals array.
    /// 
    /// This function iterates over the Compiler's locals reverselly searching for a Token which 
    /// matches parser.previous (self.previous) Token.
    /// 
    /// Returns i32 because of -1 (No var name was found) conventional fallback.
    /// 
    /// O(n)
    /// 
    pub fn resolve_local(&mut self) -> i32 {
        for i in (0..self.parser.local_count).rev() {
            let local = &self.parser.locals[i];

            #[cfg(feature = "debug")]
            {
                dbg!(&local.name);
                dbg!(&self.previous.unwrap());
            }

            if identify_constant(&local.name, &self.parser.previous.unwrap()) {
                return i as i32;
            }
        }

        return -1;
    }

    pub fn begin_scope(&mut self) {
        self.parser.scope_depth += 1;
    }

    /// Decrease compiler scope_depth sanitizing (pop) values from stack
    /// 
    pub fn end_scope(&mut self) {
        self.parser.scope_depth -= 1;

        while self.parser.local_count > 0 &&
            self.parser.locals[self.parser.local_count - 1].depth >
            self.parser.scope_depth 
        {
            self.emit_byte(OpCode::Pop);
            self.parser.local_count -= 1;
        }
    }

    /// Emit arbitrary Bytecode.
    /// 
    /// Emit: param code 
    /// 
    pub fn emit_byte(&mut self, code: OpCode) {
        self.parser.chunk
            .write(code, self.parser.current.unwrap().line);
    }

    /// Write value to constant vec and let it available in stack.
    /// 
    /// Emit: OpCode::Constant
    /// 
    pub fn emit_constant(&mut self, value: Value) {
        let const_index = self
            .parser
            .chunk
            .write_constant(value.to_owned());

        self.emit_byte(OpCode::Constant(const_index));
    }

    /// Check for errors and disassemble chunk if compiler is in debug mode.
    /// 
    fn end_compiler(&mut self) {
        if !self.parser.had_error {
            // STUB
            #[cfg(feature = "debug")]
            disassemble_chunk(self.chunk.as_ref().unwrap(), "code".to_string());
        }
    }

    pub fn error(&self, msg: &str) {
        if self.parser.panic_mode {
            return;
        }

        let token = self.parser.current.unwrap();
        match token.code {
            TokenCode::Eof => println!(" at end."),
            TokenCode::Error => (),
            _ => println!(" at line {} | position: {}", token.line + 1, token.lexeme.iter().collect::<String>()),
        }

        println!("{}", msg);
        // panic!();
    }
}

// impl Compiler {
//     pub fn new() -> Self {
//         Compiler {
//             locals: vec![],
//             local_count: 0,
//             scope_depth: 0,
//         }
//     }
// }

// impl Local {
//     fn new() -> Self {
//         Local {
//             depth: 0,
//             name: Token { code: TokenCode::Nil, length: 0, line: 0, start: 0 }
//         }
//     }
// }

/// Compare 2 identifiers by length and code.
/// 
fn identify_constant(a: &Token, b: &Token) -> bool {
    if a.lexeme.len() != b.lexeme.len() { return false; }

    return a.code == b.code;
}