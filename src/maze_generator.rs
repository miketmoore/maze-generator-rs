mod cell;
mod coord;
mod direction;
mod grid;
mod to_string;
mod wall;

use crate::maze_generator::to_string::ToString;
use cell::Cell;
use cell::GridCell;
use coord::Coord;
use direction::Direction;
use grid::Grid;
use wall::Wall;
use wall::WallsContainer;

pub fn carve_maze() {
    // println!("carve maze");

    // let coord: Coord = Coord::new();
    // println!("{}", coord.to_string());

    // let mut cell: Cell = GridCell::new(&coord);
    // cell.mark_start();
    // cell.mark_visited();
    // cell.mark_popped();
    // println!("{}", cell.to_string());

    // let wall: Wall = Wall::new(Direction::NORTH);
    // println!("{}", wall.to_string());

    // let walls: wall::Walls = wall::Walls::new(
    //     wall::Wall::new(Direction::NORTH),
    //     wall::Wall::new(Direction::EAST),
    //     wall::Wall::new(Direction::SOUTH),
    //     wall::Wall::new(Direction::WEST),
    // );

    // walls.for_each(|direction, wall| {
    //     println!(
    //         "wall for_each: direction={} wall={}",
    //         direction,
    //         wall.to_string()
    //     );
    // })

    println!("building a fresh grid");

    let grid: Grid = Grid::new();
}
