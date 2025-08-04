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
    String(String),
    Bool(bool),
    Var, // Variable declaration keyword

    // built-in functions
    Print,

    // Keywords and identifiers
    Identifier {
        name: String,
    },


    // Special
    EOF,           // End of file
    Illegal(char), // Illegal character
}
