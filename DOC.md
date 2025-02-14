# The compiler shades
This document exposes some shades the compiler/language itself has as well as other general topics as the language grammar and the compiler architecture.

## Topics
- The Language Itself
- The Language Grammar
- Basic Var Assign Flow
- Basic Mathematical Operation Flow


### The Language Itself
asterisk is a lightweight scripting language; It's code is compiled to bytecode representation and the current OP_CODES are:
- Return,
- Constant(usize),
- True,
- False,
- Equal,
- Nil,
- Pop,
- Greater,
- Less,
- Not,
- Add,
- Multiply,
- Divide,
- Negate,
- Print,
- DefineGlobal(usize),
- SetGlobal(usize),
- GetGlobal(usize),

To execute asterisk code a .ask file is required or the language REPL can be used.
Usage: 
    > Read From File: > cargo run -- {file_name}.ask_
    > REPL: > cargo run_

### The Language Grammar
asterisk uses a basic

```rust
let sum = 32 + 4;
```

### The Compiler 
Currently, asterisk can compile mathematical expresions with order operation compatibility so 
```rust
(-1 + 2) * 3 - -4
```
evaluates to **7** following the correct mathematical order.

### Basic Var Assign Flow

### Basic Mathematical Operation Flow