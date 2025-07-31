use crate::token::Token;
use crate::helper::Expr;

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n as f64,
        Expr::Float(f) => *f,
        Expr::Binary { left, op, right } => {
            let left_val = eval(left);
            let right_val = eval(right);

            match op {
                Token::Plus => left_val + right_val,
                Token::Minus => left_val - right_val,
                Token::Star => left_val * right_val,
                Token::Slash => left_val / right_val,
                _ => panic!("Unknown operator in expression"),
            }
        }
    }
}
