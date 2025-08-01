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
pub enum Type {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int(_) | Type::Float(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Type::Str(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Type::Bool(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Type::Null)
    }

    // Addition operator for Type
    pub fn add(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a + b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a + b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 + b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a + b as f64),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }
    
    // subtraction operator for Type
    pub fn subtract(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a - b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a - b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 - b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a - b as f64),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }
    // multiplication operator for Type
    pub fn multiply(self, other: Type) -> Type {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a * b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a * b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 * b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a * b as f64),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }
    // division operator for Type
    pub fn divide(self, other: Type) -> Type {
        if other.is_null() {
            panic!("Division by zero");
        }
        if let Type::Int(0) | Type::Float(0.0) = other {
            panic!("Division by zero");
        }
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a / b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a / b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f64 / b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a / b as f64),
            _ => panic!("Invalid types for addition"),  // Handle other cases as needed
        }
    }
}

