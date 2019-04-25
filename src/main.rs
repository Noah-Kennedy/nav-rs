#[macro_use]
extern crate ndarray;

use crate::grid::Grid;
use ndarray::Array2;

pub mod grid;

fn main() {
    let mut grid = Grid::new(50, 25);

    grid.add_obstruction(10, 5, -1.0);
    grid.add_obstruction(30, 10, -1.0);
    grid.add_obstruction(40, 20, -1.0);

    grid.draw();
}
