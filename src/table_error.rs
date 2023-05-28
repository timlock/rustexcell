use std::{error::Error, fmt, rc::Rc};

use crate::cell::Cell;

#[derive(Debug, Clone)]
pub enum TableError<'a> {
    InvalidTableHeader(&'a str),
    CantParseCell(&'a str, &'a str, Rc<Cell>),
}
impl<'a> TableError<'a> {}

impl<'a> fmt::Display for TableError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TableError::InvalidTableHeader(h) => write!(f, "Invalid header name: {h}"),
            TableError::CantParseCell(row, col, cell) => write!(f, "Could not compute cell: {cell:?} at {row}:{col}"),
        }
    }
}

impl Error for TableError<'_> {}
