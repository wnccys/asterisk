#[cfg(test)]
mod equal {
    use std::{io::Cursor, rc::Rc};

    use asterisk::{primitives::primitive::Primitive, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn equal() {
        let mut vm = Vm::default();
        let source = r"
            let a1 = 'str' == 'str';
            let a2 = 'str' == 'strx';
            let b1 = 2 == 2;
            let b2 = 2 == 1;
            let c1 = 2.0 == 2.0;
            let c2 = 2.0 == 3.0;
            let d1 = true == true;
            let d2 = true == false;
        ";

        let mut parser = mk_parser(Cursor::new(source));

        const QNTT: usize = 8;

        for _ in 0..QNTT {
            // var decl
            parser = parser.declaration();
        }

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let names: [&str; QNTT] = [
            "a1", "a2",
            "b1", "b2",
            "c1", "c2",
            "d1", "d2"
        ];

        for i in 0..QNTT {
            let it = vm.globals.get(&names[i].to_string()).unwrap().take();

            let cond = match i % 2 {
                1 => false,
                _ => true
            };

            assert_eq!(it.value, Primitive::Bool(cond));
        }
    }
}