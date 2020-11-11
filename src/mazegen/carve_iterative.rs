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
        println!("carve iterative 0");
        let coord = history[history.len() - 1];
        let cell = grid.cell(&coord);

        let walls = grid.get_available_cell_walls(&cell);

        if walls.len() == 0 {
            println!("carve iterative 1");
            if history.len() >= 2 {
                println!("carve iterative 2");
                let backtrackedCoord = history.pop();
                if !backtrackedCoord.is_some() {
                    panic!("backtracked coord not found");
                }
                let backtrackedCell = grid.cell(backtrackedCoord.unwrap());
            // backtrackedCell.mark_popped();
            } else {
                running = false;
            }
        } else {
            println!("carve iterative 3");
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
