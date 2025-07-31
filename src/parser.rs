use crate::lexer::Lexer;
use crate::token::Token;
use crate::helper::{Expr, Precedence, precedence};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let next = lexer.next_token();
        Parser { lexer, current, next }
    }

    // advance the parser to the next token
    fn advance(&mut self) {
        self.current = std::mem::replace(&mut self.next, self.lexer.next_token());
    }

    // This function parses an expression based on the current token and precedence level.
    // It supports numbers, parentheses, and binary operations, recursively building an expression tree.
    //
    // The parser uses a technique called "precedence climbing" to respect operator precedence.
    // This means that if it encounters an operator with higher precedence, it will parse the right-hand
    // side expression first before completing the current one.
    //
    // The function returns an `Expr` enum representing the entire expression tree.
    // 
    // Parsing continues until one of two things happens:
    // - The end of the file (EOF) is reached.
    // - A token with lower precedence than the current operation is encountered.
    //   In this case, the function returns early to ensure the higher precedence operation is grouped correctly.
    //
    // Example: For `4 + 2 * 3 - 1`:
    // - It parses `4 +`, sees `*` has higher precedence than `+`, so it recursively parses `2 * 3`.
    // - Once `*` is done, it returns and finishes the `4 + (2 * 3)` part.
    // - It then continues with `- 1`, giving the correct final parse tree.

    pub fn parse_expression(&mut self, prec: Precedence) -> Expr {
        let mut left = match &self.current {
            Token::Number(n) => {
                let expr = Expr::Number(*n);
                self.advance();
                expr
            }
            Token::Float(f) => {
                let expr = Expr::Float(*f);
                self.advance();
                expr
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression(Precedence::Lowest);
                if self.current != Token::RParen {
                    panic!("Expected closing paren");
                }
                self.advance();
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current),
        };

        while self.current != Token::EOF && precedence(&self.current) > prec {
            let op = self.current.clone();
            self.advance();
            let right = self.parse_expression(precedence(&op));
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }
}

impl Parser {
    pub fn parse(&mut self) -> Expr {
        self.parse_expression(Precedence::Lowest)
    }
}

