use std::rc::Rc;

use crate::primitives::{primitive::Primitive, types::Type};

pub mod hasher;
pub mod print;

/// Parses primitive to Type equivalent
///
pub fn parse_type(p: &Primitive) -> Type {
    match p {
        Primitive::Int(_) => Type::Int,
        Primitive::Float(_) => Type::Float,
        Primitive::String(_) => Type::String,
        Primitive::Bool(_) => Type::Bool,
        Primitive::Ref(t) => Type::Ref(Rc::new(t.borrow()._type.clone())),
        Primitive::Function(_) => Type::Fn,
        Primitive::Struct(_) => Type::Struct,
        _ => panic!("Error parsing type."),
    }
}
