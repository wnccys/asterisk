#[cfg(test)]
mod else_ {
    use std::{io::Cursor, rc::Rc};
    use asterisk::{primitives::{primitive::Primitive}, vm::Vm};
    use crate::common::mk_parser;

    #[test]
    fn else_single_condition_falls_to_else() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 0;

            if (false) {
                a = 1;
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

    #[test]
    fn else_single_condition_do_not_falls_to_else() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = 0;

            if (true) {
                a = 1;
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
        assert_eq!(a.borrow().value, Primitive::Int(1));
    }
}