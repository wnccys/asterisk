use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::scanner::*;
use crate::vm::InterpretResult;
use crate::ruler:: { get_rule, ParseRule, Precedence };

pub struct Parser<'a> {
    current: Option<Token>,
    previous: Option<Token>,
    chars: Option<&'a mut Vec<char>>,
    chunk: Option<Chunk>,
    had_error: bool,
    panic_mode: bool,
}

pub fn compile(chars: &Vec<char>) -> (Chunk, InterpretResult) {
    let mut scanner = Scanner::new();
    let mut parser = Parser::new();
    let chunk = Chunk::new();

    parser.chunk = Some(chunk);

    parser.advance(chars, &mut scanner);
    parser.expression();
    parser.consume(TokenCode::Eof, chars, &mut scanner, "expected end of expression.");
    // parser.end_compiler(&mut chunk);
    parser.end_compiler();
    if parser.had_error { return (parser.chunk.unwrap(), InterpretResult::RuntimeError) }

    (parser.chunk.unwrap(), InterpretResult::Ok)
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            current: None,
            previous: None,
            chars: None,
            chunk: None,
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

    fn emit_byte(&mut self, code: OpCode) {
        self.chunk.as_mut().unwrap().write(code, self.current.unwrap().line);
    }

    fn emit_constant(&mut self, value: &i32) {
        let const_index = self.chunk.as_mut().unwrap().write_constant(Value::Int(*value));
        self.emit_byte(OpCode::OpConstant(const_index));
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn grouping(&mut self, chars: &Vec<char>, scanner: &mut Scanner) {
        self.expression();
        self.consume(TokenCode::RightParen, chars, scanner, "expected ')' after expression.");
    }

    // NOTE possibly adds support for values != i32 / forced coersion;
    pub fn number(&mut self) {
        let value = self.chars.as_ref().unwrap()[self.previous.unwrap().start] as i32;

        self.emit_constant(&value);
    }

    fn unary(&mut self) {
        // REVIEW see if it is going to be changed in the book;
        self.parse_precedence(Precedence::Unary);
        let operator_type = self.previous.unwrap().code;

        self.expression();

        match operator_type {
            TokenCode::Minus => self.emit_byte(OpCode::OpNegate),
            _ => (),
        }
    }

    fn binary(&self) {
        let operator_type = self.previous.expect("empty token.").code;
        let rule = get_rule(&operator_type, self);
        // TODO impl += 1 to precedence;
        self.parse_precedence(rule.precedence+=1);

        if let Some(token) = Some (operator_type) {
            match token {
                TokenCode::Plus => self.emit_byte(OpCode::OpAdd),
                // REVIEW possible operation mismatch behavior 
                TokenCode::Minus => self.emit_byte(OpCode::OpAdd),
                TokenCode::Star => self.emit_byte(OpCode::OpMultiply),
                TokenCode::Slash => self.emit_byte(OpCode::OpDivide),
                _ => (),
            }
        }
    }

    fn parse_precedence(&self, rule: ParseRule) {

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