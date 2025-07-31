use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Float(f64),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Sum,     // + -
    Product, // * /
}

pub fn precedence(tok: &Token) -> Precedence {
    match tok {
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Star | Token::Slash => Precedence::Product,
        _ => Precedence::Lowest,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

