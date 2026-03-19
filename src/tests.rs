#[cfg(test)]
use crate::cli::backend::cli_execute;

// ARITHMETIC OPERATIONS

#[test]
fn test_addition() {
    let code = "fn main() { let x: int = 5 + 3; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_subtraction() {
    let code = "fn main() { let x: int = 10 - 4; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_multiplication() {
    let code = "fn main() { let x: int = 6 * 7; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_division() {
    let code = "fn main() { let x: float = 20 / 4; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_modulo() {
    let code = "fn main() { let x: float = 17 % 5; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_negation() {
    let code = "fn main() { let x: int = -42; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_complex_arithmetic_expression() {
    let code = "fn main() { let x: int = 10 + 5 * 2 - 3; }";
    assert!(cli_execute(code, false).is_ok());
}

// EXPONENTS AND ROOTS

#[test]
fn test_exponentiation() {
    let code = "fn main() { let x: int = 2 ^ 8; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_square_root() {
    let code = "fn main() { let x: int = sqrt(16); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_cubic_root() {
    let code = "fn main() { let x: int = cbrt(27); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_nth_root() {
    let code = "fn main() { let x: int = root(16, 4); }";
    assert!(cli_execute(code, false).is_ok());
}

// LOGARITHMS

#[test]
fn test_logarithm() {
    let code = "fn main() { let x: int = log(10, 100); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_natural_logarithm() {
    let code = "fn main() { let x: float = ln(2.718); }";
    assert!(cli_execute(code, false).is_ok());
}

// TRIGONOMETRIC FUNCTIONS

#[test]
fn test_cosine() {
    let code = "fn main() { let x: int = cos(0); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_sine() {
    let code = "fn main() { let x: int = sin(0); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_tangent() {
    let code = "fn main() { let x: int = tan(0); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_arccosine() {
    let code = "fn main() { let x: int = acos(1); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_arcsine() {
    let code = "fn main() { let x: int = asin(0); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_arctangent() {
    let code = "fn main() { let x: float = atan(1); }";
    assert!(cli_execute(code, false).is_ok());
}

// UTILITY FUNCTIONS

#[test]
fn test_absolute_value() {
    let code = "fn main() { let x: int = abs(-42); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_rounding() {
    let code = "fn main() { let x: int = round(3.7); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_floor() {
    let code = "fn main() { let x: int = floor(3.9); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_ceiling() {
    let code = "fn main() { let x: int = ceil(3.1); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_max_two_values() {
    let code = "fn main() { let x: int = max(10, 20); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_max_multiple_values() {
    let code = "fn main() { let x: int = max(5, 15, 10, 20, 3); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_min_two_values() {
    let code = "fn main() { let x: int = min(10, 20); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_min_multiple_values() {
    let code = "fn main() { let x: int = min(5, 15, 10, 20, 3); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_println_single_value() {
    let code = "fn main() { println(42); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_println_multiple_values() {
    let code = "fn main() { println(10, 20, \"hello\", true); }";
    assert!(cli_execute(code, false).is_ok());
}

// TYPE INFERENCE WITH OPERATIONS

#[test]
fn test_float_arithmetic() {
    let code = "fn main() { let x: float = 3.5 + 2.5; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_mixed_int_float_arithmetic() {
    let code = "fn main() { let x: float = 10 + 3.5; }";
    assert!(cli_execute(code, false).is_ok());
}

// VARIABLE OPERATIONS WITH FUNCTIONS

#[test]
fn test_variable_with_sqrt_function() {
    let code = "fn main() { let mut x: float = sqrt(25); }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_variable_reassignment_with_arithmetic() {
    let code = "fn main() { let mut x: int = 10; x = 20 + 5; }";
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_function_with_arithmetic_arguments() {
    let code = r#"
        fn main() {
            fn calculate(a: int, b: int) {
                let result: int = a + b;
            }
            calculate(5, 10);
        }
    "#;
    assert!(cli_execute(code, false).is_ok());
}

#[test]
fn test_nested_function_calls_with_math() {
    let code = r#"
        fn main() {
            fn double(x: int) {
                let result: int = x * 2;
                result;
            }
            let val: int = double(5);
        }
    "#;
    assert!(cli_execute(code, false).is_ok());
}

// EDGE CASES AND ERROR HANDLING

#[test]
fn test_division_by_zero_fails() {
    let code = "fn main() { let x: float = 10 / 0; }";
    assert!(cli_execute(code, false).is_err());
}

#[test]
fn test_invalid_function_call_fails() {
    let code = "fn main() { let x: int = invalid_func(5); }";
    assert!(cli_execute(code, false).is_err());
}

#[test]
fn test_wrong_number_of_arguments_fails() {
    let code = "fn main() { let x: float = cos(5, 6); }";
    assert!(cli_execute(code, false).is_err());
}

#[test]
fn test_type_mismatch_in_function_fails() {
    let code = r#"
        fn main() {
            fn add(a: int, b: int) {}
            add(5, "string");
        }
    "#;
    assert!(cli_execute(code, false).is_err());
}

#[test]
fn test_reassign_immutable_variable_fails() {
    let code = "fn main() { let x: int = 10; x = 20; }";
    assert!(cli_execute(code, false).is_err());
}
