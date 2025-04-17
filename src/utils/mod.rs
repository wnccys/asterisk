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
        _ => Type::Void,
    }
}
