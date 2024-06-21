use crate::chunk::OpCode;
use crate::scanner::TokenCode;
use crate::compiler::Parser;

#[derive(Debug, PartialEq, PartialOrd)]
// lower to high precedence order
pub enum Precedence {
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

#[derive(Debug)]
pub struct ParseRule {
    pub prefix: fn(&mut Parser), 
    pub infix: fn(&mut Parser), 
    pub precedence: Precedence,
}

pub fn get_rule<'a>(token_code: &TokenCode) -> ParseRule {
    match token_code {
        TokenCode::Number => ParseRule {
            prefix: number,
            infix: none,
            precedence: Precedence::None,
        },
        _ => panic!("not yet implemented."),
    }
}

fn none(_parser: &mut Parser) {
    ()
}

fn grouping(parser: &mut Parser) {
    parser.expression();
    parser.consume(TokenCode::RightParen, "expected ')' after expression.");
}

// NOTE possibly adds support for values != i32 / forced coersion;
pub fn number(parser: &mut Parser) {
    println!("passed on number function!!!");
    let value = parser.chars
                        .as_ref()
                        .unwrap()[parser.previous.unwrap().start] as i32;

    parser.emit_constant(&value);
}

fn unary(parser: &mut Parser) {
    // parser.parse_precedence(Precedence::Unary);
    let operator_type = parser.previous.unwrap().code;

    parser.expression();

    match operator_type {
        TokenCode::Minus => parser.emit_byte(OpCode::OpNegate),
        _ => (),
    }
}

pub fn binary(parser: &mut Parser) {
    let operator_type = parser.previous
                                    .expect("empty token.")
                                    .code;
    let rule = get_rule(&operator_type);
    // TODO impl += 1 to precedence;
    // parser.parse_precedence(rule.precedence+=1);

    if let Some(token) = Some (operator_type) {
        match token {
            TokenCode::Plus => parser.emit_byte(OpCode::OpAdd),
            // REVIEW possible operation mismatch behavior 
            TokenCode::Minus => parser.emit_byte(OpCode::OpAdd),
            TokenCode::Star => parser.emit_byte(OpCode::OpMultiply),
            TokenCode::Slash => parser.emit_byte(OpCode::OpDivide),
            _ => (),
        }
    }
}