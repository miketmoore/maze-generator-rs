use crate::mazegen::direction::Direction;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WallState {
    SOLID,
    CARVED,
}

// https://stackoverflow.com/a/48368826/300575
impl WallState {
    pub fn is_solid(&self) -> bool {
        match *self {
            WallState::SOLID => true,
            _ => false,
        }
    }
}

impl fmt::Display for WallState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WallState::SOLID => write!(f, "solid"),
            WallState::CARVED => write!(f, "carved"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Wall {
    pub direction: Direction,
    state: WallState,
}

impl Wall {
    pub fn new(direction: Direction) -> Self {
        return Wall {
            direction,
            state: WallState::SOLID,
        };
    }
    pub fn state(&self) -> WallState {
        self.state
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn carve(&mut self) -> () {
        self.state = WallState::CARVED;
    }
}

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wall: direction={} state={}", self.direction, self.state)
    }
}

#[cfg(test)]
mod tests {

    use crate::mazegen::direction::Direction;
    use crate::mazegen::wall::Wall;

    #[test]
    fn state() {
        let wall = Wall::new(Direction::NORTH);
        assert_eq!(wall.state().is_solid(), true);
    }

    #[test]
    fn carve() {
        let mut wall = Wall::new(Direction::NORTH);
        wall.carve();
        assert_eq!(wall.state().is_solid(), false);
    }
}
