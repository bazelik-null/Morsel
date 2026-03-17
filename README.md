<div align="center">
<h1> RetardCalc </h1>
<img src="doc/img/logo.png" width="500">
</div>

# Introduction

**RetardCalc** is a lightweight expression evaluator built in Rust as my first Rust project. <br>
RetardCalc evaluates mathematical expressions using a three-stage pipeline:

1. **Lexer** (`lexer::tokenize`) - Converts input string into tokens
2. **Parser** (`parser::parse`) - Builds an Abstract Syntax Tree (AST) from tokens
3. **Evaluator** (`evaluator::eval`) - Traverses the AST and computes the result

# Available Functions

### Arithmetic Operations

- **Addition:** `x + y`
- **Subtraction:** `x - y`
- **Multiplication:** `x * y`
- **Division:** `x / y`

### Exponent and Logarithmic Operations

- **Exponentiation:** `x ^ y`
- **Square root:** `sqrt(x)`
- **Logarithm:** `x log(y)` (where x is base, y is argument)
- **Natural logarithm:** `ln(x)`

### Trigonometric Functions

- **Cosine:** `cos(x)`
- **Sine:** `sin(x)`
- **Tangent:** `tan(x)`
- **Arccosine:** `acos(x)`
- **Arcsine:** `asin(x)`
- **Arctangent:** `atan(x)`

### Miscellaneous Operations

- **Negation:** `-x`
- **Modulo (remainder):** `x % y`
- **Absolute value:** `abs(x)`
- **Rounding:** `round(x)`

# Project Structure

1. **Entry point** (`src/main.rs`) - Launches CLI or evaluates file from argument, outputting only the result.
2. **Command Line Interface** (`src/cli/`) - User interface that accepts commands and file inputs.
3. **Interpreter** (`src/interpreter/`) - Core evaluation engine with three stages:
    1. **Lexer** (`src/interpreter/lexer/`) - Tokenizes input string into an array of tokens.
    2. **Parser** (`src/interpreter/ast/`) - Builds an Abstract Syntax Tree from tokens.
    3. **Evaluator** (`src/interpreter/evaluator.rs`) - Evaluates the AST and returns the result.

# Screenshot

<img src="doc/img/screenshot.png">
