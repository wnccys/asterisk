mod common;
use common::mk_parser;

#[cfg(test)]
mod types {
    use std::{io::Cursor, rc::Rc};
    use super::*;

    use asterisk::{primitives::types::Type, vm::Vm};

    #[test]
    fn implicit_int() {
        let mut vm = Vm::default();
        let source = r"
            let a = 32;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Int);
    }

    #[test]
    fn explicit_int() {
        let mut vm = Vm::default();
        let source = r"
            let a: Int = 32;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Int);
    }

    #[test]
    fn implicit_float() {
        let mut vm = Vm::default();
        let source = r"
            let a = 32.0;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Float);
    }

    #[test]
    fn explicit_float() {
        let mut vm = Vm::default();
        let source = r"
            let a: Float = 10.5;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Float);
    }
    
    #[test]
     fn implicit_bool() {
        let mut vm = Vm::default();
        let source = r"
            let a = false;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Bool);
     }

     #[test]
     fn explicit_bool() {
        let mut vm = Vm::default();
        let source = r"
            let a: Bool = true;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::Bool);
     }

     #[test]
     fn implicit_string() {
        let mut vm = Vm::default();
        let source = r"
            let a = 't';
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow()._type, Type::String);
     }

     #[test]
     fn explicit_string() {
        let mut vm = Vm::default();
        let source = r"
            let a: String = 'xyz';
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

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
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        let a = vm.globals.get(&String::from("a")).unwrap();
        assert_eq!(a.borrow()._type, Type::Fn);
    }

    #[test]
    fn simple_closure() {
        let mut vm = Vm::default();
        let source = r"
            let a = fn() {};
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.fun_declaration();

        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run().unwrap();
    }

    #[test]
    fn complex_closure() {
        let mut vm = Vm::default();
        let source = r"
            fn f() {
                let g = fn n(){};

                return g;
            }

            let a = f();
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.fun_declaration();

        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run().unwrap();

        let a = vm.globals.get(&String::from("a")).unwrap();
        assert_eq!(a.borrow()._type, Type::Closure);
    }

    #[test]
    fn references() {}
}