#[cfg(test)]
mod logical {
    use std::{io::Cursor, rc::Rc};
    use asterisk::{primitives::{primitive::Primitive}, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn bool_or() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = true or false;
            let mut b = true || false;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        let b = vm.globals.get(&"b".to_string()).unwrap();

        assert_eq!(a.borrow().value, Primitive::Bool(true));
        assert_eq!(b.borrow().value, Primitive::Bool(true));
    }

    #[test]
    fn bool_and() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = true and false;
            let mut b = true && false;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        let b = vm.globals.get(&"b".to_string()).unwrap();

        assert_eq!(a.borrow().value, Primitive::Bool(false));
        assert_eq!(b.borrow().value, Primitive::Bool(false));
    }

    #[test]
    fn bool_negation_true_to_false() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = !true;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();

        assert_eq!(a.borrow().value, Primitive::Bool(false));
    }
}