#[cfg(test)]
mod closures {
    use std::io::Cursor;

    use asterisk::{primitives::{primitive::Primitive, types::Type}, vm::Vm};

    #[test]
    fn closure_with_upvalue() {
        let mut vm = Vm::default();
        let source = r#"
            fn make_counter() {
                let mut i = 0;

                fn count() {
                    i = i + 1;

                    return i;
                }

                return count;
            }

            let counter = make_counter();
            let a = counter();
            let b = counter();
            let c = counter();
        "#;

        vm.interpret(Cursor::new(source));

        let a = vm.globals.get(&"a".to_string()).unwrap();

        assert_eq!(a.borrow()._type, Type::Int);
        assert_eq!(a.borrow().value.to_string(), "1");

        let b = vm.globals.get(&"b".to_string()).unwrap();

        assert_eq!(b.borrow()._type, Type::Int);
        assert_eq!(b.borrow().value.to_string(), "2");

        let c = vm.globals.get(&"c".to_string()).unwrap();
        assert_eq!(c.borrow()._type, Type::Int);
        assert_eq!(c.borrow().value.to_string(), "3");
    }

    #[test]
    fn closure_inline_anonymous() {
        let mut vm = Vm::default();
        let source = "
            let n = fn(n: String) { return 1; };
            let g = n('some');
        ";

        vm.interpret(Cursor::new(source));

        let g = vm.globals.get(&"g".to_string()).unwrap().take();
        let Primitive::Int(n) = g.value else {
            panic!()
        };

        assert_eq!(n, 1);
    }

    #[test]
    fn closure_inline_anonymous_immediate_call() {
        let mut vm = Vm::default();
        let source = "
            let n = fn() { return 1; }();
            let g = n;
        ";

        vm.interpret(Cursor::new(source));
        let g = vm.globals.get(&"g".to_string()).unwrap().take();
        let Primitive::Int(n) = g.value else {
            panic!()
        };

        assert_eq!(n, 1);
    }
}