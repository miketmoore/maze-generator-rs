use crate::mazegen::direction::Direction;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Wall {
    pub direction: Direction,
    solid: bool
}

impl Wall {
    pub fn new(direction: Direction) -> Self {
        return Wall {
            direction,
            solid: true
        };
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn carve(&mut self) -> () {
        self.solid = false;
    }
    pub fn is_solid(&self) -> bool { self.solid }
}

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wall: direction={} solid={}", self.direction, self.solid)
    }
}

#[cfg(test)]
mod tests {

    use crate::mazegen::direction::Direction;
    use crate::mazegen::wall::Wall;

    #[test]
    fn state() {
        let wall = Wall::new(Direction::NORTH);
        assert_eq!(wall.is_solid(), true);
    }

    #[test]
    fn carve() {
        let mut wall = Wall::new(Direction::NORTH);
        wall.carve();
        assert_eq!(wall.is_solid(), false);
    }
}
