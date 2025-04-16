use scanner::{Token, TokenCode, TokenStream};
use ruler::{get_rule, Precedence};

use crate::{
    chunk::{Chunk, OpCode}, errors::parser_errors::ParserResult, types::hash_table::HashTable, utils::parse_type, value::{Modifier, Primitive, Type, Value}
};

pub mod scanner;
pub mod ruler;

#[derive(Debug)]
pub struct Parser<'a> {
    pub chunk: Chunk,
    pub token_stream: TokenStream<'a>,
    pub current: Option<&'a Token>,
    pub previous: Option<&'a Token>,
    pub had_error: bool,
    pub panic_mode: bool,
    pub scope: Scope<'a>,
    /// String interning model
    ///
    pub _strings: &'a mut HashTable<String>,
}

/// Represents a block-scope.
///
#[derive(Debug)]
pub struct Local<'a> {
    pub token: &'a Token,
    pub modifier: Modifier,
    /// Scope depth of block where variable was defined.
    ///
    pub depth: u16,
}

impl<'a> Local<'a> {
    fn new(token: &'a Token, depth: u16, modifier: Modifier) -> Self {
        Local { modifier, token, depth }
    }
}

/// General scope handler.
///
#[derive(Debug)]
pub struct Scope<'a> {
    /// Represents all local variables, resolved dynamically at runtime, without a Constant Bytecode.
    ///
    pub locals: Vec<Local<'a>>,
    /// Represents how many locals are in the scope
    ///
    pub local_count: usize,
    /// Represents the number of blocks surrounding the chunk of code whose are being compiled.
    ///
    /// Note:. (0) = global scope.
    ///
    pub scope_depth: u16,
}

impl<'a> Default for Scope<'a> {
    fn default() -> Self {
        Scope {
            locals: vec![],
            local_count: 0,
            scope_depth: 0,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(_strings: &'a mut HashTable<String>, token_stream: TokenStream<'a>) -> Self {
        Parser {
            chunk: Chunk::default(),
            token_stream,
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
            _strings,
            scope: Scope::default(),
        }
    }

    /// Declaration Flow Order
    /// → classDecl
    ///    | funDecl
    ///    | varDecl
    ///    | statement ;
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
        let var_type: Type;

        // Checks if after consuming identifier '=' Token is present.
        if self.match_token(TokenCode::Equal) {
            self.expression();
            var_type = parse_type(self.chunk.constants.last().clone().unwrap());
        // Check for typedef
        } else if self.match_token(TokenCode::Colon) {
            var_type = self.parse_var_type();

            // Handle uninitialized but typed vars
            if self.match_token(TokenCode::Equal) {
                self.expression();
            }
        // Uninitialized and untyped variables handling
        } else {
            panic!("Uninitialized variables are not allowed.");
        }

        self.consume(
            TokenCode::SemiColon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(global, modifier, var_type);
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
        if self.scope.scope_depth == 0 {
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
        match self.current.unwrap().code {
            TokenCode::TypeDef(t) => {
                self.advance();
                t
            },
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
        if self.scope.scope_depth == 0 {
            return;
        }

        self.add_local(modifier);
    }

    // TODO Add variable shadowing support
    /// Set previous Token as local variable, assign it to compiler.locals, increasing Compiler's local_count
    ///
    fn add_local(&mut self, modifier: Modifier) {
        let mut local = Local::new(self.previous.unwrap(), self.scope.scope_depth, modifier);
        local.depth = self.scope.scope_depth;
        self.scope.locals.push(local);

        self.scope.local_count += 1;
    }

    /// Emit DefineGlobal ByteCode with provided index. (global variables only)
    ///
    ///
    pub fn define_variable(&mut self, name_index: usize, modifier: Modifier, var_type: Type) {
        if self.scope.scope_depth > 0 {
            return;
        }

        self.emit_byte(OpCode::DefineGlobal(name_index, modifier, var_type));
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

    /// Check for current identifier token variable name, walking backward in locals array.
    ///
    /// This function iterates over the Compiler's locals reverselly searching for a Token which
    /// matches parser.previous (self.previous) Token.
    ///
    /// Returns i32 because of -1 (No var name was found) conventional fallback.
    ///
    /// O(n)
    ///
    pub fn resolve_local(&mut self) -> Option<(i32, Modifier)> {
        for i in (0..self.scope.local_count).rev() {
            let local = &self.scope.locals[i];

            #[cfg(feature = "debug")]
            {
                dbg!(&local.name);
                dbg!(&self.previous.unwrap());
            }

            if self.identify_constant(&local.token, &self.previous.unwrap()) {
                return Some((i as i32, local.modifier));
            }
        }

        None
    }

    pub fn begin_scope(&mut self) {
        self.scope.scope_depth += 1;
    }

    /// Decrease compiler scope_depth sanitizing (pop) values from stack
    ///
    pub fn end_scope(&mut self) {
        self.scope.scope_depth -= 1;

        while self.scope.local_count > 0
            && self.scope.locals[self.scope.local_count - 1].depth > self.scope.scope_depth
        {
            self.emit_byte(OpCode::Pop);
            self.scope.local_count -= 1;
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

    pub fn identify_constant(&self, a: &Token, b: &Token) -> bool {
        if a.lexeme != b.lexeme {
            return false;
        }

        return a.code == b.code;
    }

    /// Check for errors and disassemble chunk if compiler is in debug mode.
    ///
    pub fn end_compiler(&mut self) -> Chunk {
        if !self.had_error {
            // STUB
            #[cfg(feature = "debug")]
            disassemble_chunk(self.chunk.as_ref().unwrap(), "code".to_string());
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
