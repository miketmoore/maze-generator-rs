use std::collections::HashMap;
use std::option::Option;
use std::fmt;
use rand::{thread_rng, Rng};
use std::vec::Vec;

fn main() {
    println!("Hello, world!");

    let grid = Grid::new(10, 10);
    // grid.debug();

    // let coord = Coord::new(1, 1);

    // let cell = grid.get_cell(&coord).unwrap();
    // cell.debug();

    carve_iterative(grid);
}

struct Coord {
    row: u32,
    col: u32,
}

impl Coord {
    fn new(row: u32, col: u32) -> Self {
        Coord { row, col }
    }
    fn debug(&self) -> () {
        println!("row={} col={}", self.row, self.col);
    }
}

struct Cell {
    coord: Coord,
    start: bool,
    visited: bool,
    popped: bool,
    walls: Walls,
}

impl Cell {
    fn debug(&self) -> () {
        println!("row={}, col={}", self.coord.row, self.coord.col);
    }
    fn new(coord: Coord) -> Self {
        Cell {
            start: false,
            visited: false,
            popped: false,
            coord,
            walls: WallsContainer::new(
                Wall::new(Direction::NORTH),
                Wall::new(Direction::EAST),
                Wall::new(Direction::SOUTH),
                Wall::new(Direction::WEST),
            ),
        }
    }
    fn mark_start(&mut self) {
        self.start = true;
    }
    fn is_start(&self) -> bool {
        return self.start == true;
    }
    fn mark_visited(&mut self) {
        self.visited = true;
    }
    fn is_visited(&self) -> bool {
        return self.visited == true;
    }
    fn mark_popped(&mut self) {
        self.popped = true;
    }
    fn is_popped(&self) -> bool {
        return self.popped == true;
    }
    fn get_walls<'b>(&'b self) -> &'b Walls {
        return &self.walls;
    }
    fn get_opposite_wall(wall: u32) -> u32 {
        if wall == 0 {
            return 2;
        } else if wall == 1 {
            return 3;
        } else if wall == 2 {
            return 0;
        }
        return 1;
    }
    fn get_coord(&self) -> &Coord {
        return &self.coord;
    }
}

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::NORTH => write!(f, "north"),
            Direction::EAST => write!(f, "east"),
            Direction::SOUTH => write!(f, "south"),
            Direction::WEST => write!(f, "west"),
        }
    }
}

#[derive(PartialEq, Eq)]
enum WallState {
    SOLID,
    CARVED,
}

// https://stackoverflow.com/a/48368826/300575
impl WallState {
    fn is_solid(&self) -> bool {
        match *self {
            WallState::SOLID => true,
            _ => false,
        }
    }
}

impl fmt::Display for WallState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WallState::SOLID => write!(f, "solid"),
            WallState::CARVED => write!(f, "carved"),
        }
    }
}

pub struct Wall {
    pub direction: Direction,
    state: WallState,
}

impl Wall {
    pub fn new(direction: Direction) -> Wall {
        return Wall {
            direction,
            state: WallState::SOLID,
        };
    }
}

impl ToString for Wall {
    fn to_string(&self) -> std::string::String {
        return format!("direction={} state={}", self.direction, self.state);
    }
}

pub trait WallsContainer {
    fn new(north: Wall, east: Wall, south: Wall, west: Wall) -> Self;
    // fn for_each(&self, cb: fn(direction: &Direction, wall: &Wall) -> ());
    // fn for_each<F>(&self, cb: &mut F)
    //     where
    //         F: FnMut(&Direction, &Wall) -> ();
    fn for_each(&self, cb: impl FnMut(&Direction, &Wall)) ->  ();
    fn to_array(&mut self) -> [&Wall; 4];
}

pub struct Walls {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
}

impl WallsContainer for Walls {
    fn new(north: Wall, east: Wall, south: Wall, west: Wall) -> Self {
        return Walls {
            north,
            east,
            south,
            west,
        };
    }
    fn for_each(&self, mut cb: impl FnMut(&Direction, &Wall)) -> () {
        cb(&self.north.direction, &self.north);
        cb(&self.east.direction, &self.east);
        cb(&self.west.direction, &self.west);
        cb(&self.south.direction, &self.south);
    }
    // fn for_each<F>(&self, cb: &mut F) where
    // F: FnMut(&Direction, &Wall) -> () {
    //     cb(&self.north.direction, &self.north);
    //     cb(&self.east.direction, &self.east);
    //     cb(&self.west.direction, &self.west);
    //     cb(&self.south.direction, &self.south);
    // }
    fn to_array(&mut self) -> [&Wall; 4] {
        return [&self.north, &self.east, &self.west, &self.south];
    }
}


struct Grid {
    rows: u32,
    cols: u32,
    cells: HashMap<String, Cell>,
}

impl Grid {
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

    fn debug(&self) -> () {
        println!("rows={}, cols={}", self.rows, self.cols);
    }

    // fn get_cell(&self, row: u32, col: u32) -> Option<&Cell> {
    //     let key = format!("{},{}", row, col);
    //     let found = self.cells.get(&key);
    //     found
    // }

    fn get_cell(&self, coord: &Coord) -> Option<&Cell> {
        let key = format!("{},{}", coord.row, coord.col);
        let found = self.cells.get(&key);
        found
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
    fn coord_in_bounds(&self, coord: &Coord) -> bool {
        return self.row_in_bounds(coord.row) && self.col_in_bounds(coord.col);
    }

    fn get_adjacent_cell(&self, direction: Direction, coord: Coord) -> Option<&Cell> {
        let adjacent_coord: &Coord = &self.get_adjacent_cell_coords(direction, coord);
        if self.coord_in_bounds(adjacent_coord) {
            self.get_cell(adjacent_coord)
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

    /*
    public getAvailableCellWalls = (cell: ICell, cellCoord: ICoord) => {
        // available cell walls are walls that have not been carved and that are adjacent to a cell
        // that has not been visited

        const walls = cell.getWalls()
        const results: Wall[] = []
        walls.forEach((direction, wall) => {
        if (wall.state === 'solid') {
            const adjacentCell = this.grid.getAdjacentCell(direction, cellCoord)
            if (adjacentCell && !adjacentCell.isVisited()) {
            results.push(wall)
            }
        }
        })

        return results
    }
    */
    fn get_available_cell_walls(&self, cell: &Cell, cellCoord: &Coord) -> Vec<Wall> {
        // available cell walls are walls that have not been carved and that are adjacent to a cell
        // that has not been visited

        let walls = cell.get_walls();
        let results: Vec<Wall> = Vec::new();
        
        walls.for_each(|direction: &Direction, wall: &Wall| -> () {
            println!("for each iteration direction={}", direction);
            // if wall.state.is_solid() {
            //     let adjacent_cell = self.get_adjacent_cell(*direction, *cellCoord);
            //     if adjacent_cell.is_some() && !adjacent_cell.unwrap().is_visited() {
            //         results.push(*wall);
            //     }
            // }
        });
        results
    }
}

fn carve_iterative(grid: Grid) -> () {
    let start_coord = grid.get_rand_coord();
    start_coord.debug();

    let mut history = Vec::new();
    history.push(&start_coord);

    let mut x = 0;
    let mut running = true;
    while running {

        let coord = history[history.len() - 1];
        coord.debug();

        let cell = grid.get_cell(coord);
        if !cell.is_some() {
            panic!("cell not found");
        }

        // get list of walls not carved yet, that point to adjacent cells that have not been visited yet
        let walls = grid.get_available_cell_walls(cell.unwrap(), cell.unwrap().get_coord());

        if x < 5 {
            x = x + 1;
        } else {
            running = false;
        }
    }
}

/*
function carveIterative(grid: ICarveableGrid): void {
    const coord = grid.getGrid().getRandCoord()
    const history = [coord]
  
    let running = true
    while (running) {
      const coord = history[history.length - 1]
      const cell = grid.getCell(coord)
      if (!cell) {
        throw new Error('cell not found')
      }
  
      // get list of walls not carved yet, that point to adjacent cells that have not been visited yet
      const walls = grid.getAvailableCellWalls(cell, cell.getCoord())
  
      if (walls.length === 0) {
        if (history.length >= 2) {
          const backtrackedCoord = history.pop()
          if (!backtrackedCoord) {
            throw new Error('backtracked coord not found')
          }
          const backtrackedCell = grid.getCell(backtrackedCoord)
          if (backtrackedCell) {
            backtrackedCell.markPopped()
          }
        } else {
          running = false
        }
      } else {
        const wallIndex = randInRange(0, walls.length)
        const wall = walls[wallIndex]
        wall.state = 'carved'
        cell.markVisited()
  
        const adjacentCell = grid.getAdjacentCell(wall.direction, cell.getCoord())
        if (adjacentCell && !adjacentCell.isVisited()) {
          const oppDir = getOppositeDirection(wall.direction)
          adjacentCell.getWalls()[oppDir].state = 'carved'
          adjacentCell.markVisited()
          history.push(adjacentCell.getCoord())
        }
      }
    }
  }
*/