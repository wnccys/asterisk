use std::{cell::RefCell, fmt::Write, rc::Rc, thread::{self, current}, time::Duration};

use ruler::{get_rule, Precedence};
use lexer::{Lexer, Token};

use crate::{
    chunk::{Chunk, OpCode},
    errors::parser_errors::ParserResult,
    types::hash_table::{hash_key, HashTable},
    utils::{parse_type, print::disassemble_chunk},
    value::{Function, FunctionType, Modifier, Primitive, Type, Value},
};

pub mod ruler;
pub mod lexer;

#[derive(Debug)]
pub struct Parser<R: std::io::Read> {
    pub function: Function,
    pub function_type: FunctionType,
    pub lexer: Option<Lexer<R>>,
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub scopes: Vec<Scope>,
}

impl<R: std::io::Read> Parser<R> {
    pub fn new(
        function: Function,
        function_type: FunctionType,
        lexer: Lexer<R>,
    ) -> Self {
        Parser {
            function,
            function_type,
            lexer: Some(lexer),
            current: Token::Eof,
            previous: Token::Eof,
            had_error: false,
            panic_mode: false,
            scopes: vec![],
        }
    }
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
    fn get_local(&self, lexeme: &String) -> Option<Rc<RefCell<(usize, Modifier)>>> {
        self.locals.get(lexeme)
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

impl<R: std::io::Read> Parser<R> {
    /// Declaration Flow Order
    /// → classDecl
    ///    | funDecl
    ///    | varDecl
    ///    | statement
    ///
    pub fn declaration(&mut self) {
        if self.match_token(Token::Fun) {
            self.fun_declaration();
        } else if self.match_token(Token::Var) {
            self.var_declaration();
        } else if self.match_token(Token::LeftBrace) {
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

    /// Where the fun starts
    /// 
    fn fun_declaration(&mut self) {
        let modifier = Modifier::Const;
        let name = match self.get_previous() {
            Token::Identifier(s) => s,
            _ => panic!("Expect function name")
        };
        let global_var = self.parse_variable(modifier, name);
        /* Let function as value available on top of stack */
        self.function(FunctionType::Fn);
        self.define_variable(global_var, modifier);
    }

    /// Basically, on every function call we create a new parser, which on a standalone way parse the token and return an 'standarized' function object which will be used later by VM packed in call stacks.
    /// 
    fn function(&mut self, function_t: FunctionType) {
        let func_name = match self.get_previous()
        {
            Token::Identifier(name) => name,
            _ => panic!("Expect function name.")
        };

        let current = self.get_current();
        let previous = self.get_previous();
        /* New parser creation, equivalent to initCompiler, it basically changes actual parser with a new one */
        let mut parser: Parser<R> = Parser {
            function: Function::new(func_name),
            function_type: function_t,
            lexer: self.lexer.take(),
            /* Temporally moves token_stream to inner parser */
            current,
            previous,
            had_error: false,
            panic_mode: false,
            scopes: vec![],
        };

        parser.begin_scope();
        parser.consume(Token::LeftParen, "Expect '(' after function name.");
        /* TODO Initialize parameters */
        if !parser.check(Token::RightParen) {
            let modifier = Modifier::Const;
            loop {
                parser.function.arity += 1;
                let local_name = match parser.get_current() {
                    Token::Identifier(name) => name,
                    _ => panic!("Could not parse arguments.")
                };
                parser.parse_variable(modifier, local_name.clone());

                parser.consume(Token::Colon, "Expect : Type specification on function signature.");

                let t = parser.parse_var_type();
                parser.emit_byte(OpCode::SetType(t));
                parser.mark_initialized(local_name);

                if !parser.match_token(Token::Comma) { break }
            }
        }
        parser.consume(Token::RightParen, "Expect ')' after function parameters.");
        parser.consume(Token::LeftBrace, "Expect '{' after function name.");
        parser.block();
        /* End-of-scope are automatically handled by block() */

        let function = Value {
            value: Primitive::Function(Rc::new(parser.end_compiler().unwrap())),
            _type: Type::Fn,
            modifier: Modifier::Const,
        };

        /* Re-gain ownership over Lexer and it's Tokens */
        self.lexer = parser.lexer.take();
        self.previous = parser.previous;
        self.current = parser.current;

        self.emit_constant(function);
    }

    /// Set new variable with SetGlobal or push a value to stack throught GetGlobal.
    ///
    fn var_declaration(&mut self) {
        let modifier = self.parse_modifier();
        let var_name = match self.get_previous() {
            Token::Identifier(s) => s,
            _ => panic!("Expect variable name.")
        };
        let global = self.parse_variable(modifier, var_name.clone());

        // Checks if after consuming identifier '=' Token is present.
        if self.match_token(Token::Equal) {
            self.expression();

        // Check for typedef
        } else if self.match_token(Token::Colon) {
            // Lazy-evaluated var type
            let t = self.parse_var_type();

            // Handle uninitialized but typed vars
            if self.match_token(Token::Equal) {
                self.expression();
            }

            self.emit_byte(OpCode::SetType(t));
        } else {
            panic!("Uninitialized variables are not allowed.");
        }

        self.consume(
            Token::SemiColon,
            "Expect ';' after variable declaration.",
        );

        if global == 0 { self.mark_initialized(var_name); return; }

        self.define_variable(global, modifier);
    }

    /// Match current Token for Modifier(Mut) / Identifier(Const).
    ///
    fn parse_modifier(&mut self) -> Modifier {
        match &self.current {
            Token::Modifier => {
                self.advance();

                Modifier::Mut
            }
            Token::Identifier(_) => Modifier::Const,
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
    fn parse_variable(&mut self, modifier: Modifier, name: String) -> usize {
        // Check if var is global
        if self.scopes.len() == 0 {
            return self.identifier_constant(name);
        }

        self.add_local(modifier, name);
        return 0;
    }

    /// Try to extract current type from TypeDef Token.
    ///
    /// Executed when explicit type definition is set with :
    ///
    pub fn parse_var_type(&mut self) -> Type {
        match self.get_current() {
            Token::TypeDef(t) => {
                self.advance();

                t
            }
            _ => panic!("Invalid Var Type."),
        }
    }

    /// Get variable's name by analising previous Token lexeme and emit it's Identifier as String to constants vector.
    ///
    fn identifier_constant(&mut self, name: String) -> usize {
        self.function.chunk.write_constant(Primitive::String(name.into()))
    }

    /// Set previous Token as local variable, assign it to compiler.locals, increasing Compiler's local_count
    ///
    fn add_local(&mut self, modifier: Modifier, name: String) {
        self.scopes.last_mut().unwrap().add_local(name, modifier);
    }

    /// Initialize Local Var by emitting DefineLocal
    /// 
    fn mark_initialized(&mut self, local_name: String) {
        if self.scopes.len() == 0 { return; }

        let local_index = self
            .scopes
            .last_mut()
            .unwrap()
            .get_local(&local_name)
            .unwrap();

        self.emit_byte(OpCode::DefineLocal(local_index.borrow().0, local_index.borrow().1));
    }

    /// Emit DefineGlobal ByteCode with provided index. (global variables only)
    ///
    ///
    pub fn define_variable(&mut self, name_index: usize, modifier: Modifier) {
        self.emit_byte(OpCode::DefineGlobal(name_index, modifier));
    }

    fn get_current(&mut self) -> Token {
        std::mem::replace(&mut self.current, Token::Eof)
    }

    fn get_previous(&mut self) -> Token {
        std::mem::replace(&mut self.previous, Token::Eof)
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
        if self.match_token(Token::Print) {
            self.print_statement();
        } else if self.match_token(Token::For) {
            self.for_statement();
        } else if self.match_token(Token::If) {
            self.if_statement();
        } else if self.match_token(Token::Return) {
            self.return_statement();
        } else if self.match_token(Token::While) {
            self.while_statement();
        } else if self.match_token(Token::Switch) {
            self.switch_statement();
        } else if self.check(Token::LeftBrace) {
            self.declaration();
        } else {
            self.expression_statement();
        }
    }

    pub fn syncronize(&mut self) {
        self.panic_mode = false;

        while self.current != Token::Eof {
            if self.previous == Token::SemiColon {
                match self.current {
                    Token::Class
                    | Token::Fun
                    | Token::Var
                    | Token::For
                    | Token::If
                    | Token::While
                    | Token::Print
                    | Token::Return => return,
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
        self.consume(Token::SemiColon, "Expect ';' after value.");
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
        self.consume(Token::LeftParen, "Expect '(' after 'for'.");
        if self.match_token(Token::SemiColon) {
            // No initializer
        } else if self.match_token(Token::Var) {
            self.var_declaration();
        } else {
            self.expression_statement();
        }

        /* 
            This is the condition evaluation itself, this is where the loop begins, intructionally speaking xD
        */
        let mut loop_start = self.function.chunk.code.len() - 1;
        /* 
            -1 is a fallback value, meaning the loop must not be patched, or better saying, the loop will not break.
        */
        let mut exit_jump: i32 = -1;
        /*  Verify if expression is present (x; HERE; y;) */
        if !self.match_token(Token::SemiColon) {
            self.expression();
            self.consume(Token::SemiColon, "Expect ';' after expression.");

            /* Jump out of the loop if condition is false */
            exit_jump = self.emit_jump(OpCode::JumpIfFalse(0)) as i32;
        }
        /* Pop only if middle cause (X; HERE: Y) is present */
        if exit_jump != -1 { self.emit_byte(OpCode::Pop); }

        /* 
            As asterisk uses a single-pass compiler model, to run the increment clause we first execute the body, 
            jumping to the increment instruction right after.

            Here body_jump set a jump flag bytecode, we next take the index of the current instruction (body jump)
        */
        if !self.match_token(Token::RightParen) {
            /* This jump is set on code, so the flow continues, the body jump is executed */
            /* Set jump over body */
            let body_jump = self.emit_jump(OpCode::Jump(0));
            /* Execute increment - this is executed after body */
            let increment_start = self.function.chunk.code.len() -1;
            /* Increment expression */
            self.expression();

            self.consume(Token::RightParen, "Expect ')' after for clauses.");

            /* This loop is the one which */
            self.emit_loop(loop_start);
            loop_start = increment_start;
            /* 
                Jump to the body.
                After this jump, the self.emit_loop(loop_start) come back to evaluate the increment instruction.
            */
            self.patch_jump(body_jump, OpCode::Jump(0));
        }

        self.consume(Token::LeftBrace, "Expect '{' start-of-block.");
        self.block();
        self.emit_loop(loop_start);

        if exit_jump != -1 {
            self.patch_jump(exit_jump as usize, OpCode::JumpIfFalse(0));
            self.emit_byte(OpCode::Pop);
        }

        self.end_scope();
    }

    fn if_statement(&mut self) {
        self.consume(Token::LeftParen, "Expect '(' after 'if'");
        self.expression();
        self.consume(Token::RightParen, "Expect ')' after condition");

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

        if self.match_token(Token::Else) { self.statement(); }
        self.patch_jump(else_jump, OpCode::Jump(0));
    }

    fn return_statement(&mut self) {
        if self.function_type == FunctionType::Script {
            self.error("Can't return from top-level code.");
        }

        if self.match_token(Token::SemiColon) {
            self.emit_return();
        } else {
            self.expression();
            self.consume(Token::SemiColon, "Expect ; after return value.");
            self.emit_byte(OpCode::Return);
        };
    }

    fn while_statement(&mut self) {
        /* The Bytecode index jump needs to go backward to restart loop */
        let loop_start = self.function.chunk.code.len() - 1;

        self.consume(Token::LeftParen, "Expect '(' after 'while'");
        self.expression();
        self.consume(Token::RightParen, "Expect ')' after condition");


        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);
        self.statement();
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump, OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);
    }

    fn switch_statement(&mut self) {
        self.begin_scope();

        self.consume(Token::LeftParen, "Expect '(' after switch clause.");
        self.expression();
        self.consume(Token::RightParen, "Expect ')' after expression.");
        self.consume(Token::LeftBrace, "Expect '{' start-of-block.");

        self.consume(Token::Case, "Expected 'case' statement.");
        /* This gets switch value to be compared with branch value on every iteration */
        self.expression();
        self.emit_byte(OpCode::PartialEqual);
        let stmt_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        /* 
            Statements doesnt let dangling values on stack, so no pop is needed. 
            Finally, the value available on top is going to be the expression() result one.
        */
        self.statement();
        self.patch_jump(stmt_jump, OpCode::JumpIfFalse(0));

        /* 
            Executed by getting the original switch value, copying it and comparing it with the branch expression value.
            Basically, when a branch is true, it's value is propagated until the end of loop.
        */
        while self.match_token(Token::Case) {
            /* The below jump is executed in order to skip the execution of the entire branch once a true value (from previous branch) is found. */
            let branch_jump = self.emit_jump(OpCode::JumpIfTrue(0));
            /* If conditional was indeeed false, pop it (old branch value) and continues */
            self.emit_byte(OpCode::Pop);

            /* This gets switch value to be compared with branch value on every iteration */
            self.expression();
            self.emit_byte(OpCode::PartialEqual);
            let stmt_jump = self.emit_jump(OpCode::JumpIfFalse(0));
            self.statement();
            self.patch_jump(stmt_jump, OpCode::JumpIfFalse(0));

            self.patch_jump(branch_jump, OpCode::JumpIfTrue(0));
            /* On final of loop, the expression value of branch is still available, once the pop is on next iteration */
        };

        /* If a true value was found, it will be available on top of stack, so we check if it is false. */
        let default_jump = self.emit_jump(OpCode::JumpIfTrue(0));

        if self.match_token(Token::Default) {
            self.statement();
        }

        self.patch_jump(default_jump, OpCode::JumpIfTrue(0));
        /* Pop default_jump conditional */
        self.emit_byte(OpCode::Pop);

        /* As original switch value are available, pop */
        self.emit_byte(OpCode::Pop);

        self.consume(Token::RightBrace, "Expect '}' on end-of-block.");
        self.end_scope();
    }

    /// Evaluate expression and consume ';' token.
    ///
    /// Emit: OpCode::Pop
    ///
    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(Token::SemiColon, "Expect ';' after expression.");
        // if self.scopes.len() == 0 { self.emit_byte(OpCode::Pop); }
    }

    /// Calls declaration() until LeftBrace or EOF are found, consuming RightBrace on end.
    ///
    pub fn block(&mut self) {
        while !self.check(Token::RightBrace) && !self.check(Token::Eof) {
            self.declaration();
        }

        self.consume(Token::RightBrace, "Expected '}' end-of-block.");
    }

    /// Check if current Token matches argument Token.
    ///
    /// Advance parser current Token on match.
    ///
    pub fn match_token(&mut self, token: Token) -> bool {
        if !self.check(token) {
            return false;
        }
        self.advance();
        true
    }

    /// Compare current Token with param Token.
    ///
    pub fn check(&self, token: Token) -> bool {
        self.current == token
    }

    /// Scan new token and set it as self.current.
    ///
    pub fn advance(&mut self) {
        self.previous = self.get_current();

        self.current = self.lexer.as_mut().unwrap().next();

        #[cfg(feature = "debug-scan")]
        dbg!(&self.current);

        if let Token::Error(msg) = &self.current {
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

        let prefix_rule = get_rule(&self.previous).prefix;

        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule(self, can_assign);

        while precedence <= get_rule::<R>(&self.current).precedence {
            self.advance();

            let infix_rule = get_rule(&self.previous).infix;
            (infix_rule)(self, can_assign)
        }
    }

    /// Match token_code with self.current and advance if true.
    ///
    pub fn consume(&mut self, token_code: Token, msg: &str) {
        if self.current == token_code {
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
        self.function.chunk.write(code, self.lexer.as_ref().unwrap().line);
    }

    /// Write value to constant vec and set it's bytecode.
    ///
    /// Emit: OpCode::Constant
    ///
    pub fn emit_constant(&mut self, value: Value) {
        let const_index = self.function.chunk.write_constant(value.to_owned().value);

        self.emit_byte(OpCode::Constant(const_index));
    }

    /// Emit jump instruction and return it's index on chunk.code
    /// 
    pub fn emit_jump(&mut self, instruction: OpCode) -> usize {
        /* Instruction */
        self.emit_byte(instruction);

        /* Return instruction count */
        return self.function.chunk.code.len() -1;
    }

    /// Loop is a jump * -1, it goes backward to where the flag was set (loop_start which generally are self.chunk.code.len() - 1)
    /// 
    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::Loop(self.function.chunk.code.len() - 1 - loop_start));
    }

    /// Calculate jump after evaluate conditional branch and set it to jump instruction.
    /// 
    fn patch_jump(&mut self, offset: usize, instruction: OpCode) {
        let jump = self.function.chunk.code.len() - offset;

        if jump > usize::MAX { self.error("Max jump bytes reached.") }

        match instruction {
            OpCode::JumpIfTrue(_) =>   self.function.chunk.code[offset] = OpCode::JumpIfTrue(jump),
            OpCode::JumpIfFalse(_) =>   self.function.chunk.code[offset] = OpCode::JumpIfFalse(jump),
            OpCode::Jump(_) =>          self.function.chunk.code[offset] = OpCode::Jump(jump),
            _ => panic!("Invalid jump intruction."),
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Nil);
        self.emit_byte(OpCode::Return)
    }

    /// Check for errors and disassemble chunk if compiler is in debug mode.
    ///
    pub fn end_compiler(&mut self) -> Option<Function> {
        self.emit_return();

        if !self.had_error {
            // STUB
            #[cfg(feature = "debug")]
            disassemble_chunk(&self.function.chunk, self.function.name.to_string());
        }

        match self.had_error {
            false => Some(self.function.clone()),
            true => None,
        }
    }

    /// Panic on errors with panic_mode handling.
    ///
    pub fn error(&self, msg: &str) {
        if self.panic_mode {
            return;
        }

        let token = &self.current;
        match token {
            Token::Eof => println!(" at end."),
            Token::Error(_) => (),
            _ => println!(" at line {}", self.lexer.as_ref().unwrap().line),
        }

        println!("{}", msg);
    }
}
