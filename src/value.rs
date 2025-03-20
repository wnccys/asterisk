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

#[derive(Debug, Clone)]
pub enum Modifier {
    Const,
    Mut
}

pub enum RefType {
    Owned,
    Ref,
    MutRef,
}

crate::macros::gen_primitives_eq_ord!(
    Float(f64),
    Int(i32),
    Bool(bool),
    String(String),
    Void(())
);

crate::macros::gen_values_operations!(
    Int(i32),
    Float(f64)
);