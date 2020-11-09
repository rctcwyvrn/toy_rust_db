use crate::{data::DataAccessor, parser::ParsedQuery, QueryError, QueryResult};

pub struct Driver {
    data: DataAccessor,
}

impl Driver {
    pub fn perform_query(&mut self, query: ParsedQuery) -> Result<QueryResult, QueryError> {
        println!("Available datasets (from config): {:?}", self.data.ready);
        panic!("AAA");
    }

    pub fn new() -> Result<Driver, QueryError> {
        Ok(Driver {
            data: DataAccessor::new()?,
        })
    }
}
