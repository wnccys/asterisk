use crate::chunk::OpCode;
use crate::scanner::TokenCode;
use crate::compiler::Parser;
use std::ops::AddAssign;

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

impl AddAssign for Precedence {
    fn add_assign(&mut self, other: Precedence) {
        *self = match self {
            Self::None => Self::None,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}

#[derive(Debug)]
pub struct ParseRule {
    pub prefix: fn(&mut Parser), 
    pub infix: fn(&mut Parser), 
    pub precedence: Precedence,
}

pub fn get_rule<'a>(token_code: &TokenCode) -> ParseRule {
    match token_code {
        TokenCode::LeftParen => ParseRule {
            prefix: grouping,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::RightParen => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::LeftBrace => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::RightBrace => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Comma => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Dot => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Minus => ParseRule {
            prefix: unary,
            infix: binary,
            precedence: Precedence::Term,
        },
        TokenCode::Plus => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Term,
        },
        TokenCode::SemiColon => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Slash => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Factor,
        },
        TokenCode::Star => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Factor,
        },
        TokenCode::Bang => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::BangEqual => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Equal => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::EqualEqual => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Greater => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::GreaterEqual => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Less => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::LessEqual => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Identifier => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::String => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Number => ParseRule {
            prefix: number,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::And => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Class => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Else => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::False => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::For => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Fun => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::If => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Nil => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Or => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Print => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Return => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Super => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::This => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::True => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Var => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::While => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Error => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::Eof => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
       _ => panic!("not yet implemented."),
    }
}

pub fn none(parser: &mut Parser) {
    parser.error("expected expression.")
}

fn grouping(parser: &mut Parser) {
    parser.expression();
    parser.consume(TokenCode::RightParen, "expected ')' after expression.");
}

// NOTE possibly adds support for values != i32 / remove forced coersion;
pub fn number(parser: &mut Parser) {
    let value = parser.chars
                        .unwrap()[parser.previous.unwrap().start] as i32;

    parser.emit_constant(&value);
}

fn unary(parser: &mut Parser) {
    parser.parse_precedence(Precedence::Unary);
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
    parser.parse_precedence(rule.precedence+=1);

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