pub mod lexer;
pub mod ruler;
pub mod scope;

use std::{cell::RefCell};
#[allow(unused)]
use std::{rc::Rc, thread::{self, current}, time::Duration};

use lexer::{Lexer, Token};
use ruler::{get_rule, Precedence};

use crate::primitives::{primitive::UpValue, structs::Struct, types::Dyn};
#[allow(unused)]
use crate::{
    parser::scope::Scope,
    primitives::{
        functions::{Function, FunctionType},
        primitive::{Primitive},
        types::{Modifier, Type},
        value::Value,
    },
    utils::print::disassemble_chunk,
    vm::chunk::OpCode,
};

#[derive(Debug)]
pub struct Parser<R: std::io::Read> {
    pub function: Function,
    pub upvalues: Vec<UpValue>,
    pub function_type: FunctionType,
    pub up_context: Option<Box<Parser<R>>>,
    pub lexer: Option<Lexer<R>>,
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub scopes: Vec<Scope>,
}

impl<R: std::io::Read> Parser<R> {
    pub fn new(function: Function, function_type: FunctionType, lexer: Lexer<R>) -> Self {
        Parser {
            function,
            function_type,
            up_context: None,
            lexer: Some(lexer),
            upvalues: vec![],
            current: Token::Nil,
            previous: Token::Nil,
            had_error: false,
            scopes: vec![],
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
    pub fn declaration(mut self: Parser<R>) -> Parser<R> {
        if self.match_token(Token::Fun) {
            self = self.fun_declaration();
        } else if self.match_token(Token::Var) {
            self.var_declaration();
        } else if self.match_token(Token::StructDef) {
            self.define_struct();
        } else if self.match_token(Token::LeftBrace) {
            self.begin_scope();
            self = self.block();
            self.end_scope();
        } else {
            // Declaration Control Flow Fallback
            return self.statement()
        }

        self
    }

    /// Where the fun starts
    ///
    /// Define functions / closures
    /// 
    pub fn fun_declaration(mut self: Parser<R>) -> Parser<R> {
        let modifier = Modifier::Const;
        self.advance();

        let name = match self.get_previous() {
            Token::Identifier(s) => s,
            _ => self.error("Expect function name"),
        };
        let global_var = self.parse_variable(modifier, name.clone());

        self = self.function(FunctionType::Fn, name.clone());

        /* Let function as value available on top of stack */
        match global_var {
            Some(idx) => self.define_variable(idx, modifier, Type::Fn),
            None => self.mark_initialized(name, Type::Closure),
        }

        self
    }

    /// Basically, on every function call we create a new parser, 
    /// which on a standalone way parse the token and return an 'standarized' function object which 
    /// will be used later by VM packed in call stacks.
    ///
    fn function(mut self: Parser<R>, function_t: FunctionType, func_name: String) -> Parser<R> {
        // 'i' stands for inner
        let (
            i_function,
            i_lexer,
            i_previous,
            i_current,
            mut _self
        ) = {
            let current = self.get_current();
            let previous = self.get_previous();
            /* New parser creation, it basically changes actual parser with a new one */
            let mut parser: Parser<R> = Parser {
                function: Function::new(func_name),
                lexer: self.lexer.take(),
                up_context: Some(Box::new(self)),
                function_type: function_t,
                upvalues: vec![],
                /* Temporally moves token_stream to inner parser */
                current,
                previous,
                had_error: false,
                scopes: vec![],
            };

            parser.begin_scope();
            parser.consume(Token::LeftParen, "Expect '(' after function name.");
            if !parser.check(Token::RightParen) {
                let modifier = Modifier::Const;
                loop {
                    parser.function.arity += 1;
                    let local_name = match parser.get_current() {
                        Token::Identifier(name) => name,
                        _ => parser.error("Could not parse arguments."),
                    };
                    parser.advance();
                    parser.parse_variable(modifier, local_name.clone());

                    /* Type defs: (a: x, b: y, c: z) */
                    parser.consume(
                        Token::Colon,
                        "Expect: Type specification on function signature.",
                    );

                    let t = parser.parse_var_type();
                    parser.mark_initialized(local_name, t);

                    if !parser.match_token(Token::Comma) {
                        break;
                    }
                }
            }
            parser.consume(Token::RightParen, "Expect ')' after function parameters.");
            parser.consume(Token::LeftBrace, "Expect '{' after function name.");
            parser = parser.block();
            /* End-of-scope are automatically handled by block() */

            let function = Value {
                value: Primitive::Function(Rc::new(parser.end_compiler())),
                _type: Type::Fn,
                modifier: Modifier::Const,
            };

            (
                function,
                parser.lexer.take(),
                parser.previous,
                parser.current,
                parser.up_context.take().unwrap()
            )
        };

    /* Re-assign ownership over Lexer and it's Tokens to parent Parser */
        _self.lexer = i_lexer;
        _self.previous = i_previous;
        _self.current = i_current;

        _self.emit_constant(i_function);

    /* Differs between fn and closure */
        if _self.scopes.len() > 0 {
            _self.emit_byte(OpCode::Closure);
        }

        *_self
    }

    /// Set new variable with SetGlobal or push a value to stack throught GetGlobal.
    ///
    pub fn var_declaration(&mut self) {
        let modifier = self.parse_modifier();
        let var_name = match self.get_current() {
            Token::Identifier(s) => s,
            _ => self.error("Expect variable name."),
        };
        let global = self.parse_variable(modifier, var_name.clone());
        let mut _type = None;

        self.advance();

        // Checks if after consuming identifier '=' Token is present.
        if self.match_token(Token::Equal) {
            self.expression();

        // Check for typedef
        } else if self.match_token(Token::Colon) {
            // Lazy-evaluated var type
            _type = Some(self.parse_var_type());

            // Handle uninitialized but typed vars
            if self.match_token(Token::Equal) {
                self.expression();
            }
        } else {
            self.error("Uninitialized variables are not allowed.");
        }

        self.consume(Token::SemiColon, "Expect ';' after variable declaration.");

        if global.is_none() {
            return self.mark_initialized(var_name, _type.unwrap_or_default());
        }

        self.define_variable(global.unwrap(), modifier, _type.unwrap_or_default());
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
            _ => self.error("Error parsing variable."),
        }
    }

    /// Set local/global variables to scopes by emitting new constant (if global), returning it's index.
    ///
    /// Local Variables are auto-declared so to speak, It follows a convention on var declaration
    /// and scope-flow, so there's no need to set them to constants vector, the Compiler (Parser) object already take care
    /// of which indexes behaves to which variables by scope_depth and local_count when local vars are set.
    ///
    /// Return 0 when variable is local, which will be ignored by define_variable(), so it is not set to constants.
    ///
    fn parse_variable(&mut self, modifier: Modifier, name: String) -> Option<usize> {
        // Check if var is global
        if self.scopes.len() == 0 {
            return self.identifier_constant(name);
        }

        self.add_local(modifier, name);
        return None;
    }

    /// Try to extract current type from TypeDef Token recursivelly.
    ///
    /// Executed when explicit type definition is set with :
    ///
    pub fn parse_var_type(&mut self) -> Type {
        match self.get_current() {
            Token::Ampersand => {
                self.advance();
                Type::Ref(Rc::new(self.parse_var_type()))
            }
            Token::TypeDef(t) => {
                self.advance();
                t
            }
            _ => self.error("Invalid Var Type."),
        }
    }

    /// Receive variable's name and emit it's Identifier as String to constants vector.
    ///
    fn identifier_constant(&mut self, name: String) -> Option<usize> {
        Some(
            self.function
                .chunk
                .write_constant(Primitive::String(name.into())),
        )
    }

    /// Set previous Token as local variable, assign it to compiler.locals, increasing Compiler's local_count
    ///
    fn add_local(&mut self, modifier: Modifier, name: String) {
        let mut total_locals = 0;
        for i in self.scopes.iter() {
            total_locals += i.local_count;
        }

        // Add var to last scope, as the file is read from top to bottom
        self.scopes
            .last_mut()
            .unwrap()
            .add_local(name, modifier, total_locals);
    }

    /// Initialize Local Var by emitting DefineLocal, 
    /// it basically 'reserves' a slot on stack before the value is even in there.
    ///
    fn mark_initialized(&mut self, local_name: String, _type: Type) {
        let local_index = self
            .scopes
            .last_mut()
            .unwrap()
            .get_local(&local_name)
            .unwrap();

        // (idx on stack, Modifier, Type)
        self.emit_byte(OpCode::DefineLocal(
            local_index.borrow().0,
            local_index.borrow().1,
            _type
        ));
    }

    /// Emit DefineGlobal ByteCode with provided index. (global variables only)
    ///
    pub fn define_variable(&mut self, name_index: usize, modifier: Modifier, _type: Type) {
        self.emit_byte(OpCode::DefineGlobal(name_index, modifier, _type));
    }

    /// Build struct blueprint by parsing name and it's types
    /// 
    pub fn define_struct(&mut self) {
        let name = match self.get_current() {
            Token::Identifier(s) => s,
            _ => self.error("Expect struct name."),
        };

        let mut field_count = 0;
        let mut field_indices = std::collections::HashMap::<String, (Type, usize)>::new();

        self.advance();
        self.consume(Token::LeftBrace, "Expect '{'.");

        // If field type has dynamically resolved types
        let mut dyn_count = 0usize;

        // Struct fields parsing
        while !self.check(Token::RightBrace) {
            // Identifier
            let tok = self.get_current();

            // :
            self.advance();
            self.consume(Token::Colon, "Expect ':'.");

            let _type = match self.get_current() {
                Token::TypeDef(t) => t,
                Token::Identifier(id) => {
                    dyn_count += 1;

                    // get global or get local
                    self.previous = Token::Identifier(id);
                    let rule = get_rule::<R>(&self.previous).prefix;
                    rule(self, false);

                    Type::Dyn(Dyn::default())
                },
                _ => self.error("Expect field type."),
            };

            // : or }
            self.advance();

            match tok {
                Token::Identifier(id) => {
                    field_indices.insert(id, (_type, field_count.clone()));
                    field_count += 1;
                }
                _ => self.error("Invalid token when defining struct."),
            }

            if !self.check(Token::RightBrace) {
                self.consume(Token::Comma, "Expect ',' after field definition.");
            }
        }

        self.consume(Token::RightBrace, "Expect '}' after struct fields.");

        let _struct = Struct {
            name: name.clone(),
            field_count,
            field_indices,
        };
        let is_global = self.scopes.len() == 0;
        let global_idx = self.parse_variable(Modifier::Const, name.clone());

        self.emit_constant(_struct.into());

        if dyn_count > 0 {
            self.emit_byte(OpCode::ParseStructDyn(dyn_count));
        }

        if is_global {
            self.define_variable(global_idx.unwrap(), Modifier::Const, Type::Struct);
        } else {
            self.mark_initialized(name, Type::Struct);
        }
    }

    fn get_current(&mut self) -> Token {
        std::mem::replace(&mut self.current, Token::Nil)
    }

    fn get_previous(&mut self) -> Token {
        std::mem::replace(&mut self.previous, Token::Nil)
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    /// Decrease compiler scope_depth sanitizing (pop) values from stack
    ///
    pub fn end_scope(&mut self) {
        /* Remove scope Locals when it ends */
        while self.scopes.last().unwrap().local_count > 0 {
            self.emit_byte(OpCode::Pop);
            self.scopes.last_mut().unwrap().local_count -= 1;
        }

        self.scopes.pop();
    }

    /// Iterates over all parser scope's searching for local variable, returning it's (index, Mod)
    /// 
    pub fn resolve_local(&self, var_name: &String) -> Option<Rc<RefCell<(usize, Modifier)>>> {
        let mut local = None;

        for scope in self.scopes.iter().rev() {
            local = scope.get_local(var_name);

            if local.is_some() { break; }
        }

        local
    }

    pub fn resolve_upvalue(&mut self, name: &String) -> Option<usize> {
        if self.up_context.is_none() { return None; };

        let local = 
            self
            .up_context
            .as_mut()
            .unwrap()
            .resolve_local(name);

        if local.is_some() {
            return Some(
                    self
                    .up_context
                    .as_mut()
                    .unwrap()
                    .add_upvalue(local.unwrap().borrow().0, true)
                );
        };

        let upvalue =
            self 
            .up_context
            .as_mut()
            .unwrap()
            .resolve_upvalue(name);

        if upvalue.is_some() {
            return Some(self.add_upvalue(local.unwrap().borrow().0, false));
        }

        None
    }

    pub fn add_upvalue(&mut self, index: usize, is_local: bool) -> usize {
        if self.upvalues.iter().find(
            |up| up.index == index && up.is_local == is_local
        ).is_some() { return index; }

        let upvalue = UpValue { index, is_local };
        self.upvalues.push(upvalue);
        self.function.upv_count += 1;

        index
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
    pub fn statement(mut self: Parser<R>) -> Parser<R> {
        if self.match_token(Token::Print) {
            self.print_statement();
        } else if self.match_token(Token::For) {
            return self.for_statement();
        } else if self.match_token(Token::If) {
            return self.if_statement()
        } else if self.match_token(Token::Return) {
            self.return_statement();
        } else if self.match_token(Token::While) {
            return self.while_statement();
        } else if self.match_token(Token::Switch) {
            return self.switch_statement();
        } else if self.check(Token::LeftBrace) {
            return self.declaration();
        } else {
            self.expression_statement();
        }

        self
    }

    // pub fn syncronize(&mut self) {
    //     self.error_handler.panic_mode = false;

    //     while self.current != Token::Eof {
    //         if self.previous == Token::SemiColon {
    //             match self.current {
    //                 Token::Class
    //                 | Token::Fun
    //                 | Token::Var
    //                 | Token::For
    //                 | Token::If
    //                 | Token::While
    //                 | Token::Print
    //                 | Token::Return => return,
    //                 _ => (),
    //             }
    //         }

    //         self.advance();
    //     }
    // }

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
    fn for_statement(mut self: Parser<R>) -> Parser<R> {
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
        if exit_jump != -1 {
            self.emit_byte(OpCode::Pop);
        }

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
            let increment_start = self.function.chunk.code.len() - 1;
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
        self = self.block();
        self.emit_loop(loop_start);

        if exit_jump != -1 {
            self.patch_jump(exit_jump as usize, OpCode::JumpIfFalse(0));
            self.emit_byte(OpCode::Pop);
        }

        self.end_scope();

        self
    }

    fn if_statement(mut self: Parser<R>) -> Parser<R> {
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
        let mut _self = Self::statement(self);

        /*
            Set jump to else branch.
            Even if else is not set explicitly it is compiled, executing nothing.
        */
        let else_jump = _self.emit_jump(OpCode::Jump(0));

        /*
            Set correct calculated offset to earlier set then_jump.
            This is needed because jump doesn't know primarily how many instructions to jump
        */
        _self.patch_jump(then_jump, OpCode::JumpIfFalse(0));
        _self.emit_byte(OpCode::Pop);

        if _self.match_token(Token::Else) {
            let mut __self =Self::statement(_self);
            __self.patch_jump(else_jump, OpCode::Jump(0));

            return __self;
        }
        _self.patch_jump(else_jump, OpCode::Jump(0));

        _self
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

    fn while_statement(mut self: Parser<R>) -> Parser<R> {
        /* The Bytecode index jump needs to go backward to restart loop */
        let loop_start = self.function.chunk.code.len() - 1;

        self.consume(Token::LeftParen, "Expect '(' after 'while'");
        self.expression();
        self.consume(Token::RightParen, "Expect ')' after condition");

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);

        self = self.statement();
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump, OpCode::JumpIfFalse(0));
        self.emit_byte(OpCode::Pop);
        
        self
    }

    fn switch_statement(mut self: Parser<R>) -> Parser<R> {
        self.begin_scope();

        self.consume(Token::LeftParen, "Expect '(' after switch clause.");
        self.expression();
        self.consume(Token::RightParen, "Expect ')' after expression.");
        self.consume(Token::LeftBrace, "Expect '{' start-of-block.");

        self.consume(Token::Case, "Expected 'case' statement.");
        /* This gets switch value to be compared with branch value on every iteration */
        self.expression();
        self.consume(Token::Arrow, "Expect '=>' after expression.");
        self.emit_byte(OpCode::PartialEqual);
        let stmt_jump = self.emit_jump(OpCode::JumpIfFalse(0));
        /*
            Statements doesnt let dangling values on stack, so no pop is needed.
            Finally, the value available on top is going to be the expression() result one.
        */
        // Validate block();
        self = self.statement();
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
            self.consume(Token::Arrow, "Expect '=>' after expression.");
            self.emit_byte(OpCode::PartialEqual);
            let stmt_jump = self.emit_jump(OpCode::JumpIfFalse(0));

            // Validate block();
            self = self.statement();
            self.patch_jump(stmt_jump, OpCode::JumpIfFalse(0));

            self.patch_jump(branch_jump, OpCode::JumpIfTrue(0));
            /* On final of loop, the expression value of branch is still available, once the pop is on next iteration */
        }

        /* If a true value was found, it will be available on top of stack, so we check if it is false. */
        let default_jump = self.emit_jump(OpCode::JumpIfTrue(0));

        if self.match_token(Token::Default) {
            self.consume(Token::Arrow, "Expect '=>' after expression.");
            self = self.statement();
        }

        self.patch_jump(default_jump, OpCode::JumpIfTrue(0));
        /* Pop default_jump conditional */
        self.emit_byte(OpCode::Pop);

        /* As original switch value are available, pop */
        self.emit_byte(OpCode::Pop);

        self.consume(Token::RightBrace, "Expect '}' on end-of-block.");
        self.end_scope();

        self
    }

    /// Evaluate expression and consume ';' token.
    ///
    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(Token::SemiColon, "Expect ';' after expression.");
    }

    /// Calls declaration() until LeftBrace or EOF are found, consuming RightBrace on end.
    ///
    pub fn block(mut self: Parser<R>) -> Parser<R> {
        while !self.check(Token::RightBrace) && !self.check(Token::Eof) {
            self = self.declaration();
        }

        self.consume(Token::RightBrace, "Expected '}' end-of-block.");

        self
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

    /// This is the Ruler core itself, it orchestrate the expressions' values / order.
    ///
    pub fn parse_precedence(&mut self, precedence: Precedence) {
        #[cfg(feature = "debug-expr")]
        println!("\n parsing precedence for {:?}", &self.previous);
        self.advance();

        let prefix_rule = get_rule(&self.previous).prefix;

        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule(self, can_assign);

        #[cfg(feature = "debug-expr")] {
            println!("just executed it's precedence.");
            println!("now {:?}, {:?}", &self.current, &self.previous);
        }

        while precedence <= get_rule::<R>(&self.current).precedence {
            #[cfg(feature = "debug-expr")] {
                println!("=== entered loop ===");
                println!("precedence of {:?} is lower than {:?}, executing infix", &self.previous, &self.current);
            }

            self.advance();

            let infix_rule = get_rule(&self.previous).infix;
            infix_rule(self, can_assign)
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
        self.function.chunk.write(code);
    }

    /// Write value to constant vec and set it's bytecode.
    ///
    /// Emit: OpCode::Constant
    ///
    pub fn emit_constant(&mut self, value: Value) -> usize {
        let const_index = self.function.chunk.write_constant(value.to_owned().value);

        self.emit_byte(OpCode::Constant(const_index));
        const_index
    }

    /// Emit jump instruction and return it's index on chunk.code
    ///
    pub fn emit_jump(&mut self, instruction: OpCode) -> usize {
        /* Instruction */
        self.emit_byte(instruction);

        /* Return instruction count */
        return self.function.chunk.code.len() - 1;
    }

    /// Loop is a jump * -1, it goes backward to where the flag was set (loop_start which generally are self.chunk.code.len() - 1)
    ///
    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::Loop(
            self.function.chunk.code.len() - 1 - loop_start,
        ));
    }

    /// Calculate jump after evaluate conditional branch and set it to jump instruction.
    ///
    fn patch_jump(&mut self, offset: usize, instruction: OpCode) {
        let jump = self.function.chunk.code.len() - offset;

        if jump > usize::MAX {
            self.error("Max jump bytes reached.")
        }

        match instruction {
            OpCode::JumpIfTrue(_) => self.function.chunk.code[offset] = OpCode::JumpIfTrue(jump),
            OpCode::JumpIfFalse(_) => self.function.chunk.code[offset] = OpCode::JumpIfFalse(jump),
            OpCode::Jump(_) => self.function.chunk.code[offset] = OpCode::Jump(jump),
            _ => self.error("Invalid jump intruction."),
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Nil);
        self.emit_byte(OpCode::Return)
    }

    /// Check for errors and disassemble chunk if compiler is in debug mode.
    ///
    pub fn end_compiler(&mut self) -> Function {
        self.emit_return();

        if !self.had_error {
            // STUB
            #[cfg(feature = "debug")]
            disassemble_chunk(&self.function.chunk, self.function.name.to_string());
        }

        std::mem::replace(&mut self.function, Function::default())
    }

    /// Panic on errors with panic_mode handling.
    ///
    pub fn error(&mut self, msg: &str) -> ! {
        let token = &self.current;
        let mut curr_line = self.lexer.as_ref().unwrap().line;
        if curr_line == 0 { curr_line = 1 };

        let complement = match token {
            Token::Eof => String::from(" at end."),
            Token::Error(s) => format!("{} at line {}", s, curr_line),
            _ => format!("at line {}", curr_line)
        };

        panic!(
            "{}",
            format!(
                "{msg} | {complement} -> {}",
                self.lexer.as_mut().unwrap().curr_tok()
            )
        );
    }
}