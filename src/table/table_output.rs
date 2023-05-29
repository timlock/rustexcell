use std::fmt;

pub struct TableOutput {
    row_names: Vec<String>,
    col_names: Vec<String>,
    rows: Vec<Vec<String>>,
    cell_width: usize,
}

impl TableOutput {
    pub fn new(cell_width: usize) -> TableOutput {
        TableOutput {
            row_names: Vec::new(),
            col_names: Vec::new(),
            rows: Vec::new(),
            cell_width,
        }
    }
    pub fn add_row(&mut self, row_name: String) {
        self.row_names.push(row_name);
        let new_row = self
            .col_names
            .iter()
            .enumerate()
            .map(|_| String::new())
            .collect::<Vec<String>>();
        self.rows.push(new_row);
    }

    pub fn add_col(&mut self, col_name: String) {
        self.col_names.push(col_name);
        self.rows.iter_mut().for_each(|row| row.push(String::new()));
    }

    pub fn insert_cell(
        &mut self,
        cell: String,
        row_index: usize,
        col_index: usize,
    ) -> Result<(), String> {
        let length = self.row_names.len();
        if row_index >= self.row_names.len() {
            return Err(format!(
                "Index out of bounds for row, index is {} length is {}",
                row_index, length
            ));
        }
        let length = self.col_names.len();
        if col_index >= self.col_names.len() {
            return Err(format!(
                "Index out of bounds for column, index is {} length is {}",
                col_index, length
            ));
        }
        let row = &mut self.rows[row_index];
        row[col_index] = cell;
        Ok(())
    }

    fn header_to_string(&self) -> String {
        let mut output = String::from("0,");
        for header in &self.col_names {
            output += self.fill_with_white_space(&header).as_str();
            output += ","
        }
        output.remove(output.len() - 1);
        output
    }

    fn row_to_string(&self, row_index: usize) -> Option<String> {
        if self.row_names.get(row_index).is_none() {
            return None;
        }
        let mut output = self.row_names.get(row_index).unwrap().clone();
        output += ",";
        if let Some(row) = self.rows.get(row_index) {
            output = row
                .iter()
                .map(|c| self.trim_length(c))
                .fold(output, |acc, c| acc + c.as_str() + ",");
            output.remove(output.len() - 1);
            return Some(output);
        }
        None
    }

    fn trim_length(&self, cell: &str) -> String {
        if cell.len() == self.cell_width {
            return cell.to_string();
        } else if cell.len() < self.cell_width {
            return self.fill_with_white_space(cell);
        }
        self.reduce_cell_length(cell)
    }

    fn reduce_cell_length(&self, cell: &str) -> String {
        if let Ok(mut num) = cell.parse::<i32>() {
            let mut power_of_ten = 0;
            let mut output = String::new();
            while num > 10 {
                num /= 10;
                power_of_ten += 1;
            }
            output += num.to_string().as_str();
            output += "*10^";
            output += power_of_ten.to_string().as_str();
            return self.fill_with_white_space(&output);
        }
        String::from(&cell[0..self.cell_width])
    }

    fn fill_with_white_space(&self, cell: &str) -> String {
        let dif = self.cell_width - cell.len();
        let mut output = String::new();
        for _ in 0..dif {
            output += " ";
        }
        output += cell;
        output
    }
}

impl fmt::Display for TableOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = self.header_to_string();
        output += "\n";
        for i in 0..self.row_names.len() {
            if let Some(row_string) = self.row_to_string(i) {
                output += row_string.as_str();
                output += "\n";
            }
        }
        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_table() {
        let mut actual = TableOutput::new(8);
        actual.add_col(String::from("A"));
        actual.add_row(String::from("1"));
        let expected = String::from("0,       A\n1,        \n");
        assert_eq!(expected.as_str(), actual.to_string());
    }
    #[test]
    fn test_small_value_table() {
        let mut actual = TableOutput::new(8);
        actual.add_col(String::from("A"));
        actual.add_row(String::from("1"));
        actual.insert_cell(String::from("5"), 0, 0).unwrap();
        let expected = String::from("0,       A\n1,       5\n");
        let actual = actual.to_string();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_large_value() {
        let mut actual = TableOutput::new(8);
        actual.add_col(String::from("A"));
        actual.add_row(String::from("1"));
        actual.insert_cell(String::from("123456789"), 0, 0).unwrap();
        let expected = String::from("0,       A\n1,       1*10^8\n");
        let actual = actual.to_string();
        assert_eq!(expected, actual);
    }
}
