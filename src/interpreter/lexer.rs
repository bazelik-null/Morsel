use crate::interpreter::tokens::{OperatorType, Token};
use regex::Regex;

pub fn tokenize(string: &String) -> Result<Vec<Token>, &'static str> {
    let mut tokens: Vec<Token> = Vec::new();

    // Parse String into raw tokens using regex
    // This regex splits at math operators
    let regex: Regex = Regex::new(r"(\d+|[+\-*/()=])").unwrap();
    let lexeme_vec: Vec<String> = regex
        .find_iter(string.as_ref())
        .map(|m| m.as_str().to_string())
        .collect();

    if string.is_empty() || string.chars().all(char::is_whitespace) {
        return Err("String is empty");
    }

    // Parse each token
    for lexeme in lexeme_vec {
        let token: Token = parse_token(&lexeme);
        tokens.push(token);
    }

    Ok(tokens)
}

fn parse_token(lexeme: &str) -> Token {
    let token: Token;

    // Handle digits
    if lexeme.parse::<f64>().is_ok() {
        let value: f64 = lexeme.parse::<f64>().unwrap();

        token = Token::Number(value);
    }
    // Handle symbols
    else {
        match lexeme {
            "+" => token = Token::Operator(OperatorType::ADD),
            "-" => token = Token::Operator(OperatorType::SUBTRACT),
            "*" => token = Token::Operator(OperatorType::MULTIPLY),
            "/" => token = Token::Operator(OperatorType::DIVIDE),
            _ => token = Token::Operator(OperatorType::UNKNOWN),
        }
    }
    token
}
