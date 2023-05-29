use std::str::FromStr;

use crate::cell::Cell;

use super::Table;

pub struct TableParser {
    table: Table,
}

impl TableParser {
    pub fn new() -> TableParser {
        let table = Table::empty();
        TableParser { table }
    }
    pub fn parse_header(&mut self, header_string: &str) {
        header_string
            .split(',')
            .into_iter()
            .filter(|c| *c != " " && !c.is_empty())
            .for_each(|c| self.table.columns.push(c.to_string()));
    }

    pub fn parse_row(&mut self, row_string: &str) {
        let mut seperated = row_string.split(',').collect::<Vec<&str>>();
        if seperated.len() < 1 {
            panic!("Row {seperated:?} is missing commas");
        }
        let row_id = String::from(seperated.remove(0));
        self.table.rows.push(row_id);
        let row_index = self.table.rows.len() - 1;
        seperated
            .into_iter()
            .map(|c| Cell::from_str(c).expect(format!("Could not parse cell {}", c).as_str()))
            .enumerate()
            .for_each(|(i, c)| {
                if !c.is_empty() {
                    self.table.insert_cell_by_index(row_index, i, c).unwrap();
                }
            });
    }
    pub fn build(self) -> Table {
        self.table
    }
}
