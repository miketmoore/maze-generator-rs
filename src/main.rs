use std::fmt;

fn main() {
    println!("Hello, world!");
    carve_maze();
}

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Coord {
    fn to_string(&self) -> String {
        return format!("row={} x col={}", self.row, self.col);
    }
}

struct Coord { row: u32, col: u32 }

impl Coord {
    fn new() -> Coord {
        Coord { row: 2, col: 3 }
    }
}


trait GridCell {
    fn new(coord: Coord) -> Self;
    fn mark_start(&mut self);
    fn is_start(&self) -> bool;
    fn mark_visited(&mut self);
    fn is_visited(&self) -> bool;
    fn mark_popped(&mut self);
    fn is_popped(&self) -> bool;
}

struct Cell { coord: Coord, start: bool, visited: bool, popped: bool }

impl Cell { }

impl GridCell for Cell {
    fn new(coord: Coord) -> Cell {
        Cell {coord, start: false, visited: false, popped: false }
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
} 

impl ToString for Cell {
    fn to_string(&self) -> String {
        return format!("cell coord={} start={} visited={} popped={}", self.coord.to_string(), self.is_start(), self.is_visited(), self.is_popped());
    }
}

enum Direction {
    NORTH, EAST, SOUTH, WEST
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

enum WallState {
    SOLID, CARVED
}

impl fmt::Display for WallState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WallState::SOLID => write!(f, "solid"),
            WallState::CARVED => write!(f, "carved"),
        }
     }
}

struct Wall {
    pub direction: Direction,
    state: WallState
}

impl Wall {
    fn new(direction: Direction) -> Wall {
        return Wall{direction, state: WallState::SOLID};
    }
}

impl ToString for Wall {
    fn to_string(&self) -> std::string::String { 
        return format!("direction={} state={}", self.direction, self.state);
     }
}

// fn for_each () -> ();

trait WallsContainer {
    fn for_each(&self, cb: fn(direction: &Direction, wall: &Wall) -> ());
}

struct Walls {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
}

impl WallsContainer for Walls {
    fn for_each(&self, cb: fn(direction: &Direction, wall: &Wall) -> ()) {
        cb(&self.north.direction, &self.north);
        cb(&self.east.direction, &self.east);
        cb(&self.west.direction, &self.west);
        cb(&self.south.direction, &self.south);
    }
}

fn carve_maze() {
    println!("carve maze");

    let coord: Coord = Coord::new();
    println!("{}", coord.to_string());

    let mut cell: Cell = GridCell::new(coord);
    cell.mark_start();
    cell.mark_visited();
    cell.mark_popped();
    println!("{}", cell.to_string());

    let wall: Wall = Wall::new(Direction::NORTH);
    println!("{}", wall.to_string());

    let walls: Walls = Walls{
        north: Wall::new(Direction::NORTH),
        east: Wall::new(Direction::EAST),
        south: Wall::new(Direction::SOUTH),
        west: Wall::new(Direction::WEST),
    };

    walls.for_each(|direction, wall| {
        println!("wall for_each: direction={} wall={}", direction, wall.to_string());
    })
}
