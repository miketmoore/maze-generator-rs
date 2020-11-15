use crate::mazegen::coord::Coord;
use crate::mazegen::grid::Grid;
use crate::mazegen::grid::Griddy;
use std::vec::Vec;

pub fn carve_iterative(grid: &Grid) {
    println!("carve iterative start");

    let mut history: Vec<&Coord> = Vec::new();

    let coord = grid.get_rand_coord();
    history.push(&coord);

    let mut running = true;
    while running {
        println!("carve iterative loop start");
        let coord = history[history.len() - 1];
        let cell = grid.cell(&coord);

        let walls = grid.get_available_cell_walls(coord, &cell);

        if walls.len() == 0 {
            println!("carve iterative - no walls available");
            if history.len() >= 2 {
                println!("carve iterative - backtrack");
                history.pop();
            } else {
                println!("carve iterative - end");
                running = false;
            }
        } else {
            println!("carve iterative - walls are available - TODO not implemented yet");
            running = false; // TODO temporary
                             // let mut wall = Walls::get_rand(walls);
                             // wall.carve();
                             // cell.mark_visited();

            // let adjacent_cell = grid.get_adjacent_cell(&wall.direction(), cell);
            // if adjacent_cell.is_some() && !adjacent_cell.unwrap().visited() {
            //     let opp_dir = get_opposite_direction(wall.direction());
            //     adjacent_cell.walls()[opp_dir].carve();
            //     adjacent_cell.mark_visited();
            //     history.push(adjacent_cell.coord());
            // }
        }
    }
}
