use crate::value::{Primitive, Type};

pub mod print;

/// Parses primitive to Type equivalent
///
pub fn parse_type(p: &Primitive) -> Type {
    dbg!(p);
    match p {
        Primitive::Int(_) => Type::Int,
        Primitive::Float(_) => Type::Float,
        Primitive::String(_) => Type::String,
        Primitive::Bool(_) => Type::Bool,
        Primitive::Ref(t) => Type::Ref(unsafe { &t.read()._type }),
        _ => panic!("Error parsing type."),
    }
}
