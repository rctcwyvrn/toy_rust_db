use std::fmt::Debug;

use csv::StringRecord;

use crate::QueryError;

pub trait FilterRule: Debug {
    fn filter(&self, x: &StringRecord) -> Result<bool, QueryError>;
}
#[derive(Debug)]
pub enum NumberOp {
    LT,
    LEQ,
    GT,
    GEQ,
    EQ,
}
#[derive(Debug)]
pub struct NumberFilter {
    pub col: usize,
    pub op: NumberOp,
    pub val: usize,
}

impl FilterRule for NumberFilter {
    fn filter(&self, x: &StringRecord) -> Result<bool, QueryError> {
        let num = x[self.col].parse::<usize>().map_err(|_| {
            QueryError::QueryFailed(
                "Expected a number in this column but got something else instead",
            )
        })?;
        match self.op {
            NumberOp::EQ => Ok(num == self.val),
            NumberOp::LT => Ok(num < self.val),
            NumberOp::LEQ => Ok(num <= self.val),
            NumberOp::GT => Ok(num > self.val),
            NumberOp::GEQ => Ok(num >= self.val),
        }
    }
}
#[derive(Debug)]
pub struct StringFilter {
    pub col: usize,
    pub val: String,
}

impl FilterRule for StringFilter {
    fn filter(&self, x: &StringRecord) -> Result<bool, QueryError> {
        Ok(x[self.col] == self.val)
    }
}
#[derive(Debug)]
pub enum LogicalOp {
    And,
    Or,
}
#[derive(Debug)]
pub struct LogicalFilter {
    pub f1: Box<dyn FilterRule>,
    pub op: LogicalOp,
    pub f2: Box<dyn FilterRule>,
}

impl FilterRule for LogicalFilter {
    fn filter(&self, x: &StringRecord) -> Result<bool, QueryError> {
        let r1 = self.f1.filter(x)?;
        let r2 = self.f2.filter(x)?;
        match self.op {
            LogicalOp::And => Ok(r1 && r2),
            LogicalOp::Or => Ok(r1 || r2),
        }
    }
}
