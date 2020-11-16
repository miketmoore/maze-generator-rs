use crate::mazegen::coord::Coord;
use crate::mazegen::direction::Direction;
use crate::mazegen::grid::Grid;
use crate::mazegen::walls::WallsContainer;
use std::vec::Vec;

pub fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::NORTH => Direction::SOUTH,
        Direction::EAST => Direction::WEST,
        Direction::SOUTH => Direction::NORTH,
        Direction::WEST => Direction::EAST,
    }
}

fn log(enabled: bool, msg: &str) -> () {
    if enabled {
        println!("MAZEGEN_DEBUG {}", msg);
    }
}

pub fn carve_iterative(rows: i32, cols: i32, verbose: bool) {
    log(verbose, "start");
    let mut grid = Grid::new(rows, cols);

    let mut history: Vec<Coord> = Vec::new();

    let coord = grid.get_rand_coord();
    history.push(coord);

    let mut running = true;
    while running {
        log(verbose, "loop start");
        let coord_opt = history.get(history.len() - 1);
        if !coord_opt.is_some() {
            panic!("coord not found");
        }
        let cell_opt = grid.cell_mut(coord_opt.unwrap());
        if !cell_opt.is_some() {
            panic!("cell not found");
        }

        let coord = coord_opt.unwrap();

        // what is actually needed here
        // walls length (total available for cell at coord)
        // find random wall from this list, carve it, and know it's direction

        let result = grid.carve_random_wall_from_available(coord);

        if !result.is_some() {
            log(verbose, "no walls available");
            if history.len() >= 2 {
                log(verbose, "backtrack");
                history.pop();
            } else {
                log(verbose, "end");
                running = false;
            }
        } else {
            log(verbose, "walls are available");
            let cell = grid.cell_mut(coord).unwrap();
            cell.mark_visited();

            let adjacent_coord = grid.get_adjacent_coord(coord, result.unwrap());
            if adjacent_coord.is_some() {
                let adjacent_cell_opt = grid.cell_mut(&adjacent_coord.unwrap());
                if adjacent_cell_opt.is_some() {
                    let adjacent_cell = adjacent_cell_opt.unwrap();
                    if !adjacent_cell.visited() {
                        let opp_direction = get_opposite_direction(result.unwrap());
                        let adjacent_walls = adjacent_cell.walls_mut();
                        match opp_direction {
                            Direction::NORTH => adjacent_walls.north_mut().carve(),
                            Direction::EAST => adjacent_walls.east_mut().carve(),
                            Direction::SOUTH => adjacent_walls.south_mut().carve(),
                            Direction::WEST => adjacent_walls.west_mut().carve(),
                        }
                        adjacent_cell.mark_visited();
                        history.push(adjacent_coord.unwrap());
                    }
                }
            }
        }
    }
}
