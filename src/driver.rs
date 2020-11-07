use crate::{QueryResult, parser::ParsedQuery, QueryError};

pub struct DataRow {
    
}
pub struct Driver {

}

impl Driver {
    pub fn perform_query(&mut self, query: ParsedQuery) -> Result<QueryResult, QueryError> {
        panic!("AAA");
    }

    pub fn new() -> Driver {
        Driver {
            
        }
    }
}