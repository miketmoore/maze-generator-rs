use std::fmt;

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::NORTH => write!(f, "north"),
            Direction::EAST => write!(f, "east"),
            Direction::SOUTH => write!(f, "south"),
            Direction::WEST => write!(f, "west"),
        }
    }
}
