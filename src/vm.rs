use std::cell::RefCell;
use std::rc::Rc;

use crate::chunk::*;
use crate::compiler::compile;
use crate::types::hash_table::HashTable;
use crate::utils::parse_type;
use crate::utils::print::{print_stack, print_value};
use crate::value::{Function, Modifier, Primitive, Type, Value};

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub type Stack = Vec<Rc<RefCell<Value>>>;
pub struct Vm {
    frames: Vec<CallFrame>,
    stack: Stack,
    globals: HashTable<String, Value>,
    strings: HashTable<String, String>,
}

#[derive(Debug)]
pub struct CallFrame {
    pub function: Function,
    pub ip: *const OpCode,
    /* Init and final of frame stack scope range */
    pub slots: (usize, usize),
}

// impl CallFrame {
//     const FRAMES_MAX: u32 = u32::MAX;
//     const STACK_MAX: u32 = u32::MAX;
// }

impl Default for CallFrame {
    fn default() -> Self {
        CallFrame {
            function: &Function::default(),
            op_code: None,
            slots: None,
        }
    }
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
    /// Parse a Vec<char> into valid asterisk state.
    ///
    /// This function is the compiler itself, compile the source code into chunks and run it's emitted Bytecodes.
    ///
    pub fn interpret(&mut self, source_code: String) -> InterpretResult {
        let source_code = format!(
            "{}
        EOF",
            source_code
        );

        let result = compile(source_code, &mut self.stack);
        if result.is_none() { return InterpretResult::CompileError };

        self.call(result.unwrap().0, 0);

        self.run()
    }

    /// Loop throught returned Bytecode vector (code vec) handling it's behavior.
    ///
    fn run(&mut self) -> InterpretResult {
        let mut op_status = InterpretResult::CompileError;
        let frame = self.frames.last().unwrap();

        #[cfg(feature = "debug")]
        println!("Constants Vec: {:?}", self.function.chunk.constants);

        let mut bytecode_index: usize = 0;
        let code_len =  self.function.chunk.code.len();

        while bytecode_index < code_len {
            let code = self.function.chunk.code[bytecode_index].clone();

            #[cfg(feature = "debug")]
            {
                print!("\n");
                print_stack(&self.function.chunk);
                println!("current code: {:?}", self.function.chunk.code[bytecode_index]);
            }

            let chunk = &mut self.function.chunk;

            op_status = match code {
                OpCode::Return => {
                    let _return = self
                        .stack
                        .pop()
                        .expect(
                            &format!("Could not pop empty value from: {:?}", self.frames.last().unwrap().function.name)
                        );

                    self.frames.pop().unwrap();
                    if self.frames.len() == 0 {
                        return InterpretResult::Ok;
                    }

                    /* Advance current call ip offset by 1 */
                    unsafe { self.frames.last_mut().unwrap().ip = self.frames.last().unwrap().ip.offset(1); }
                    self.stack.push(_return);

                    continue
                }
                OpCode::Negate => {
                    {
                        // let chunk = self.chunk.as_mut();
                        let to_be_negated = chunk.stack.pop().unwrap().take();

                        match to_be_negated {
                            Value {
                                value: Primitive::Int(value),
                                modifier,
                                _type,
                            } => chunk.stack.push(Rc::new(RefCell::new(Value {
                                value: Primitive::Int(-value),
                                modifier,
                                _type,
                            }))),
                            Value {
                                value: Primitive::Float(value),
                                modifier,
                                _type,
                            } => chunk.stack.push(Rc::new(RefCell::new(Value {
                                value: Primitive::Float(-value),
                                modifier,
                                _type,
                            }))),
                            Value {
                                value: Primitive::Bool(value),
                                modifier,
                                _type,
                            } => chunk.stack.push(Rc::new(RefCell::new(Value {
                                value: Primitive::Bool(!value),
                                modifier,
                                _type,
                            }))),
                            _ => panic!("Operation not allowed."),
                        }
                    }

                    InterpretResult::Ok
                }
                OpCode::Not => {
                    // let chunk = self.chunk.as_mut();
                    let to_be_negated = chunk.stack.last().unwrap().take();

                    match to_be_negated {
                        Value { value: Primitive::Bool(value), .. } => {
                            chunk.stack.last().unwrap().borrow_mut().value = Primitive::Bool(!value)
                        }
                        _ => panic!("Value should be a boolean."),
                    };

                    InterpretResult::Ok
                }
                OpCode::Add => {
                    self.binary_op("+");

                    InterpretResult::Ok
                }
                OpCode::Multiply => {
                    self.binary_op("*");

                    InterpretResult::Ok
                }
                OpCode::Divide => {
                    self.binary_op("/");

                    InterpretResult::Ok
                }
                OpCode::True => {
                    // let chunk = self.chunk.as_mut();

                    chunk.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(true),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));

                    InterpretResult::Ok
                }
                OpCode::False => {
                    // let chunk = self.chunk.as_mut();
                    chunk.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(false),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));

                    InterpretResult::Ok
                }
                OpCode::Equal => {
                    // let chunk = self.chunk.as_mut();
                    let a = chunk.stack.pop().unwrap();
                    let b = chunk.stack.pop().unwrap();

                    chunk.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(a == b),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));

                    InterpretResult::Ok
                }
                OpCode::PartialEqual => {
                    // let chunk = self.chunk.as_mut();
                    let a = chunk.stack.pop().unwrap();
                    let b = chunk.stack.pop().unwrap();

                    chunk.stack.push(Rc::clone(&b));
                    chunk.stack.push(Rc::new(RefCell::new(Value {
                        value: Primitive::Bool(a == b),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    })));

                    InterpretResult::Ok
                }
                OpCode::Greater => {
                    self.binary_op(">");

                    InterpretResult::Ok
                }
                OpCode::Less => {
                    self.binary_op("<");

                    InterpretResult::Ok
                }
                OpCode::Print => {
                    let value = chunk
                        .stack
                        .pop()
                        .expect("Could not find value to print.");

                    print_value(&value.borrow().value);

                    InterpretResult::Ok
                }
                OpCode::Pop => {
                    // let chunk = self.chunk.as_mut();

                    chunk.stack.pop().expect("Error on pop: stack underflow.");

                    InterpretResult::Ok
                }
                // Bring value from constants vector to stack
                OpCode::Constant(var_index) => {
                    // let chunk = self.chunk.as_mut();
                    let constant = chunk.constants[var_index].clone();
                    let _type = parse_type(&constant);

                    chunk.stack.push(Rc::new(RefCell::new(Value {
                        value: constant,
                        modifier: Modifier::Unassigned,
                        _type,
                    })));

                    InterpretResult::Ok
                }
                /* Check Local Type */
                OpCode::DefineLocal(var_index, modifier) => {
                    // let chunk = self.chunk.as_mut();

                    let variable = Rc::clone(&chunk.stack[var_index]);

                    /* Type Check */
                    if chunk.stack.last().unwrap().borrow().value == Primitive::Void(()) {
                        if chunk.stack.last().unwrap().borrow()._type != variable.borrow()._type {
                            panic!("Cannot assign {:?} to {:?}",  variable.borrow()._type, chunk.stack.last().unwrap().borrow()._type)
                        }

                        chunk.stack.pop();
                    }

                    variable.borrow_mut().modifier = modifier;

                    InterpretResult::Ok
                }
                /*
                    Set new value to local variable.
                */
                OpCode::SetLocal(var_index, modifier) => {
                    // let chunk = self.chunk.as_mut();

                    let variable = Rc::clone(&chunk.stack[var_index]);

                    /* Type Check */
                    if chunk.stack.last().unwrap().borrow().value == Primitive::Void(()) {
                        if chunk.stack.last().unwrap().borrow()._type != variable.borrow()._type {
                            panic!("Cannot assign {:?} to {:?}", chunk.stack.last().unwrap().borrow()._type, variable.borrow()._type)
                        }

                        chunk.stack.pop();
                    }

                    if modifier != Modifier::Mut {
                        panic!("Cannot assign to immutable variable.")
                    }

                    let value = chunk.stack.pop().unwrap().take();

                    variable.borrow_mut().value = value.value;

                    InterpretResult::Ok
                }
                /*
                    // NOTE Check for duplicated variable
                    Get value from value position and load it into the top of stack,
                    this way other operations can interact with the value.
                */
                OpCode::GetLocal(var_index) => {
                    // let chunk = self.chunk.as_mut();

                    let variable = Rc::clone(&chunk.stack[var_index]);

                    chunk.stack.push(variable);

                    InterpretResult::Ok
                }
                /* 
                    As local variables are defined as not the same as global ones, it needs a different treatment
                    Set ref to stack bucket where variable value is and let it available on stack.
                */
                OpCode::SetRefLocal(var_value_index) => {
                    // let chunk = self.chunk.as_mut();

                    let referenced_value = Rc::clone(&chunk.stack[var_value_index]);

                    if chunk.stack.last().unwrap().borrow().value == Primitive::Void(()) {
                        if let Type::Ref(r) = &chunk.stack.last().unwrap().borrow()._type {
                            if **r != referenced_value.borrow()._type {
                                panic!("Cannot assign {:?} to Ref({:?})", chunk.stack.last().unwrap().borrow()._type, referenced_value.borrow()._type);
                            };
                        };

                        chunk.stack.pop();
                    }

                    let _ref = Value {
                        value: Primitive::Ref(Rc::clone(&referenced_value)),
                        _type: Type::Ref(Rc::new((referenced_value).borrow()._type.clone())),
                        modifier: Modifier::Const,
                    };

                    chunk.stack.push(Rc::new(RefCell::new(_ref)));

                    InterpretResult::Ok
                }
                /*
                    Get variable name from constants and value from top of stack assigning it to globals HashMap
                */
                OpCode::DefineGlobal(var_name_index, modifier) => {
                    // let chunk = self.chunk.as_mut();
                    let var_name = &chunk.constants[var_name_index];

                    let mut variable = match chunk.stack.pop().unwrap().take() {
                        /* This match only a dummy type specifier */
                        Value { value: Primitive::Void(..), _type, .. } => {
                            let value = chunk.stack.pop().unwrap().take();

                            if value._type != _type { panic!("Cannot assign {:?} to {:?}", value._type, _type) }

                            value
                        },
                        /* Inferred Type, so no match is needed */
                        v => v,
                    };
                    variable.modifier = modifier;

                    /*  Only strings are allowed to be var names */
                    match var_name {
                        Primitive::String(name) => {
                            self.globals.insert(name, variable);
                        }
                        _ => panic!("Invalid global variable name."),
                    }

                    InterpretResult::Ok
                }
                /* 
                    Get address from get globals and set it in stack.
                    This means every value referencing this value is referencing the value itself, not a copy on stack as globals and stack are exchangeable.
                */
                OpCode::GetGlobal(var_index) => {
                    // let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[var_index] {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let value = match self.globals.get(name) {
                        Some(value) => value,
                        None => panic!("Use of undeclared variable '{}'", &name),
                    };

                    chunk.stack.push(Rc::clone(&value));

                    InterpretResult::Ok
                }
                /*
                    Re-assign to already set global variable.
                */
                OpCode::SetGlobal(name_index) => {
                    // let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[name_index] {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let variable = self.globals.get(name).unwrap();
                    if variable.borrow().modifier != Modifier::Mut {
                        panic!("Cannot assign to a immutable variable.")
                    }

                    let mut to_be_inserted = chunk.stack.pop().unwrap().take();

                    /* Check if type of dangling value are equal the to-be-assigned variable */
                    if variable.borrow()._type != to_be_inserted._type {
                        panic!(
                            "Error: Cannot assign {:?} to {:?} ",
                            to_be_inserted._type, variable.borrow()._type
                        );
                    }

                    to_be_inserted.modifier = variable.borrow().modifier;

                    if self.globals.insert(name, to_be_inserted) {
                        let _ = self.globals.delete(name);
                        panic!("Global variable is used before it's initialization.");
                    }

                    InterpretResult::Ok
                }
                /* Let var type information available on stack, this is used in explicit variable declaration */
                OpCode::SetType(t) => {
                    let dummy_value = Value { _type: t, ..Default::default() };
                    chunk.stack.push(Rc::new(RefCell::new(dummy_value)));

                    InterpretResult::Ok
                }
                /* 
                    Get var name from constants and craft a ref value based on globals' referenced Value 
                */
                OpCode::SetRefGlobal(var_index) => {
                    let referenced_name = match chunk.constants[var_index].clone() {
                        Primitive::String(str) => str,
                        _ => panic!("Invalid var name reference."),
                    };

                    /* Get value to be referenced */
                    let referenced_value = self.globals.get(&referenced_name).unwrap_or_else(|| panic!("Invalid referenced value."));
                    let referenced_type = referenced_value.borrow()._type.clone();

                    let _ref = Value {
                        value: Primitive::Ref(referenced_value),
                        _type: Type::Ref(Rc::new(referenced_type)),
                        modifier: Modifier::Const
                    };

                    match chunk.stack.pop() {
                        Some(value) => {
                            match value.take() {
                                Value { _type, .. } => { 
                                    if _type != _ref._type {
                                        dbg!(&_type, &_ref._type);
                                        panic!("Cannot assign {:?} to {:?}", _ref._type, _type)
                                    };
                                },
                            }
                        },
                        None => (),
                    }

                    chunk.stack.push(Rc::new(RefCell::new(_ref)));

                    InterpretResult::Ok
                }
                OpCode::JumpIfFalse(offset) => {
                    /* Check for false conditional on top of stack */
                   match chunk.stack.last().unwrap().borrow().value {
                        Primitive::Bool(v) => {
                            if v == false {
                                /* Set current opcode index to current + offset */
                                bytecode_index += offset;

                                continue;
                            }
                        }
                        _ => ()
                    }

                    InterpretResult::Ok
                }
                OpCode::JumpIfTrue(offset) => {
                    /* Check for false conditional on top of stack */
                   match chunk.stack.last().unwrap().borrow().value {
                        Primitive::Bool(v) => {
                            if v == true {
                                /* Set current opcode index to current + offset */
                                bytecode_index += offset;

                                continue;
                            }
                        }
                        _ => ()
                    }

                    InterpretResult::Ok
                }
                OpCode::Jump(offset) => {
                    bytecode_index += offset;

                    continue;
                }
                OpCode::Loop(offset) => {
                    bytecode_index -= offset;

                    continue;
                }
            };

            bytecode_index += 1;
        }

        op_status
    }

    fn call_value(&mut self, args_count: usize) -> bool {
        /* The function calling the code */
        let callee = Rc::clone(&self.stack[self.stack.len() - 1 - args_count]);
        let value = callee.borrow();

        match value.clone() {
            Value { _type: Type::Fn, value, .. } => {
                let callee_function = match value {
                    Primitive::Function(fun) => fun,
                    _ => panic!("Tried to call not callabble object: {value:?}")
                };

                return self.call(callee_function, args_count);
            },
            _ => panic!("Object {callee:?} is not callabble"),
        }
    }

    fn call(&mut self, function: Function, args_count: usize) -> bool {
        if function.arity != args_count {
            println!("Expected {} but got {} arguments.", function.arity, args_count);
            self.runtime_error();
            return false;
        }

        let stack_len = self.stack.len();
        let bytecode_ptr= &function.chunk.code[0] as *const OpCode;

        let frame = CallFrame {
            function,
            ip: bytecode_ptr,
            slots: (stack_len - args_count, stack_len),
        };

        self.frames.push(frame);
        return true;
    }

    fn binary_op(&mut self, op: &str) -> InterpretResult {
        let b = Rc::clone(&self.stack.pop().expect("Value b not loaded."));
        let a = Rc::clone(&self.stack.pop().expect("Value a not loaded"));

        let mut c = Value::default();

        c.modifier = a.borrow().modifier;
        c._type = a.borrow()._type.clone();

        match op {
            "+" => c.value = a.borrow().value.clone() + b.borrow().value.clone(),
            "*" => c.value = a.borrow().value.clone() * b.borrow().value.clone(),
            "/" => c.value = a.borrow().value.clone() / b.borrow().value.clone(),
            ">" => { c.value = Primitive::Bool(a.borrow().value > b.borrow().value); c._type = Type::Bool },
            "<" => { c.value = Primitive::Bool(a.borrow().value < b.borrow().value); c._type = Type::Bool },
            _ => panic!("Invalid binary operation."),
        }
        
        self.stack.push(Rc::new(RefCell::new(c)));

        InterpretResult::Ok
    }

    fn runtime_error(&self) {
        println!("TODO");
        for call_frame in self.frames.iter().rev() {
            println!("<{}>()", call_frame.function.name);
        }
    }
}