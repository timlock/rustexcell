use std::{
    borrow::BorrowMut, collections::HashMap, hash::Hash, rc::Rc, str::FromStr,
    string::FromUtf8Error,
};

use crate::cell::{cell_clone::CellClone, expression::Expression, Cell};

#[derive(Debug)]
pub struct Table {
    cells: Vec<Rc<Cell>>,
    columns: Vec<Rc<String>>,
    rows: Vec<Rc<String>>,
    sheet: HashMap<Rc<String>, HashMap<Rc<String>, Rc<Cell>>>,
}

impl TryFrom<String> for Table {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut table = Table::empty();
        let mut lines = value.lines();
        if let Some(l) = lines.next() {
            table.parse_header(l);
        };
        while let Some(l) = lines.next() {
            table.parse_row(l);
        }
        Ok(table)
    }
}

impl Table {
    pub fn empty() -> Table {
        Table {
            cells: Vec::new(),
            columns: Vec::new(),
            rows: Vec::new(),
            sheet: HashMap::new(),
        }
    }

    fn parse_header(&mut self, header_string: &str) {
        header_string
            .split(',')
            .into_iter()
            .for_each(|c| self.columns.push(Rc::new(c.to_string())));
    }

    fn parse_row(&mut self, row_string: &str) {
        let mut seperated = row_string.split(',');
        let row_id = parse_row_id(&mut seperated);
        self.rows.push(Rc::new(row_id));
        let mut row = HashMap::new();
        parse_cells(seperated)
            .into_iter()
            .enumerate()
            .for_each(|(id, c)| {
                if let Some(cell) = c {
                    let col_id = self.columns.get(id + 1).unwrap();
                    row.insert(col_id.clone(), cell.clone());
                    self.cells.push(cell.clone());
                }
            });

        let row_id = self.rows.last().unwrap().clone();
        self.sheet.insert(row_id, row);
    }

    pub fn compute(&self) -> Result<String, String> {
        let mut output = String::from(" ,");
        self.rows.iter().enumerate().for_each(|(index, row_id)| {
            let letter = match base10_to_base26(index) {
                Ok(l) => l,
                Err(e) => panic!("Invalid tableheader"),
            };
            output += letter.as_str();
            output += ",";
        });
        output += "\n";

        self.rows.iter().for_each(|row_id| {
            output += row_id;
            output += ",";
            self.columns.iter().for_each(|col_id| {
                if let Some(cell) = self.get_cell(row_id, col_id) {
                    // let result = self.compute_cell(&cell).as_str();
                    // output += result;
                    // output += ",";
                }
            });
            output += "\n";
        });

        for row_id in &self.rows {
            if let Some(row) = self.sheet.get(row_id) {
                output += row_id.as_str();
                output += ",";
                for column_id in &self.columns {
                    if let Some(c) = row.get(column_id) {
                        output += column_id;
                        output += ",";
                        let cell_str = match **c {
                            Cell::Value(v) => v.to_string(),
                            Cell::Expression(_) => todo!(),
                            Cell::Clone(_) => todo!(),
                        };
                        output += cell_str.as_str();
                    } else {
                        output += " ";
                    }
                    output += ",";
                }
                output += "\n";
            }
        }
        Ok(output)
    }

    fn get_cell(&self, row_id: &Rc<String>, col_id: &Rc<String>) -> Option<&Rc<Cell>> {
        if let Some(row) = self.sheet.get(row_id) {
            if let Some(cell) = row.get(col_id) {
                return Some(cell);
            }
        }
        None
    }

    fn compute_cell(&self, cell: &Rc<Cell>) -> String {
        match &**cell {
            Cell::Value(v) => v.to_string(),
            Cell::Expression(e) => self.compute_expression(e),
            Cell::Clone(c) => self.compute_clone(c),
        }
    }

    fn compute_expression(&self, e: &Expression) -> String {
        todo!()
    }

    fn compute_clone(&self, c: &CellClone) -> String {
        todo!()
    }
}

fn parse_cells(seperated: std::str::Split<char>) -> Vec<Option<Rc<Cell>>> {
    seperated
        .map(|s| {
            if let Ok(c) = Cell::from_str(s) {
                let cell_ref = Rc::new(c);
                return Some(cell_ref);
            }
            None
        })
        .collect()
}

fn parse_row_id(seperated: &mut std::str::Split<char>) -> String {
    seperated
        .next()
        .expect(format!("Row {seperated:?} is missing commas").as_str())
        .to_string()
}

fn base10_to_base26(mut number: usize) -> Result<String, FromUtf8Error> {
    let mut bytes = Vec::new();
    let n = number / 26;
    for _ in 0..=n {
        let remainder = number % 26;
        bytes.push(remainder as u8);
        number /= 26;
    }
    let b = bytes
        .iter()
        .rev()
        .map(|byte| *byte + 64)
        .collect::<Vec<u8>>();
    String::from_utf8(b)
}

fn base26_to_base10(letter: String) -> u32 {
    let base: u32 = 26;
    letter
        .as_bytes()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(mut index, mut byte)| {
            let letter = (byte - 64) as u32;
            (index, letter)
        })
        .fold(0, |accumulator, (index, letter)| {
            let b = letter * base.pow(index as u32);
            accumulator + b
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base10_to_base26() {
        let mut number = 3;
        assert_eq!("C", base10_to_base26(number).unwrap());
        number = 28;
        assert_eq!("AB", base10_to_base26(number).unwrap());
    }

    #[test]
    fn test_base26_to_base10() {
        let mut letter = String::from("C");
        assert_eq!(3, base26_to_base10(letter));
        letter = String::from("AB");
        assert_eq!(28, base26_to_base10(letter));
    }
}
