#[cfg(test)]
pub mod structs {
    use std::{io::Cursor, panic::{AssertUnwindSafe, catch_unwind}, rc::Rc};

    use asterisk::{primitives::{primitive::Primitive, types::{Dyn, Modifier, Type}, value::Value}, vm::Vm};

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
    fn struct_global_field_access() {
        let mut vm = Vm::default();

        let source = r"
            struct L {
                name: String
            }

            let l = L { name: 'some' };

            let n = l.name;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // struct decl
        parser = parser.declaration();
        // var decl
        parser = parser.declaration();
        // var decl
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_val = match n.value {
            Primitive::String(str) => str,
            _ => panic!("Invalid object value.")
        };

        assert_eq!(n._type, Type::String);
        assert_eq!(n_val, "some");
    }

    #[test]
    fn struct_local_field_access() {
        let mut vm = Vm::default();

        let source = r"
            struct L {
                name: String
            }

            {
                let l = L { name: 'some' };
                let n = l.name;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // struct decl
        parser = parser.declaration();
        // var decl
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);

        for _ in 0..12 {
            vm.exec_code().unwrap();
        }

        let n = vm.stack.get(1).unwrap().take();

        let n_val = match n.value {
            Primitive::String(str) => str,
            _ => panic!("Invalid object value.")
        };

        assert_eq!(n._type, Type::String);
        assert_eq!(n_val, "some");
    }

    #[test]
    fn struct_custom_struct_type_definition() {
        let source = r"
            struct L {
                name: String
            }

            struct S {
                id: Int,
                str: L,
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let l = vm.globals.get(&"L".to_string()).unwrap();
        let s = vm.globals.get(&"S".to_string()).unwrap().take();

        let Primitive::Struct(_struct) = s.value else {
            panic!("Expect struct.")
        };

        let field = _struct.field_indices.get(&"str".to_string()).unwrap();
        assert_eq!(field.0, Type::Dyn(Dyn { 0: l }));
    }

    #[test]
    fn struct_custom_struct_as_instance_value_var() {
        let source = r"
            struct L {
                name: String
            }

            struct S {
                id: Int,
                str: L,
            }

            let l = L { name: 'some' };
            let s = S { id: 1, str: l };
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // define_struct()
        parser = parser.declaration();

        // var_declaration()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let s = vm.globals.get(&"s".to_string()).unwrap().take();
        assert_eq!(s._type, Type::Struct);

        let Primitive::Instance(instance) = s.value else {
            panic!();
        };

        let Primitive::Struct(ref _struct) = instance._struct.borrow().value else {
            panic!()
        };

        assert_eq!(instance.values[_struct.field_indices.get("str").unwrap().1]._type, Type::Struct);
    }

    #[test]
    fn struct_custom_struct_as_instance_value_no_var() {
        let source = r"
            struct L {
                name: String
            }

            struct S {
                id: Int,
                str: L,
            }

            let s = S { id: 1, str: L { name: 'some' } };
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let s = vm.globals.get(&"s".to_string()).unwrap().take();
        assert_eq!(s._type, Type::Struct);

        let Primitive::Instance(instance) = s.value else {
            panic!();
        };

        let Primitive::Struct(ref _struct) = instance._struct.borrow().value else {
            panic!()
        };

        assert_eq!(instance.values[_struct.field_indices.get("str").unwrap().1]._type, Type::Struct);
    }

    #[test]
    fn struct_custom_struct_as_instance_value_no_var_field_access() {
        let source = r"
            struct L {
                name: String
            }

            struct S {
                id: Int,
                str: L,
            }

            let s = S { id: 1, str: L { name: 'some' } };
            let n = s.str;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // define_struct()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();

        let Primitive::Instance(ref instance) = n.value else {
            panic!();
        };

        let Primitive::Struct(ref _struct) = instance._struct.borrow().value else {
            panic!()
        };

        assert_eq!(n._type, Type::Struct);
        assert_eq!(instance.values[_struct.field_indices.get("name").unwrap().1]._type, Type::String);
    }

    #[test]
    fn struct_basic_instance_type_check_fail_on_wrong_type() {
        let source = r"
            struct L {
                name: String
            }

            let s = L { name: 2 };

            fn n(arg: Struct) { return arg; }

            let g = n(s);
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // define_struct()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();
        }));

        assert!(result.is_err());
    }

    #[test]
    fn struct_complex_instance_type_check_fail_on_wrong_type() {
        let source = r"
            struct L {
                name: String
            }

            struct S {
                other: L
            }

            let g = S { other: 'some' };
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // define_struct()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();
        }));

        assert!(result.is_err());
    }

    #[test]
    fn struct_as_argument_on_function() {
        let source = r"
            struct L {
                name: String
            }

            let s = L { name: 'some' };

            fn n(arg: Struct) { return arg; }

            let g = n(s);
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // define_struct()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();
        // var_declaration()
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let g = vm.globals.get(&"g".to_string()).unwrap().take();

        let Primitive::Instance(ref instance) = g.value else {
            panic!();
        };

        let Primitive::Struct(ref _struct) = instance._struct.borrow().value else {
            panic!()
        };

        assert_eq!(g._type, Type::Struct);
        assert_eq!(instance.values[_struct.field_indices.get("name").unwrap().1]._type, Type::String);
    }

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