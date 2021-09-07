use crate::*;
use std::ops::{ Index, IndexMut };

impl<T: Clone> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl<T: Clone> IndexMut<usize> for Row<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.cells[index]
    }
}

impl<T: Clone> Index<usize> for Grid<T> {
    type Output = Row<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T: Clone> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.rows[y][x]
    }
}

impl<T: Clone> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Row<T> {
        &mut self.rows[index]
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.rows[y][x]
    }
}
