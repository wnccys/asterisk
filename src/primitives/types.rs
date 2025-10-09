use std::rc::Rc;

/* Primitives are variable assigned data, Type are the contract for this data to be valid throught the runtime */
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    Int,
    Bool,
    String,
    Fn,
    NativeFn,
    Closure,
    Ref(Rc<Type>),
    #[default]
    UnInit,
    Void,
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
