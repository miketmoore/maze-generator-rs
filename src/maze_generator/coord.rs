use crate::maze_generator::to_string::ToString;

pub struct Coord {
    pub row: u32,
    pub col: u32,
}

impl Coord {
    pub fn new(row: u32, col: u32) -> Coord {
        Coord { row, col }
    }
}

impl ToString for Coord {
    fn to_string(&self) -> String {
        return format!("row={} x col={}", self.row, self.col);
    }
}
