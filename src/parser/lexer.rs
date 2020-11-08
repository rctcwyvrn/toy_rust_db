use std::{str::Chars, iter::Peekable};

use super::{Token, TokenType};

fn is_whitespace(c: &char) -> bool {
    c.is_whitespace()
}

#[derive(Clone)]
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn numeric(&mut self, first: char) -> Option<Token> {
        let mut num = String::new();
        num.push(first);
        while self.iter.peek()?.is_numeric() {
            num.push(self.iter.next()?);
        }

        Some(Token {
            kind: TokenType::Number,
            lexemme: Some(num),
        })
    }

    fn alpha(&mut self, first: char) -> Option<Token> {
        let mut data = String::new();
        data.push(first);
        loop {
            match self.iter.peek()? {
                c if c.is_alphanumeric() || c == &'_' || c == &'.' || c == &'*' => {
                    data.push(self.iter.next()?);
                }
                _ => break,
            }
        }
        let token = match data.as_str() {
            "select" => Token {
                kind: TokenType::Select,
                lexemme: None,
            },
            "where" => Token {
                kind: TokenType::Where,
                lexemme: None,
            },
            "from" => Token {
                kind: TokenType::From,
                lexemme: None,
            },
            "and" => Token {
                kind: TokenType::And,
                lexemme: None,
            },
            "or" => Token {
                kind: TokenType::Or,
                lexemme: None,
            },
            _ => Token {
                kind: TokenType::Identifier,
                lexemme: Some(data),
            },
        };
        Some(token)
    }

    pub fn new(query: &'a str) -> Lexer<'a> {
        Lexer {
            iter: query.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while is_whitespace(self.iter.peek()?) {
            self.iter.next()?;
        }

        let token = match self.iter.next()? {
            ',' => Token {
                kind: TokenType::Comma,
                lexemme: None,
            },
            '<' => {
                let kind = if let Some('=') = self.iter.peek() {
                    self.iter.next();
                    TokenType::LEQ
                } else {
                    TokenType::LT
                };
                Token {
                    kind,
                    lexemme: None,
                }
            }
            '>' => {
                let kind = if let Some('=') = self.iter.peek() {
                    self.iter.next();
                    TokenType::GEQ
                } else {
                    TokenType::GT
                };
                Token {
                    kind,
                    lexemme: None,
                }
            }
            '=' => {
                let kind = if let Some('=') = self.iter.peek() {
                    self.iter.next();
                    TokenType::EQ
                } else {
                    TokenType::Invalid // Single = is invalid
                };
                Token {
                    kind,
                    lexemme: None,
                }
            }
            c if c.is_numeric() => self.numeric(c)?,
            c if c.is_alphabetic() => self.alpha(c)?,
            _ => Token {
                kind: TokenType::Invalid,
                lexemme: None,
            }
        };
        Some(token)
    }
}