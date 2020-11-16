use crate::mazegen::direction::Direction;
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

    pub fn carve_wall(&mut self, direction: &Direction) {
        match direction {
            Direction::NORTH => self.walls_mut().north_mut().carve(),
            Direction::EAST => self.walls_mut().east_mut().carve(),
            Direction::SOUTH => self.walls_mut().south_mut().carve(),
            Direction::WEST => self.walls_mut().west_mut().carve(),
        };
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
