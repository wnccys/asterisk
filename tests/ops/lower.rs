#[cfg(test)]
mod lower {
    use std::{io::Cursor, rc::Rc};

    use asterisk::{primitives::primitive::Primitive, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn lower() {
        let mut vm = Vm::default();
        let source = r"
            let a = 1 < 2;
            let b = 1.0 < 2.0;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap().take();
        let b = vm.globals.get(&"b".to_string()).unwrap().take();

        assert_eq!(a.value, Primitive::Bool(true));
        assert_eq!(b.value, Primitive::Bool(true));
    }
}