use crate::chunk::OpCode;
use crate::parser::scanner::TokenCode;
use crate::compiler::Parser;
use crate::value::Value;

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

impl Precedence {
    fn increment(&mut self) {
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

pub fn none(parser: &mut Parser) {
    parser.error("expected expression.")
}

fn grouping(parser: &mut Parser) {
    parser.expression();
    parser.consume(TokenCode::RightParen, "expected ')' after expression.");
}

pub fn number(parser: &mut Parser) {
    // gets slice containing token number (token start .. token length);
    let value = &parser.chars
                        .unwrap()[parser
                                .previous
                                .unwrap()
                                .start..parser
                                        .previous
                                        .unwrap()
                                        .start + parser
                                                .previous
                                                .unwrap()
                                                .length];

    if value.contains(&'.') {
        let str_value: String = value
                                .iter()
                                .collect();
        let float_value: f64 = str_value
                                .parse()
                                .expect("invalid float value.");

        parser.emit_constant(&Value::Float(float_value));
    } else {
        let str_value: String = value
                                .iter()
                                .collect();
        let int_value: i32 = str_value
                                .parse()
                                .expect("invalid int value.");

        parser.emit_constant(&Value::Int(int_value));
    }
}

fn unary(parser: &mut Parser) {
    let operator_type = parser.previous.unwrap().code;

    parser.parse_precedence(Precedence::Unary);

    match operator_type {
        TokenCode::Minus => parser.emit_byte(OpCode::OpNegate),
        _ => (),
    }
}

pub fn binary(parser: &mut Parser) {
    let operator_type = parser.previous.unwrap().code;

    let mut rule = get_rule(&operator_type);
    rule.precedence.increment();

    parser.parse_precedence(rule.precedence);

    if let Some(token) = Some(operator_type) {
        match token {
            TokenCode::Plus => parser.emit_byte(OpCode::OpAdd),
            TokenCode::Minus => { parser.emit_byte(OpCode::OpNegate); parser.emit_byte(OpCode::OpAdd) },
            TokenCode::Star => parser.emit_byte(OpCode::OpMultiply),
            TokenCode::Slash => parser.emit_byte(OpCode::OpDivide),
            _ => (),
        }
    }
}

pub fn get_rule(token_code: &TokenCode) -> ParseRule {
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
       _ => panic!("invalid rule identified."),
    }
}