use core::fmt;
use std::{ops, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
}

impl ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(s) => match rhs {
                Value::Int(r) => Value::Int(s + r),
                Value::Float(r) => Value::Float(s as f32 + r),
            },
            Value::Float(s) => match rhs {
                Value::Int(r) => Value::Float(s + (r as f32)),
                Value::Float(r) => Value::Float(s + r),
            },
        }
    }
}

impl ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(s) => match rhs {
                Value::Int(r) => Value::Int(s - r),
                Value::Float(r) => Value::Float(s as f32 - r),
            },
            Value::Float(s) => match rhs {
                Value::Int(r) => Value::Float(s - (r as f32)),
                Value::Float(r) => Value::Float(s - r),
            },
        }
    }
}

impl ops::SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i32>() {
            return Ok(Value::Int(n));
        }
        if let Ok(n) = s.parse::<f32>() {
            return Ok(Value::Float(n));
        }
        Err(format!("Could not parse {s}"))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
        }
    }
}
