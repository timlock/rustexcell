use std::{rc::Rc, str::FromStr};

use crate::cell::{value::Value, Cell};
#[derive(Debug)]
pub enum Expression {
    Add(Vec<String>),
    Sub(Vec<String>),
}

impl Expression {
    pub fn sum(&self, cells: Vec<Rc<Cell>>) -> Result<Value, &str> {
        let mut sum = Value::Int(0);
        for cell in cells {
            let value = match *cell.as_ref() {
                Cell::Value(v) => v,
                _ => return Err("Can't add cell type"),
            };
            sum += value;
        }
        Ok(sum)
    }

    pub fn difference(&self, cells: Vec<Rc<Cell>>) -> Result<Value, &str> {
        let mut sum = Value::Int(0);
        for cell in cells {
            let value = match *cell.as_ref() {
                Cell::Value(v) => v,
                _ => return Err("Can't add cell type"),
            };
            sum -= value;
        }
        Ok(sum)
    }

    pub fn arguments_len(&self) -> usize {
        match self {
            Expression::Add(v) => v.len(),
            Expression::Sub(v) => v.len(),
        }
    }
    fn add_argument(&mut self, argument: String) {
        match self {
            Expression::Add(v) => v.push(argument),
            Expression::Sub(v) => v.push(argument),
        };
    }
}

impl FromStr for Expression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        match iter.next() {
            Some(m) => {
                if m != "=" {
                    return Err(String::from("Expression marker is missing"));
                }
            }
            None => return Err(String::from("Empty row")),
        };
        let mut expression = if s.contains("+") {
            Expression::Add(Vec::new())
        } else if s.contains("-") {
            Expression::Sub(Vec::new())
        } else {
            return Err(String::from("Expression operators missing"));
        };
        while let Some(s) = iter.next() {
            if s != "+" && s != "-" {
                expression.add_argument(s.to_string());
            }
        }
        if expression.arguments_len() < 2 {
            return Err(String::from("Not enough arguments"));
        }
        Ok(expression)
    }
}
