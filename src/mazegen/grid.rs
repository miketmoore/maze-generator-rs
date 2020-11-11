use crate::mazegen::cell::Cell;
use crate::mazegen::coord::Coord;
use crate::mazegen::direction::Direction;
use crate::mazegen::wall::Wall;
use crate::mazegen::walls::WallsContainer;
use rand::Rng;
use std::collections::HashMap;
use std::option::Option;

pub struct Grid {
    rows: i32,
    cols: i32,
    cells: HashMap<String, Cell>,
}

pub trait Griddy<'a> {
    fn new(rows: i32, cols: i32) -> Self;
    fn cell(&self, coord: &Coord) -> &Cell;
    fn get_available_cell_walls(&self, cell: &'a Cell) -> Vec<&'a Wall>;
    fn get_adjacent_cell(&self, direction: &Direction, cell: &Cell) -> Option<&Cell>;
    fn get_adjacent_cell_coord(&self, direction: &Direction, coord: &Coord) -> Option<&Coord>;
    fn row_in_bounds(&self, row: i32) -> bool;
    fn col_in_bounds(&self, col: i32) -> bool;
    fn coord_in_bounds(&self, coord: &Coord) -> bool;
    fn get_rand_coord(&self) -> &Coord;
}

impl<'a> Griddy<'a> for Grid {
    fn new(rows: i32, cols: i32) -> Self {
        let mut cells = HashMap::new();

        for row in 0..rows {
            for col in 0..cols {
                let key = format!("{},{}", row, col);
                // let coord = Coord::new(row, col);
                let cell = Cell::new(row, col);
                cells.insert(key, cell);
            }
        }

        Grid { rows, cols, cells }
    }
    fn cell(&self, coord: &Coord) -> &Cell {
        let key = format!("{},{}", coord.row(), coord.col());
        let opt = self.cells.get(&key);
        if !opt.is_some() {
            panic!("cell not found");
        }
        opt.unwrap()
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
    fn get_adjacent_cell_coord(&self, direction: &Direction, coord: &Coord) -> Option<&Coord> {
        let row = coord.row();
        let col = coord.col();
        match direction {
            Direction::NORTH => {
                if row == 0 {
                    None
                } else {
                    let key = format!("{},{}", row - 1, col);
                    let cell = self.cells.get(&key);
                    Some(cell.unwrap().coord())
                }
            }
            Direction::EAST => {
                if col == (self.cols - 1) {
                    None
                } else {
                    let key = format!("{},{}", row, col + 1);
                    let cell = self.cells.get(&key);
                    Some(cell.unwrap().coord())
                }
            }
            Direction::SOUTH => {
                if row == (self.rows - 1) {
                    None
                } else {
                    let key = format!("{},{}", row + 1, col);
                    let cell = self.cells.get(&key);
                    Some(cell.unwrap().coord())
                }
            }
            Direction::WEST => {
                if col == 0 {
                    None
                } else {
                    let key = format!("{},{}", row, col - 1);
                    let cell = self.cells.get(&key);
                    Some(cell.unwrap().coord())
                }
            }
        }
    }
    fn get_adjacent_cell(&self, direction: &Direction, cell: &Cell) -> Option<&Cell> {
        let adjacent_coords_opt = self.get_adjacent_cell_coord(&direction, &cell.coord());
        if !adjacent_coords_opt.is_some() {
            return None;
        }
        let adjacent_coords = adjacent_coords_opt.unwrap();

        if self.coord_in_bounds(&adjacent_coords) {
            let adjacent_cell = self.cell(&adjacent_coords);
            Some(adjacent_cell)
        } else {
            None
        }
    }
    fn get_available_cell_walls(&self, cell: &'a Cell) -> Vec<&'a Wall> {
        let mut results: Vec<&Wall> = Vec::new();

        let cell_walls = cell.walls();
        let cell_walls_vec = cell_walls.to_vec();

        for wall in cell_walls_vec {
            if wall.state().is_solid() {
                let adjacent_cell = self.get_adjacent_cell(&wall.direction, cell);
                if adjacent_cell.is_some() && !adjacent_cell.unwrap().visited() {
                    results.push(wall);
                }
            }
        }

        results
    }
    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range
    fn get_rand_coord(&self) -> &Coord {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0, self.rows);
        let col = rng.gen_range(0, self.cols);
        let key = format!("{},{}", row, col);
        self.cells.get(&key).unwrap().coord()
    }
}

#[cfg(test)]
mod tests {
    use crate::mazegen::cell::Cell;
    use crate::mazegen::coord::Coord;
    use crate::mazegen::direction::Direction;
    use crate::mazegen::grid::Grid;
    use crate::mazegen::grid::Griddy;

    #[test]
    fn cell() {
        /*
               0   1   2   3
           0|0,0|0,1|0,2|0,3|
           1|1,0|1,1|1,2|1,3|
        */
        let grid: Grid = Griddy::new(2, 4);

        let tests: [[i32; 2]; 8] = [
            [0, 0],
            [0, 1],
            [0, 2],
            [0, 3],
            [1, 0],
            [1, 1],
            [1, 2],
            [1, 3],
        ];

        for test in tests.iter() {
            let coord = Coord::new(test[0], test[1]);
            let cell = grid.cell(&coord);
            assert_eq!(cell.coord().row(), test[0]);
            assert_eq!(cell.coord().col(), test[1]);
        }
    }

    #[test]
    fn row_in_bounds() {
        let grid: Grid = Griddy::new(2, 4);

        assert_eq!(grid.row_in_bounds(-1), false);
        assert_eq!(grid.row_in_bounds(0), true);
        assert_eq!(grid.row_in_bounds(1), true);
        assert_eq!(grid.row_in_bounds(2), false);
    }

    #[test]
    fn col_in_bounds() {
        let grid: Grid = Griddy::new(2, 4);

        assert_eq!(grid.col_in_bounds(-1), false);
        assert_eq!(grid.col_in_bounds(0), true);
        assert_eq!(grid.col_in_bounds(1), true);
        assert_eq!(grid.col_in_bounds(2), true);
        assert_eq!(grid.col_in_bounds(3), true);
        assert_eq!(grid.col_in_bounds(4), false);
    }

    #[test]
    fn coord_in_bounds() {
        let grid: Grid = Griddy::new(2, 4);

        assert_eq!(grid.coord_in_bounds(&Coord::new(-1, -1)), false);
        assert_eq!(grid.coord_in_bounds(&Coord::new(0, 0)), true);
        assert_eq!(grid.coord_in_bounds(&Coord::new(1, 3)), true);
        assert_eq!(grid.coord_in_bounds(&Coord::new(2, 4)), false);
    }

    #[test]
    fn get_adjacent_cell_coord() {
        let grid: Grid = Griddy::new(2, 4);

        let mut coord_opt;
        let mut coord;

        {
            let coord_arg = &Coord::new(0, 0);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::NORTH, coord_arg);
            assert_eq!(coord_opt.is_some(), false);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::EAST, coord_arg);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 0);
            assert_eq!(coord.col(), 1);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::SOUTH, coord_arg);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 1);
            assert_eq!(coord.col(), 0);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::WEST, coord_arg);
            assert_eq!(coord_opt.is_some(), false);
        }

        {
            let coord_arg = &Coord::new(1, 1);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::NORTH, coord_arg);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 0);
            assert_eq!(coord.col(), 1);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::EAST, coord_arg);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 1);
            assert_eq!(coord.col(), 2);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::SOUTH, coord_arg);
            assert_eq!(coord_opt.is_some(), false);

            coord_opt = grid.get_adjacent_cell_coord(&Direction::WEST, coord_arg);
            assert_eq!(coord_opt.is_some(), true);
            coord = coord_opt.unwrap();
            assert_eq!(coord.row(), 1);
            assert_eq!(coord.col(), 0);
        }
    }

    #[test]
    fn get_adjacent_cell() {
        let grid: Grid = Griddy::new(2, 4);

        let direction = &Direction::NORTH;
        let coord = &Cell::new(2, 2);
        let cell_opt = grid.get_adjacent_cell(direction, coord);
        assert_eq!(cell_opt.is_some(), true);
        let cell = cell_opt.unwrap();
        assert_eq!(cell.coord().row(), 1);
        assert_eq!(cell.coord().col(), 2);
    }

    #[test]
    fn get_rand_coord() {
        let grid: Grid = Griddy::new(2, 2);
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
