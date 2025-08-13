

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character symbols
    LParen, RParen,
    Plus, Minus, Star, Slash,
    Equal,
    Semicolon,

    // Multi-character symbols
    DblEqual, // Double equal for equality check
    NotEqual, // Not equal check
    LessThan, GreaterThan, LessThanEqual, GreaterThanEqual,

    // types
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),

    // Keywords
    // If,
    // Else,
    // ElIf,
    // While,
    // For,
    // Function, // Function definition
    // Return,

    // built-in functions
    Print,

    // identifiers
    Var,
    Identifier {
        name: String,
    },


    // Special
    EOF,           // End of file
    Illegal(char), // Illegal character
}
