use std::str::FromStr;

const SUM: &str = "SUM";

const MIN: &str = "MIN";

#[derive(Debug)]
pub enum Formula {
    Sum((String, String)),
    Min((String, String)),
}

impl Formula {
    pub fn begin(&self) -> &String {
        match self {
            Formula::Sum(a) => &a.0,
            Formula::Min(a) => &a.0,
        }
    }

    pub fn end(&self) -> &String {
        match self {
            Formula::Sum(a) => &a.1,
            Formula::Min(a) => &a.1,
        }
    }
}

impl FromStr for Formula {
    type Err = String;

    fn from_str(formula_str: &str) -> Result<Self, Self::Err> {
        let mut addr_vec = formula_str
            .split(['=', '(', ':', ')'])
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if addr_vec.len() < 4 {
            return Err(format!("Invalid formula {}", formula_str));
        }
        addr_vec.pop();
        let end = addr_vec.pop().unwrap();
        let begin = addr_vec.pop().unwrap();
        let function = addr_vec.pop().unwrap();
        match function.as_str() {
            SUM => Ok(Formula::Sum((begin, end))),
            MIN => Ok(Formula::Min((begin, end))),
            _ => return Err(format!("Unkown function {}", formula_str)),
        }
    }
}

pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Addition),
            "-" => Ok(Operator::Subtraction),
            "*" => Ok(Operator::Multiplication),
            "/" => Ok(Operator::Division),
            _ => Err(format!("Cant parse operator {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let sum = "=SUM(A1:A2)";
        let actual = Formula::from_str(sum);
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert_eq!("A1", actual.begin());
        assert_eq!("A2", actual.end());
    }
}
