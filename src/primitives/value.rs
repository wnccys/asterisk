use super::{
    primitive::Primitive,
    types::{Modifier, Type},
};
use std::{fmt::Display, ops::{Add, Div, Mul, Not}};

/// All Asterisk Values definition.
///
#[derive(Debug, Clone)]
pub struct Value {
    pub value: Primitive,
    pub _type: Type,
    pub modifier: Modifier,
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.value)
    }
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
