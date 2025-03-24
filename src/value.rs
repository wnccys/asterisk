use std::ops::{Add, Div, Mul};

/// Asterisk types definition.
///
#[derive(Debug, Clone)]
pub struct Value {
    pub value: Primitive,
    pub modifier: Modifier,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Float(f64),
    Int(i32),
    Bool(bool),
    String(String),
    Void(()),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Unassigned,
    Const,
    Mut,
}

pub enum RefType {
    Owned,
    Ref,
    MutRef,
}

crate::macros::gen_primitives_operations!(Float(f64), Int(i32));

crate::macros::gen_values_operations!(Int(i32), Float(f64));
