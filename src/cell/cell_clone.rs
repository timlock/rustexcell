use std::{rc::Rc, str::FromStr};

use super::Cell;

    #[derive(Debug)]
    pub enum CellClone {
        Left(Rc<Cell>),
        Right(Rc<Cell>),
        Up(Rc<Cell>),
        Down(Rc<Cell>),
    }

    impl FromStr for CellClone {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!()
        }
    }

