/// Generate equal comparison code enabling Primitive(_) == Primitive(_), Primitive(_) > Primitive(_)
///
macro_rules! gen_primitives_operations {
    ($($variant:ident($inner:ty)), *) => {
        use std::cmp::Ordering;

        impl Add for Primitive {
            type Output = Primitive;

            fn add(self, other: Self) -> Primitive {
                match (self, other) {
                    $(
                        (
                            Primitive::$variant(value_a),
                            Primitive::$variant(value_b)
                        ) => { Primitive::$variant(value_a + value_b) }
                    ), *
                    (
                        Primitive::String(str1), Primitive::String(str2)
                    ) => {
                        Primitive::String(str1.add(&str2[..]))
                    },
                    _ => panic!("Add not allowed.")
                }
            }
        }

        impl Mul for Primitive {
            type Output = Primitive;

            fn mul(self, other: Self) -> Primitive {
                match (self, other) {
                    $(
                        (
                            Primitive::$variant(value_a),
                            Primitive::$variant(value_b)
                        ) => { Primitive::$variant(value_a * value_b) }
                    ), *
                    _ => panic!("Operation mul not allowed")
                }
            }
        }

        impl Div for Primitive {
            type Output = Primitive;

            fn div(self, other: Self) -> Primitive {
                match (self, other) {
                    $(
                        (
                            Primitive::$variant(value_a),
                            Primitive::$variant(value_b)
                        ) => { Primitive::$variant(value_a / value_b) }
                    ), *
                    _ => panic!("Operation div not allowed")
                }
            }
        }

        impl PartialEq for Primitive {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (Primitive::$variant(value_a), Primitive::$variant(value_b)) => {
                            value_a == value_b
                        }
                    ),*
                    _ => panic!("Cannot compare different primitives")
                }
            }
        }

        impl PartialOrd for Primitive {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match (self, other) {
                    // Set basic Primitive variants compare (Rust primitive compared underneath the hood)
                    $(
                        (
                            Primitive::$variant(value_a), Primitive::$variant(value_b)
                        ) => { value_a.partial_cmp(value_b) }
                    ), *
                    _ => panic!("Cannot compare different types.")
                }
            }
        }
    };
}

/// Generate add, mul, and div for it's respective values.
/// Strings can only be added; Other implementations are only allowed in numbers (Int and Float)
///
macro_rules! gen_values_operations {
    ($($variant:ident($inner:ty)), *) => {
        impl Add for Value {
            type Output = Value;

            fn add(self, other: Self) -> Value {
                match (self, other) {
                    $(
                        (
                            Value { value: Primitive::$variant(value_a), modifier },
                            Value { value: Primitive::$variant(value_b), modifier: _},
                        ) => { Value { value: Primitive::$variant(value_a + value_b), modifier} }
                    ), *
                    (
                        Value { value: Primitive::String(str1) , modifier },
                        Value { value: Primitive::String(str2), modifier: _ },
                    ) => {
                        Value { value: Primitive::String(str1.add(&str2[..])), modifier }
                    },
                    _ => panic!("Add not allowed.")
                }
            }
        }

        impl Mul for Value {
            type Output = Value;

            fn mul(self, other: Self) -> Value {
                match (self, other) {
                    $(
                        (
                            Value { value: Primitive::$variant(value_a), modifier },
                            Value { value: Primitive::$variant(value_b), modifier: _},
                        ) => { Value { value: Primitive::$variant(value_a * value_b), modifier  } }
                    ), *
                    _ => panic!("Operation mul not allowed")
                }
            }
        }

        impl Div for Value {
            type Output = Value;

            fn div(self, other: Self) -> Value {
                match (self, other) {
                    $(
                        (
                            Value { value: Primitive::$variant(value_a), modifier },
                            Value { value: Primitive::$variant(value_b), modifier: _},
                        ) => { Value { value: Primitive::$variant(value_a / value_b), modifier } }
                    ), *
                    _ => panic!("Operation div not allowed")
                }
            }
        }

        impl PartialEq for Value {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl PartialOrd for Value {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.value.partial_cmp(&other.value)
            }
        }
    }
}

pub(crate) use gen_primitives_operations;
pub(crate) use gen_values_operations;
