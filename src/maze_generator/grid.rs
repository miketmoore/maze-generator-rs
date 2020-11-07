use crate::maze_generator::Cell;
use crate::maze_generator::Coord;
use crate::maze_generator::Direction;
use crate::maze_generator::GridCell;
use std::option::Option;
use std::string::String;
// use rand::Rng;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub trait GridTrait {
    fn new(rows: u32, cols: u32) -> Self;
    fn get_cell<'b>(&self, coord: Coord) -> Option<&'b Cell>;
    fn get_adjacent_cell<'b>(&self, direction: Direction, coord: Coord) -> Option<&'b Cell>;
    fn get_rand_coord(&self) -> Coord;
    fn get_rand_cell<'b>(&self) -> &'b Cell;
    fn debug(&self) -> ();
    fn get_adjacent_cell_coords(&self, direction: Direction, coord: Coord) -> Coord;
    fn row_in_bounds(&self, row: u32) -> bool;
    fn col_in_bounds(&self, col: u32) -> bool;
    fn coord_in_bounds(&self, coord: Coord) -> bool;
}

pub struct Grid {
    rows: u32,
    cols: u32,
    cells: HashMap<String, Cell>,
}

impl<'a> GridTrait for Grid {
    fn new(rows: u32, cols: u32) -> Self {
        let mut cells = HashMap::new();

        for row in 0..rows {
            for col in 0..cols {
                let cell = Cell::new(row, col);
                let key = format!("{},{}", row, col);
                cells.insert(key, cell);
            }
        }

        Grid { rows, cols, cells }
    }

    fn get_cell<'b>(&self, coord: Coord) -> Option<&'b Cell> {
        let key = format!("{},{}", coord.row, coord.col);
        let cells = self.cells;
        let cell = cells.get(&key);
        return cell;
    }

    fn get_adjacent_cell<'b>(&self, direction: Direction, coord: Coord) -> Option<&'b Cell> {
        let adjacentCoord = self.get_adjacent_cell_coords(direction, coord);
        if self.coord_in_bounds(adjacentCoord) {
            self.get_cell(adjacentCoord)
        } else {
            None
        }
    }

    fn get_rand_coord(&self) -> Coord {
        let mut rng = thread_rng();
        let row: u32 = rng.gen_range(0, self.rows - 1);
        let col: u32 = rng.gen_range(0, self.cols - 1);
        Coord::new(row, col)
    }

    fn get_rand_cell<'b>(&self) -> &'b Cell {
        let coord = self.get_rand_coord();
        self.get_cell(coord).unwrap()
    }

    fn debug(&self) -> () {
        println!("rows={} x cols={}", self.rows, self.cols);
    }
    fn get_adjacent_cell_coords(&self, direction: Direction, coord: Coord) -> Coord {
        match direction {
            Direction::NORTH => Coord::new(coord.row - 1, coord.col),
            Direction::EAST => Coord::new(coord.row, coord.col + 1),
            Direction::SOUTH => Coord::new(coord.row + 1, coord.col),
            Direction::WEST => Coord::new(coord.row, coord.col - 1),
        }
    }
    fn row_in_bounds(&self, row: u32) -> bool {
        return row >= 0 && row < self.rows;
    }
    fn col_in_bounds(&self, col: u32) -> bool {
        return col >= 0 && col < self.cols;
    }
    fn coord_in_bounds(&self, coord: Coord) -> bool {
        return self.row_in_bounds(coord.row) && self.col_in_bounds(coord.col);
    }
}
