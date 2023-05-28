use std::str::FromStr;

use self::{expression::Expression, value::Value};

pub mod expression;
pub mod value;
#[derive(Debug)]
pub enum Cell {
    Value(Value),
    Expression(Expression),
    Empty,
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        if let Cell::Empty = self {
            return true;
        }
        false
    }
}

impl FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Cell::Empty);
        }
        if let Ok(c) = Value::from_str(s) {
            return Ok(Cell::Value(c));
        }
        if let Ok(c) = Expression::from_str(s) {
            return Ok(Cell::Expression(c));
        }
        Err(format!("Could not parse cell: {s}"))
    }
}
