use std::fmt;

#[derive(Copy, Clone)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn is_north(&self) -> bool {
        match *self {
            Direction::NORTH => true,
            _ => false,
        }
    }
    pub fn is_east(&self) -> bool {
        match *self {
            Direction::EAST => true,
            _ => false,
        }
    }
    pub fn is_south(&self) -> bool {
        match *self {
            Direction::SOUTH => true,
            _ => false,
        }
    }
    pub fn is_west(&self) -> bool {
        match *self {
            Direction::WEST => true,
            _ => false,
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
