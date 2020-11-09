#[derive(Copy, Clone)]
pub struct Coord {
    row: u32,
    col: u32,
}

impl Coord {
    pub fn new(row: u32, col: u32) -> Self {
        Coord { row, col }
    }
    pub fn row(&self) -> u32 {
        self.row
    }
    pub fn col(&self) -> u32 {
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
