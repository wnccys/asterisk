use std::{cell::RefCell, fmt::Write, rc::Rc};

use ruler::{get_rule, Precedence};
use scanner::{Token, TokenCode, TokenStream};

use crate::{
    chunk::{Chunk, OpCode},
    errors::parser_errors::ParserResult,
    types::hash_table::{hash_key, HashTable},
    utils::{parse_type, print::disassemble_chunk},
    value::{Modifier, Primitive, Type, Value},
};

pub mod ruler;
pub mod scanner;

#[derive(Debug)]
pub struct Parser<'a> {
    pub chunk: Chunk,
    pub token_stream: TokenStream<'a>,
    pub current: Option<&'a Token>,
    pub previous: Option<&'a Token>,
    pub had_error: bool,
    pub panic_mode: bool,
    pub scopes: Vec<Scope>,
    /// String interning model
    ///
    pub _strings: &'a mut HashTable<String, String>,
}

/// General scope handler.
///
#[derive(Debug)]
pub struct Scope {
    /// Represents all local variables, resolved dynamically at runtime, without a Constant Bytecode.
    ///
    /// (Var position on locals [consequently on Stack], Modifier))
    pub locals: HashTable<String, (usize, Modifier)>,
    pub local_count: usize,
}

/// Represent a block scope
/// 
impl Scope {
    /// Add new Local by hashing and inserting it
    /// 
    fn add_local(&mut self, lexeme: String, modifier: Modifier) {
        self.locals.insert(&lexeme, (self.local_count, modifier));
        self.local_count += 1;
    }

    /// Return Local index to be used by stack if it exists
    /// 
    fn get_local(&self, lexeme: String) -> Option<Rc<RefCell<(usize, Modifier)>>> {
        self.locals.get(&lexeme)
    }
}

impl<'a> Default for Scope {
    fn default() -> Self {
        Scope {
            locals: HashTable::default(),
            local_count: 0,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(_strings: &'a mut HashTable<String, String>, token_stream: TokenStream<'a>) -> Self {
        Parser {
            chunk: Chunk::default(),
            token_stream,
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
            _strings,
            scopes: vec![],
        }
    }

    /// Declaration Flow Order
    /// → classDecl
    ///    | funDecl
    ///    | varDecl
    ///    | statement
    ///
    pub fn declaration(&mut self) {
        if self.match_token(TokenCode::Var) {
            self.var_declaration();
        } else if self.match_token(TokenCode::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
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
        let modifier = self.parse_modifier();
        let global = self.parse_variable("Expect variable name.", modifier);

        // Checks if after consuming identifier '=' Token is present.
        if self.match_token(TokenCode::Equal) {
            self.expression();

        // Check for typedef
        } else if self.match_token(TokenCode::Colon) {
            // Lazy-evaluated var type
            let t = self.parse_var_type();

            // Handle uninitialized but typed vars
            if self.match_token(TokenCode::Equal) {
                self.expression();
            }

            self.emit_byte(OpCode::SetType(t));
        // Uninitialized and untyped variables handling
        } else {
            panic!("Uninitialized variables are not allowed.");
        }

        self.consume(
            TokenCode::SemiColon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(global, modifier);
    }

    /// Match current Token for Modifier(Mut) / Identifier(Const).
    ///
    pub fn parse_modifier(&mut self) -> Modifier {
        match &self.current.unwrap().code {
            TokenCode::Modifier => {
                self.advance();
                Modifier::Mut
            }
            TokenCode::Identifier => Modifier::Const,
            _ => panic!("Error parsing variable."),
        }
    }

    /// Consume identifier token and emit new constant (if global).
    ///
    /// Local Variables are auto-declared so to speak, It follows a convention on var declaration
    /// and scope-flow, so there's no need to set them to constants vector, the Compiler object already take care
    /// of which indexes behaves to which variables by scope_depth and local_count when local vars are set.
    ///
    /// Return 0 when variable is local, which will be ignored by define_variable(), so it is not set to constants.
    ///
    pub fn parse_variable(&mut self, error_msg: &str, modifier: Modifier) -> usize {
        self.consume(TokenCode::Identifier, error_msg);

        // Check if var is global
        if self.scopes.len() == 0 {
            return self.identifier_constant();
        }

        self.declare_variable(modifier);
        return 0;
    }

    /// Try to extract current type from TypeDef.
    ///
    /// Executed when explicit type definition is set, with :
    ///
    pub fn parse_var_type(&mut self) -> Type {
        match self.current.unwrap().code.clone() {
            TokenCode::TypeDef(t) => {
                self.advance();
                t
            }
            _ => panic!("Invalid Var Type."),
        }
    }

    /// Get variable's name by analising previous Token lexeme and emit it's Identifier as String to constants vector.
    ///
    pub fn identifier_constant(&mut self) -> usize {
        // Gets chars from token and set it as var name
        let value = &self.previous.unwrap().lexeme;

        self.chunk.write_constant(Primitive::String(value.clone()))
    }

    pub fn declare_variable(&mut self, modifier: Modifier) {
        if self.scopes.len() == 0 {
            return;
        }

        self.add_local(modifier);
    }

    /// Set previous Token as local variable, assign it to compiler.locals, increasing Compiler's local_count
    ///
    fn add_local(&mut self, modifier: Modifier) {
        self.scopes.last_mut().unwrap().add_local(self.previous.unwrap().lexeme.clone(), modifier);
    }

    /// Emit DefineGlobal ByteCode with provided index. (global variables only)
    ///
    ///
    pub fn define_variable(&mut self, name_index: usize, modifier: Modifier) {
        if self.scopes.len() > 0 {
            return;
        }

        self.emit_byte(OpCode::DefineGlobal(name_index, modifier));
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    /// Decrease compiler scope_depth sanitizing (pop) values from stack
    ///
    pub fn end_scope(&mut self) {
        /* Remove scope Locals when it ends */
        while self.scopes.last().unwrap().local_count > 0
        {
            self.emit_byte(OpCode::Pop);
            self.scopes.last_mut().unwrap().local_count -= 1;
        }

        self.scopes.pop();
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
            if self.previous.unwrap().code == TokenCode::SemiColon {
                match self.current.unwrap().code {
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
    pub fn block(&mut self) {
        while !self.check(TokenCode::RightBrace) && !self.check(TokenCode::Eof) {
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
        self.current.unwrap().code == token
    }

    /// Scan new token and set it as self.current.
    ///
    pub fn advance(&mut self) {
        self.previous = self.current;

        self.current = self.token_stream.next();

        #[cfg(feature = "debug")]
        dbg!(self.current);

        if let TokenCode::Error(msg) = self.current.unwrap().code {
            self.error(&format!("Error advancing token. {}", msg));
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

    /// Emit arbitrary Bytecode.
    ///
    /// Emit: param code
    ///
    pub fn emit_byte(&mut self, code: OpCode) {
        self.chunk.write(code, self.current.unwrap().line);
    }

    /// Write value to constant vec and set it's bytecode.
    ///
    /// Emit: OpCode::Constant
    ///
    pub fn emit_constant(&mut self, value: Value) {
        let const_index = self.chunk.write_constant(value.to_owned().value);

        self.emit_byte(OpCode::Constant(const_index));
    }

    /// Check for errors and disassemble chunk if compiler is in debug mode.
    ///
    pub fn end_compiler(&mut self) -> Chunk {
        if !self.had_error {
            // STUB
            #[cfg(feature = "debug")]
            disassemble_chunk(&self.chunk, "code".to_string());
        }

        self.chunk.clone()
    }

    /// Panic on errors with panic_mode handling.
    ///
    pub fn error(&self, msg: &str) {
        if self.panic_mode {
            return;
        }

        let token = self.current.unwrap();
        match token.code {
            TokenCode::Eof => println!(" at end."),
            TokenCode::Error(_) => (),
            _ => println!(" at line {} | position: {}", token.line + 1, token.lexeme),
        }

        println!("{}", msg);
    }
}
