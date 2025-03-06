/// This file repreesents all macros definition, as well as it's imports;
/// 

macro_rules! gen_values_equal{
    ($($variant:ident($inner:ty)), *) => {
        pub fn values_equal(a: Value, b: Value) -> Value {
            match (a, b) {
                $(
                    (Value::$variant(value_a), Value::$variant(value_b)) => {
                        Value::Bool(value_a == value_b)
                    }
                ),*
                _ => panic!("invalid value comparison."),
            }
        }
    };
}
pub(crate) use gen_values_equal;