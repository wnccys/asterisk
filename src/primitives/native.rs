use std::{cell::RefCell, rc::Rc, time::Instant};

use super::{
    primitive::Primitive,
    types::{Modifier, Type},
    value::Value,
};

pub fn duration(_args: &[Rc<RefCell<Value>>]) -> Value {
    Value {
        value: Primitive::Int(
            Instant::now()
                .elapsed()
                .as_nanos()
                .try_into()
                .expect("Too much time has passed."),
        ),
        _type: Type::Int,
        modifier: Modifier::Const,
    }
}
