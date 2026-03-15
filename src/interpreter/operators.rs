use std::fmt;

#[derive(Debug, Default, Copy, Clone)]
pub enum OperatorType {
    Add,      // num + num
    Subtract, // num - num
    Multiply, // num * num
    Divide,   // num / num
    Negate,   // -num
    #[default]
    Unknown,
}

impl OperatorType {
    pub fn is_additive(&self) -> bool {
        matches!(self, OperatorType::Add | OperatorType::Subtract)
    }

    pub fn is_multiplicative(&self) -> bool {
        matches!(self, OperatorType::Multiply | OperatorType::Divide)
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, OperatorType::Negate)
    }
}

impl fmt::Display for OperatorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatorType::Add => write!(f, "+"),
            OperatorType::Subtract => write!(f, "-"),
            OperatorType::Negate => write!(f, "-"),
            OperatorType::Multiply => write!(f, "*"),
            OperatorType::Divide => write!(f, "/"),
            OperatorType::Unknown => write!(f, "?"),
        }
    }
}
