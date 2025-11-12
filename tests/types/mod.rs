#[cfg(test)]
mod types {
    use std::{io::Cursor, rc::Rc};
    use asterisk::{primitives::types::Type, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn int_implicit() {
        let mut vm = Vm::default();
        let source = r"
            let a = 32;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Int);
    }

    #[test]
    fn int_explicit() {
        let mut vm = Vm::default();
        let source = r"
            let a: Int = 32;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Int);
    }

    #[test]
    fn float_implicit() {
        let mut vm = Vm::default();
        let source = r"
            let a = 32.0;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Float);
    }

    #[test]
    fn float_explicit() {
        let mut vm = Vm::default();
        let source = r"
            let a: Float = 10.5;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Float);
    }
    
    #[test]
     fn bool_implicit() {
        let mut vm = Vm::default();
        let source = r"
            let a = false;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Bool);
     }

     #[test]
     fn bool_explicit() {
        let mut vm = Vm::default();
        let source = r"
            let a: Bool = true;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Bool);
     }

     #[test]
     fn string_implicit() {
        let mut vm = Vm::default();
        let source = r"
            let a = 't';
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::String);
     }

     #[test]
     fn string_explicit() {
        let mut vm = Vm::default();
        let source = r"
            let a: String = 'xyz';
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::String);
     }

    #[test]
    fn function() {
        let mut vm = Vm::default();
        let source = r"
            fn f() {}
            let a = f;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.fun_declaration();

        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        let a = vm.globals.get(&String::from("a")).unwrap();
        assert_eq!(a.borrow()._type, Type::Fn);
    }

    #[test]
    fn closure() {
        let mut vm = Vm::default();
        let source = r"
            fn f() {
                fn inner() { print 'hello from g!'; }

                return inner;
            }

            let a = f();
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.fun_declaration();

        parser.advance();
        parser = parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run().unwrap();

        let a = vm.globals.get(&String::from("a")).unwrap();
        assert_eq!(a.borrow()._type, Type::Closure);
    }

    #[test]
    fn references() {}
}
    