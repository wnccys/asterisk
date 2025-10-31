#[cfg(test)]
mod for_ {
    use std::{io::Cursor, rc::Rc};

    use asterisk::{primitives::primitive::Primitive, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn for_basic_loop() {
        let source = r"
            let mut n: Int = 0;

            for (let mut i = 0; i < 10; i = i + 1) {
                n = n + 1;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // for
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_value = match n.value {
            Primitive::Int(int) => int,
            _ => panic!("Invalid type")
        };

        assert_eq!(n_value, 10);
    }

    #[test]
    fn for_basic_loop_without_first_expr() {
        let source = r"
            let mut n: Int = 0;

            for (; n < 10; n = n + 1) {
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // for
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_value = match n.value {
            Primitive::Int(int) => int,
            _ => panic!("Invalid type")
        };

        assert_eq!(n_value, 10);
    }

    // TODO set return / break instr;
    fn for_basic_loop_without_second_expr() {
        let source = r"
            let mut n: Int = 0;

            for (;; n = n + 1) {
                if (n == 10) {
                    return;
                }
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // define_struct()
        parser = parser.declaration();
        // for
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_value = match n.value {
            Primitive::Int(int) => int,
            _ => panic!("Invalid type")
        };

        assert_eq!(n_value, 10);
    }

    #[test]
    fn for_basic_loop_without_third_expr() {
        let source = r"
            let mut n: Int = 0;

            for (; n < 10 ;) {
                n = n + 1;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var_declaration()
        parser = parser.declaration();
        // for
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_value = match n.value {
            Primitive::Int(int) => int,
            _ => panic!("Invalid type")
        };

        assert_eq!(n_value, 10);
    }
}