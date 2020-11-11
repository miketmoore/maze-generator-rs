use crate::mazegen::cell::Cell;
use crate::mazegen::coord::Coord;
use crate::mazegen::direction::Direction;
use crate::mazegen::walls::Wall;
use crate::mazegen::walls::WallsContainer;
use std::collections::HashMap;
use std::option::Option;

pub struct Grid {
    rows: i32,
    cols: i32,
    cells: HashMap<String, Cell>,
}

pub trait Griddy {
    fn new(rows: i32, cols: i32) -> Self;
    fn cell(&self, coord: &Coord) -> Cell;
    // getAvailableCellWalls
    fn get_available_cell_walls(&self, cell: &Cell) -> Vec<Wall>;
    fn get_adjacent_cell(&self, direction: Direction, cell: Cell) -> Option<Cell>;
    fn get_adjacent_cell_coord(&self, direction: Direction, coord: Coord) -> Coord;
    fn row_in_bounds(&self, row: i32) -> bool;
    fn col_in_bounds(&self, col: i32) -> bool;
    fn coord_in_bounds(&self, coord: Coord) -> bool;
}

impl<'a> Griddy for Grid {
    fn new(rows: i32, cols: i32) -> Self {
        let mut cells = HashMap::new();

        for row in 0..rows {
            for col in 0..cols {
                let key = format!("{},{}", row, col);
                let coord = Coord::new(row, col);
                let cell = Cell::new(coord);
                cells.insert(key, cell);
            }
        }

        Grid { rows, cols, cells }
    }
    fn cell(&self, coord: &Coord) -> Cell {
        let key = format!("{},{}", coord.row(), coord.col());
        let opt = self.cells.get(&key);
        if !opt.is_some() {
            panic!("cell not found");
        }
        *opt.unwrap()
    }
    fn row_in_bounds(&self, row: i32) -> bool {
        row >= 0 && row < self.rows
    }
    fn col_in_bounds(&self, col: i32) -> bool {
        col >= 0 && col < self.cols
    }
    fn coord_in_bounds(&self, coord: Coord) -> bool {
        let row = coord.row();
        let col = coord.col();
        self.row_in_bounds(row) && self.col_in_bounds(col)
    }
    fn get_adjacent_cell_coord(&self, direction: Direction, coord: Coord) -> Coord {
        match direction {
            Direction::NORTH => Coord::new(coord.row() - 1, coord.col()),
            Direction::EAST => Coord::new(coord.row(), coord.col() + 1),
            Direction::SOUTH => Coord::new(coord.row() + 1, coord.col()),
            Direction::WEST => Coord::new(coord.row(), coord.col() - 1),
        }
    }
    fn get_adjacent_cell(&self, direction: Direction, cell: Cell) -> Option<Cell> {
        let adjacent_coords = self.get_adjacent_cell_coord(direction, cell.coord());
        if self.coord_in_bounds(adjacent_coords) {
            Some(self.cell(&adjacent_coords))
        } else {
            None
        }
    }
    fn get_available_cell_walls(&self, cell: &Cell) -> Vec<Wall> {
        let mut results: Vec<Wall> = Vec::new();

        let cell_walls = cell.walls();
        cell_walls.for_each(|wall: &Wall| -> () {
            if wall.state().is_solid() {
                let adjacent_cell: Option<Cell> = self.get_adjacent_cell(wall.direction, *cell);
                if adjacent_cell.is_some() && !adjacent_cell.unwrap().visited() {
                    results.push(*wall);
                }
            }
        });
        results
    }
}

#[cfg(test)]
mod tests {
    use crate::mazegen::coord::Coord;
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
}
