use std::fmt::Debug;

use crate::driver::DataRow;

pub trait FilterRule: Debug {
    fn filter(&self, x: DataRow) -> bool;
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
    pub col: String,
    pub op: NumberOp,
    pub val: usize,
}

impl FilterRule for NumberFilter {
    fn filter(&self, x: DataRow) -> bool {
        todo!()
    }
}
#[derive(Debug)]
pub struct StringFilter {
    pub col: String,
    pub val: String,
}

impl FilterRule for StringFilter {
    fn filter(&self, x: DataRow) -> bool {
        todo!()
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
    fn filter(&self, x: DataRow) -> bool {
        todo!()
    }
}