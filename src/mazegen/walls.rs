use crate::mazegen::direction::Direction;
use crate::mazegen::wall::Wall;
use rand::Rng;
use std::vec::Vec;

pub trait WallsContainer {
    fn new() -> Self;
    fn to_vec(&self) -> Vec<&Wall>;
    fn north(&self) -> &Wall;
    fn east(&self) -> &Wall;
    fn south(&self) -> &Wall;
    fn west(&self) -> &Wall;
    fn get_rand(walls: Vec<&Wall>) -> &Wall;
}

#[derive(Copy, Clone)]
pub struct Walls {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
    curr: Wall,
    next: Wall,
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
            curr: north,
            next: east,
        };
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
    fn east(&self) -> &Wall {
        &self.east
    }
    fn south(&self) -> &Wall {
        &self.south
    }
    fn west(&self) -> &Wall {
        &self.west
    }
    fn get_rand(walls: Vec<&Wall>) -> &Wall {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, 4);
        walls.get(i).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::mazegen::walls::Walls;
    use crate::mazegen::walls::WallsContainer;

    #[test]
    fn to_vec() {
        let walls: &Walls = &WallsContainer::new();

        let v = walls.to_vec();
        assert_eq!(v.len(), 4);

        assert_eq!(v[0].is_solid(), true);
        assert_eq!(v[1].is_solid(), true);
        assert_eq!(v[2].is_solid(), true);
        assert_eq!(v[3].is_solid(), true);
    }
}
