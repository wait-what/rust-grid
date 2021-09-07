use crate::*;

impl<T: Clone> Row<T> {
    pub fn new(width: usize, initial_value: T) -> Self {
        Row {
            cells: {
                let mut row = Vec::with_capacity(width);

                for x in 0..width {
                    row.insert(x, initial_value.clone());
                }

                row
            }
        }
    }
}


impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, initial_value: T) -> Self {
       Grid {
            rows: {
                let initial_row = Row::new(width, initial_value);

                let mut cells = Vec::with_capacity(height);

                for y in 0..height {
                    cells.insert(y, initial_row.clone());
                }

                cells
            }
       }
    }
}
