use super::{primitive::Primitive, types::{Modifier, Type}};
use std::ops::{Div, Mul, Add, Not};

/// All Asterisk Values definition.
///
#[derive(Debug, Clone)]
pub struct Value {
    pub value: Primitive,
    pub _type: Type,
    pub modifier: Modifier,
}

impl Default for Value {
    fn default() -> Self {
        Value {
            value: Primitive::Void(()),
            _type: Type::Void,
            modifier: Modifier::Unassigned,
        }
    }
}

crate::macros::gen_primitives_operations!(Float, Int);
crate::macros::gen_values_operations!(Int, Float);
