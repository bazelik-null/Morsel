pub enum TokenType {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NUMBER,
    UNKNOWN,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: Option<f64>, // None if TokenType != Number
}