use crate::token::Token;
// use crate::helper::Type;

pub struct Lexer {
    src: Vec<char>, // Source code as a vector of characters
    pos: usize,     // Current position in the source code
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            src: input.chars().collect(),
            pos: 0,
        }
    }

    // Peek the next character without consuming it.
    // Returns None if at the end of input.
    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    // // Like peek, but looks at the next next character instead
    // fn peek_next(&self) -> Option<char> {
    //     self.src.get(self.pos + 1).copied()
    // }

    // Advance to the next character and return it.
    // Returns None if at the end of input.
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    // Skip whitespace characters.
    // Advances the position until a non-whitespace character is found.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Get the next token from the input.
    // then we find out what kind of token that is, and return it.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ch = self.advance();
        // println!("Lexer: Read character: {:?}", ch);

        match ch {
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('+') => Token::Plus,
            Some('-') => {
                match self.peek() {
                    Some(c) if c.is_ascii_digit() => {
                        // Negative number: parse as number with '-' prefix
                        let next_digit = self.advance().unwrap();
                        let num_token = self.read_number(next_digit);
                        match num_token {
                            Token::Number(n) => Token::Number(-n),
                            Token::Float(f) => Token::Float(-f),
                            other => other,
                        }
                    }
                    _ => Token::Minus,
                }
            }
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance(); // consume the second '='
                    Token::DblEqual
                } else {
                    Token::Equal
                }
            },
            Some('!') => {
                if self.peek() == Some('=') {
                    self.advance(); // consume the '='
                    Token::NotEqual
                } else {
                    Token::Illegal('!')
                }
            },
            Some('<') => {
                if self.peek() == Some('=') {
                    self.advance(); // consume the '='
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            },
            Some('>') => {
                if self.peek() == Some('=') {
                    self.advance(); // consume the '='
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            },
            Some('"') => self.read_string(),
            Some(';') => Token::Semicolon,
            Some(c) if c.is_ascii_digit() => self.read_number(c),
            None => Token::EOF,
            Some(c) => {
                if c.is_alphanumeric() {
                    let identifier = self.read_identifier();
                    match identifier {
                        Token::Identifier { name, .. } if name == "var" => Token::Var,
                        Token::Identifier { name, .. } if name == "true" => Token::Bool(true),
                        Token::Identifier { name, .. } if name == "false" => Token::Bool(false),
                        Token::Identifier { name, .. } if name == "print" => Token::Print,
                        _ => identifier,
                    }
                } else {
                    // If we reach here, it's an illegal character
                    Token::Illegal(c)
                }
            },
        }
    }

    // we read a number, which can be multiple digits.
    // If it has a decimal point, we treat it as a float.
    fn read_number(&mut self, first: char) -> Token {
        let mut result = first.to_string();
        let is_float = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                result.push(self.advance().unwrap());
            } else {
                if c == '.' {
                    if is_float {
                        // If we already saw a dot, this is an illegal number
                        println!("Illegal number: multiple decimal points");
                        return Token::Illegal(c);
                    }
                    result.push(self.advance().unwrap());
                    while let Some(d) = self.peek() {
                        if d.is_ascii_digit() {
                            result.push(self.advance().unwrap());
                        } else {
                            if d.is_whitespace() || d == ';' {
                                // If we hit whitespace, we stop reading the number
                                // if we hit a semicolon, we also stop reading the number
                                break;
                            } else {
                                // If we hit a non-digit, non-whitespace character, it's illegal
                                return Token::Illegal(d);
                            }
                        }
                    }
                    return Token::Float(result.parse::<f64>().unwrap());
                } else {
                    break;
                }
            }
        }
        if is_float {
            Token::Float(result.parse::<f64>().unwrap())
        } else {
            Token::Number(result.parse::<i64>().unwrap())
        }
    }

    fn read_string(&mut self) -> Token {
        let mut string_val = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // consume closing quote
                return Token::String(string_val);
            } else if c == '\\' {
                self.advance(); // consume backslash
                if let Some(escaped_char) = self.advance() {
                    string_val.push(escaped_char);
                }
            } else {
                string_val.push(self.advance().unwrap());
            }
        }
        // If we reach here, it means the string was not properly closed
        Token::Illegal('"')
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        identifier.push(self.src[self.pos - 1]); // Start with the first character
        // get the name of the var
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        return Token::Identifier {
            name: identifier,
        };
    }
}
