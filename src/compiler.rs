use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::scanner::*;
use crate::vm::InterpretResult;

#[derive(PartialEq, PartialOrd)]
// lower to high precedence order
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

struct Parser {
    current: Option<Token>,
    previous: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

pub fn compile(chars: &Vec<char>) -> (Chunk, InterpretResult) {
    let mut scanner = Scanner::new();
    let mut parser = Parser::new();
    let mut chunk = Chunk::new();

    parser.advance(chars, &mut scanner);
    parser.expression();
    parser.consume(TokenCode::Eof, chars, &mut scanner, "expected end of expression.");
    parser.end_compiler(&mut chunk);
    if parser.had_error { return (chunk, InterpretResult::RuntimeError) }

    (chunk, InterpretResult::Ok)
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, chars: &Vec<char>, scanner: &mut Scanner) {
        self.previous = self.current;

        loop {
            self.current = Some(scanner.scan_token(chars));

            if let Some(current) = self.current {
                if current.code != TokenCode::Error { break }
            }

            self.error("error advancing token.");
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn consume(&mut self, 
        token_code: TokenCode, 
        chars: &Vec<char>, 
        scanner: &mut Scanner, 
        msg: &str) 
    {
        if self.current.unwrap().code == token_code { self.advance(chars, scanner) }

        self.error(msg);
    }

    fn emit_byte(&self, chunk: &mut Chunk, code: OpCode) {
        chunk.write(code, self.current.unwrap().line);
    }

    fn end_compiler(&mut self, chunk: &mut Chunk) {
        self.emit_byte(chunk, OpCode::OpReturn);
    }

    fn grouping(&mut self, chars: &Vec<char>, scanner: &mut Scanner) {
        self.expression();
        self.consume(TokenCode::RightParen, chars, scanner, "expected ')' after expression.");
    }

    // NOTE possibly adds support for values != i32;
    fn number(&self, chars: &Vec<char>, chunk: &mut Chunk) {
        let value = chars[self.previous.unwrap().start] as i32;

        self.emit_constant(&value, chunk);
    }

    fn unary(&mut self, chunk: &mut Chunk) {
        self.parse_precedence(Precedence::Unary);
        let operator_type = self.previous.unwrap().code;

        self.expression();

        match operator_type {
            TokenCode::Minus => self.emit_byte(chunk, OpCode::OpNegate),
            _ => (),
        }
    }

    fn binary(&self, chunk: &mut Chunk) {
        let operator_type = self.previous.expect("empty token.").code;
        let rule = self.get_rule(&operator_type);
        self.parse_precedence(rule.precedence+=1);

        if let Some(token) = Some (operator_type) {
            match token {
                TokenCode::Plus => self.emit_byte(chunk, OpCode::OpAdd),
                // REVIEW possible operation mismatch behavior 
                TokenCode::Minus => self.emit_byte(chunk, OpCode::OpAdd),
                TokenCode::Star => self.emit_byte(chunk, OpCode::OpMultiply),
                TokenCode::Slash => self.emit_byte(chunk, OpCode::OpDivide),
                _ => (),
            }
        }
    }

    fn get_rule(&self, code: &TokenCode) {

    }

    fn parse_precedence(&self, precedence: Precedence) {

    }

    fn emit_constant(&self, value: &i32, chunk: &mut Chunk) {
        let const_index = chunk.write_constant(Value::Int(*value));
        self.emit_byte(chunk, OpCode::OpConstant(const_index));
    }

    fn error(&self, msg: &str) {
        if self.panic_mode { return }

        let token = self.current.unwrap();
        print!("[{}] error", token.line);

        match token.code {
            TokenCode::Eof => println!(" at end."),
            TokenCode::Error => (),
            _ => println!(" at {} {}", token.length, token.start),
        }

        println!("{}", msg);
    }
}