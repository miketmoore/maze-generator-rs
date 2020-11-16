use crate::mazegen::coord::Coord;
use crate::mazegen::grid::Grid;
use crate::mazegen::direction::Direction;
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

pub fn carve_iterative(rows: i32, cols: i32) {
    println!("carve iterative start");
    let mut grid = Grid::new(rows, cols);

    let mut history: Vec<Coord> = Vec::new();

    let coord = grid.get_rand_coord();
    history.push(coord);

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

            /*
      const adjacentCoord = grid.getAdjacentCoord(wall.getDirection(), coord)
      if (adjacentCoord) {
        const adjacentCell = grid.getCell(adjacentCoord)
        if (adjacentCell && !adjacentCell.isVisited()) {
          const oppDir = getOppositeDirection(wall.getDirection())
          adjacentCell.getWalls()[oppDir].carve()
          adjacentCell.markVisited()
          history.push(adjacentCoord)
        }
      }
            */
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
