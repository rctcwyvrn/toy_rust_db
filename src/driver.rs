use csv::StringRecord;

use crate::{data::DataAccessor, parser::ParsedQuery, QueryError};

pub struct Driver {
    data: DataAccessor,
}

impl Driver {
    pub fn perform_query(&mut self, query: ParsedQuery) -> Result<Vec<Vec<String>>, QueryError> {
        let rows = self.data.get(query.from)?;
        println!("before filter: {:?}", rows);
        if let Some(filter) = query.filter {
            let filtered: Result<Vec<&StringRecord>, QueryError> = rows
                .iter()
                .filter_map(|row| {
                    let res = filter.filter(row);
                    match res {
                        Ok(keep) => {
                            if keep {
                                Some(Ok(row))
                            } else {
                                None
                            }
                        }
                        Err(e) => Some(Err(e)),
                    }
                })
                .collect();
            println!("after filter: {:?}", filtered);
            let x: Vec<Vec<String>> = filtered?
                .iter()
                .map(|rec| rec.iter().map(|s| s.to_string()).collect())
                .collect();
            return Ok(x);
        }
        todo!()
    }

    pub fn new() -> Result<Driver, QueryError> {
        Ok(Driver {
            data: DataAccessor::new()?,
        })
    }
}
