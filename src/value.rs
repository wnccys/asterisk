use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value<'a> {
    Float(f64),
    Int(i32),
    Bool(bool),
    String(&'a String),
}

// REVIEW probably resolvable with macro
pub fn values_equal<'a>(a: Value, b: Value) -> Value<'a> {
    match (a, b) {
        (Value::Bool(value_a), Value::Bool(value_b)) => Value::Bool(value_a == value_b),
        (Value::Int(value_a), Value::Int(value_b)) => Value::Bool(value_a == value_b),
        (Value::Float(value_a), Value::Float(value_b)) => Value::Bool(value_a == value_b),
        _ => panic!("only equal-types are allowed to be compared."),
    }
}

impl<'a> Copy for Value<'a> {}

impl<'a> Add for Value<'a> {
    type Output = Value<'a>;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            _ => panic!("operations with different types are not allowed"),
        }
    }
}

impl<'a> Mul for Value<'a> {
    type Output = Value<'a>;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            _ => panic!("operation with different types are not allowed."),
        }
    }
}

impl<'a> Div for Value<'a> {
    type Output = Value<'a>;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
            _ => panic!("operation with different types are not allowed."),
        }
    }
}
