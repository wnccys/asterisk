use std::rc::Rc;

/* Primitives are variable assigned data, Type are the check for this data to be valid throught the runtime */
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    Int,
    Bool,
    String,
    Fn,
    NativeFn,
    Closure,
    Ref(Rc<Type>),
    UnInit,
    Void,
}
impl Default for Type {
    fn default() -> Self {
        Type::UnInit
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Unassigned,
    Const,
    Mut,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::Unassigned
    }
}
