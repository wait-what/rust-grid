#[cfg(test)]

#[test]
fn initialization() {
    use crate::*;
    let _grid: Grid<bool> = Grid::new(16, 16, false);
}

#[test]
fn yx_access() {
    use crate::*;
    let grid: Grid<bool> = Grid::new(16, 16, false);
    assert_eq!(grid[5][9], false);
}

#[test]
fn xy_tuple_access() {
    use crate::*;
    let grid: Grid<bool> = Grid::new(16, 16, false);
    assert_eq!(grid[(9, 5)], false);
}

#[test]
fn yx_assignment() {
    use crate::*;
    let mut grid: Grid<bool> = Grid::new(16, 16, false);
    assert_eq!(grid[5][9], false);
    grid[5][9] = true;
    assert_eq!(grid[5][9], true);
}

#[test]
fn xy_tuple_assignment() {
    use crate::*;
    let mut grid: Grid<bool> = Grid::new(16, 16, false);
    assert_eq!(grid[(9, 5)], false);
    grid[(9, 5)] = true;
    assert_eq!(grid[(9, 5)], true);
}

#[test]
fn yx_vs_xy_tuple() {
    use crate::*;
    let mut grid: Grid<bool> = Grid::new(16, 16, false);
    grid[(9, 5)] = true;
    assert_eq!(grid[(9, 5)], grid[5][9]);
}

#[test]
fn size() {
    use crate::*;
    let grid: Grid<bool> = Grid::new(1, 2, false);
    assert_eq!(grid.size(), (1, 2));
}
