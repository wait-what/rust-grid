//! # rust-grid
//!
//! rust-grid is a very minimal library to store large grids of any type in memory, with a user-friendly interface.
//!
//! # Examples
//! ```
//! use rust_grid::*;
//! 
//! fn main() {
//!     let grid: Grid<bool> = Grid::new(10, 15, false);
//!
//!     // Two ways to access data
//!     let (x, y) = (3, 4);
//!     println!("{}", grid[y][x]);
//!     println!("{}", grid[(x, y)]);
//! 
//!     // Get the size
//!     let (width, height) = grid.size();
//! }
//! ```
//!

mod index;
mod new;
mod size;
mod structure;

pub use index::*;
pub use new::*;
pub use size::*;
pub use structure::*;

mod tests;
