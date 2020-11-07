use std::{iter::Peekable};
use crate::{filter::FilterRule, QueryError};

use super::{ParsedQuery, Token, TokenType, lexer::Lexer};

const SELECT_TOKEN: Token = Token {
    kind: TokenType::Select,
    lexemme: None,
};

const FROM_TOKEN: Token = Token {
    kind: TokenType::From,
    lexemme: None,
};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn match_next(&mut self, token_type: TokenType, err: QueryError) -> Result<Token, QueryError> {
        let next_token = self.lexer.next().ok_or(QueryError::BadSyntax("EOF reached"))?;
        if next_token.kind == token_type {
            Ok(next_token)
        } else {
            Err(err)
        }
    }

    fn peek_next_type(&mut self, token_type: TokenType) -> bool {
        let peek = self.lexer.peek();
        return peek.map(|p| p.kind) == Some(token_type);
    }

    fn parse_select(&mut self) -> Result<Vec<String>, QueryError> {
        self.match_next(TokenType::Select, QueryError::BadSyntax("Missing 'select'"))?;
        let first_col = self.match_next(TokenType::Identifier, QueryError::BadSyntax("Expected at least one column after select"))?;
        let mut cols = Vec::new();
        cols.push(first_col.lexemme.unwrap());

        while self.peek_next_type(TokenType::Comma) {
            self.lexer.next();
            let next_col = self.match_next(TokenType::Select, QueryError::BadSyntax("Expected column identifier after comma in select"))?;
            cols.push(next_col.lexemme.unwrap());
        }
        Ok(cols)
    }

    fn parse_from(&mut self) -> Result<String, QueryError> {
        // todo
    }

    fn parse_filter(&mut self) -> Result<Option<Box<dyn FilterRule>>, QueryError> {
        // todo
    }

    pub fn parse(&mut self) -> Result<ParsedQuery, QueryError> {
        let cols = self.parse_select()?;
        let from = self.parse_from()?;
        let filter = self.parse_filter()?;
        Ok(ParsedQuery {
            cols,
            from,
            filter,
        })
    }

    pub fn new<'b>(input_query: &'b str) -> Parser<'b> {
        Parser {
            lexer: Lexer::new(input_query).peekable(),
        }
    }
}