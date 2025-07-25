use crate::parser::Parser;
use crate::primitives::primitive::{Primitive};
use crate::primitives::types::{Modifier, Type};
use crate::primitives::value::Value;
use crate::vm::chunk::OpCode;

use super::lexer::Token;

#[derive(Debug, PartialEq, PartialOrd)]
/// Defines lower to higher operation precedence order.
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
            Self::Primary => panic!("Cannot increment primary precedence."),
        }
    }
}

/// Determine which Rules are equivalent to which Token.
///
#[derive(Debug)]
pub struct ParseRule<R: std::io::Read> {
    pub prefix: fn(&mut Parser<R>, bool),
    pub infix: fn(&mut Parser<R>, bool),
    pub precedence: Precedence,
}

impl<R: std::io::Read> ParseRule<R> {
    /// Dummy prefix Rule.
    /// Every Token shall have a prefix, this one is a placeholder over the ones whose emit no Bytecodes.
    ///
    fn none(_parser: &mut Parser<R>, _can_assign: bool) {}

    /// Handle "()" precedence operator consuming ")" on end.
    ///
    fn grouping(parser: &mut Parser<R>, _can_assign: bool) {
        parser.expression();
        parser.consume(Token::RightParen, "expected ')' after expression.");
    }

    /// Get number as string and parses it to the number itself, setting it on the constants (immediatelly) and stack (via the generated bytecode) vector.
    ///
    fn number(parser: &mut Parser<R>, _can_assign: bool) {
        match parser.get_previous() {
            Token::Integer(i) => {
                parser.emit_constant(Value {
                    value: Primitive::Int(i),
                    _type: Type::Int,
                    modifier: Modifier::Unassigned,
                });
            }
            Token::Float(f) => {
                parser.emit_constant(Value {
                    value: Primitive::Float(f),
                    _type: Type::Float,
                    modifier: Modifier::Unassigned,
                });
            }
            _ => panic!("invalid number value."),
        }
    }

    /// Distinguish between negate (!) and minus (-) operations.
    ///
    fn unary(parser: &mut Parser<R>, _can_assign: bool) {
        let operator_type = parser.get_previous();
        parser.parse_precedence(Precedence::Unary);

        match operator_type {
            Token::Bang => parser.emit_byte(OpCode::Not),
            Token::Minus => parser.emit_byte(OpCode::Negate),
            _ => (),
        }
    }

    /// Parse math operators recursivelly until all operations are evaluated in correct order.
    ///
    fn binary(parser: &mut Parser<R>, _can_assign: bool) {
        let operator_type = &parser.get_previous();

        let mut rule: ParseRule<R> = get_rule(&operator_type);
        rule.precedence.increment();

        parser.parse_precedence(rule.precedence);

        if let Some(token) = Some(operator_type) {
            match token {
                Token::Plus => parser.emit_byte(OpCode::Add),
                Token::Minus => {
                    parser.emit_byte(OpCode::Negate);
                    parser.emit_byte(OpCode::Add)
                }
                Token::Star => parser.emit_byte(OpCode::Multiply),
                Token::Slash => parser.emit_byte(OpCode::Divide),
                Token::BangEqual => {
                    parser.emit_byte(OpCode::Equal);
                    parser.emit_byte(OpCode::Not);
                }
                Token::EqualEqual => parser.emit_byte(OpCode::Equal),
                Token::Greater => parser.emit_byte(OpCode::Greater),
                Token::GreaterEqual => {
                    parser.emit_byte(OpCode::Less);
                    parser.emit_byte(OpCode::Not);
                }
                Token::Less => parser.emit_byte(OpCode::Less),
                Token::LessEqual => {
                    parser.emit_byte(OpCode::Greater);
                    parser.emit_byte(OpCode::Not);
                }
                _ => panic!("invalid binary call."),
            }
        }
    }

    /// Emit bool values Bytecodes.
    ///
    fn literal(parser: &mut Parser<R>, _can_assign: bool) {
        match &parser.previous {
            Token::True => parser.emit_byte(OpCode::True),
            Token::False => parser.emit_byte(OpCode::False),
            _ => panic!("invalid literal operation."),
        }
    }

    // TODO Set string interning model
    /// Emit String to Contants.
    ///
    /// Emit: Constant
    ///
    fn string(parser: &mut Parser<R>, _can_assign: bool) {
        let str = match parser.get_previous() {
            Token::String(s) => s,
            _ => panic!("Invalid string value"),
        };

        let index = parser
            .function
            .chunk
            .write_constant(Primitive::String(String::from_utf8(str).unwrap()));
        parser.emit_byte(OpCode::Constant(index));
    }

    /// & -> Reference
    /// -> Get current token (Value to-be-parsed)
    /// -> Emit bytecode which set referenced named variable to the stack
    /// -> Ref must reference the value in the stack itself
    ///
    fn reference(parser: &mut Parser<R>, _can_assign: bool) {
        parser.advance();
        let name = match parser.get_previous() {
            Token::Identifier(s) => s,
            _ => panic!("Invalid reference."),
        };

        match parser.scopes.len() {
            /* global */
            0 => {
                let var_index = parser.identifier_constant(name);
                parser.emit_byte(OpCode::SetRefGlobal(var_index.unwrap()));
            }
            _ => {
                let var_index = parser
                    .scopes
                    .last()
                    .unwrap()
                    .get_local(&name)
                    .expect("Invalid variable name: {name}");

                parser.emit_byte(OpCode::SetRefLocal(var_index.borrow().0));
            }
        }
    }

    fn variable(parser: &mut Parser<R>, can_assign: bool) {
        Self::named_variable(parser, can_assign);
    }

    /// Distinguish between re-assign and get variable already set value as well as local and global variables.
    ///
    /// Emit: (Local set or get) or (Global set or get Bytecode).
    ///
    fn named_variable(parser: &mut Parser<R>, can_assign: bool) {
        let (get_op, set_op): (OpCode, OpCode);

        let var_name = match parser.get_previous() {
            Token::Identifier(s) => s,
            _ => panic!("Could not get named_var"),
        };

        let scopes = &mut parser.scopes;

        if scopes.len() > 0 {
            /* Pass check on all scopes */
            let local = parser.resolve_local(&var_name);

            /* Global variables inside scope handling */
            if local.is_some() {
                let local = local.unwrap();

                get_op = OpCode::GetLocal(local.borrow().0);
                set_op = OpCode::SetLocal(local.borrow().0, local.borrow().1);
            } else if let Some(up_idx) = parser.resolve_upvalue(&var_name) {
                get_op = OpCode::GetUpValue(up_idx);
                set_op = OpCode::SetUpValue(up_idx);
            } else {
                let var_index = parser.identifier_constant(var_name);

                get_op = OpCode::GetGlobal(var_index.unwrap());
                set_op = OpCode::SetGlobal(var_index.unwrap());
            }
        } else if let Some(up_idx) = parser.resolve_upvalue(&var_name) {
            get_op = OpCode::GetUpValue(up_idx);
            set_op = OpCode::SetUpValue(up_idx);
        } else {
            let var_index = parser.identifier_constant(var_name);

            get_op = OpCode::GetGlobal(var_index.unwrap());
            set_op = OpCode::SetGlobal(var_index.unwrap());
        }

        if can_assign && parser.match_token(Token::Equal) {
            parser.expression();
            parser.emit_byte(set_op);
        } else {
            parser.emit_byte(get_op);
        }
    }

    /// Jump if first condition of expression is false, verifying the second for a possible jump.
    ///
    fn and_(parser: &mut Parser<R>, _can_assign: bool) {
        let end_jump = parser.emit_jump(OpCode::JumpIfFalse(0));

        parser.emit_byte(OpCode::Pop);
        parser.parse_precedence(Precedence::And);

        parser.patch_jump(end_jump, OpCode::JumpIfFalse(0));
    }

    /// Verify for dangling expression on stack, jumpiong to the second evaluation and jumping over the entire instructions if false.
    ///
    fn or_(parser: &mut Parser<R>, _can_assign: bool) {
        let else_jump = parser.emit_jump(OpCode::JumpIfFalse(0));
        let end_jump = parser.emit_jump(OpCode::Jump(0));

        parser.patch_jump(else_jump, OpCode::JumpIfFalse(0));
        parser.emit_byte(OpCode::Pop);

        parser.parse_precedence(Precedence::Or);
        parser.patch_jump(end_jump, OpCode::Jump(0));
    }

    /// Get argument count by evaluating expression on function arguments.
    ///
    fn call(parser: &mut Parser<R>, _can_assign: bool) {
        let arg_count = Self::arg_list(parser);

        parser.emit_byte(OpCode::Call(arg_count));
    }

    fn arg_list(parser: &mut Parser<R>) -> usize {
        let mut arg_count = 0;

        if !parser.check(Token::RightParen) {
            loop {
                parser.expression();
                arg_count += 1;

                if !parser.match_token(Token::Comma) {
                    break;
                }
            }
        }
        parser.consume(Token::RightParen, "Expect ')' after function arguments.");

        arg_count
    }
}

/// Define which tokens will call which functions on prefix or infix while it's precedence is being parsed.
///
pub fn get_rule<R: std::io::Read>(token_code: &crate::parser::Token) -> ParseRule<R> {
    match token_code {
        Token::LeftParen => ParseRule {
            prefix: ParseRule::grouping,
            infix: ParseRule::call,
            precedence: Precedence::Call,
        },
        Token::RightParen => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::LeftBrace => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::RightBrace => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Comma => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Dot => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Minus => ParseRule {
            prefix: ParseRule::unary,
            infix: ParseRule::binary,
            precedence: Precedence::Term,
        },
        Token::Plus => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::binary,
            precedence: Precedence::Term,
        },
        Token::Colon => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::SemiColon => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Slash => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::binary,
            precedence: Precedence::Factor,
        },
        Token::Ampersand => ParseRule {
            prefix: ParseRule::reference,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Star => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::binary,
            precedence: Precedence::Factor,
        },
        Token::Bang => ParseRule {
            prefix: ParseRule::unary,
            infix: ParseRule::none,

            precedence: Precedence::None,
        },
        Token::BangEqual => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::binary,
            precedence: Precedence::Equality,
        },
        Token::Default => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Equal => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::EqualEqual => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::binary,
            precedence: Precedence::Equality,
        },
        Token::Greater => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::binary,
            precedence: Precedence::Comparison,
        },
        Token::GreaterEqual => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::binary,
            precedence: Precedence::Comparison,
        },

        Token::Less => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::binary,
            precedence: Precedence::Comparison,
        },
        Token::LessEqual => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::binary,
            precedence: Precedence::Comparison,
        },
        Token::Identifier(_) => ParseRule {
            prefix: ParseRule::variable,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::String(_) => ParseRule {
            prefix: ParseRule::string,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Integer(_) => ParseRule {
            prefix: ParseRule::number,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Float(_) => ParseRule {
            prefix: ParseRule::number,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::And => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::and_,
            precedence: Precedence::And,
        },
        Token::Class => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Case => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },

        Token::Else => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::False => ParseRule {
            prefix: ParseRule::literal,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::For => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,

            precedence: Precedence::None,
        },
        Token::Fun => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::If => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Modifier => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Nil => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,

            precedence: Precedence::None,
        },
        Token::Or => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::or_,
            precedence: Precedence::Or,
        },
        Token::Print => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Switch => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Return => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },

        Token::Continue => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },

        Token::Super => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::TypeDef(_) => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::This => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,

            precedence: Precedence::None,
        },
        Token::True => ParseRule {
            prefix: ParseRule::literal,

            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Var => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Const => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::While => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Error(_) => ParseRule {
            prefix: ParseRule::none,

            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Eof => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
        Token::Comment => ParseRule {
            prefix: ParseRule::none,
            infix: ParseRule::none,
            precedence: Precedence::None,
        },
    }
}
