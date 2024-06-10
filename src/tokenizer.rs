use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Mut,
    TypeSpecifier(String),
    NumberLiteral(String),
    Equals,
    Semicolon,
    Symbol(String),
    LParen,
    RParen,
    Minus,
    Plus,
    Star,
    FSlash,
}

#[derive(Debug)]
pub struct InvalidTokenError;

impl FromStr for Token {
    type Err = InvalidTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        match s.as_str() {
            "mut" => Ok(Self::Mut),
            "=" => Ok(Self::Equals),
            ";" => Ok(Self::Semicolon),
            "i32" => Ok(Self::TypeSpecifier(s)),
            "(" => Ok(Self::LParen),
            ")" => Ok(Self::RParen),
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Star),
            "/" => Ok(Self::FSlash),
            _ => {
                if let Ok(_) = s.parse::<i32>() {
                    Ok(Self::NumberLiteral(s))
                } else {
                    Ok(Self::Symbol(s))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    source: String,
    pos: usize,
    current_char: Option<char>,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        let mut tokenizer = Self {
            source,
            pos: 0,
            current_char: None,
        };
        if !tokenizer.source.is_empty() {
            tokenizer.current_char = Some(tokenizer.source.as_bytes()[0] as char);
        }

        return tokenizer;
    }

    fn advance(&mut self) {
        self.pos += 1;

        if self.pos > self.source.len() - 1 {
            self.current_char = None; // EOF
        } else {
            self.current_char = Some(self.source.as_bytes()[self.pos] as char);
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                return;
            }
        }
    }

    fn number_literal(&mut self) -> String {
        let mut buf = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                buf.push(c);
                self.advance();
            } else {
                return buf;
            }
        }
        buf
    }

    fn symbol(&mut self) -> String {
        match self.current_char {
            Some('+') => {
                self.advance();
                "+".to_string()
            }
            Some('-') => {
                self.advance();
                "-".to_string()
            }
            Some('=') => {
                self.advance();
                '='.to_string()
            }
            Some(';') => {
                self.advance();
                ';'.to_string()
            }
            Some('(') => {
                self.advance();
                '('.to_string()
            }
            Some(')') => {
                self.advance();
                ')'.to_string()
            }
            Some('*') => {
                self.advance();
                '*'.to_string()
            }
            Some('/') => {
                self.advance();
                '/'.to_string()
            }
            _ => {
                let mut buffer = String::new();
                while let Some(c) = self.current_char {
                    if c.is_whitespace() || c == ';' {
                        break;
                    }
                    buffer.push(c);
                    self.advance();
                }
                buffer
            }
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if c.is_digit(10) {
                return Some(Token::NumberLiteral(self.number_literal()));
            }

            let symbol = self.symbol();
            if let Ok(token) = Token::from_str(&symbol) {
                return Some(token);
            }

            return None;
        }

        None
    }
}
