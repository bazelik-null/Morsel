# Morsel Bytecode Specification

## Overview

This document describes the bytecode instruction set, stack model, and function calling mechanism for the Morsel
stack-based virtual machine.

## Stack Model

### Stack Semantics

The **stack** is a Last-In-First-Out (LIFO) data structure where all computation occurs. Every instruction either:

- **Pushes** values onto the stack
- **Pops** values from the stack to perform operations
- **Manipulates** the stack structure itself

### Stack Representation

The stack grows upward, with the **top** being the most recently pushed value.

### Stack Overflow and Underflow

- **Stack overflow** occurs when pushing exceeds available memory.
- **Stack underflow** occurs when popping from an empty stack.

Both conditions halt execution with an error.

## Calling Convention

### Function Call Sequence

**Caller:**

1. Push all arguments onto the stack in order (first argument pushed first, last argument pushed last).
2. Emit `CALL function_label`.
3. After the call returns, the return value is at the top of the stack.

**Callee (start):**

1. Arguments are already on the stack in the order they were pushed.
2. Optionally, save argument values to local variables if needed for nested calls.
3. Perform function logic.

**Callee (end):**

1. Ensure the return value is at the top of the stack.
2. Emit `RET`.

### Example: Function Call

```
Caller:
  PUSH 10          ; Push first argument
  PUSH 20          ; Push second argument
  CALL add_label   ; Call add(10, 20)
  ; Return value now on top of stack

Callee (add):
  ; Arguments: 10 (second from top), 20 (top of stack)
  ADD              ; Pop 20 and 10, push their sum (30)
  RET              ; Return with 30 on top of stack
```

## Instruction Set

### Stack Manipulation Instructions

- **PUSH** (operand: imm i32): Push immediate value onto stack
- **POP** (operand: none): Pop and discard top stack value
- **DUP** (operand: none): Duplicate top stack value: `[a] -> [a, a]`
- **SWAP** (operand: none): Swap top two values: `[a, b] -> [b, a]`
- **ROT** (operand: none): Rotate top 3 values: `[a, b, c] -> [c, a, b]`

### Arithmetic Instructions

- **ADD** (stack effect: `[a, b] -> [a+b]`): Pop two values, push their sum. Polymorphic (works with integers
  and strings)
- **SUB** (stack effect: `[a, b] -> [a-b]`): Pop two values, push difference
- **MUL** (stack effect: `[a, b] -> [a*b]`): Pop two values, push product
- **DIV** (stack effect: `[a, b] -> [a/b]`): Pop two values, push quotient (integer division)
- **REM** (stack effect: `[a, b] -> [a%b]`): Pop two values, push remainder
- **POW** (stack effect: `[a, b] -> [a^b]`): Pop two values, push power (a raised to b)
- **NEG** (stack effect: `[a] -> [-a]`): Negate top stack value

### Logical Instructions

- **AND** (stack effect: `[a, b] -> [a&b]`): Pop two values, push bitwise AND
- **OR** (stack effect: `[a, b] -> [a|b]`): Pop two values, push bitwise OR
- **XOR** (stack effect: `[a, b] -> [a^b]`): Pop two values, push bitwise XOR
- **NOT** (stack effect: `[a] -> [!a]`): Bitwise NOT of top stack value

### Shift Instructions

- **SLA** (stack effect: `[a, b] -> [a << b]`): Pop two values, push left-shifted result
- **SRA** (stack effect: `[a, b] -> [a >> b]`): Pop two values, push right-shifted result

### Comparison Instructions

- **EQ** (stack effect: `[a, b] -> [a == b ? 1 : 0]`): Pop two values, push 1 if equal, 0 otherwise
- **NE** (stack effect: `[a, b] -> [a != b ? 1 : 0]`): Pop two values, push 1 if not equal, 0 otherwise
- **LT** (stack effect: `[a, b] -> [a < b ? 1 : 0]`): Pop two values, push 1 if less than, 0 otherwise
- **GT** (stack effect: `[a, b] -> [a > b ? 1 : 0]`): Pop two values, push 1 if greater than, 0 otherwise
- **LE** (stack effect: `[a, b] -> [a <= b ? 1 : 0]`): Pop two values, push 1 if less or equal, 0 otherwise
- **GE** (stack effect: `[a, b] -> [a >= b ? 1 : 0]`): Pop two values, push 1 if greater or equal, 0 otherwise

### Memory Instructions

- **LOAD** (operand: none, stack effect: `[addr] -> [value]`): Pop address from stack, get value from that
  memory address
- **STORE** (operand: none, stack effect: `[addr, value] -> [...]`): Pop value and address, store value to memory
  address
- **LOAD_LOCAL** (operand: index u8, stack effect: `[id] -> [value]`): Load local variable at index onto stack
- **STORE_LOCAL** (operand: index u8, stack effect: `[value] -> [id]`): Pop value from stack, store to local
  variable at index

### Control Flow Instructions

- **JMP** (operand: label i32, stack effect: `[...] -> [...]`): Unconditional jump to instruction at label
- **JMPT** (operand: label i32, stack effect: `[cond] -> [...]`): Pop condition; jump if non-zero (true)
- **JMPF** (operand: label i32, stack effect: `[cond] -> [...]`): Pop condition; jump if zero (false)
- **CALL** (operand: label i32, stack effect: `[args...] -> [return_value]`): Call function at label; return
  address saved implicitly
- **RET** (operand: none, stack effect: `[return_value] -> [return_value]`): Return from function; stack cleared
  except return value

### Miscellaneous Instructions

- **NOP** (operand: none): No operation
- **HALT** (operand: none): Stop execution

## Local Variables

### Allocation and Access

Local variables are stored in a **local frame** associated with each function call. Each function has a frame with a
fixed number of slots, indexed from 0 onwards.

- **LOAD_LOCAL index**: Pushes the value at local slot `index` onto the stack.
- **STORE_LOCAL index**: Pops a value from the stack and stores it in local slot `index`.

### Lifetime

Local variables exist for the duration of a function call. When a function returns via `RET`, the local frame is
destroyed and the next function call gets a fresh frame.

### Limits

A function can have at most **256 local variables** (indices 0–255). Attempting to allocate more is an error.

### Example: Multiple Locals

```
Function: compute()
  Locals: a (index 0), b (index 1), c (index 2), d (index 3)
  
  Bytecode:
    PUSH 10           ; Push 10
    STORE_LOCAL 0     ; a = 10
    
    PUSH 20           ; Push 20
    STORE_LOCAL 1     ; b = 20
    
    LOAD_LOCAL 0      ; Push a
    LOAD_LOCAL 1      ; Push b
    ADD               ; Compute a + b
    STORE_LOCAL 2     ; c = a + b
    
    LOAD_LOCAL 2      ; Push c
    PUSH 2            ; Push 2
    MUL               ; Compute c * 2
    STORE_LOCAL 3     ; d = c * 2
    
    LOAD_LOCAL 3      ; Push d (return value)
    RET               ; Return d
```

## Data Sections and Globals

### Global Data Storage

Global variables, string literals, and array constants are stored in the **heap** (data blob) managed by the
`Executable`.

### Accessing Global Data

1. **Allocate space**: During code generation, call `Executable::insert_data(id, bytes, name)` to store data.
2. **Load data ID**: Use `PUSH data_id` to push the data section ID onto the stack.
3. **Resolve at compilation**: The linker resolves the data ID to a heap address via the `RelocationTable`.
4. **Access via memory**: Use `LOAD` or `STORE` to read from or write to the resolved heap address.

### Example: Global String

```
Data section ID: 0 contains "hello" (5 bytes)

Bytecode:
  PUSH 0      ; Push data_id (section 0)
  LOAD        ; Dereference to get heap address; push first byte
```

## Labels and Jumps

### Label Registration

Labels are resolved at executable construction time. Each label is a unique 32-bit identifier that maps to an
instruction offset.

When emitting a `CALL`, `JMP`, `JMPT`, or `JMPF` instruction, use the label ID, The linker resolves label IDs to
instruction offsets.

### Example: If-Then-Else

```
Condition: x > 5

Bytecode:
  PUSH 5              ; Push 5
  GT                  ; Pop x and 5, push (x > 5) as 1 or 0
  JMPF else_label     ; If false (0), jump to else
  
  ; Then branch
  PUSH 10
  JMP end_label
  
  ; Else branch
  else_label:
  PUSH 20
  
  end_label:
  ; Continue (value is on stack)
```

## Function Declarations and Calls

### Declaring a Function

A function declaration in the AST generates:

1. A unique label for the function entry point.
2. Bytecode for the function body.
3. A `RET` instruction at the end.

The label is registered so that `CALL` instructions can reference it.

### Calling a Function

1. Push all arguments onto the stack in order.
2. Emit `CALL function_label`.
3. After the call, the return value is at the top of the stack.

### Example: Multi-Argument Function

```
Function definition: add(x, y)
  add_label:
  LOAD_LOCAL 0      ; Load x (first argument)
  LOAD_LOCAL 1      ; Load y (second argument)
  ADD               ; Compute x + y
  RET               ; Return result

Function call: result = add(10, 20)
  PUSH 10           ; Push first argument
  PUSH 20           ; Push second argument
  CALL add_label    ; Call function
  STORE_LOCAL 2     ; Store result in local variable
```

## Instruction Encoding

### Fixed 5-Byte Format

Each instruction is encoded as **5 bytes**:

- **Byte 0**: Opcode (1 byte, u8)
- **Bytes 1–4**: Operand (4 bytes, i32, little-endian)

## Executable Format

The `Executable` struct contains:

- **Header**: Contains magic number, version info, etc.
- **Instructions**: An array of bytecode instructions.
- **Data blob**: Raw bytes for global data (strings, arrays, constants).