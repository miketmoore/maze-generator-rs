fn main() {
    println!("Hello, world!");
    carve_maze();
}

trait ToString {
    fn to_string(&self) -> String;
}

struct Coord { row: u32, col: u32 }

impl Coord {
    fn new() -> Coord {
        Coord { row: 2, col: 3 }
    }
}

impl ToString for Coord {
    fn to_string(&self) -> String {
        return format!("row={} x col={}", self.row, self.col);
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

fn carve_maze() {
    println!("carve maze");

    let coord: Coord = Coord::new();
    println!("{}", coord.to_string());

    let mut cell: Cell = GridCell::new(coord);
    cell.mark_start();
    cell.mark_visited();
    cell.mark_popped();
    println!("{}", cell.to_string())

}
