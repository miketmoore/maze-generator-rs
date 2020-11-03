use crate::maze_generator::to_string;
use crate::maze_generator::coord;

pub trait GridCell {
    fn new(coord: coord::Coord) -> Self;
    fn mark_start(&mut self);
    fn is_start(&self) -> bool;
    fn mark_visited(&mut self);
    fn is_visited(&self) -> bool;
    fn mark_popped(&mut self);
    fn is_popped(&self) -> bool;
}

pub struct Cell {
    coord: coord::Coord,
    start: bool,
    visited: bool,
    popped: bool,
}

impl Cell {}

impl GridCell for Cell {
    fn new(coord: coord::Coord) -> Cell {
        Cell {
            coord,
            start: false,
            visited: false,
            popped: false,
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
}

impl to_string::ToString for Cell {
    fn to_string(&self) -> String {
        return format!(
            "cell coord={} start={} visited={} popped={}",
            self.coord.to_string(),
            self.is_start(),
            self.is_visited(),
            self.is_popped()
        );
    }
}