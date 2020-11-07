use crate::driver::DataRow;

pub trait FilterRule {
    fn filter(&self, x: DataRow) -> bool;
}

pub struct NumericalFilter {

}

pub struct LogicalFilter {
    
}