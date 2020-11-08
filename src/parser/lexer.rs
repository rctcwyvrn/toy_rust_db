use std::{str::Chars, iter::Peekable};

use crate::QueryError;

use super::{Token, TokenType};

fn is_whitespace(c: &char) -> bool {
    c.is_whitespace()
}

#[derive(Clone)]
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

// Borrowed a lot of code from micro-mitten and past lexers ive written
impl<'a> Lexer<'a> {
    fn numeric(&mut self, first: char) -> Option<Result<Token, QueryError>> {
        let mut num = String::new();
        num.push(first);
        while self.iter.peek()?.is_numeric() {
            num.push(self.iter.next()?);
        }

        Ok(Token {
            kind: TokenType::Number,
            lexemme: Some(num),
        }).into()
    }

    fn string(&mut self) -> Option<Result<Token, QueryError>> {
        let mut str = String::new();
        loop {
            // Note: Might need to change this in the future, but this should work for now. 
            // Strings capture all whitespace and whatever weirdness the user throws inbetween the quotes
            match self.iter.peek()? {
                c if !(c == &'\"') => {
                    str.push(self.iter.next()?);
                }
                _ => break,
            }
        }
        self.iter.next()?; // the closing "
        Ok(Token {
            kind: TokenType::String,
            lexemme: Some(str),
        }).into()
    }

    fn alpha(&mut self, first: char) -> Option<Result<Token, QueryError>> {
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
            "is" => Token {
                kind: TokenType::Is,
                lexemme: None,
            },
            _ => Token {
                kind: TokenType::Identifier,
                lexemme: Some(data),
            },
        };
        Some(Ok(token))
    }

    pub fn new(query: &'a str) -> Lexer<'a> {
        Lexer {
            iter: query.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, QueryError>;

    fn next(&mut self) -> Option<Self::Item> {
        while is_whitespace(self.iter.peek()?) {
            self.iter.next()?;
        }

        let token = match self.iter.next()? {
            ',' => Token {
                kind: TokenType::Comma,
                lexemme: None,
            }.into(),
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
                }.into()
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
                }.into()
            }
            '=' => {
                let kind = if let Some('=') = self.iter.peek() {
                    self.iter.next();
                    TokenType::EQ
                } else {
                    return Some(Err(QueryError::BadLex("Single '=' is invalid")))
                };
                Token {
                    kind,
                    lexemme: None,
                }.into()
            }
            '\"' => self.string()?,
            c if c.is_numeric() => self.numeric(c)?,
            c if c.is_alphabetic() => self.alpha(c)?,
            _ => Err(QueryError::BadLex("Invalid token, unable to lex"))
        };
        Some(token)
    }
}

impl Into<Result<Token,QueryError>> for Token {
    fn into(self) -> Result<Token,QueryError> {
        Ok(self)
    }
}