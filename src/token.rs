use crate::helper::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character symbols
    LParen, RParen,
    OQuote, CQuote,
    Plus, Minus, Star, Slash,
    Equal,
    Semicolon,

    Number(i64),
    Float(f64),

    // Keywords and identifiers
    Identifier {
        name: String,
        value: Option<Type>,
    },


    // Special
    EOF,           // End of file
    Illegal(char), // Illegal character
}
