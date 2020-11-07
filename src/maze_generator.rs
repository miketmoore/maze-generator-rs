mod cell;
mod coord;
mod direction;
mod grid;
mod to_string;
mod wall;

use cell::Cell;
use cell::GridCell;
use coord::Coord;
use direction::Direction;
use grid::Grid;
use grid::GridTrait;

pub fn carve_maze() {
    println!("building a fresh grid");

    let rows = 10;
    let cols = 10;
    let grid: Grid = GridTrait::new(rows, cols);
    grid.debug();
}
