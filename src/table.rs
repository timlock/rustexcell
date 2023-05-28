use std::{collections::HashMap, rc::Rc, str::FromStr, string::FromUtf8Error};

use crate::{
    cell::{expression::Expression, Cell},
    table_error::TableError,
    table_output::TableOutput,
};
type TableResult<'a> = Result<TableOutput, TableError<'a>>;

#[derive(Debug)]
pub struct Table {
    cells: Vec<Rc<Cell>>,
    columns: Vec<String>,
    rows: Vec<String>,
    sheet: HashMap<String, HashMap<String, Rc<Cell>>>,
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
    pub fn insert_cell_by_index(
        &mut self,
        row_index: usize,
        col_index: usize,
        cell: Cell,
    ) -> Result<Rc<Cell>, String> {
        if self.rows.get(row_index).is_none() {
            return Err(format!("Invalid row index: {}", row_index));
        }
        let row_id = &self.rows[row_index].clone();
        if self.columns.get(col_index).is_none() {
            return Err(format!("Invalid column index: {}", col_index));
        }
        let col_id = &self.columns[col_index].clone();
        self.insert_cell_by_id(row_id, col_id, cell)
    }

    pub fn insert_cell_by_id(
        &mut self,
        row_id: &String,
        col_id: &String,
        cell: Cell,
    ) -> Result<Rc<Cell>, String> {
        if !self.rows.contains(row_id) {
            return Err(format!("Invalid row_id: {}", row_id));
        }
        if !self.columns.contains(col_id) {
            return Err(format!("Invalid col_id: {}", col_id));
        }
        let cell = Rc::new(cell);
        if self.sheet.get(row_id).is_none() {
            self.sheet.insert(row_id.clone(), HashMap::new());
        }
        let row = self.sheet.get_mut(row_id).unwrap();
        row.insert(col_id.clone(), cell.clone());
        self.cells.push(cell.clone());
        Ok(cell)
    }

    fn parse_header(&mut self, header_string: &str) {
        header_string
            .split(',')
            .into_iter()
            .filter(|c| *c != " " && !c.is_empty())
            .for_each(|c| self.columns.push(c.to_string()));
    }

    fn parse_row(&mut self, row_string: &str) {
        let mut seperated = row_string.split(',').collect::<Vec<&str>>();
        if seperated.len() < 1 {
            panic!("Row {seperated:?} is missing commas");
        }
        let row_id = String::from(seperated.remove(0));
        self.rows.push(row_id);
        let row_index = self.rows.len() - 1;
        seperated
            .into_iter()
            .map(|c| Cell::from_str(c).expect(format!("Could not parse cell {}", c).as_str()))
            .enumerate()
            .for_each(|(i, c)| {
                if !c.is_empty() {
                    self.insert_cell_by_index(row_index, i, c).unwrap();
                }
            });
        // .collect::<Vec<Cell>>();
        // .map(|c| Cell::from_str(c))
        // .collect::<Vec<Result<Cell, String>>>();

        // let mut row = HashMap::new();
        // parse_cells(seperated)
        //     .into_iter()
        //     .enumerate()
        //     .for_each(|(id, c)| {
        //         if let Some(cell) = c {
        //             let col_id = self.columns.get(id).unwrap();
        //             row.insert(col_id.clone(), cell.clone());
        //             self.cells.push(cell.clone());
        //         }
        //     });

        // let row_id = self.rows.last().unwrap().clone();
        // self.sheet.insert(row_id, row);
    }

    pub fn compute(&self) -> TableResult {
        let mut table_output = TableOutput::new(8);
        self.rows.iter().enumerate().for_each(|(index, _)| {
            let letter = match base10_to_base26(index + 1) {
                Ok(l) => l,
                Err(_) => panic!("Invalid tableheader"),
            };
            table_output.add_col(letter);
        });
        self.rows
            .iter()
            .enumerate()
            .for_each(|(row_index, row_id)| {
                table_output.add_row((row_index + 1).to_string());
                self.columns
                    .iter()
                    .enumerate()
                    .for_each(|(col_index, col_id)| {
                        if let Some(cell) = self.get_cell(row_id, col_id) {
                            let result = self.compute_cell(&cell);
                            table_output
                                .insert_cell(result, row_index, col_index)
                                .unwrap();
                        }
                    });
            });
        TableResult::Ok(table_output)
    }

    fn get_cell(&self, row_id: &String, col_id: &String) -> Option<&Rc<Cell>> {
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
            Cell::Empty => String::new(),
        }
    }

    fn compute_expression(&self, e: &Expression) -> String {
        todo!()
    }
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
fn parse_cells(seperated: Vec<&str>) -> Vec<Option<Rc<Cell>>> {
    seperated
        .iter()
        .map(|s| {
            if let Ok(c) = Cell::from_str(s) {
                let cell_ref = Rc::new(c);
                return Some(cell_ref);
            }
            None
        })
        .collect()
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
