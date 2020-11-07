pub mod parser;
mod lexer;

use std::fmt::Debug;

use crate::filter::FilterRule;

// #[derive(Debug)]
pub struct ParsedQuery{
    cols: Vec<String>,
    from: String,
    filter: Option<Box<dyn FilterRule>>,
}

#[derive(Debug, PartialEq)]
struct Token {
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

    // Errors
    Invalid,
}