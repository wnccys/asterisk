#[cfg(test)]
mod div {
    use std::{cell::RefCell, rc::Rc};
    use asterisk::{primitives::{primitive::Primitive, value::Value}, vm::Vm};

    #[test]
    fn div_int_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(12),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 4);
    }

    #[test]
    fn div_int_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(-30),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(10),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -3);
    }

    #[test]
    fn div_float_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(15.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(3.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 5.0);
    }

    #[test]
    fn div_float_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(-12.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(3.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -4.0);
    }
}