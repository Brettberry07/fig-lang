use crate::token::Token;
use crate::helper::{Expr, Stmt};
use crate::types::Type;
use crate::enviorment::Environment;

/// Evaluate an expression in the given environment.
fn eval_expr(expr: &Expr, env: &Environment) -> Type {
    // println!("Evaluating expression: {:?}", expr);
    match expr {
        Expr::Number(n) => Type::Int(*n),
        Expr::Float(f)  => Type::Float(*f),
        Expr::String(s) => Type::Str(s.clone()),
        Expr::Bool(b) => Type::Bool(*b),
        Expr::Var(name) => env.get(name),
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Token::Plus  => Type::add(l, r),
                Token::Minus => Type::subtract(l, r),
                Token::Star  => Type::multiply(l, r),
                Token::Slash => Type::divide(l, r),
                Token::DblEqual => Type::equal(l, r),
                Token::NotEqual => Type::not_equal(l, r),
                Token::LessThan => Type::less_than(l, r),
                Token::GreaterThan => Type::greater_than(l, r),
                Token::LessThanEqual => Type::less_than_equal(l, r),
                Token::GreaterThanEqual => Type::greater_than_equal(l, r),
                _ => panic!("Unknown operator {:?}", op),
            }
        }
    }
}

// Execute a single statement, updating the environment.
// Returns Some(f64) if it is an expression statement, None for var declarations.
fn eval_stmt(stmt: &Stmt, env: &mut Environment) -> Option<Type> {
    match stmt {
        Stmt::VarDecl { name, value } => {
            let v = eval_expr(value, env);
            if env.is_defined(name) {
                env.update(name.clone(), v.clone());
            } else {
                env.define(name.clone(), v);
            }
            None
        }
        Stmt::ExprStmt(expr) => Some(eval_expr(expr, env)),
        Stmt::PrntStmt(expr) => {
            let value = eval_expr(expr, env);
            println!("{}", value); // Print the value
            None
        }
        Stmt::IfStmt { condition, then_branch, else_branch } => {
            let cond_value = eval_expr(condition, env);
            match cond_value {
                Type::Bool(true) => {
                    eval_stmt(then_branch, env);
                }
                Type::Bool(false) => {
                    if let Some(else_stmt) = else_branch {
                        eval_stmt(else_stmt, env);
                    }
                }
                _ => panic!("Condition must be a boolean, got {:?}", cond_value),
            }
            // if cond_value.is_truthy() {
            //     eval_stmt(then_branch, env);
            // } else if let Some(else_stmt) = else_branch {
            //     eval_stmt(else_stmt, env);
            // }
            None
        }
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
