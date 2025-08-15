use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Var(String), // Variable reference
}

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        name: String,
        value: Expr,
    },
    ExprStmt(Expr),
    PrntStmt(Expr), // Print statement
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Sum,     // + -
    Product, // * /
    DblEqual,
}

pub fn precedence(tok: &Token) -> Precedence {
    match tok {
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Star | Token::Slash => Precedence::Product,
        Token::DblEqual | Token::NotEqual => Precedence::DblEqual,
        Token::LessThan | Token::GreaterThan | Token::LessThanEqual | Token::GreaterThanEqual => Precedence::DblEqual,
        _ => Precedence::Lowest,
    }
}
