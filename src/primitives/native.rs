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

pub fn _typeof(args: &[Rc<RefCell<Value>>]) -> Value {
    let obj = args[0].borrow();

    let t = match obj.value {
        Primitive::String(_) => "String",
        Primitive::Bool(_) => "Boolean",
        Primitive::Int(_) => "Integer",
        Primitive::Float(_) => "Float",
        Primitive::Function(_) => "Function",
        Primitive::NativeFunction(_) => "NativeFunction",
        Primitive::Ref(_) => "Reference",
        Primitive::Void(_) => "Void",
    };

    Value {
        value: Primitive::String(t.to_string()),
        _type: Type::String,
        modifier: Modifier::Const,
    }
}
