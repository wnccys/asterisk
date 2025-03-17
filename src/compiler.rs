#![allow(dead_code, unused)]
use crate::chunk::{Chunk, OpCode};
use crate::parser::ruler::*;
use crate::parser::scanner::*;
use crate::types::hash_table::HashTable;
use crate::utils::print::{disassemble_chunk, print_stack};
use crate::value::Value;
use crate::vm::{InterpretResult, Vm};

#[derive(Debug)]
pub struct Parser<'a> {
    compiler: Compiler,
    pub current: Option<Token>,
    pub previous: Option<Token>,
    pub chunk: Option<Chunk>,
    pub scanner: Option<Scanner>,
    pub had_error: bool,
    pub panic_mode: bool,
    /// String interning model
    pub strings: Option<&'a mut HashTable<String>>,
}

#[derive(Debug)]
pub struct Compiler {
    pub locals: Vec<Local>,
    /// Represents how many locals are in the scope
    /// 
    pub local_count: usize,
    /// Represents the number of blocks surrounding the chunk of code whose are being compiled. 
    /// 
    /// Note:. (0) = global scope.
    /// 
    pub scope_depth: u16,
}

/// Represents a block-scope
/// 
#[derive(Debug)]
pub struct Local {
    name: Token,
    /// Scope depth of block where variable was defined
    /// 
    depth: u16,
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Self {
            compiler: Compiler::new(),
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

pub fn compile(strings: &mut HashTable<String>, chars: Vec<char>) -> (Chunk, InterpretResult) {
    let mut parser = Parser {
        scanner: Some(Scanner {
            chars,
            ..Default::default()
        }),
        strings: Some(strings),
        ..Default::default()
    };

    parser.advance();

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

        if self.panic_mode {
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

    /// Consume identifier token and emit new constant.
    /// 
    /// Local Variables are auto-declared so to speak, It follows a convention on var declaration
    /// and scope-flow, so there's no need to set them to constants vector, the Compiler object already take care
    /// of which indexes behaves to which variables by scope_depth and local_count when local vars are set.
    /// 
    pub fn parse_variable(&mut self, error_msg: &str) -> usize {
        self.consume(TokenCode::Identifier, error_msg);

        self.declare_variable();
        if (self.compiler.scope_depth > 0) { return 0 }

        self.identifier_constant()
    }

    /// Get variable's name by analising lexeme and emit it's Identifier as String to constants vector.
    /// 
    pub fn identifier_constant(&mut self) -> usize {
        // Gets chars from token and set it as var name
        let value = self.scanner.as_ref().unwrap()
            .chars[self.previous.as_ref().unwrap().start
                ..self.previous.unwrap().start + self.previous.as_ref().unwrap().length]
            .iter()
            .cloned()
            .collect::<String>();

        self.chunk
            .as_mut()
            .unwrap()
            .write_constant(Value::String(value))
    }

    pub fn declare_variable(&mut self) {
        if (self.compiler.scope_depth == 0) { return }

        self.add_local();
    }

    // TODO Add variable shadowing support
    /// 
    fn add_local(&mut self) {
        let name = &self.previous.unwrap_or_else(|| panic!("Could not get previous variable"));

        let mut local = Local::new();
        local.name = name.clone();
        local.depth = self.compiler.scope_depth;

        self
        .compiler
        .locals.push(local);

        self.compiler.local_count += 1;
    }

    // TODO Remove call when variable just need to be peek'd.
    /// Emit DefineGlobal ByteCode with provided index.
    fn define_variable(&mut self, var_index: usize) {
        if (self.compiler.scope_depth > 0) { return }

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

    /// Evaluate expression and consume ';' token.
    /// 
    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenCode::SemiColon, "Expect ';' after expression.");
        self.emit_byte(OpCode::Pop);
    }

    fn block(&mut self) {
        while (!self.check(TokenCode::RightBrace) && !self.check(TokenCode::Eof)) {
            self.declaration();
        }

        self.consume(TokenCode::RightBrace, "Expected '}' end-of-block.");
    }

    /// Check if current Token matches argument Token. </br>
    /// Advance parser current Token on match.
    /// 
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

    /// Scan new token and set it as self.current.
    /// 
    pub fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = Some(self.scanner.as_mut().unwrap().scan_token());
            #[cfg(feature = "debug")]
            dbg!(self.current);

            if let Some(current) = self.current {
                if current.code != TokenCode::Error {
                    break;
                }
            }

            self.error("error advancing token.");
        }
    }

    /// Evaluate and emit or get values from Stack.
    /// 
    /// Expressions are Pratt-Parsed evaluated, each expression Bytecode are emitted
    /// throught the prefix and infix rules which matches a Token and behavior like it.
    /// 
    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = get_rule(&self.previous.as_ref().unwrap().code).prefix;

        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule(self, can_assign);

        while precedence <= get_rule(&self.current.as_ref().unwrap().code).precedence {
            self.advance();

            let infix_rule = get_rule(&self.previous.as_ref().unwrap().code).infix;
            (infix_rule)(self, can_assign)
        }
    }

    /// Match token_code with self.current and advance if true.
    /// 
    pub fn consume(&mut self, token_code: TokenCode, msg: &str) {
        if self.current.unwrap().code == token_code {
            self.advance();
        } else {
            self.error(msg);
        }
    }

    // TODO Change to Option<usize>
    /// Check for current identifier token variable name, walking backward in locals array.
    /// Returns i32 because of -1 (No var name was found) fallback.
    pub fn resolve_local(&mut self) -> i32 {
        for i in (0..self.compiler.local_count).rev() {
            let local = &self.compiler.locals[i];

            #[cfg(feature = "debug")]
            {
                dbg!(&local.name);
                dbg!(&self.previous.unwrap());
            }

            if identify_constant(&local.name, &self.previous.unwrap()) {
                return i as i32;
            }
        }

        return -1;
    }

    pub fn begin_scope(&mut self) {
        self.compiler.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.compiler.scope_depth -= 1;

        while self.compiler.local_count > 0 &&
            self.compiler.locals[self.compiler.local_count - 1].depth >
            self.compiler.scope_depth 
        {
            self.emit_byte(OpCode::Pop);
            self.compiler.local_count -= 1;
        }
    }

    pub fn emit_byte(&mut self, code: OpCode) {
        self.chunk
            .as_mut()
            .unwrap()
            .write(code, self.current.unwrap().line);
    }

    /// Write value to constant vec and emit Constant bytecode to let it available in stack.
    /// 
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
            // STUB
            #[cfg(feature = "debug")]
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
            _ => println!(" at line {} | position: {}", token.line + 1, token.start),
        }

        println!("{}", msg);
        // panic!();
    }
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            locals: vec![],
            local_count: 0,
            scope_depth: 0,
        }
    }
}

impl Local {
    fn new() -> Self {
        Local {
            depth: 0,
            name: Token { code: TokenCode::Nil, length: 0, line: 0, start: 0 }
        }
    }
}

/// Compare 2 identifiers.
/// 
fn identify_constant(a: &Token, b: &Token) -> bool {
    if a.length != b.length { return false; }

    return a.code == b.code;
}