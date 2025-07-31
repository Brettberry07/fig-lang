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
            Some(';') => Token::Semicolon,
            Some(c) if c.is_ascii_digit() => self.read_number(c),
            None => Token::EOF,
            Some(c) => {
                if c.is_alphanumeric() && c == 'v' {
                    if self.peek() == Some('a') && self.peek_next() == Some('r') {
                        // This is the start of a variable declaration
                        self.advance(); // consume 'a'
                        self.advance(); // consume 'r'
                        self.skip_whitespace(); // skip any whitespace after 'var'
                        if let Some(c) = self.peek() {
                            if c.is_alphanumeric() || c == '_' {
                                // Read the identifier for the variable
                                let identifier = self.read_identifier();
                                return identifier;
                            } else {
                                // If the next character is not valid for an identifier, it's illegal
                                return Token::Illegal(c);
                            }
                        } else {
                            // If we reached the end of input after 'var', it's illegal
                            return Token::Illegal('v');
                        }
                    } else {
                        // This is just a regular identifier
                        let identifier = self.read_identifier();
                        return identifier;
                    }
                } else {
                    // If not alphanumeric or not 'v', return Illegal token
                    Token::Illegal(c)
                }
            },
        }
    }

    // we read a number, which can be multiple digits.
    // If it has a decimal point, we treat it as a float.
    fn read_number(&mut self, first: char) -> Token {
        let mut is_float = false;
        let mut result = first.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                result.push(self.advance().unwrap());
            } else {
                if c == '.' {
                    if is_float {
                        // If we already saw a dot, this is an illegal number
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
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // consume the closing quote
                return Token::CQuote; // Return a closing quote token
            } else if c == '\\' {
                // Handle escape sequences
                self.advance(); // consume the backslash
                if let Some(escaped_char) = self.advance() {
                    result.push(escaped_char);
                }
            } else {
                result.push(self.advance().unwrap());
            }
        }
        Token::Illegal('"') // If we reach here, it means we didn't find a closing quote
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let mut value: Option<Type> = None;
        // get the name of the var
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        // get the value of the var
        self.skip_whitespace();
        if self.peek() == Some('=') {
            self.advance(); // consume '='
            self.skip_whitespace();
            if let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    let first_digit = self.advance().unwrap();
                    let num_token = self.read_number(first_digit);
                    match num_token {
                        Token::Number(n) => value = Some(Type::Int(n)),
                        Token::Float(f) => value = Some(Type::Float(f)),
                        _ => return Token::Illegal('='),
                    }
                } else if c.is_alphabetic() || c == '_' {
                    let _ = self.advance();
                    let identifier_token = self.read_identifier();
                    if let Token::Identifier { value: Some(val), .. } = identifier_token {
                        value = Some(val);
                    } else {
                        return Token::Illegal('='); // Illegal identifier
                    }
                } else {
                    return Token::Illegal(c); // Illegal character after '='
                }
            } else {
                return Token::Illegal('='); // No value after '='
            }
        }
        Token::Identifier { name: identifier, value }
    }
}
