use std::{iter::Peekable};
use crate::{QueryError, filter::{FilterRule, LogicalFilter, LogicalOp, NumberFilter, NumberOp, StringFilter}};

use super::{ParsedQuery, Token, TokenType, lexer::Lexer};

const STRANGE_MISSING_LEXEMME_ERR: QueryError = QueryError::BadSyntax("?? How did this token not have a lexemme?? This should never happen!");
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

// Public interface
impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<ParsedQuery, QueryError> {
        let cols = self.parse_select()?;
        let from = self.parse_from()?;
        let filter = self.parse_where()?;
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

// Query parsing
impl<'a> Parser<'a> {
    fn get_next(&mut self) -> Result<Token, QueryError> {
        self.lexer.next().ok_or(QueryError::BadSyntax("EOF reached"))
    }

    /// Matches the next token with the given token type
    /// 
    /// Returns Err(QueryError::BadSyntax("EOF reached")) if there is no next token
    /// 
    /// Return Err(QueryError::BadSyntax(err)) if the token type doesn't match
    /// 
    /// Returns Ok(token) otherwise
    fn match_next(&mut self, token_type: TokenType, err: &'static str) -> Result<Token, QueryError> {
        let next_token = self.get_next()?;
        if next_token.kind == token_type {
            Ok(next_token)
        } else {
            Err(QueryError::BadSyntax(err))
        }
    }

    /// Peeks the next token, returns true if there is a next token and it is the correct type
    fn peek_next_type(&mut self, token_type: TokenType) -> bool {
        let peek = self.lexer.peek();
        return peek.map(|p| p.kind) == Some(token_type);
    }

    fn parse_select(&mut self) -> Result<Vec<String>, QueryError> {
        self.match_next(TokenType::Select, "Missing 'select'")?;
        let first_col = self.match_next(TokenType::Identifier, "Expected at least one column after select")?;
        let mut cols = Vec::new();
        cols.push(first_col.lexemme.unwrap());

        while self.peek_next_type(TokenType::Comma) {
            self.lexer.next();
            let next_col = self.match_next(TokenType::Select, "Expected column identifier after comma in select")?;
            cols.push(next_col.lexemme.unwrap());
        }
        Ok(cols)
    }

    fn parse_from(&mut self) -> Result<String, QueryError> {
        self.match_next(TokenType::From, "Missing 'from'")?;
        let from = self.match_next(TokenType::Identifier, "Expected dataset name after from")?;
        let db_name = from.lexemme.ok_or(STRANGE_MISSING_LEXEMME_ERR)?;
        Ok(db_name)
    }

    fn parse_where(&mut self) -> Result<Option<Box<dyn FilterRule>>, QueryError> {
        if !self.peek_next_type(TokenType::Where) {
            Ok(None)
        } else {
            self.lexer.next(); // where token
            let filter = self.parse_filter()?;
            Ok(Some(filter))
        }
    }
}

// Filter parsing
impl<'a> Parser<'a> {
    fn parse_filter(&mut self) -> Result<Box<dyn FilterRule>, QueryError> {
        let col_token = self.match_next(TokenType::Identifier, "Expected col name as first token in filter")?;
        let col = col_token.lexemme.ok_or(STRANGE_MISSING_LEXEMME_ERR)?;

        let filter_kind = self.get_next()?;
        let filter_val = self.get_next()?;

        let filter_res: Result<Box<dyn FilterRule>, QueryError> = match filter_val.kind {
            TokenType::Number => {
                let num_lexemme = filter_val.lexemme.ok_or(STRANGE_MISSING_LEXEMME_ERR)?;
                let num = num_lexemme.parse::<usize>().map_err( |num_err| QueryError::NumParseError(num_err.to_string()))?;
                let num_filter = NumberFilter {
                    col,
                    op: Parser::map_num_op(filter_kind.kind)?,
                    val: num,
                };
                Ok(Box::new(num_filter))
            }
            TokenType::Identifier => {
                let val = filter_val.lexemme.ok_or(STRANGE_MISSING_LEXEMME_ERR)?;
                let str_filter = StringFilter {
                    col,
                    val,
                };
                Ok(Box::new(str_filter))
            }
            _ => Err(QueryError::BadSyntax("Invalid token type for a filter value, must be a string or a number and not a keyword"))
        };
        let filter = filter_res?;

        // Check if this is the first filter of a logical op
        // Note: The parsed filter structure of `f1 AND f2 AND f3 AND f4 ...` will be (f1, (f2, (f3, (...))))
        // Allows for short circuiting on the left filter before recursing in to the right filter
        let is_logical = self.peek_next_type(TokenType::And) || self.peek_next_type(TokenType::Or);
        if is_logical {
            let logical_kind = self.get_next()?;
            let f2 = self.parse_filter()?;
            let logical_filter = LogicalFilter {
                f1: filter,
                f2,
                op: Parser::map_logic_op(logical_kind.kind)?,
            };
            Ok(Box::new(logical_filter))
        } else {
            // If no logical op, then just return the filter as is
            Ok(filter)
        }
    }

    fn map_num_op(kind: TokenType) -> Result<NumberOp, QueryError> {
        match kind {
            TokenType::LT => Ok(NumberOp::LT),
            TokenType::GT => Ok(NumberOp::GT),
            TokenType::LEQ => Ok(NumberOp::LEQ),
            TokenType::GEQ => Ok(NumberOp::GEQ),
            TokenType::EQ => Ok(NumberOp::EQ),
            _ => Err(QueryError::BadSyntax("Invalid operator for number comparisons"))
        }
    }

    fn map_logic_op(kind: TokenType) -> Result<LogicalOp, QueryError> {
        match kind {
            TokenType::And => Ok(LogicalOp::And),
            TokenType::Or => Ok(LogicalOp::Or),
            _ => Err(QueryError::BadSyntax("?? How did I get an invalid logical op type, didn't I just check this just before?"))
        }
    }
}