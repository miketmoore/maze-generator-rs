#[derive(Copy, Clone)]
pub struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    pub fn new(row: i32, col: i32) -> Self {
        Coord { row, col }
    }
    pub fn row(&self) -> i32 {
        self.row
    }
    pub fn col(&self) -> i32 {
        self.col
    }
}

#[cfg(test)]
mod tests {

    use crate::mazegen::coord::Coord;

    #[test]
    fn new() {
        let coord = Coord::new(1, 2);
        assert_eq!(coord.row(), 1);
        assert_eq!(coord.col(), 2);
    }
}
