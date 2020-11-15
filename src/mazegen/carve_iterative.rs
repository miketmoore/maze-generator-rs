use crate::mazegen::coord::Coord;
use crate::mazegen::grid::Grid;
use crate::mazegen::grid::Griddy;
use crate::mazegen::wall::Wall;
use crate::mazegen::walls::Walls;
use crate::mazegen::walls::WallsContainer;
use std::vec::Vec;

pub fn carve_iterative(grid: &mut Grid) {
    println!("carve iterative start");

    let mut history: Vec<&Coord> = Vec::new();

    let coord = grid.get_rand_coord();
    history.push(&coord);

    let mut running = true;
    while running {
        println!("carve iterative loop start");
        let coord = history[history.len() - 1];
        let cell = grid.cell_mut(&coord);

        let walls: &mut Vec<&mut Wall> = &mut Vec::new();
/*
cannot borrow `*grid` as mutable more than once at a time

second mutable borrow occurs hererustc(E0499)
carve_iterative.rs(21, 20): first mutable borrow occurs here
carve_iterative.rs(24, 9): second mutable borrow occurs here
carve_iterative.rs(24, 46): first borrow later used here
*/
        grid.get_available_cell_walls(coord, cell, walls);

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
            let wall = Walls::get_rand(walls);
            wall.carve();
            cell.mark_visited();

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
