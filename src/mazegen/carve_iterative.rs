use crate::mazegen::grid::Grid;
use crate::mazegen::grid::Griddy;
use crate::mazegen::wall::Wall;
use crate::mazegen::walls::Walls;
use crate::mazegen::walls::WallsContainer;
use std::vec::Vec;

pub fn carve_iterative(grid: &Grid) {
    println!("carve iterative start");

    let coord = grid.get_rand_coord();
    let mut history = Vec::new();
    history.push(coord);

    let mut running = true;
    while running {
        println!("carve iterative loop start");
        let coord = history[history.len() - 1];
        let cell = grid.cell(&coord);

        let walls = grid.get_available_cell_walls(&cell);

        if walls.len() == 0 {
            println!("carve iterative - no walls available");
            if history.len() >= 2 {
                println!("carve iterative - backtrack");
                let backtracked_coord = history.pop();
                if !backtracked_coord.is_some() {
                    panic!("carve iterative - backtracked coord not found");
                }
                // let backtracked_cell = grid.cell(backtracked_coord.unwrap());
                // backtracked_cell.mark_popped();
                /*
cannot borrow `*backtracked_cell` as mutable, as it is behind a `&` reference

`backtracked_cell` is a `&` reference, so the data it refers to cannot be borrowed as mutablerustc(E0596)
carve_iterative.rs(32, 17): `backtracked_cell` is a `&` reference, so the data it refers to cannot be borrowed as mutable
                */
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
