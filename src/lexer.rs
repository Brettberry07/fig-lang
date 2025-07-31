use crate::token::Token;

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
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(';') => Token::Semicolon,
            Some(':') => Token::Colon,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('-') => {
                if self.peek() == Some('>') {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::DoubleEqual
                } else {
                    Token::Equal
                }
            }
            Some('!') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::NotEqual
                } else {
                    Token::Illegal('!')
                }
            }
            Some('"') => self.read_string(),
            Some('\'') => self.read_char(),
            Some(c) if c.is_ascii_digit() => self.read_number(c),
            Some(c) if Self::is_ident_start(c) => self.read_identifier(c),
            None => Token::EOF,
            Some(c) => Token::Illegal(c),
        }
    }

    // we collect the chars until we find a closing quote.
    fn read_string(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }
            result.push(c);
        }
        Token::StringLiteral(result)
    }

    // we read a single character literal, expecting it to be surrounded by single quotes.
    // If the closing quote is missing, we return an Illegal token.
    fn read_char(&mut self) -> Token {
        let c = self.advance();
        if self.advance() != Some('\'') {
            return Token::Illegal('\'');
        }
        Token::CharLiteral(c.unwrap_or('\0'))
    }

    // we read a number, which can be multiple digits.
    // We continue reading digits until we hit a non-digit character.
    fn read_number(&mut self, first: char) -> Token {
        let mut result = first.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                result.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        Token::Number(result.parse::<i64>().unwrap())
    }

    // we read an identifier, which can start with a letter or underscore,
    // and can contain letters, digits, and underscores.

    fn read_identifier(&mut self, first: char) -> Token {
        let mut ident = first.to_string();
        while let Some(c) = self.peek() {
            if Self::is_ident_char(c) {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        match ident.as_str() {
            "fn" => Token::Fn,
            "var" => Token::Var,
            "type" => Token::Type,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "return" => Token::Return,
            "null" => Token::Null,
            "true" => Token::True,
            "false" => Token::False,
            "int" | "str" | "bool" | "float" => Token::TypeName(ident),
            _ => Token::Identifier(ident),
        }
    }

    // Check if a character can start an identifier (letter or underscore).
    fn is_ident_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    // Check if a character can be part of an identifier (letter, digit, or underscore).
    fn is_ident_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}
