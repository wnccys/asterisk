use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Float(f64),
    Int(i32),
    Bool(bool),
    String(Vec<char>),
    Void(()),
}

crate::macros::gen_values_equal!(
    Float(f64),
    Int(i32),
    Bool(bool),
    String(Vec<char>),
    Void(())
);

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            (Value::String(str1), Value::String(str2)) => {
                let mut result = str1.clone();
                result.extend(str2);
                Value::String(result)
            }
            _ => panic!("operation add not allowed."),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            _ => panic!("operation mult not allowed."),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
            _ => panic!("operation divide not allowed."),
        }
    }
}
