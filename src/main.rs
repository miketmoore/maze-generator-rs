mod mazegen;

use mazegen::carve_iterative::carve_iterative;
use mazegen::coord::Coord;
use mazegen::grid::Grid;
use mazegen::grid::Griddy;

fn main() {
    // let grid: Grid = Griddy::new(2, 2);

    // let a = grid.cell(&Coord::new(0, 0));
    // // let b = grid.cell(0, 1);
    // // let c = grid.cell(1, 0);
    // // let d = grid.cell(1, 1);

    // println!("{},{}", a.coord().row(), a.coord().col());
    // // println!("{},{}", b.coord().row(), b.coord().col());
    // // println!("{},{}", c.coord().row(), c.coord().col());
    // // println!("{},{}", d.coord().row(), d.coord().col());

    // let available = grid.get_available_cell_walls(&a);
    // println!("{}", available.len());

    carve_iterative(2, 2);
}
