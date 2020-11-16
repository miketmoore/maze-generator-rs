use crate::mazegen::walls::Walls;
use crate::mazegen::walls::WallsContainer;

#[derive(Copy, Clone)]
pub struct Cell {
    visited: bool,
    walls: Walls,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            visited: false,
            walls: WallsContainer::new(),
        }
    }
    pub fn walls(&self) -> &Walls {
        &self.walls
    }
    pub fn walls_mut(&mut self) -> &mut Walls {
        &mut self.walls
    }
    pub fn mark_visited(&mut self) {
        self.visited = true;
    }
    pub fn visited(&self) -> bool {
        return self.visited == true;
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
    use crate::mazegen::walls::WallsContainer;

    #[test]
    fn walls() {
        let mut cell = Cell::new();
        // TODO how to assert walls?

        let got = cell.walls();
        assert_eq!(got.north().is_solid(), true);
    }

    #[test]
    fn visited_field() {
        let mut cell = Cell::new();

        assert_eq!(cell.visited(), false);
        cell.mark_visited();
        assert_eq!(cell.visited(), true);
    }

    #[test]
    fn get_opposite_wall_index() {
        let cell = Cell::new();
        assert_eq!(cell.get_opposite_wall_index(0), 2);
        assert_eq!(cell.get_opposite_wall_index(1), 3);
        assert_eq!(cell.get_opposite_wall_index(2), 0);
        assert_eq!(cell.get_opposite_wall_index(3), 1);
    }
}
