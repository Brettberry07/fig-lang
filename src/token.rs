#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character symbols
    LParen, RParen,
    LBrace, RBrace,
    Colon, Semicolon, Comma,
    Plus, Minus, Star, Slash,
    Equal, // =
    
    // Multi-char operators
    DoubleEqual, NotEqual,
    Greater, Less,
    GreaterEqual, LessEqual,
    Arrow, // ->

    // Keywords
    Fn, Var, Type, If, Else, For, While, Return, Null, True, False,

    // Literals
    Identifier(String),
    TypeName(String),
    Number(i64),
    StringLiteral(String),
    CharLiteral(char),

    // Special
    EOF,           // End of file
    Illegal(char), // Illegal character
}
