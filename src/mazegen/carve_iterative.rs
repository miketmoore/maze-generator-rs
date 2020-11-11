use crate::mazegen::grid::Grid;
use crate::mazegen::grid::Griddy;
use std::vec::Vec;

pub fn carve_iterative(rows: i32, cols: i32) {
    println!("carve iterative start");

    let grid: Grid = Griddy::new(rows, cols);

    let coord = grid.get_rand_coord();
    let mut history = Vec::new();
    history.push(coord);
}
