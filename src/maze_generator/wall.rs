use std::fmt;

use crate::maze_generator::direction::Direction;

enum WallState {
    SOLID,
    CARVED,
}

impl fmt::Display for WallState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WallState::SOLID => write!(f, "solid"),
            WallState::CARVED => write!(f, "carved"),
        }
    }
}

pub struct Wall {
    pub direction: Direction,
    state: WallState,
}

impl Wall {
    pub fn new(direction: Direction) -> Wall {
        return Wall {
            direction,
            state: WallState::SOLID,
        };
    }
}

impl ToString for Wall {
    fn to_string(&self) -> std::string::String {
        return format!("direction={} state={}", self.direction, self.state);
    }
}

pub trait WallsContainer {
    fn new(north: Wall, east: Wall, south: Wall, west: Wall) -> Self;
    fn for_each(&self, cb: fn(direction: &Direction, wall: &Wall) -> ());
    fn to_array(&mut self) -> [&Wall; 4];
}

pub struct Walls {
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
}

impl WallsContainer for Walls {
    fn new(north: Wall, east: Wall, south: Wall, west: Wall) -> Self {
        return Walls{
            north,east,south,west
        }
    }
    fn for_each(&self, cb: fn(direction: &Direction, wall: &Wall) -> ()) {
        cb(&self.north.direction, &self.north);
        cb(&self.east.direction, &self.east);
        cb(&self.west.direction, &self.west);
        cb(&self.south.direction, &self.south);
    }
    fn to_array(&mut self) -> [&Wall; 4] {
        return [&self.north, &self.east, &self.west, &self.south];
    }
}