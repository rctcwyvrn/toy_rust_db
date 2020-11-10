mod lexer;
pub mod parser;

use std::fmt::{Debug, Display};

use crate::filter::FilterRule;

#[derive(Debug)]
pub struct ParsedQuery {
    pub cols: Vec<usize>,
    pub from: String,
    pub filter: Option<Box<dyn FilterRule>>,
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

    Comma,
    Identifier,

    Number,
    String,

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
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
