#[cfg(test)]
mod if_ {
    use std::{io::Cursor, rc::Rc};

    use asterisk::{primitives::{primitive::Primitive}, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn if_single_simple_condition_true_to_false() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = true;

            if (a == true) {
                a = false;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Bool(false));
    }

    #[test]
    fn if_single_simple_condition_false_to_true() {
        let mut vm = Vm::default(); let source = r"
            let mut a = false;

            if (a == false) {
                a = true;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Bool(true));
    }

    #[test]
    fn if_single_complex_condition_or_true() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            if (a == 20 || a == 32) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(10));
    }

    #[test]
    fn if_single_complex_condition_or_false() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            if (a == 20 || a == 10) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(32));
    }

    #[test]
    fn if_single_complex_condition_and_true() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            if (a == 32 && a > 10) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(10));
    }

    #[test]
    fn if_single_complex_condition_and_false() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            if (a == 20 && a < 10) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(32));
    }

    #[test]
    fn if_single_complex_condition_or_and_precedence_true() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            // (false && false) || true
            if (a == 20 && a < 10 || a == 32) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(10));
    }

    #[test]
    fn if_single_complex_condition_and_or_precedence_false() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            // true || (false && true)
            if (a == 32 || a < 50 && a > 10) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(10));
    }

    #[test]
    fn if_single_complex_condition_grouping_precedence_true() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            // (false || true) && true
            if ((a == 10 || a < 50) && a > 10) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(10));
    }

    #[test]
    fn if_single_complex_condition_grouping_precedence_false() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 32;

            // (false || true) && false
            if ((a == 10 || a < 50) && a < 10) {
                a = 10;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        // if statement
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        assert_eq!(a.borrow().value, Primitive::Int(32));
    }
}