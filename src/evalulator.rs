use crate::token::Token;
use crate::helper::{Expr, Stmt};
use crate::types::Type;
use crate::enviorment::{Environment, Function};
use std::cell::RefCell;
use std::rc::Rc;

enum ExecResult {
    Value(Option<Type>),
    Return(Type),
}

/// Evaluate an expression in the given environment.
fn eval_expr(expr: &Expr, env: &Rc<RefCell<Environment>>) -> Type {
    match expr {
        Expr::Number(n) => Type::Int(*n),
        Expr::Float(f)  => Type::Float(*f),
        Expr::String(s) => Type::Str(s.clone()),
        Expr::Bool(b) => Type::Bool(*b),
        Expr::Var(name) => env.borrow().get(name),
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Token::Plus  => Type::add(l, r),
                Token::Minus => Type::subtract(l, r),
                Token::Star  => Type::multiply(l, r),
                Token::Slash => Type::divide(l, r),
                Token::DblEqual => Type::equal(l, r),
                Token::Range => {
                    match l {
                        Type::Int(n) if n >= 0 => Type::Range(n),
                        Type::Int(_) => panic!("Range argument must be non-negative"),
                        _ => panic!("Range argument must be an integer"),
                    }
                }
                Token::NotEqual => Type::not_equal(l, r),
                Token::LessThan => Type::less_than(l, r),
                Token::GreaterThan => Type::greater_than(l, r),
                Token::LessThanEqual => Type::less_than_equal(l, r),
                Token::GreaterThanEqual => Type::greater_than_equal(l, r),
                _ => panic!("Unknown operator {:?}", op),
            }
        }
        Expr::Call { callee, arguments } => {
            let function = env
                .borrow()
                .get_function(callee)
                .unwrap_or_else(|| panic!("Undefined function: {}", callee));

            if arguments.len() != function.params.len() {
                panic!(
                    "Function '{}' expected {} arguments, got {}",
                    callee,
                    function.params.len(),
                    arguments.len()
                );
            }

            let arg_values: Vec<Type> = arguments
                .iter()
                .map(|arg| eval_expr(arg, env))
                .collect();

            let call_env = Rc::new(RefCell::new(Environment::new_scope(Rc::clone(&function.closure))));
            for (param, value) in function.params.iter().cloned().zip(arg_values.into_iter()) {
                call_env.borrow_mut().define(param, value);
            }

            match eval_stmt(&function.body, Rc::clone(&call_env)) {
                ExecResult::Return(val) => val,
                ExecResult::Value(Some(val)) => val,
                ExecResult::Value(None) => Type::Null,
            }
        }
    }
}

// Execute a single statement, updating the environment.
fn eval_stmt(stmt: &Stmt, env: Rc<RefCell<Environment>>) -> ExecResult {
    match stmt {
        Stmt::ForStmt { var_name, range, body } => {
            let range_value = eval_expr(range, &env);
            match range_value {
                Type::Range(n) => {
                    let mut last_val: Option<Type> = None;
                    for i in 0..n {
                        let iter_env = Rc::new(RefCell::new(Environment::new_scope(Rc::clone(&env))));
                        iter_env.borrow_mut().define(var_name.clone(), Type::Int(i));
                        match eval_stmt(body.as_ref(), Rc::clone(&iter_env)) {
                            ExecResult::Return(val) => return ExecResult::Return(val),
                            ExecResult::Value(value) => {
                                if let Some(v) = value {
                                    last_val = Some(v);
                                }
                            }
                        }
                    }
                    ExecResult::Value(last_val)
                }
                _ => panic!("Expected range value in for loop"),
            }
        }
        Stmt::VarDecl { name, value } => {
            let v = eval_expr(value, &env);
            let defined = env.borrow().is_defined(name);
            if defined {
                env.borrow_mut().update(name.clone(), v.clone());
            } else {
                env.borrow_mut().define(name.clone(), v);
            }
            ExecResult::Value(None)
        }
        Stmt::Block(stmts) => {
            let mut last_val: Option<Type> = None;
            let block_env = Rc::new(RefCell::new(Environment::new_scope(Rc::clone(&env))));
            for stmt in stmts {
                match eval_stmt(stmt, Rc::clone(&block_env)) {
                    ExecResult::Return(val) => return ExecResult::Return(val),
                    ExecResult::Value(value) => {
                        if let Some(v) = value {
                            last_val = Some(v);
                        }
                    }
                }
            }
            ExecResult::Value(last_val)
        }
        Stmt::ExprStmt(expr) => ExecResult::Value(Some(eval_expr(expr, &env))),
        Stmt::PrntStmt(expr) => {
            let value = eval_expr(expr, &env);
            println!("{}", value);
            ExecResult::Value(None)
        }
        Stmt::IfStmt { condition, then_branch, else_branch } => {
            let cond_value = eval_expr(condition, &env);
            match cond_value {
                Type::Bool(true) => {
                    let then_env = Rc::new(RefCell::new(Environment::new_scope(Rc::clone(&env))));
                    match eval_stmt(then_branch.as_ref(), then_env) {
                        ExecResult::Return(val) => return ExecResult::Return(val),
                        ExecResult::Value(value) => return ExecResult::Value(value),
                    }
                }
                Type::Bool(false) => {
                    if let Some(else_stmt) = else_branch {
                        let else_env = Rc::new(RefCell::new(Environment::new_scope(Rc::clone(&env))));
                        match eval_stmt(else_stmt.as_ref(), else_env) {
                            ExecResult::Return(val) => return ExecResult::Return(val),
                            ExecResult::Value(value) => return ExecResult::Value(value),
                        }
                    }
                    ExecResult::Value(None)
                }
                _ => panic!("Condition must be a boolean, got {:?}", cond_value),
            }
        }
        Stmt::Function { name, params, body } => {
            let function = Function {
                params: params.clone(),
                body: (*body.clone()),
                closure: Rc::clone(&env),
            };
            env.borrow_mut().define_function(name.clone(), function);
            ExecResult::Value(None)
        }
        Stmt::Return(expr) => {
            let value = expr
                .as_ref()
                .map(|e| eval_expr(e, &env))
                .unwrap_or(Type::Null);
            ExecResult::Return(value)
        }
    }
}

/// Run all statements and return the last expression's value.
pub fn eval_program(stmts: &[Stmt]) -> Type {
    let env = Rc::new(RefCell::new(Environment::new()));
    let mut last_val: Option<Type> = None;

    for stmt in stmts {
        match eval_stmt(stmt, Rc::clone(&env)) {
            ExecResult::Return(val) => return val,
            ExecResult::Value(value) => {
                if let Some(v) = value {
                    last_val = Some(v);
                }
            }
        }
    }

    last_val.unwrap_or(Type::Null)
}
