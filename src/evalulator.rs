use crate::token::Token;
use crate::helper::{Expr, Stmt};
use crate::enviorment::Environment;

/// Evaluate an expression in the given environment.
fn eval_expr(expr: &Expr, env: &Environment) -> f64 {
    match expr {
        Expr::Number(n) => *n as f64,
        Expr::Float(f)  => *f,
        Expr::Var(name) => env.get(name),
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Token::Plus  => l + r,
                Token::Minus => l - r,
                Token::Star  => l * r,
                Token::Slash => {
                    if r == 0.0 { panic!("Division by zero"); }
                    l / r
                }
                _ => panic!("Unknown operator {:?}", op),
            }
        }
    }
}

/// Execute a single statement, updating the environment.
/// Returns Some(f64) if it is an expression statement, None for var declarations.
fn eval_stmt(stmt: &Stmt, env: &mut Environment) -> Option<f64> {
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
pub fn eval_program(stmts: &[Stmt]) -> f64 {
    let mut env      = Environment::new();
    let mut last_val = 0.0;

    for stmt in stmts {
        if let Some(v) = eval_stmt(stmt, &mut env) {
            last_val = v;
        }
    }

    last_val
}
