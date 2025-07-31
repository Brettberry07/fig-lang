#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character symbols
    LParen, RParen,
    Plus, Minus, Star, Slash,

    Number(i64),
    Float(f64),

    // Special
    EOF,           // End of file
    Illegal(char), // Illegal character
}
