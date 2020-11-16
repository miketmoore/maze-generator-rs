use crate::mazegen::coord::Coord;
use crate::mazegen::grid::Grid;
use std::vec::Vec;

pub fn carve_iterative(rows: i32, cols: i32) {
    println!("carve iterative start");
    let mut grid = Grid::new(rows, cols);

    let mut history: Vec<&Coord> = Vec::new();

    let coord = grid.get_rand_coord();
    history.push(&coord);

    let mut running = true;
    while running {
        println!("carve iterative loop start");
        // const coord = history[history.length - 1]
        let coord_opt = history.get(history.len() - 1);
        if !coord_opt.is_some() {
            panic!("coord not found");
        }
        let cell_opt = grid.cell_mut(coord_opt.unwrap());
        if !cell_opt.is_some() {
            panic!("cell not found");
        }

        let coord = coord_opt.unwrap();
        // let cell = cell_opt.unwrap();

        // what is actually needed here
        // walls length (total available for cell at coord)
        // find random wall from this list, carve it, and know it's direction

        let result = grid.carve_random_wall_from_available(coord);

        if !result.is_some() {
            println!("carve iterative - no walls available");
            if history.len() >= 2 {
                println!("carve iterative - backtrack");
                history.pop();
            } else {
                println!("carve iterative - end");
                running = false;
            }
        } else {
            println!("carve iterative - walls are available");
            // TODO
        }

        // cannot borrow `grid` as mutable more than once at a time
        // let walls = grid.get_available_cell_walls(cell, coord);

        // println!("walls.len()={}", walls.len());

        // get list of walls not carved yet, that point to adjacent cells that have not been visited yet
    }
    // while running {
    //     println!("carve iterative loop start");
    //     let coord = history[history.len() - 1];

    //     // first mutable borrow occurs here
    //     let cell = grid.cell_mut(&coord);

    //     let walls: &mut Vec<&mut Wall> = &mut Vec::new();

    //     // second mutable borrow occurs here
    //     // first borrow later used here
    //     grid.update_available_cell_walls_mut(cell, coord, walls);

    //     if walls.len() == 0 {
    //         println!("carve iterative - no walls available");
    //         if history.len() >= 2 {
    //             println!("carve iterative - backtrack");
    //             history.pop();
    //         } else {
    //             println!("carve iterative - end");
    //             running = false;
    //         }
    //     } else {
    //         println!("carve iterative - walls are available - TODO not implemented yet");
    //         running = false; // TODO temporary
    //         let wall = Walls::get_rand(walls);
    //         wall.carve();
    //         cell.mark_visited();

    //         // let adjacent_cell = grid.get_adjacent_cell(&wall.direction(), cell);
    //         // if adjacent_cell.is_some() && !adjacent_cell.unwrap().visited() {
    //         //     let opp_dir = get_opposite_direction(wall.direction());
    //         //     adjacent_cell.walls()[opp_dir].carve();
    //         //     adjacent_cell.mark_visited();
    //         //     history.push(adjacent_cell.coord());
    //         // }
    //     }
    // }
}
