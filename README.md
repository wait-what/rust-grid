# rust-grid

rust-grid is a very minimal library to store large grids of any type in memory, with a user-friendly interface.

# Examples
```rs
use rust_grid::*;

fn main() {
    let grid: Grid<bool> = Grid::new(10, 15, false);

    // Two ways to access data
    let (x, y) = (3, 4);
    println!("{}", grid[y][x]);
    println!("{}", grid[(x, y)]);

    // Get the size
    let (width, height) = grid.size();
}
```

# Bound checking
Bound checking comes from `std::vec`, so it will only panic if you attempt to access data out
of bounds in debug mode. In release mode, this could result in undefined behaviour.
