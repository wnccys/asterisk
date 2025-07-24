mod common;
use common::mk_parser;

#[cfg(test)]
mod types {
    use std::{io::Cursor, rc::Rc};
    use super::*;

    use asterisk::{primitives::types::Type, vm::Vm};

    #[test]
    fn functions() {
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
    fn closures() {
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
        let _ = vm.run();

        let a = vm.globals.get(&String::from("a")).unwrap();
        assert_eq!(a.borrow()._type, Type::Closure);
    }

    #[test]
    fn references() {}
}