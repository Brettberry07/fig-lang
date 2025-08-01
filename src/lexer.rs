use crate::token::Token;
use crate::helper::Type;

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

    // Like peek, but looks at the next next character instead
    fn peek_next(&self) -> Option<char> {
        self.src.get(self.pos + 1).copied()
    }

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
            Some('=') => Token::Equal,
            Some('"') => self.read_string(),
            Some(';') => Token::Semicolon,
            Some(c) if c.is_ascii_digit() => self.read_number(c),
            None => Token::EOF,
            Some(c) => {
                if c.is_alphanumeric() {
                    let identifier = self.read_identifier();
                    match identifier {
                        Token::Identifier { name, .. } if name == "var" => Token::Var,
                        _ => identifier,
                    }
                } else if c == 't' || c == 'f' {
                    // Boolean literals
                    self.read_bool()
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
        let mut is_float = false;
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
                    is_float = true;
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

    fn read_bool(&mut self) -> Token {
        let mut bool_val = String::new();
        let current = self.src[self.pos - 1]; // Start with the first character
        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                bool_val.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        match bool_val.as_str() {
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Illegal(current), // Illegal boolean value
        }
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

    // fn read_identifier(&mut self) -> Token {
    //     let mut identifier = String::new();
    //     let value: Option<Value>;
    //     // get the name of the var
    //     while let Some(c) = self.peek() {
    //         if c.is_alphanumeric() || c == '_' {
    //             identifier.push(self.advance().unwrap());
    //         } else {
    //             break;
    //         }
    //     }
    //     // get the value of the var
    //     self.skip_whitespace();
    //     if self.peek() == Some('=') {
    //         self.advance(); // consume '='
    //         self.skip_whitespace();
    //         if let Some(c) = self.peek() {
    //             // if the value is a digit it must be int, float, or illegal
    //             if c.is_ascii_digit() {
    //                 let first_digit = self.advance().unwrap();
    //                 let num_token = self.read_number(first_digit);
    //                 match num_token {
    //                 Token::Number(n) => value = Some(Value::Int(n)),
    //                 Token::Float(f) => value = Some(Value::Float(f)),
    //                 _ => return Token::Illegal('='),
    //                 }
    //             // it it's a opening quote, it must be a string or illegal
    //             } else if c == '"' {
    //                 self.advance(); // consume opening quote
    //                 let mut string_val = String::new();
    //                 while let Some(ch) = self.peek() {
    //                 if ch == '"' {
    //                     self.advance(); // consume closing quote
    //                     break;
    //                 } else if ch == '\\' {
    //                     self.advance(); // consume backslash
    //                     if let Some(escaped_char) = self.advance() {
    //                     string_val.push(escaped_char);
    //                     }
    //                 } else {
    //                     string_val.push(self.advance().unwrap());
    //                 }
    //                 }
    //                 value = Some(Value::Str(string_val));
                
    //             // if it's a t or f, it must be a bool or illegal
    //             } else if c == 't' || c == 'f' {
    //                 let mut bool_val = String::new();
    //                 while let Some(ch) = self.peek() {
    //                     if ch.is_alphanumeric() {
    //                         bool_val.push(self.advance().unwrap());
    //                     } else {
    //                         break;
    //                     }
    //                 }
    //                 match bool_val.as_str() {
    //                     "true" => value = Some(Value::Bool(true)),
    //                     "false" => value = Some(Value::Bool(false)),
    //                     _ => return Token::Illegal('='),
    //                 }

    //             } else if c.is_alphabetic() || c == '_' {
    //                 let _ = self.advance();
    //                 let identifier_token = self.read_identifier();
    //                 if let Token::Identifier { value: Some(val), .. } = identifier_token {
    //                 value = Some(val);
    //                 } else {
    //                 return Token::Illegal('='); // Illegal identifier
    //                 }
    //             } else {
    //                 return Token::Illegal(c); // Illegal character after '='
    //             }
    //         } else {
    //         return Token::Illegal('='); // No value after '='
    //         }
    //     } else {
    //         return Token::Identifier { name: identifier, value: Some(Value::Null) };
    //     }
    //     Token::Identifier { name: identifier, value }
    // }
}
