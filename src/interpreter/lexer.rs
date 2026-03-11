use regex::Regex;
use crate::interpreter::tokens;

pub fn tokenize(string: String) -> Option<Vec<tokens::Token>> {
	// Create a token vector
	let mut tokens: Vec<tokens::Token> = Vec::new();
	tokens.reserve(string.len());

	// Parse String into raw tokens using regex
	// This regex splits at math operators
	let re = Regex::new(r"(\d+|[+\-*/()=])").unwrap();
	let lexeme_vec: Vec<String> = re
		.find_iter(string.as_ref())
		.map(|m| m.as_str().to_string())
		.collect();

	// Validate string
	if string.is_empty() || string.chars().all(char::is_whitespace) || string.len() == 0 {
		return None;
	}

	// Parse each token
	for lexeme in lexeme_vec {
		let token: tokens::Token = parse_token(&lexeme);
		tokens.push(token);
	}
	return Some(tokens);
}

fn parse_token(lexeme: &String) -> tokens::Token {
	let mut token: tokens::Token = tokens::Token { token_type: tokens::TokenType::UNKNOWN, value: None };

	// Handle digits
	if lexeme.parse::<f64>().is_ok() {
		let value: f64 = lexeme.parse::<f64>().unwrap();

		token.token_type = tokens::TokenType::NUMBER;
		token.value = Some(value.clone());
	}
	// Handle symbols
	else {
		match lexeme.as_str() {
			"+" => token.token_type = tokens::TokenType::ADD,
			"-" => token.token_type = tokens::TokenType::SUBTRACT,
			"*" => token.token_type = tokens::TokenType::MULTIPLY,
			"/" => token.token_type = tokens::TokenType::DIVIDE,
			_   => token.token_type = tokens::TokenType::UNKNOWN,
		}
	}
	return token
}
