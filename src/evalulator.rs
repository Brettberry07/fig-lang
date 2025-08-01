use crate::token::Token;
use crate::helper::{Expr, Stmt, Type};
use crate::enviorment::Environment;

/// Evaluate an expression in the given environment.
fn eval_expr(expr: &Expr, env: &Environment) -> Type {
    match expr {
        Expr::Number(n) => Type::Int(*n),
        Expr::Float(f)  => Type::Float(*f),
        Expr::String(s) => Type::Str(s.clone()),
        Expr::Var(name) => env.get(name),
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Token::Plus  => Type::add(l, r),
                Token::Minus => Type::subtract(l, r),
                Token::Star  => Type::multiply(l, r),
                Token::Slash => Type::divide(l, r),
                _ => panic!("Unknown operator {:?}", op),
            }
        }
    }
}

/// Execute a single statement, updating the environment.
/// Returns Some(f64) if it is an expression statement, None for var declarations.
fn eval_stmt(stmt: &Stmt, env: &mut Environment) -> Option<Type> {
    match stmt {
        Stmt::VarDecl { name, value } => {
            let v = eval_expr(value, env);
            env.define(name.clone(), v);
            None
        }
        Stmt::ExprStmt(expr) => Some(eval_expr(expr, env)),
    }
}

/// Run all statements and return the last expression's value.
pub fn eval_program(stmts: &[Stmt]) -> Type {
    let mut env      = Environment::new();
    let mut last_val: Option<Type> = None;

    for stmt in stmts {
        if let Some(v) = eval_stmt(stmt, &mut env) {
            last_val = Some(v);
        }
    }

    last_val.unwrap_or(Type::Null)
}
