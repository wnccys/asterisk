use std::rc::Rc;

use crate::value::{Primitive, Type};

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
        _ => panic!("Error parsing type."),
    }
}
