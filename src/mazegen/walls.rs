use crate::mazegen::direction::Direction;
use crate::mazegen::wall::Wall;
use std::vec::Vec;

pub trait WallsContainer {
    fn new() -> Self;
    fn for_each(&self, cb: impl FnMut(&Wall)) -> ();
    fn to_vec(&self) -> Vec<&Wall>;
    fn north(&self) -> &Wall;
}

#[derive(Copy, Clone)]
pub struct Walls {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
}

impl WallsContainer for Walls {
    fn new() -> Self {
        let north = Wall::new(Direction::NORTH);
        let east = Wall::new(Direction::EAST);
        let south = Wall::new(Direction::SOUTH);
        let west = Wall::new(Direction::WEST);
        return Walls {
            north,
            east,
            south,
            west,
        };
    }
    fn for_each(&self, mut cb: impl FnMut(&Wall)) -> () {
        cb(&self.north);
        cb(&self.east);
        cb(&self.west);
        cb(&self.south);
    }
    fn to_vec(&self) -> Vec<&Wall> {
        let mut v = Vec::new();
        v.push(&self.north);
        v.push(&self.east);
        v.push(&self.south);
        v.push(&self.west);
        v
    }
    fn north(&self) -> &Wall {
        &self.north
    }
}

#[cfg(test)]
mod tests {

    use crate::mazegen::wall::Wall;
    use crate::mazegen::walls::Walls;
    use crate::mazegen::walls::WallsContainer;

    #[test]
    fn for_each() {
        let walls: &Walls = &WallsContainer::new();

        let mut count = 0;
        let mut directions: String = String::new();
        walls.for_each(|wall: &Wall| -> () {
            count = count + 1;
            directions = format!("{};{}", directions, wall.direction);
        });
        assert_eq!(count, 4);
        assert_eq!(directions, ";north;east;west;south");
    }
    #[test]
    fn to_vec() {
        let walls: &Walls = &WallsContainer::new();

        let v = walls.to_vec();
        assert_eq!(v.len(), 4);

        assert_eq!(v[0].state().is_solid(), true);
        assert_eq!(v[1].state().is_solid(), true);
        assert_eq!(v[2].state().is_solid(), true);
        assert_eq!(v[3].state().is_solid(), true);
    }
}
