mod common;
use common::mk_parser;

#[cfg(test)]
mod ops {
    use std::{cell::RefCell, rc::Rc};

    use asterisk::{primitives::{primitive::Primitive, types::Type, value::Value}, vm::Vm};

    use super::*;

    #[test]
    fn sum_int_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(2),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(2),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("")
        };

        assert_eq!(result, 4);
    }

    #[test]
    fn sum_int_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(2),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(-2),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 0);
    }

    #[test]
    fn sum_float_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 4.0);
    }

    #[test]
    fn sum_float_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(-2.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 0.0);
    }

    #[test]
    fn mul_int_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 9);
    }

    #[test]
    fn mul_int_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(-3),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -9);
    }

    #[test]
    fn mul_float_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(5.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 10.0);
    }

    #[test]
    fn mul_float_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(-5.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -10.0);
    }

    fn div() {

    }
}