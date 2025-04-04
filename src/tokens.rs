#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: String,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: String, lexeme: String) -> Self {
        Token { token_type, lexeme }
    }
}