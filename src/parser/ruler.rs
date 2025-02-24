use crate::chunk::OpCode;
use crate::compiler::Parser;
use crate::parser::scanner::TokenCode;
use crate::value::Value;

#[derive(Debug, PartialEq, PartialOrd)]
/// Defines lower to higher operation precedence order
/// 
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

/// Determine the rules which each Token are equivalent to
/// 
#[derive(Debug)]
pub struct ParseRule {
    pub prefix: fn(&mut Parser, bool),
    pub infix: fn(&mut Parser, bool),
    pub precedence: Precedence,
}

fn none(_parser: &mut Parser, _can_assign: bool) {}

fn grouping(parser: &mut Parser, _can_assign: bool) {
    parser.expression();
    parser.consume(TokenCode::RightParen, "expected ')' after expression.");
}

fn number(parser: &mut Parser, _can_assign: bool) {
    // Gets slice containing token stringify'ed number (token start .. token length);
    let value = &parser.scanner.as_ref().unwrap().chars[parser.previous.unwrap().start
        ..parser.previous.unwrap().start + parser.previous.unwrap().length];

    if value.contains(&'.') {
        let str_value: String = value.iter().collect();
        let float_value: f64 = str_value.parse().expect("invalid float value.");

        parser.emit_constant(Value::Float(float_value));
    } else {
        let str_value: String = value.iter().collect();
        let int_value: i32 = str_value.parse().expect("invalid int value.");

        parser.emit_constant(Value::Int(int_value));
    }
}

fn unary(parser: &mut Parser, _can_assign: bool) {
    let operator_type = parser.previous.unwrap().code;

    parser.parse_precedence(Precedence::Unary);

    match operator_type {
        TokenCode::Bang => parser.emit_byte(OpCode::Not),
        TokenCode::Minus => parser.emit_byte(OpCode::Negate),
        _ => (),
    }
}

fn binary(parser: &mut Parser, _can_assign: bool) {
    let operator_type = parser.previous.unwrap().code;

    let mut rule = get_rule(&operator_type);
    rule.precedence.increment();

    parser.parse_precedence(rule.precedence);

    if let Some(token) = Some(operator_type) {
        match token {
            TokenCode::Plus => parser.emit_byte(OpCode::Add),
            TokenCode::Minus => {
                parser.emit_byte(OpCode::Negate);
                parser.emit_byte(OpCode::Add)
            }
            TokenCode::Star => parser.emit_byte(OpCode::Multiply),
            TokenCode::Slash => parser.emit_byte(OpCode::Divide),
            TokenCode::BangEqual => {
                parser.emit_byte(OpCode::Equal);
                parser.emit_byte(OpCode::Not);
            }
            TokenCode::EqualEqual => parser.emit_byte(OpCode::Equal),
            TokenCode::Greater => parser.emit_byte(OpCode::Greater),
            TokenCode::GreaterEqual => {
                parser.emit_byte(OpCode::Less);
                parser.emit_byte(OpCode::Not);
            }
            TokenCode::Less => parser.emit_byte(OpCode::Less),
            TokenCode::LessEqual => {
                parser.emit_byte(OpCode::Greater);
                parser.emit_byte(OpCode::Not);
            }
            _ => panic!("invalid binary call."),
        }
    }
}

fn literal(parser: &mut Parser, _can_assign: bool) {
    match parser.previous.unwrap().code {
        TokenCode::True => parser.emit_byte(OpCode::True),
        TokenCode::False => parser.emit_byte(OpCode::False),
        _ => panic!("invalid literal operation."),
    }
}

// TODO Set string interning model
fn string(parser: &mut Parser, _can_assign: bool) {
    let str = parser.scanner.as_ref().unwrap().chars[parser.previous.unwrap().start + 1
        ..parser.previous.unwrap().start + parser.previous.unwrap().length - 1]
        .to_owned();

    // match get_table_intern(parser) {
    //     Some(intern) => {
    //         let index = parser
    //             .chunk
    //             .as_mut()
    //             .unwrap()
    //             .write_constant(intern.value.clone());
    //         parser.emit_byte(OpCode::Constant(index));
    //     },
    //     None => {
    //     }
    // };

    let index = parser
        .chunk
        .as_mut()
        .unwrap()
        .write_constant(Value::String(str));
    parser.emit_byte(OpCode::Constant(index));
}

fn variable(parser: &mut Parser, can_assign: bool) {
    named_variable(parser, can_assign)
}

/// Distinguish between re-assign and get variable already set value
/// 
fn named_variable(parser: &mut Parser, can_assign: bool) {
    let (get_op, set_op): (OpCode, OpCode); 

    let var_index = parser.resolve_local();

    if var_index != -1 {
        get_op = OpCode::GetLocal(var_index as usize);
        set_op = OpCode::SetLocal(var_index as usize);
    } else {
        let var_index = parser.identifier_constant();

        get_op = OpCode::GetGlobal(var_index);
        set_op = OpCode::SetGlobal(var_index);
    }


    if can_assign && parser.match_token(TokenCode::Equal) {
        parser.expression();
        parser.emit_byte(set_op);
    } else {
        parser.emit_byte(get_op);
    }
}

// pub fn get_table_intern(parser: &mut Parser) -> Option<Rc<Entry>> {

// }

/// Define which tokens will call which functions on prefix or infix while it's precedence is being parsed.
/// 
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
            prefix: unary,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::BangEqual => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Equality,
        },
        TokenCode::Equal => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::EqualEqual => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Equality,
        },
        TokenCode::Greater => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Comparison,
        },
        TokenCode::GreaterEqual => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Comparison,
        },
        TokenCode::Less => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Comparison,
        },
        TokenCode::LessEqual => ParseRule {
            prefix: none,
            infix: binary,
            precedence: Precedence::Comparison,
        },
        TokenCode::Identifier => ParseRule {
            prefix: variable,
            infix: none,
            precedence: Precedence::None,
        },
        TokenCode::String => ParseRule {
            prefix: string,
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
            prefix: literal,
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
            prefix: literal,
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
        TokenCode::Comment => ParseRule {
            prefix: none,
            infix: none,
            precedence: Precedence::None,
        },
    }
}
