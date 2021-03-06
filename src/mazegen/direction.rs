use std::fmt;

#[derive(Copy, Clone)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn get_opposite(direction: Direction) -> Direction {
        match direction {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }
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
