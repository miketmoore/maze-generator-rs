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
}

#[cfg(test)]
mod tests {

    use crate::mazegen::cell::Cell;
    // use crate::mazegen::walls::WallsContainer;

    // #[test]
    // fn walls() {
    //     let mut cell = Cell::new();
    //     // TODO how to assert walls?

    //     let got = cell.walls();
    //     assert_eq!(got.north().is_solid(), true);
    // }

    #[test]
    fn visited_field() {
        let mut cell = Cell::new();

        assert_eq!(cell.visited(), false);
        cell.mark_visited();
        assert_eq!(cell.visited(), true);
    }
}
