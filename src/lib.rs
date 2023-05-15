use std::{rc::Rc, error::Error};

pub struct ExcelSheet {
    cells: Vec<Cell>,
}

pub enum Cell {
    Value(Value),
    Function(Function),
    Clone(Clone),
}
#[derive(Clone, Copy)]
pub enum Value {
    Int(u32),
    Float(f32),
}


pub enum Function {
    Add(Add),
    Subtract(Subtract),
}

pub struct Add(Vec<Rc<Cell>>);

impl Add {
    pub fn sum(&self) -> Result<Value, &str> {
        let cells = &self.0;
        let mut sum = 0;
        for cell in cells {
            let value = match *cell.as_ref() {
                Cell::Value(v) => v,
                _ => return Err("Invalid cell"), 
            };
            sum += match value {
                Value::Int(i) => i,
                Value::Float(f) => f,
            }
        }
        Ok(Value::Float(1.1))
    }
}

pub struct Subtract(Vec<Rc<Cell>>);

pub enum Clone {
    Left(Rc<Cell>),
    Right(Rc<Cell>),
    Up(Rc<Cell>),
    Down(Rc<Cell>),
}
