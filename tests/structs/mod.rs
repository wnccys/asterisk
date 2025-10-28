#[cfg(test)]
pub mod structs {
    use std::{io::Cursor, rc::Rc};

    use asterisk::{primitives::{primitive::Primitive, types::{Modifier, Type}, value::Value}, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn struct_basic_def() {
        let source = r"
            struct S {
                id: Int,
                name: String,
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let a = vm.globals.get(&"S".to_string()).unwrap();
        let a_struct = match a.borrow().value {
            Primitive::Struct(ref _struct) => {
                _struct.clone()
            }
            _ => panic!("Invalid var struct.")
        };

        assert_eq!(a_struct.name, "S");
        assert_eq!(a_struct.field_count, 2);
        assert_eq!(a.borrow()._type, Type::Struct);
        assert!(a_struct.field_indices.contains_key(&"id".to_string()));
        assert!(a_struct.field_indices.contains_key(&"name".to_string()));
    }

    #[test]
    fn struct_basic_instance() {
        let source = r"
            struct L {
                name: String
            }

            let l = L { name: 'some' };
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let wrapped_l = vm.globals.get(&"l".to_string()).unwrap().take();
        let l = match wrapped_l {
            Value { value: Primitive::Instance(inst), ..} => {
                inst
            }
            _ => panic!("Invalid instance object.")
        };

        let l_struct = match l._struct.borrow().value {
            Primitive::Struct(ref _struct) => {
                _struct.clone()
            }
            _ => panic!("Invalid instance's blueprint state.")
        };

        let l_value = l.values[l_struct.field_indices.get("name").unwrap().1].clone();

        assert_eq!(l_struct.name, "L");
        assert_eq!(l_value, Value { value: Primitive::String("some".to_string()), _type: Type::String, modifier: Modifier::Const});
    }

    #[test]
    fn struct_complex_instance() {
        let source = r"
            struct L {
                name: String
            }

            struct S {
                id: Int,
                name: L,
            }
        ";
    }

    #[test]
    fn struct_basic_instance_type_check() {}

    #[test]
    fn struct_complex_instance_type_check() {}

    #[test]
    fn struct_complex_def_receive_struct_as_field() {}

    #[test]
    fn struct_as_argument_on_function() {}

    #[test]
    fn struct_local() {
        let source = r"
            {
                struct S {
                    id: Int,
                    name: String,
                }
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        // const
        vm.exec_code().unwrap();
        // def_local
        vm.exec_code().unwrap();

        let s = Rc::clone(&vm.stack[0]);

        let s = match s.borrow().value {
            Primitive::Struct(ref _struct) => {
                _struct.clone()
            }
            _ => panic!("Invalid var struct.")
        };

        assert_eq!(s.name, "S");
        assert_eq!(s.field_count, 2);
        assert!(s.field_indices.contains_key(&"id".to_string()));
        assert!(s.field_indices.contains_key(&"name".to_string()));
    }
}