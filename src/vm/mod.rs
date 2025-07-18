pub mod chunk;
pub mod compiler;

use std::cell::RefCell;
use std::rc::Rc;
#[allow(unused)]
use std::time::Duration;

use crate::errors::vm::{InterpretResult, VmError};
use crate::objects::hash_table::HashTable;
use crate::primitives::native::_typeof;
use crate::primitives::primitive::NativeFn;
use crate::primitives::{
    primitive::{Function, Primitive},
    types::{Modifier, Type},
    value::Value,
};
use crate::utils::parse_type;
#[allow(unused)]
use crate::utils::print::print_stack;
use crate::vm::chunk::OpCode;
use crate::vm::compiler::compile;
use crate::{errors::vm::VmResult, primitives::native::duration};

pub type Stack = Vec<Rc<RefCell<Value>>>;
pub struct Vm {
    pub frames: Vec<CallFrame>,
    pub stack: Stack,
    pub globals: HashTable<String, Value>,
    pub strings: HashTable<String, String>,
}

#[derive(Debug)]
pub struct CallFrame {
    pub function: Rc<Function>,
    pub ip: *const OpCode,
    /* Init of frame function arg variables scope range */
    pub arg_offset: usize,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            frames: Vec::default(),
            stack: Vec::default(),
            globals: HashTable::default(),
            strings: HashTable::default(),
        }
    }
}

impl Vm {
    /// This function is the "compiler" itself, running chunk's Bytecodes.
    ///
    pub fn interpret<T: std::io::Read>(&mut self, source_code: T) {
        self.init_std_lib();

        let main = compile(source_code);

        self.call(Rc::new(main), 0);

        #[cfg(feature = "debug")]
        println!(
            "Constants Vec: {:?}",
            self.frames.last_mut().unwrap().function.chunk.constants
        );

        match self.run() {
            Err(e) => panic!("{:?}", e),
            _ => (),
        }
    }

    pub fn init_std_lib(&mut self) {
        self.globals.insert(
            &String::from("duration"),
            Value {
                value: Primitive::NativeFunction(NativeFn {
                    arity: 0,
                    _fn: duration,
                }),
                _type: Type::NativeFn,
                modifier: Modifier::Const,
            },
        );

        self.globals.insert(
            &String::from("typeof"),
            Value {
                value: Primitive::NativeFunction(NativeFn {
                    arity: 1,
                    _fn: _typeof,
                }),
                _type: Type::NativeFn,
                modifier: Modifier::Const,
            }
        );
    }

    pub fn run(&mut self) -> VmResult {
        while self.frames.len() > 0 {
            #[cfg(feature = "debug")]
            {
                print!("\n");
                print_stack(&self.stack);
                println!(
                    "current frame: {:?}",
                    self.frames.last().unwrap().function.name
                );
                println!("current code: {:?}", unsafe {
                    self.frames.last().unwrap().ip.read()
                });
            }

            #[cfg(feature = "delay-exec")]
            std::thread::sleep(Duration::from_secs(1));

            match unsafe { self.frames.last().unwrap().ip.read() } {
                OpCode::Return => {
                    let _return = self.stack.pop().ok_or(VmError::new(
                        "Could not return from function".to_string(),
                        InterpretResult::CompilerError,
                    ))?;

                    let last_frame = self.frames.pop().unwrap();
                    let last_frame_args = last_frame.function.arity;

                    /* Sanitize locals after frame is dropped */
                    for _ in 0..last_frame_args {
                        self.stack.pop();
                    }

                    if self.frames.len() == 0 {
                        return Ok(());
                    }

                    unsafe { self.advance_ip() }
                    self.stack.push(_return);

                    continue;
                }
                OpCode::Negate => {
                    let n = self.stack.pop().unwrap().take();

                    match n {
                        Value {
                            _type: Type::Bool, ..
                        }
                        | Value {
                            _type: Type::Float, ..
                        }
                        | Value {
                            _type: Type::Int, ..
                        } => {
                            self.stack.push(Rc::new(RefCell::new(!n)));
                            VmResult::Ok(())?
                        }
                        _ => VmResult::Err(VmError::new(
                            "Could not negate value.".to_string(),
                            InterpretResult::RuntimeError,
                        ))?,
                    }
                }
                OpCode::Not => {
                    let to_be_negated = self.stack.last().unwrap().take();

                    match to_be_negated {
                        Value {
                            value: Primitive::Bool(value),
                            ..
                        } => {
                            self.stack.last().unwrap().borrow_mut().value = Primitive::Bool(!value)
                        }
                        _ => panic!("Value should be a boolean."),
                    };
                }
                OpCode::Add => self.binary_op("+")?,
                OpCode::Multiply => self.binary_op("*")?,
                OpCode::Divide => self.binary_op("/")?,
                OpCode::True => {
                    self.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(true),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));
                }
                OpCode::False => {
                    self.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(false),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));
                }
                OpCode::Equal => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    self.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(a == b),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));
                }
                OpCode::PartialEqual => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    self.stack.push(Rc::clone(&b));
                    self.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(a == b),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));
                }
                OpCode::Greater => self.binary_op(">")?,
                OpCode::Less => self.binary_op("<")?,
                OpCode::Print => {
                    let value = self.stack.pop().expect("Could not find value to print.");

                    println!("{}", &value.borrow().value);
                }
                OpCode::Nil => {
                    self.stack.push(Rc::new(RefCell::new(Value::default())));
                }
                OpCode::Pop => {
                    self.stack.pop().expect("Error on pop: stack underflow.");
                }
                // Bring value from constants vector to stack
                OpCode::Constant(var_index) => {
                    let constant =
                        self.frames.last_mut().unwrap().function.chunk.constants[var_index].clone();
                    let _type = parse_type(&constant);

                    self.stack.push(Rc::new(RefCell::new(Value {
                        value: constant,
                        modifier: Modifier::Unassigned,
                        _type,
                    })));
                }
                /* Check Local Type */
                OpCode::DefineLocal(var_index, modifier, t) => {
                    let var_offset = self.frames.last().unwrap().arg_offset;

                    let variable = Rc::clone(
                        &self.stack[
                            // checked_sub handle global and defined function args handling
                            var_index + var_offset.checked_sub(1).unwrap_or(var_offset)
                        ],
                    );

                    // Rc explicit drop 
                    {
                        let var_type = &variable.borrow()._type;
                        if *var_type != t && t != Type::UnInit {
                            self.error(format!("Cannot assign {:?} to {:?}", t, variable.borrow()._type))?
                        }
                    }

                    let mut v_borrow = variable.borrow_mut();
                    v_borrow.modifier = modifier;
                    v_borrow._type = t;
                }
                /*
                    Set new value to local variable.
                */
                OpCode::SetLocal(var_index, modifier) => {
                    let variable = Rc::clone(&self.stack[var_index]);

                    if modifier != Modifier::Mut {
                        self.error("Cannot assign to immutable variable.".to_string())?
                    }

                    let incoming_value = self.stack.pop().unwrap().take();

                    if variable.borrow()._type != incoming_value._type 
                        && variable.borrow()._type != Type::UnInit {
                        self.error(format!("Cannot assign {:?} to {:?}", incoming_value._type, variable.borrow()._type))?
                    }

                    variable.borrow_mut().value = incoming_value.value;
                }
                /*
                    Get value from value position and load it into the top of stack,
                    this way other operations can interact with the value.
                */
                OpCode::GetLocal(var_index) => {
                    let variable = Rc::clone(
                        &self.stack[var_index
                            + (self
                                .frames
                                .last()
                                .unwrap()
                                .arg_offset
                                .checked_sub(1)
                                .unwrap_or(0))],
                    );

                    self.stack.push(variable);
                }
                /*
                    As local variables are defined as not the same as global ones, it needs a different treatment
                    Set ref to stack bucket where variable value is and let it available on stack.
                */
                OpCode::SetRefLocal(var_value_index) => {
                    let referenced_value = Rc::clone(&self.stack[var_value_index]);

                    let _ref = Value {
                        value: Primitive::Ref(Rc::clone(&referenced_value)),
                        _type: Type::Ref(Rc::new((referenced_value).borrow()._type.clone())),
                        modifier: Modifier::Const,
                    };

                    self.stack.push(Rc::new(RefCell::new(_ref)));
                }
                /*
                    Get variable name from constants and value from top of stack assigning it to globals HashMap
                */
                OpCode::DefineGlobal(var_name_index, modifier, t) => {
                    let var_name =
                        &self.frames.last().unwrap().function.chunk.constants[var_name_index];

                    let mut var_value = self.stack.pop().unwrap().take();
                    var_value.modifier = modifier;
                    var_value._type = t;

                    /*  Only strings are allowed to be var names */
                    self.globals.insert(var_name.into(), var_value);
                }
                /*
                    Get address from get globals and set it in stack.
                    This means every value referencing this value is referencing the value itself, not a copy on stack as globals and stack are exchangeable.
                */
                OpCode::GetGlobal(var_index) => {
                    let name = match &self.frames.last_mut().unwrap().function.chunk.constants
                        [var_index]
                    {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let value = match self.globals.get(&name) {
                        Some(value) => value,
                        None => panic!("Use of undeclared variable '{}'", name),
                    };

                    self.stack.push(Rc::clone(&value));
                }
                /*
                    Re-assign to already set global variable.
                */
                OpCode::SetGlobal(name_index) => {
                    let name = match &self.frames.last_mut().unwrap().function.chunk.constants
                        [name_index]
                    {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let variable = self.globals.get(name).unwrap();
                    if variable.borrow().modifier != Modifier::Mut {
                        panic!("Cannot assign to a immutable variable.")
                    }

                    let mut to_be_inserted = self.stack.pop().unwrap().take();

                    /* Check if type of dangling value are equal the to-be-assigned variable */
                    if variable.borrow()._type != to_be_inserted._type 
                        && variable.borrow()._type != Type::UnInit {
                        panic!(
                            "Error: Cannot assign {:?} to {:?} ",
                            to_be_inserted._type,
                            variable.borrow()._type
                        );
                    }

                    to_be_inserted.modifier = variable.borrow().modifier;
                    to_be_inserted._type = variable.borrow()._type.clone();

                    if self.globals.insert(name, to_be_inserted) {
                        let _ = self.globals.delete(name);
                        panic!("Global variable is used before it's initialization.");
                    }
                }
                /*
                    Get var name from constants and craft a ref value based on globals' referenced Value
                */
                OpCode::SetRefGlobal(var_index) => {
                    let referenced_name =
                        match self.frames.last_mut().unwrap().function.chunk.constants[var_index]
                            .clone()
                        {
                            Primitive::String(str) => str,
                            _ => panic!("Invalid var name reference."),
                        };

                    /* Get value to be referenced */
                    let referenced_value = self
                        .globals
                        .get(&referenced_name)
                        .unwrap_or_else(|| panic!("Invalid referenced value."));
                    let referenced_type = referenced_value.borrow()._type.clone();

                    let _ref = Value {
                        value: Primitive::Ref(referenced_value),
                        _type: Type::Ref(Rc::new(referenced_type)),
                        modifier: Modifier::Const,
                    };

                    match self.stack.pop() {
                        Some(value) => match value.take() {
                            Value { _type, .. } => {
                                if _type != _ref._type {
                                    dbg!(&_type, &_ref._type);
                                    panic!("Cannot assign {:?} to {:?}", _ref._type, _type)
                                };
                            }
                        },
                        None => (),
                    }

                    self.stack.push(Rc::new(RefCell::new(_ref)));
                }
                OpCode::JumpIfFalse(offset) => {
                    let value = Rc::clone(self.stack.last().unwrap());

                    match value.borrow().value {
                        Primitive::Bool(v) => {
                            if v == false {
                                /* Set current opcode index to current + offset */
                                unsafe {
                                    self.jump_ip(offset as isize);
                                }

                                continue;
                            }
                        }
                        _ => (),
                    };
                }
                OpCode::JumpIfTrue(offset) => {
                    let value = Rc::clone(self.stack.last().unwrap());

                    match value.borrow().value {
                        Primitive::Bool(v) => {
                            if v == true {
                                /* Set current opcode index to current + offset */
                                unsafe { self.jump_ip(offset as isize) }

                                continue;
                            }
                        }
                        _ => (),
                    };
                }
                OpCode::Jump(offset) => {
                    unsafe {
                        self.jump_ip(offset as isize);
                    }
                    continue;
                }
                OpCode::Loop(offset) => {
                    unsafe { self.putback_ip(offset) }
                    continue;
                }
                OpCode::Call(args_count) => {
                    if self.call_value(args_count) {
                        self.stack.remove(self.stack.len() - 1 - args_count);
                    }

                    continue;
                }
            };

            unsafe { self.advance_ip() }
        }

        Ok(())
    }

    unsafe fn advance_ip(&mut self) {
        unsafe { self.frames.last_mut().unwrap().ip = self.frames.last().unwrap().ip.offset(1) };
    }

    unsafe fn jump_ip(&mut self, offset: isize) {
        unsafe {
            self.frames.last_mut().unwrap().ip = self.frames.last().unwrap().ip.offset(offset)
        };
    }

    unsafe fn putback_ip(&mut self, offset: usize) {
        unsafe { self.frames.last_mut().unwrap().ip = self.frames.last().unwrap().ip.sub(offset) };
    }

    fn call_value(&mut self, args_count: usize) -> bool {
        /* The function calling the code */
        let callee = Rc::clone(
            &self.stack[self
                .stack
                .len()
                .checked_sub(1)
                .unwrap_or(0)
                .checked_sub(args_count)
                .unwrap_or(0)],
        );
        let value = callee.borrow();

        match *value {
            Value {
                value: Primitive::Function(ref f),
                ..
            } => {
                return self.call(f.clone(), args_count);
            }
            Value {
                value: Primitive::NativeFunction(ref f),
                ..
            } => {
                /* Pop function from stack so it remains clean */
                self.stack.remove(self.stack.len() - 1 - args_count);

                let args = &self.stack[
                    (self.stack.len().checked_sub(args_count).unwrap_or(0))
                    ..self.stack.len()
                ];

                self.stack.push(Rc::new(RefCell::new(f.clone().call(args))));

                unsafe { self.advance_ip() }
                false
            }
            _ => panic!("Object {callee:?} is not callabble"),
        }
    }

    pub fn call(&mut self, function: Rc<Function>, args_count: usize) -> bool {
        if function.arity != args_count {
            println!(
                "Expected {} but got {} arguments.",
                function.arity, args_count
            );

            self.runtime_error();
        }

        let stack_len = self.stack.len();
        let bytecode_ptr = &function.chunk.code[0] as *const OpCode;

        let frame = CallFrame {
            function,
            ip: bytecode_ptr,
            arg_offset: stack_len - args_count,
        };

        self.frames.push(frame);

        return true;
    }

    pub fn binary_op(&mut self, op: &str) -> VmResult {
        let b = Rc::clone(&self.stack.pop().ok_or(VmError::new(
            "Value 'b' not loaded. (a [op] b)".to_string(),
            InterpretResult::RuntimeError,
        ))?);

        let a = Rc::clone(&self.stack.pop().ok_or(VmError::new(
            "Value 'a' not loaded. (a [op] b)".to_string(),
            InterpretResult::RuntimeError,
        ))?);

        let mut c = Value::default();

        c.modifier = a.borrow().modifier;
        c._type = a.borrow()._type.clone();

        match op {
            "+" => c.value = a.borrow().value.clone() + b.borrow().value.clone(),
            "*" => c.value = a.borrow().value.clone() * b.borrow().value.clone(),
            "/" => c.value = a.borrow().value.clone() / b.borrow().value.clone(),
            ">" => {
                c.value = Primitive::Bool(a.borrow().value > b.borrow().value);
                c._type = Type::Bool
            }
            "<" => {
                c.value = Primitive::Bool(a.borrow().value < b.borrow().value);
                c._type = Type::Bool
            }
            _ => panic!("Invalid binary operation."),
        }

        self.stack.push(Rc::new(RefCell::new(c)));

        Ok(())
    }

    fn runtime_error(&self) -> ! {
        for call_frame in self.frames.iter().rev() {
            println!("<{}>()", call_frame.function.name);
        }
        panic!()
    }

    fn error(&self, message: String) -> VmResult {
        VmResult::Err(
            VmError { message, _type: InterpretResult::CompilerError }
        )
    }

}
