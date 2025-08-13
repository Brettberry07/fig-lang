use crate::lexer::Lexer;
use crate::token::Token;
use crate::helper::{Expr, Stmt, Precedence, precedence};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let next    = lexer.next_token();
        Parser { lexer, current, next }
    }

    fn advance(&mut self) {
        self.current = std::mem::replace(&mut self.next, self.lexer.next_token());
    }


    /// Parses a single statement (either `var x = …;` or an expression-stmt like `x + 2;`)
    pub fn parse_stmt(&mut self) -> Stmt {
        // println!("Parsing token: {:?}", self.current);
        match self.current.clone() {
            Token::Var => {
                // var-declaration
                self.advance(); // consume 'var'
                // println!("Should be identifier now: {:?}", self.current);


                // expect identifier
                let name = if let Token::Identifier { name } = self.current.clone() {
                    name
                } else {
                    panic!("Expected identifier after 'var', got {:?}", self.current);
                };
                self.advance(); // consume the identifier
                // println!("Should be = now: {:?}", self.current);

                // expect '='
                if self.current != Token::Equal {
                    panic!("Expected '=' after variable name, got {:?}", self.current);
                }
                self.advance(); // consume '='

                // parse the initializer expression
                let value = self.parse_expression(Precedence::Lowest);
                // println!("The value {:?}", value);

                // expect semicolon
                if self.current != Token::Semicolon {
                    panic!("Expected ';' after var declaration, got {:?}", self.current);
                }
                self.advance(); // consume ';'

                Stmt::VarDecl { name, value }
            }

            Token::Print => {
                // print statement
                self.advance(); // consume 'print'

                // parse the expression to print
                let expr = self.parse_expression(Precedence::Lowest);

                // expect semicolon
                if self.current != Token::Semicolon {
                    panic!("Expected ';' after print expression, got {:?}", self.current);
                }
                self.advance(); // consume ';'

                Stmt::PrntStmt(expr)
            }

            Token::Identifier { name } => {
                self.advance(); // consume identifier
                // println!("Should be = now: {:?}", self.current);

                // expect '='
                if self.current != Token::Equal {
                    panic!("Expected '=' after variable name, got {:?}", self.current);
                }
                self.advance(); // consume '='

                // parse the initializer expression
                let value = self.parse_expression(Precedence::Lowest);
                // println!("The value {:?}", value);

                // expect semicolon
                if self.current != Token::Semicolon {
                    panic!("Expected ';' after var declaration, got {:?}", self.current);
                }
                self.advance(); // consume ';'
                
                // println!("Parsed variable declaration: {} = {:?}", name, value);
                Stmt::VarDecl { name: name.to_string(), value }
            }

            _ => {
                // expression statement
                let expr = self.parse_expression(Precedence::Lowest);

                // expect semicolon
                if self.current != Token::Semicolon {
                    panic!("Expected ';' after expression, got {:?}", self.current);
                }
                self.advance(); // consume ';'
                Stmt::ExprStmt(expr)
            }
        }
    }

    /// Parses an expression with precedence climbing
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
            Token::String(s) => {
                let expr = Expr::String(s.clone());
                self.advance();
                expr
            }
            Token::Bool(b) => {
                let expr = Expr::Bool(*b);
                self.advance();
                expr
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression(Precedence::Lowest);
                // println!("expr: {:?}", expr);
                if self.current != Token::RParen {
                    panic!("Expected closing parenthesis, got {:?}", self.current);
                }
                self.advance();
                expr
            }
            Token::Identifier { name } => {
                let expr = Expr::Var(name.clone());
                self.advance();
                expr
            }
            other => panic!("Unexpected token in expression: {:?}", other),
        };

        // precedence loop
        while self.current != Token::EOF && precedence(&self.current) > prec {
            let op = self.current.clone();
            // println!("Current operator: {:?}", op);
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

    /// Parse a *program* (zero or more statements) until EOF
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current != Token::EOF {
            stmts.push(self.parse_stmt());
            println!("total stmts {:?}", stmts);
        }
        println!("Finished parsing, stmts: {:?}", stmts);
        stmts
    }
}
