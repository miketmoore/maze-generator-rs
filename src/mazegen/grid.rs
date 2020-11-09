use crate::mazegen::cell::Cell;
use crate::mazegen::coord::Coord;
use crate::mazegen::direction::Direction;
use crate::mazegen::walls::Wall;
use crate::mazegen::walls::WallsContainer;
use std::collections::HashMap;
use std::option::Option;

pub struct Grid {
    rows: u32,
    cols: u32,
    cells: HashMap<String, Cell>,
}

pub trait Griddy {
    fn new(rows: u32, cols: u32) -> Self;
    fn cell(&self, coord: &Coord) -> Cell;
    // getAvailableCellWalls
    fn get_available_cell_walls(&self, cell: &Cell) -> Vec<Wall>;
    fn get_adjacent_cell(&self, direction: Direction, cell: Cell) -> Option<Cell>;
    fn get_adjacent_cell_coord(&self, direction: Direction, coord: Coord) -> Coord;
    fn row_in_bounds(&self, row: u32) -> bool;
    fn col_in_bounds(&self, col: u32) -> bool;
    fn coord_in_bounds(&self, coord: Coord) -> bool;
}

impl<'a> Griddy for Grid {
    fn new(rows: u32, cols: u32) -> Self {
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
    fn row_in_bounds(&self, row: u32) -> bool {
        row >= 0 && row < self.rows
    }
    fn col_in_bounds(&self, col: u32) -> bool {
        col >= 0 && col < self.cols
    }
    fn coord_in_bounds(&self, coord: Coord) -> bool {
        let row = coord.row();
        let col = coord.col();
        self.row_in_bounds(row) && self.col_in_bounds(col)
    }
    fn get_adjacent_cell_coord(&self, direction: Direction, coord: Coord) -> Coord {
        match direction {
            Direction::NORTH => {
                // TODO
                Coord::new(coord.row() - 1, coord.col())
            }
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
