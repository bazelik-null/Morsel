<div align="center">
<h1> Morsel </h1>
<img src="doc/img/logo.png" width="500">
</div>

> [!WARNING]
>
> Work in progress.

## Introduction

**Morsel** is an **interpreted** programming language built in **Rust** as my first Rust project. It combines the
performance and
memory safety of Rust with an easy, expression-based syntax inspired by C, Python, and Rust itself.

## Pipeline

**Morsel** evaluates expressions through a three-stage pipeline:

1. **Lexer** - Converts input string into tokens
2. **Parser** - Builds an Abstract Syntax Tree (AST) from tokens
3. **Evaluator** - Executes the AST and returns a result

```
Input -> [Lexer] -> Tokens -> [Parser] -> AST -> [Evaluator]
```

The `Interpreter` wrapper compiles this pipeline. Use `execute()` to run without returning a result, or
`execute_with_result()` to get the computed value (from last expression). Enable debug mode to see intermediate outputs
at each stage.

## Features

- **Strict typing:** All variables require explicit or inferred types at declaration
- **Immutability by default:** Variables are immutable unless explicitly marked as `mut`
- **Built in Rust:** RUST🚀 RUSTRUSTRUST BLAZINGLY FAST🚀🚀 YEEEAH MEMORY SAFETY🏳️‍🌈 (sorry)
- **Many mathematical functions:** Full support for arithmetic, trigonometric, logarithmic, and exponential functions
- **Familiar syntax:** Morsel inherits syntax from C, Rust, and Python
- **Variable management:** Declare, assign, and manipulate variables with full type safety

## Getting Started

1. **Install Rust**
   ```
   https://rust-lang.org/tools/install/
   ```

2. **Clone the repository**
   ```bash
   git clone https://github.com/bazelik-null/Morsel.git
   cd Morsel
   ```

3. **Build the project**
   ```bash
   cargo build -r  # -r for release mode (optimized)
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

5. **Learn the basics**

- Review `example.msl` to see language in practice

6. **Execute a script**
   ```bash
   ./target/release/Morsel example.msl
   ```

7. **Try the interactive CLI**
   ```bash
   ./target/release/Morsel
   ```

## Language Reference

### Basic Operations

#### Arithmetic

- **Addition:** `x + y`
- **Subtraction:** `x - y`
- **Multiplication:** `x * y`
- **Division:** `x / y`
- **Modulo (remainder):** `x % y`
- **Negation:** `-x`

#### Exponents and Roots

- **Exponentiation:** `x ^ y` - Raises x to the power of y
- **Square root:** `sqrt(x)` - Returns the square root of x
- **Cubic root:** `cbrt(x)` - Returns the cube root of x
- **Nth root:** `root(x, y)` - Returns the y-th root of x

#### Logarithms

- **Logarithm:** `log(x, y)` - Logarithm with base x of argument y
- **Natural logarithm:** `ln(x)` - Logarithm with base e of x

#### Trigonometric Functions

- **Cosine:** `cos(x)`
- **Sine:** `sin(x)`
- **Tangent:** `tan(x)`
- **Arccosine:** `acos(x)`
- **Arcsine:** `asin(x)`
- **Arctangent:** `atan(x)`

### Utility Functions

- **Absolute value:** `abs(x)` - Returns the absolute value of x
- **Rounding:** `round(x)` - Rounds x to the nearest integer
- **Floor:** `floor(x)` - Rounds x down to the nearest integer
- **Ceiling:** `ceil(x)` - Rounds x up to the nearest integer
- **Maximum:** `max(x, ...)` - Returns the largest of the given values
- **Minimum:** `min(x, ...)` - Returns the smallest of the given values
- **Print:** `println(x, ...)` - Outputs x to the console

## Variables and Types

### Variable Declaration

```
let {mut} x{: type} = y;
```

- **`mut` (optional):** Makes the variable mutable so it can be reassigned later
- **`x` (required):** Variable name. Required for variable referencing.
- **`: type` (optional):** Specifies the data type. If omitted, the type is inferred (works only with literals)
- **`y` (required):** All variable declarations must include an initial value or expression

### Available Data Types

- **Integer:** `int` - 64-bit integer
- **Float:** `float` - 64-bit floating-point number
- **String:** `string` - Text data
- **Boolean:** `bool` - Boolean value (true/false)

**Note:** `null` and `any` types exist but cannot be used for variable initialization.

### Variable Assignment

- **Assignment:** `x = y;` - Reassign an existing variable to a new value (variable must be declared as `mut` and types
  should match)

### Comments

- **Comments:** `// Comment` - Comments like anywhere else. Nothing special.

## Project Structure

- **Entry point** (`src/main.rs`) - Launches CLI or evaluates file from argument
- **Command Line Interface** (`src/cli/`) - User interface that accepts commands and file inputs
- **Core** (`src/morsel_core/`) - Core of the Morsel interpreter
    - **Interpreter** (`src/morsel_core/interpreter.rs`) - Wrapper for easy code execution
    - **Lexer** (`src/morsel_core/lexing/`) - Tokenizes input string into an array of tokens
    - **Parser** (`src/morsel_core/parsing/`) - Builds an Abstract Syntax Tree from tokens
    - **Evaluator** (`src/morsel_core/evaluating/`) - Executes AST

## Screenshot

<img src="doc/img/screenshot.png">