use crate::mazegen::cell::Cell;
use crate::mazegen::coord::Coord;
use crate::mazegen::direction::Direction;
use crate::mazegen::walls::WallsContainer;
use rand::Rng;
use std::collections::HashMap;
use std::option::Option;

pub struct Grid {
    rows: i32,
    cols: i32,
    cells: HashMap<String, Cell>,
}

impl Grid {
    fn key(row: i32, col: i32) -> String {
        format!("{},{}", row, col)
    }
    pub fn new(rows: i32, cols: i32) -> Self {
        let mut cells = HashMap::new();

        for row in 0..rows {
            for col in 0..cols {
                cells.insert(Grid::key(row, col), Cell::new());
            }
        }

        Grid { rows, cols, cells }
    }
    fn cell(&self, coord: &Coord) -> &Cell {
        let key = Grid::key(coord.row(), coord.col());
        let opt = self.cells.get(&key);
        if !opt.is_some() {
            panic!("cell not found");
        }
        opt.unwrap()
    }
    pub fn cell_mut(&mut self, coord: &Coord) -> Option<&mut Cell> {
        let key = Grid::key(coord.row(), coord.col());
        self.cells.get_mut(&key)
    }
    fn row_in_bounds(&self, row: i32) -> bool {
        row >= 0 && row < self.rows
    }
    fn col_in_bounds(&self, col: i32) -> bool {
        col >= 0 && col < self.cols
    }
    fn coord_in_bounds(&self, coord: &Coord) -> bool {
        let row = coord.row();
        let col = coord.col();
        self.row_in_bounds(row) && self.col_in_bounds(col)
    }
    fn get_adjacent_cell_coord(&self, coord: &Coord, direction: Direction) -> Option<Coord> {
        let row = coord.row();
        let col = coord.col();
        let mut new_row = row;
        let mut new_col = col;
        match direction {
            Direction::NORTH => {
                if row == 0 {
                    return None;
                }
                new_row = row - 1;
            }
            Direction::EAST => {
                if col == (self.cols - 1) {
                    return None;
                }
                new_col = col + 1;
            }
            Direction::SOUTH => {
                if row == (self.rows - 1) {
                    return None;
                }
                new_row = row + 1;
            }
            Direction::WEST => {
                if col == 0 {
                    return None;
                }
                new_col = col - 1;
            }
        }
        let key = Grid::key(new_row, new_col);
        let cell_opt = self.cells.get(&key);
        if cell_opt.is_some() {
            Some(Coord::new(new_row, new_col))
        } else {
            None
        }
    }
    /**
     * Return the cell that is adjacent to the specified wall.
     */
    pub fn get_adjacent_coord(&self, coord: &Coord, direction: Direction) -> Option<Coord> {
        let adjacent_coords_opt = self.get_adjacent_cell_coord(coord, direction);
        if !adjacent_coords_opt.is_some() {
            return None;
        }
        let adjacent_coords = adjacent_coords_opt.unwrap();

        if self.coord_in_bounds(&adjacent_coords) {
            // let adjacent_cell = self.cell(&adjacent_coords);
            // Some(adjacent_cell)
            Some(adjacent_coords)
        } else {
            None
        }
    }

    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range
    pub fn get_rand_coord(&self) -> Coord {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0, self.rows);
        let col = rng.gen_range(0, self.cols);
        // let key = Grid::key(row, col);
        // self.cells.get(&key).unwrap().coord()
        Coord::new(row, col)
    }

    pub fn get_available_directions_at_coord(&self, coord: &Coord) -> Vec<Direction> {
        let cell = self.cell(coord);
        let walls = cell.walls();

        let mut result = Vec::new();

        for direction in &[
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ] {
            let wall;
            match direction {
                Direction::NORTH => wall = walls.north(),
                Direction::EAST => wall = walls.east(),
                Direction::SOUTH => wall = walls.south(),
                Direction::WEST => wall = walls.west(),
            }
            if wall.is_solid() {
                let adjacent_coord_opt = self.get_adjacent_coord(coord, *direction);
                if adjacent_coord_opt.is_some() {
                    let adjacent_cell = self.cell(&adjacent_coord_opt.unwrap());
                    if !adjacent_cell.visited() {
                        result.push(*direction);
                    }
                }
            }
        }

        result
    }

    // available cell walls are walls that have not been carved and that are adjacent to a cell
    // that has not been visited
    pub fn carve_random_wall_from_available(
        &mut self,
        // cell: &mut Cell,
        coord: &Coord,
    ) -> Option<Direction> {
        // we have a coord
        // we want to get a list of all solid walls for the cell at that coord
        let available_directions = self.get_available_directions_at_coord(coord);

        if available_directions.len() == 0 {
            return None;
        }

        let direction;
        if available_directions.len() == 1 {
            // there is only one wall so we just select it... not random at this point
            direction = available_directions.get(0).unwrap();
        } else {
            // get a random from available
            let mut rng = rand::thread_rng();
            let wall_index = rng.gen_range(0, available_directions.len() - 1);
            direction = available_directions.get(wall_index).unwrap();
        }

        // carve the wall
        let cell = self.cell_mut(coord).unwrap();
        match direction {
            Direction::NORTH => cell.walls_mut().north_mut().carve(),
            Direction::EAST => cell.walls_mut().east_mut().carve(),
            Direction::SOUTH => cell.walls_mut().south_mut().carve(),
            Direction::WEST => cell.walls_mut().west_mut().carve(),
        }

        Some(*direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::mazegen::coord::Coord;
    use crate::mazegen::direction::Direction;
    use crate::mazegen::grid::Grid;

    // #[test]
    // fn cell() {
    //     /*
    //            0   1   2   3
    //        0|0,0|0,1|0,2|0,3|
    //        1|1,0|1,1|1,2|1,3|
    //     */
    //     let grid = Grid::new(2, 4);

    //     let tests: [[i32; 2]; 8] = [
    //         [0, 0],
    //         [0, 1],
    //         [0, 2],
    //         [0, 3],
    //         [1, 0],
    //         [1, 1],
    //         [1, 2],
    //         [1, 3],
    //     ];

    //     for test in tests.iter() {
    //         let coord = Coord::new(test[0], test[1]);
    //         let cell = grid.cell(&coord);
    //         // TODO
    //         // assert_eq!(cell.coord().row(), test[0]);
    //         // assert_eq!(cell.coord().col(), test[1]);
    //     }
    // }

    #[test]
    fn row_in_bounds() {
        let grid = Grid::new(2, 4);

        assert_eq!(grid.row_in_bounds(-1), false);
        assert_eq!(grid.row_in_bounds(0), true);
        assert_eq!(grid.row_in_bounds(1), true);
        assert_eq!(grid.row_in_bounds(2), false);
    }

    #[test]
    fn col_in_bounds() {
        let grid = Grid::new(2, 4);

        assert_eq!(grid.col_in_bounds(-1), false);
        assert_eq!(grid.col_in_bounds(0), true);
        assert_eq!(grid.col_in_bounds(1), true);
        assert_eq!(grid.col_in_bounds(2), true);
        assert_eq!(grid.col_in_bounds(3), true);
        assert_eq!(grid.col_in_bounds(4), false);
    }

    #[test]
    fn coord_in_bounds() {
        let grid = Grid::new(2, 4);

        assert_eq!(grid.coord_in_bounds(&Coord::new(-1, -1)), false);
        assert_eq!(grid.coord_in_bounds(&Coord::new(0, 0)), true);
        assert_eq!(grid.coord_in_bounds(&Coord::new(1, 3)), true);
        assert_eq!(grid.coord_in_bounds(&Coord::new(2, 4)), false);
    }

    #[test]
    fn get_adjacent_cell_coord() {
        let grid = Grid::new(2, 4);

        let mut coord_opt;
        let mut coord;

        {
            let coord_arg = &Coord::new(0, 0);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::NORTH);
            assert_eq!(coord_opt.is_some(), false);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::EAST);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 0);
            assert_eq!(coord.col(), 1);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::SOUTH);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 1);
            assert_eq!(coord.col(), 0);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::WEST);
            assert_eq!(coord_opt.is_some(), false);
        }

        {
            let coord_arg = &Coord::new(1, 1);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::NORTH);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 0);
            assert_eq!(coord.col(), 1);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::EAST);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 1);
            assert_eq!(coord.col(), 2);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::SOUTH);
            assert_eq!(coord_opt.is_some(), false);

            coord_opt = grid.get_adjacent_cell_coord(coord_arg, Direction::WEST);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 1);
            assert_eq!(coord.col(), 0);
        }
    }

    // #[test]
    // fn get_adjacent_cell() {
    //     let grid = Grid::new(2, 4);

    //     let direction = &Direction::NORTH;
    //     let cell_opt = grid.get_adjacent_cell(&Coord::new(1, 2), direction);
    //     assert_eq!(cell_opt.is_some(), true);
    //     let cell = cell_opt.unwrap();
    //     // TODO
    //     // assert_eq!(cell.coord().row(), 1);
    //     // assert_eq!(cell.coord().col(), 2);
    // }

    #[test]
    fn get_rand_coord() {
        let grid = Grid::new(2, 2);
        let mut count = 0;
        let max = 100;
        for _ in 0..max {
            let coord = grid.get_rand_coord();
            assert_eq!(coord.row() >= 0, true);
            assert_eq!(coord.row() <= 1, true);
            assert_eq!(coord.col() >= 0, true);
            assert_eq!(coord.col() <= 1, true);
            count = count + 1;
        }
        assert_eq!(count, max);
    }
}
