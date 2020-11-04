use std::{str::Chars, iter::Peekable};

use crate::QueryError;

fn is_whitespace(c: &char) -> bool {
    c.is_whitespace()
}

#[derive(Clone)]
struct Lexer<'a> {
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
            _ => Token {
                kind: TokenType::Identifier,
                lexemme: Some(data),
            },
        };
        Some(token)
    }

    fn new(query: &'a str) -> Lexer<'a> {
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

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenType,
    lexemme: Option<String>,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Select,
    Identifier,
    Number,
    Comma,
    Invalid,
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

#[derive(Debug)]
pub struct ParsedQuery{
    cols: Vec<Token>,
}

impl<'a> Parser<'a> {
    fn match_next(&mut self, token: Token, err: QueryError) -> Result<Token, QueryError>{
        let next_token = self.lexer.next().ok_or(QueryError::BadSyntax("EOF reached"))?;
        //println!("next: {:?}", next_token);
        if next_token != token {
            Err(err)
        } else {
            Ok(next_token)
        }
    }

    fn match_next_type(&mut self, token_type: TokenType, err: QueryError) -> Result<Token, QueryError> {
        let next_token = self.lexer.next().ok_or(QueryError::BadSyntax("EOF reached"))?;
        if next_token.kind == token_type {
            Ok(next_token)
        } else {
            Err(err)
        }
    }

    pub fn parse_select(&mut self) -> Result<Vec<Token>, QueryError> {
        let select_token = Token {
            kind: TokenType::Select,
            lexemme: None,
        };

        self.match_next(select_token, QueryError::BadSyntax("Missing 'select'"))?;
        let first_col = self.match_next_type(TokenType::Identifier, QueryError::BadSyntax("Expected columns after select"))?;
        let mut cols = Vec::new();
        cols.push(first_col);

        while self.lexer.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
            self.lexer.next();
            let next_col = self.match_next_type(TokenType::Identifier, QueryError::BadSyntax("Expected columns after select"))?;
            cols.push(next_col);
        }
        Ok(cols)
    }

    pub fn parse(&mut self) -> Result<ParsedQuery, QueryError> {
        let cols = self.parse_select()?;
        Ok(ParsedQuery {
            cols,
        })
    }

    pub fn new<'b>(input_query: &'b str) -> Parser<'b> {
        Parser {
            lexer: Lexer::new(input_query).peekable(),
        }
    }
}