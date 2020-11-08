mod driver;
mod parser;
mod filter;

use driver::Driver;
use parser::parser::Parser;

pub struct QueryResult {
    data: Vec<Vec<String>>,
    query: String,
}

impl std::fmt::Display for QueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}':\n", self.query)?;
        for row in self.data.iter() {
            write!(f, "[")?;
            write!(f, "{}", row.first().unwrap())?; // Queries that display 0 columns are invalid
            for column in row.iter().skip(1) {
                write!(f, ",{}", column)?;
            }
            write!(f, "]\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum QueryError {
    BadSyntax(&'static str),
    BadLex(&'static str),
    NumParseError(String),
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}


pub fn perform_query(input_query: String) -> Result<QueryResult, QueryError> {
    let mut parser = Parser::new(&input_query);
    let parsed_query = parser.parse()?;
    println!("query: {:?}", parsed_query);
    let mut driver = Driver::new();
    driver.perform_query(parsed_query)
}