use crate::*;

impl<T> Row<T> {
    pub fn size(&self) -> usize {
        self.cells.len()
    }
}

impl<T> Grid<T> {
    pub fn size(&self) -> (usize, usize) {
        (
            self.rows[0].size(),
            self.rows.len()
        )
    }
}
