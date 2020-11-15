mod mazegen;

use mazegen::carve_iterative::carve_iterative;
use mazegen::grid::Grid;
use mazegen::grid::Griddy;

fn main() {
    let rows = 2;
    let cols = 2;
    let mut grid: Grid = Griddy::new(rows, cols);
    carve_iterative(&mut grid);
}
