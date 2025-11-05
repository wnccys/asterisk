#[cfg(test)]
mod elsif {
    use std::{io::Cursor, rc::Rc};
    use asterisk::{primitives::{primitive::Primitive}, vm::Vm};
    use crate::common::mk_parser;

    #[test]
    fn elsif_single_condition_falls_to_elsif() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 0;

            if (false) {
                a = 1;
            } else if (a == 0) {
                a = 0;
            } else {
                a = 2;
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
        assert_eq!(a.borrow().value, Primitive::Int(0));
    }

    #[test]
    fn elsif_single_condition_do_not_falls_to_elsif() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 0;

            if (false) {
                a = 1;
            } else if (a == 100) {
                a = 0;
            } else {
                a = 2;
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
        assert_eq!(a.borrow().value, Primitive::Int(2));
    }
}