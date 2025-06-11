use std::{cell::RefCell, rc::Rc, time::{Duration, Instant}};

use crate::value::{Modifier, Primitive, Type, Value};

pub fn duration(_args: &[Rc<RefCell<Value>>]) -> Value {
    Value {
        value: Primitive::Int(Instant::now().elapsed().as_nanos().try_into().expect("Too much time has passed.")),
        _type: Type::Int,
        modifier: Modifier::Const,
    }
} 
