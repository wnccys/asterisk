use std::{cell::RefCell, fmt::Write, rc::Rc, thread::{self, current}, time::Duration};

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
        let mut local_name: Option<String> = None;

        if global == 0 { local_name = Some(self.previous.unwrap().lexeme.clone()); };

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

            if local_name.is_some() { self.mark_initialized(local_name.unwrap()); }
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

    /// Try to extract current type from TypeDef Token.
    ///
    /// Executed when explicit type definition is set with :
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

    /// Initialize Local Var by emitting DefineLocal
    /// 
    fn mark_initialized(&mut self, local_name: String) {
        if self.scopes.len() == 0 { return; }

        let local_index = self
            .scopes
            .last_mut()
            .unwrap()
            .get_local(local_name)
            .unwrap();

        self.emit_byte(OpCode::DefineLocal(local_index.borrow().0, local_index.borrow().1));
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

    /// Statement manager function
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
        } else if self.match_token(TokenCode::For) {
            self.for_statement();
        } else if self.match_token(TokenCode::If) {
            self.if_statement();
        } else if self.match_token(TokenCode::While) {
            self.while_statement();
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
    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenCode::SemiColon, "Expect ';' after value.");
        self.emit_byte(OpCode::Print);
    }

    /// The for loop statement handling
    /// 
    /// On every clause, we check if ; is present, what means the clause is omitted.
    /// First, it checks for the first clause, which is a var declaration or a expression which will be executed on every start of loop.
    /// After we state a loop jump, which is the jump made if the condition on (X; HERE; Z) is false, it must evaluate to a bool, or a compiler error on stack will be throw
    /// Last we 
    /// 
    fn for_statement(&mut self) {
        self.begin_scope();

        /* Match (HERE; Y; Z) */
        self.consume(TokenCode::LeftParen, "Expect '(' after 'for'.");
        if self.match_token(TokenCode::SemiColon) {
            // No initializer
        } else if self.match_token(TokenCode::Var) {
            self.var_declaration();
        } else {
            self.expression_statement();
        }

        /* 
            This is the condition evaluation itself, this is where the loop begins, intructionally speaking xD
        */
        let mut loop_start = self.chunk.code.len() - 1;
        /* 
            -1 is a fallback value, meaning the loop must not be patched, or better saying, the loop will not break.
        */
        let mut exit_jump: i32 = -1;
        /*  Verify if expression is present (x; HERE; y;) */
        if !self.match_token(TokenCode::SemiColon) {
            self.expression();
            self.consume(TokenCode::SemiColon, "Expect ';' after expression.");

            /* Jump out of the loop if condition is false */
            exit_jump = self.emit_jump(OpCode::JumpIfFalse(0)) as i32;
        }

        /* 
            As asterisk uses a single-pass compiler model, to run the increment clause we first execute the body, 
            jumping to the increment instruction right after.

            Here body_jump set a jump flag bytecode, we next take the index of the current instruction (body jump)
        */
        if !self.match_token(TokenCode::RightParen) {
            /* This jump is set on code, so the flow continues, the body jump is executed */
            /* Set body anchor */
            let body_jump = self.emit_jump(OpCode::Jump(0));
            /* Execute increment */
            let increment_start = self.chunk.code.len() -1;
            self.expression();
            self.emit_byte(OpCode::Pop);
            self.consume(TokenCode::RightParen, "Expect ')' after for clauses.");

            self.emit_loop(loop_start);
            loop_start = increment_start;
            /* Map body anchor, to be executed before the increment */
            self.patch_jump(body_jump, OpCode::Jump(0));
        }

        self.statement();
        self.emit_loop(loop_start);

        if exit_jump != -1 {
            self.patch_jump(exit_jump as usize, OpCode::JumpIfFalse(0));
            self.emit_byte(OpCode::Pop);
        }

        self.end_scope();
    }

    fn if_statement(&mut self) {
        self.consume(TokenCode::LeftParen, "Expect '(' after 'if'");
        self.expression();
        self.consume(TokenCode::RightParen, "Expect ')' after condition");

        /* 
            Keep track of where then jump is located by checking chunk.code.len() 
            This argument ByteCode is a placeholder, which will be lazy-populated by
            patch_jump function.
        */
        let then_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        /* Remove bool expression value used for verification from stack */
        self.emit_byte(OpCode::Pop);
        /* Execute code in then branch so we know how many jumps we need */ 
        self.statement();

        /* 
            Set jump to else branch.
            Even if else is not set explicitly it is compiled, executing nothing.
        */
        let else_jump = self.emit_jump(OpCode::Jump(0));

        /* 
            Set correct calculated offset to earlier set then_jump.
            This is needed because jump doesn't know primarily how many instructions to jump
        */
        self.patch_jump(then_jump, OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);

        if self.match_token(TokenCode::Else) { self.statement(); }
        self.patch_jump(else_jump, OpCode::Jump(0));
    }

    fn while_statement(&mut self) {
        /* The Bytecode index jump needs to go backward to restart loop */
        let loop_start = self.chunk.code.len() - 1;

        self.consume(TokenCode::LeftParen, "Expect '(' after 'while'");
        self.expression();
        self.consume(TokenCode::RightParen, "Expect ')' after condition");


        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);
        self.statement();

        self.emit_loop(loop_start);
        self.patch_jump(exit_jump, OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);
    }

    /// Evaluate expression and consume ';' token.
    ///
    /// Emit: OpCode::Pop
    ///
    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenCode::SemiColon, "Expect ';' after expression.");
        // if self.scopes.len() == 0 { self.emit_byte(OpCode::Pop); }
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

    /// This is the Ruler core itself, it orchestrate the expressions' values.
    /// 
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

    /// Emit jump instruction and return it's index on chunk.code
    /// 
    pub fn emit_jump(&mut self, instruction: OpCode) -> usize {
        /* Instruction */
        self.emit_byte(instruction);

        /* Return instruction count */
        return self.chunk.code.len() -1;
    }

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::Loop(self.chunk.code.len() - loop_start));
    }

    /// Calculate jump after evaluate conditional branch and set it to jump instruction.
    /// 
    fn patch_jump(&mut self, offset: usize, instruction: OpCode) {
        let jump = self.chunk.code.len() - offset;

        if jump > usize::MAX { self.error("Max jump bytes reached.") }

        match instruction {
            OpCode::JumpIfFalse(_) =>   self.chunk.code[offset] = OpCode::JumpIfFalse(jump),
            OpCode::Jump(_) =>          self.chunk.code[offset] = OpCode::Jump(jump),
            _ => panic!("Invalid jump intruction."),
        }
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
