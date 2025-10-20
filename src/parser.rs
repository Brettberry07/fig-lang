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

            Token::Fn => {
                self.advance(); // consume 'fn'

                let name = if let Token::Identifier { name } = self.current.clone() {
                    name
                } else {
                    panic!("Expected function name after 'fn', got {:?}", self.current);
                };
                self.advance(); // consume identifier

                if self.current != Token::LParen {
                    panic!("Expected '(' after function name, got {:?}", self.current);
                }
                self.advance(); // consume '('

                let mut params = Vec::new();
                if self.current != Token::RParen {
                    loop {
                        if let Token::Identifier { name } = self.current.clone() {
                            params.push(name);
                        } else {
                            panic!("Expected parameter name, got {:?}", self.current);
                        }
                        self.advance(); // consume parameter

                        if self.current == Token::Comma {
                            self.advance(); // consume ','
                            continue;
                        }
                        break;
                    }
                }

                if self.current != Token::RParen {
                    panic!("Expected ')' after parameters, got {:?}", self.current);
                }
                self.advance(); // consume ')'

                if self.current != Token::LBrace {
                    panic!("Expected '{{' to start function body, got {:?}", self.current);
                }
                self.advance(); // consume '{'

                let body = self.parse_block();

                Stmt::Function {
                    name,
                    params,
                    body: Box::new(body),
                }
            }

            Token::Return => {
                self.advance(); // consume 'return'

                if self.current == Token::Semicolon {
                    self.advance(); // consume ';'
                    Stmt::Return(None)
                } else {
                    let value = self.parse_expression(Precedence::Lowest);
                    if self.current != Token::Semicolon {
                        panic!("Expected ';' after return value, got {:?}", self.current);
                    }
                    self.advance(); // consume ';'
                    Stmt::Return(Some(value))
                }
            }

            Token::Identifier { .. } => {
                if self.next == Token::Equal {
                    let name = if let Token::Identifier { name } = self.current.clone() {
                        name
                    } else {
                        unreachable!();
                    };
                    self.advance(); // consume identifier
                    self.advance(); // consume '='
                    let value = self.parse_expression(Precedence::Lowest);
                    if self.current != Token::Semicolon {
                        panic!("Expected ';' after assignment, got {:?}", self.current);
                    }
                    self.advance(); // consume ';'
                    Stmt::VarDecl { name, value }
                } else {
                    let expr = self.parse_expression(Precedence::Lowest);
                    if self.current != Token::Semicolon {
                        panic!("Expected ';' after expression, got {:?}", self.current);
                    }
                    self.advance(); // consume ';'
                    Stmt::ExprStmt(expr)
                }
            }

            Token::For => {
                self.advance(); // consume 'for'

                // expect identifier
                let var_name = if let Token::Identifier { name } = self.current.clone() {
                    name
                } else {
                    panic!("Expected identifier after 'for', got {:?}", self.current);
                };
                self.advance(); // consume identifier

                // expect 'in'
                if !matches!(self.current, Token::In) {
                    panic!("Expected 'in' after variable name in for loop, got {:?}", self.current);
                }
                self.advance(); // consume 'in'

                // Handle range expression directly
                let range = self.parse_expression(Precedence::Lowest);
                // expect '{'
                if !matches!(self.current, Token::LBrace) {
                    panic!("Expected '{{' after for loop header, got {:?}", self.current);
                }
                self.advance(); // consume '{'

                let body = Box::new(self.parse_block());

                Stmt::ForStmt {
                    var_name,
                    range,
                    body,
                }
            }

            Token::If | Token::Elif => {
                self.advance(); // consume 'if' or 'elif'

                // parse the condition expression
                let condition = self.parse_expression(Precedence::Lowest);

                // expect '{' for then branch
                if self.current != Token::LBrace {
                    panic!("Expected '{{' after condition, got {:?}", self.current);
                }
                self.advance(); // consume '{'

                // parse the then branch (multiple statements)
                let then_block = self.parse_block();
                
                // check for else or elif branch
                let else_branch = if self.current == Token::Else {
                    self.advance(); // consume 'else'
                    if self.current != Token::LBrace {
                        panic!("Expected '{{' after 'else', got {:?}", self.current);
                    }
                    self.advance(); // consume '{'
                    let block = self.parse_block();
                    Some(Box::new(block))
                } else if self.current == Token::Elif {
                    Some(Box::new(self.parse_stmt()))
                } else {
                    None
                };

                Stmt::IfStmt {
                    condition,
                    then_branch: Box::new(then_block),
                    else_branch,
                }
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
            Token::Range => {
                self.advance(); // consume 'range'
                if self.current != Token::LParen {
                    panic!("Expected '(' after 'range', got {:?}", self.current);
                }
                self.advance(); // consume '('
                let arg = self.parse_expression(Precedence::Lowest);
                if self.current != Token::RParen {
                    panic!("Expected ')' after range argument, got {:?}", self.current);
                }
                self.advance(); // consume ')'
                Expr::Binary {
                    left: Box::new(arg),
                    op: Token::Range,
                    right: Box::new(Expr::Number(0)), // Dummy right operand consumed in evaluator
                }
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
            Token::LBrace => {
                self.advance();
                let expr = self.parse_expression(Precedence::Lowest);
                if self.current != Token::RBrace {
                    panic!("Expected closing brace, got {:?}", self.current);
                }
                self.advance();
                expr
            }
            Token::Identifier { name } => {
                let name_clone = name.clone();
                self.advance(); // consume identifier
                if self.current == Token::LParen {
                    let arguments = self.parse_call_arguments();
                    Expr::Call {
                        callee: name_clone,
                        arguments,
                    }
                } else {
                    Expr::Var(name_clone)
                }
            }
            other => panic!("Unexpected token in expression: {:?}", other),
        };

        // precedence loop
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

    fn parse_call_arguments(&mut self) -> Vec<Expr> {
        if self.current != Token::LParen {
            panic!("Expected '(' to start argument list, got {:?}", self.current);
        }
        self.advance(); // consume '('

        let mut arguments = Vec::new();
        if self.current != Token::RParen {
            loop {
                let arg = self.parse_expression(Precedence::Lowest);
                arguments.push(arg);

                if self.current == Token::Comma {
                    self.advance(); // consume ','
                    continue;
                }
                break;
            }
        }

        if self.current != Token::RParen {
            panic!("Expected ')' after arguments, got {:?}", self.current);
        }
        self.advance(); // consume ')'

        arguments
    }

    /// Parse a block of statements until closing brace
    fn parse_block(&mut self) -> Stmt {
        let mut stmts = Vec::new();
        
        while self.current != Token::RBrace && self.current != Token::EOF {
            stmts.push(self.parse_stmt());
        }
        
        if self.current != Token::RBrace {
            panic!("Expected '}}' at end of block, got {:?}", self.current);
        }
        self.advance(); // consume '}'
        
        Stmt::Block(stmts)
    }

    /// Parse a *program* (zero or more statements) until EOF
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current != Token::EOF {
            stmts.push(self.parse_stmt());
        }
        stmts
    }
}
