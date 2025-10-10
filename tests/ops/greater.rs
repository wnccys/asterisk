#[cfg(test)]
mod greater {
    use std::{io::Cursor, rc::Rc};
    use asterisk::{primitives::{primitive::Primitive}, vm::Vm};
    use crate::common::{mk_parser};

    #[test]
    fn greater_valid_types() {
        let mut vm = Vm::default();
        let source = r"
            let a = 2 > 1;
            let b = 2.0 > 1.0;
            let c = 2 > 1;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap().take();
        let b = vm.globals.get(&"a".to_string()).unwrap().take();

        assert_eq!(a.value, Primitive::Bool(true));
        assert_eq!(b.value, Primitive::Bool(true));
    }
}