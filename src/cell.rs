use std::str::FromStr;

use self::{value::Value, expression::Expression, cell_clone::CellClone};

mod cell_clone;
mod expression;
mod value;
#[derive(Debug)]
pub enum Cell {
    Value(Value),
    Expression(Expression),
    Clone(CellClone),
}

impl FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = Value::from_str(s) {
            return Ok(Cell::Value(c));
        }
        if let Ok(c) = Expression::from_str(s) {
            return Ok(Cell::Expression(c));
        }
        // if let Ok(c) = CellClone::from_str(s) {
        //     return Ok(Cell::Clone(c));
        // }
        Err(format!("Could not parse cell: {s}"))
    }
}
