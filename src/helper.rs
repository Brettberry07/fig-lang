use crate::token::Token;

#[derive(Debug, Clone)]
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
    Call {
        callee: String,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, value: Expr },
    ExprStmt(Expr),
    PrntStmt(Expr),
    Block(Vec<Stmt>),
    IfStmt { condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    ForStmt { var_name: String, range: Expr, body: Box<Stmt> },
    Function { name: String, params: Vec<String>, body: Box<Stmt> },
    Return(Option<Expr>),
}

#[derive(Debug)]
pub enum BuiltInFunction {
    Range(i64),
}#[derive(PartialEq, PartialOrd)]
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
        Token::Range => Precedence::Lowest,
        _ => Precedence::Lowest,
    }
}
