use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::scanner::*;
use crate::vm::InterpretResult;
use crate::ruler::*;

#[derive(Debug)]
pub struct Parser<'a> {
    pub current: Option<Token>,
    pub previous: Option<Token>,
    pub chars: Option<&'a Vec<char>>,
    pub chunk: Option<Chunk>,
    pub scanner: Option<Scanner>,
    // TODO possibly add scanner as a field;
    pub had_error: bool,
    pub panic_mode: bool,
}

pub fn compile(chars: &Vec<char>) -> (Chunk, InterpretResult) {
    let mut parser = Parser::new();
    let chars = vec!['1'; 2];

    parser.chunk = Some(Chunk::new());
    parser.scanner = Some(Scanner::new());
    parser.chars = Some(&chars);
    parser.advance();
    // dbg!(parser);

    let rule = get_rule(&TokenCode::Number);
    (rule.prefix)(&mut parser);
    // let mut parser = Parser::new();
    // let scanner = Scanner::new();
    // let chunk = Chunk::new();

    // parser.chunk = Some(chunk);
    // parser.scanner = Some(scanner);
    // parser.chars = Some(chars);

    // parser.advance();
    // parser.expression();
    // parser.consume(TokenCode::Eof, "expected end of expression.");
    // parser.end_compiler();
    // if parser.had_error { return (parser.chunk.unwrap(), InterpretResult::RuntimeError) }

    (parser.chunk.unwrap(), InterpretResult::Ok)
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            current: None,
            previous: None,
            chars: None,
            chunk: None,
            scanner: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self){
        self.previous = self.current;

        loop {
            self.current = Some(
                self.scanner
                    .as_mut()
                    .unwrap()
                    .scan_token(self.chars.as_ref().unwrap())
            );

            if let Some(current) = self.current {
                if current.code != TokenCode::Error { break }
            }

            self.error("error advancing token.");
        }
    }

    pub fn expression(&mut self) {
        // self.parse_precedence(Precedence::Assignment);
    }

    pub fn consume(&mut self, token_code: TokenCode, msg: &str) 
    {
        if self.current.unwrap().code == token_code { self.advance(); }

        self.error(msg);
    }

    pub fn emit_byte(&mut self, code: OpCode) {
        self.chunk.as_mut().unwrap().write(code, self.current.unwrap().line);
    }

    pub fn emit_constant(&mut self, value: &i32) {
        let const_index = self.chunk.as_mut().unwrap().write_constant(Value::Int(*value));
        self.emit_byte(OpCode::OpConstant(const_index));
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    pub fn parse_precedence(&self, rule: ParseRule) {

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn number_ruler() {
//         let mut parser = Parser::new();
//         let chars = vec!['1'; 2];

//         parser.chunk = Some(Chunk::new());
//         parser.scanner = Some(Scanner::new());
//         parser.chars = Some(&chars);

//         let rule = get_rule(&TokenCode::Number);
//         println!("{:?}", rule);
//         (rule.prefix)(&mut parser);
//     }
// }

    // fn grouping(&mut self, chars: &Vec<char>, scanner: &mut Scanner) {
    //     self.expression();
    //     self.consume(TokenCode::RightParen, chars, scanner, "expected ')' after expression.");
    // }

    // // NOTE possibly adds support for values != i32 / forced coersion;
    // pub fn number(&mut self) {
    //     let value = self.chars.as_ref().unwrap()[self.previous.unwrap().start] as i32;

    //     self.emit_constant(&value);
    // }

    // fn unary(&mut self) {
    //     // REVIEW see if it is going to be changed in the book;
    //     self.parse_precedence(Precedence::Unary);
    //     let operator_type = self.previous.unwrap().code;

    //     self.expression();

    //     match operator_type {
    //         TokenCode::Minus => self.emit_byte(OpCode::OpNegate),
    //         _ => (),
    //     }
    // }

    // fn binary(&self) {
    //     let operator_type = self.previous.expect("empty token.").code;
    //     let rule = get_rule(&operator_type, self);
    //     // TODO impl += 1 to precedence;
    //     self.parse_precedence(rule.precedence+=1);

    //     if let Some(token) = Some (operator_type) {
    //         match token {
    //             TokenCode::Plus => self.emit_byte(OpCode::OpAdd),
    //             // REVIEW possible operation mismatch behavior 
    //             TokenCode::Minus => self.emit_byte(OpCode::OpAdd),
    //             TokenCode::Star => self.emit_byte(OpCode::OpMultiply),
    //             TokenCode::Slash => self.emit_byte(OpCode::OpDivide),
    //             _ => (),
    //         }
    //     }
    // }