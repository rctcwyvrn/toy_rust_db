pub mod parser;
mod lexer;

use std::fmt::{Debug, Display};

use crate::filter::FilterRule;

#[derive(Debug)]
pub struct ParsedQuery{
    cols: Vec<String>,
    from: String,
    filter: Option<Box<dyn FilterRule>>,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenType,
    lexemme: Option<String>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TokenType {
    // Keywords
    Select,
    From,
    Where,

    Identifier,
    Number,
    Comma,

    // Operators

    // num
    LT,
    LEQ,
    GT,
    GEQ,
    EQ,
    
    // string
    Is,

    // logical
    And,
    Or,

    // Errors
    Invalid,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}