use crate::maze_generator::to_string::ToString;

pub struct Coord {
    row: u32,
    col: u32,
}

impl Coord {
    pub fn new() -> Coord {
        Coord { row: 2, col: 3 }
    }
}

impl ToString for Coord {
    fn to_string(&self) -> String {
        return format!("row={} x col={}", self.row, self.col);
    }
}
