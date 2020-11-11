use crate::mazegen::coord::Coord;
use crate::mazegen::walls::Walls;
use crate::mazegen::walls::WallsContainer;

#[derive(Copy, Clone)]
pub struct Cell {
    coord: Coord,
    start: bool,
    visited: bool,
    popped: bool,
    walls: Walls,
}

impl Cell {
    pub fn new(row: i32, col: i32) -> Self {
        Cell {
            start: false,
            visited: false,
            popped: false,
            coord: Coord::new(row, col),
            walls: WallsContainer::new(),
        }
    }
    pub fn coord(&self) -> &Coord {
        &self.coord
    }
    pub fn walls(&self) -> &Walls {
        &self.walls
    }
    fn mark_start(&mut self) {
        self.start = true;
    }
    fn start(&self) -> bool {
        self.start == true
    }

    fn mark_visited(&mut self) {
        self.visited = true;
    }
    pub fn visited(&self) -> bool {
        return self.visited == true;
    }
    fn mark_popped(&mut self) {
        self.popped = true;
    }
    fn popped(&self) -> bool {
        return self.popped == true;
    }
    fn get_opposite_wall_index(&self, wall: i32) -> i32 {
        if wall == 0 {
            return 2;
        } else if wall == 1 {
            return 3;
        } else if wall == 2 {
            return 0;
        }
        return 1;
    }
}

#[cfg(test)]
mod tests {

    use crate::mazegen::cell::Cell;
    use crate::mazegen::direction::Direction;
    use crate::mazegen::walls::WallsContainer;

    #[test]
    fn coord() {
        let cell = Cell::new(123, 234);
        assert_eq!(cell.coord().row(), 123);
        assert_eq!(cell.coord().col(), 234);
    }

    #[test]
    fn walls() {
        let cell = Cell::new(123, 234);
        // TODO how to assert walls?

        let got = cell.walls();
        assert_eq!(got.north().state().is_solid(), true);
    }

    #[test]
    fn start_field() {
        let mut cell = Cell::new(123, 234);

        assert_eq!(cell.start(), false);
        cell.mark_start();
        assert_eq!(cell.start(), true);
    }

    #[test]
    fn visited_field() {
        let mut cell = Cell::new(123, 234);

        assert_eq!(cell.visited(), false);
        cell.mark_visited();
        assert_eq!(cell.visited(), true);
    }

    #[test]
    fn popped_field() {
        let mut cell = Cell::new(123, 234);

        assert_eq!(cell.popped(), false);
        cell.mark_popped();
        assert_eq!(cell.popped(), true);
    }

    #[test]
    fn get_opposite_wall_index() {
        let cell = Cell::new(123, 234);
        assert_eq!(cell.get_opposite_wall_index(0), 2);
        assert_eq!(cell.get_opposite_wall_index(1), 3);
        assert_eq!(cell.get_opposite_wall_index(2), 0);
        assert_eq!(cell.get_opposite_wall_index(3), 1);
    }
}
