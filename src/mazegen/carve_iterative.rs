use crate::mazegen::coord::Coord;
use crate::mazegen::direction::Direction;
use crate::mazegen::grid::Grid;
use crate::mazegen::walls::Walls;
use std::vec::Vec;

pub fn carve_iterative(rows: i32, cols: i32, verbose: bool) {
    log(verbose, "start");
    let mut grid = Grid::new(rows, cols);

    let mut history: Vec<Coord> = Vec::new();

    let coord = grid.get_rand_coord();
    history.push(coord);

    let mut running = true;
    while running {
        log(verbose, "loop start");
        let coord = match history.get(history.len() - 1) {
            Some(coord) => coord,
            None => panic!("coord not found"),
        };

        match grid.carve_random_wall_from_available(coord) {
            Some(random_direction) => {
                log(verbose, "walls are available");
                let cell = grid.cell_mut(coord).unwrap();
                cell.mark_visited();

                match grid.get_adjacent_coord(coord, random_direction) {
                    Some(adjacent_coord) => {
                        match grid.cell_mut(&adjacent_coord) {
                            Some(adjacent_cell) => {
                                log(verbose, "found adjacent cell");
                                if !adjacent_cell.visited() {
                                    log(verbose, "carving opposite wall");
                                    let opp_direction = Direction::get_opposite(random_direction);
                                    let adjacent_walls = adjacent_cell.walls_mut();
                                    Walls::carve_opposite(opp_direction, adjacent_walls);
                                    adjacent_cell.mark_visited();
                                    history.push(adjacent_coord);
                                }
                            }
                            _ => (),
                        };
                    }
                    _ => (),
                };
            }
            None => {
                log(verbose, "no walls available");
                if history.len() >= 2 {
                    log(verbose, "backtrack");
                    history.pop();
                } else {
                    log(verbose, "end");
                    running = false;
                }
            }
        };
    }
}

fn log(enabled: bool, msg: &str) -> () {
    if enabled {
        println!("MAZEGEN_DEBUG {}", msg);
    }
}
