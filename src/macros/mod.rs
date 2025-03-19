/// Generate equal comparison code enabling Value(_) == Value(_) 
/// 
macro_rules! gen_primitives_equal {
    ($($variant:ident($inner:ty)), *) => {
        pub fn values_equal(a: Primitive, b: Primitive) -> Primitive {
            match (a, b) {
                $(
                    (Primitive::$variant(value_a), Primitive::$variant(value_b)) => {
                        Primitive::Bool(value_a == value_b)
                    }
                ),*
                _ => panic!("invalid value comparison."),
            }
        }
    };
}

macro_rules! gen_values_operations {
    () => {
        impl Add for Value {
            type Output = Value

            fn add(self, other: Value) -> Value {
            }
        }
    }
}

pub(crate) use gen_primitives_equal;