use std::{borrow::BorrowMut, collections::HashMap, hash::Hash, rc::Rc, str::FromStr};

use crate::cell::Cell;

#[derive(Debug)]
pub struct Table {
    cells: Vec<Rc<Cell>>,
    columns: Vec<Rc<String>>,
    rows: Vec<Rc<String>>,
    sheet: HashMap<Rc<String>, HashMap<Rc<String>, Rc<Cell>>>,
}

impl Table {
    pub fn parse(data: String) -> Table {
        let mut table = Table::empty();
        let mut lines = data.lines();
        if let Some(l) = lines.next() {
            table.parse_header(l);
        };
        while let Some(l) = lines.next() {
            table.parse_row(l);
        }
        table
    }

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
        let mut output = String::new();
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
                    }else {
                        output += " ";
                    }
                    output += ",";
                }
                output += "\n";
            }
        }
        Ok(output)
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
