use crate::maze_generator::coord::Coord;
use crate::maze_generator::direction::Direction;
use crate::maze_generator::to_string;
use crate::maze_generator::wall::Wall;
use crate::maze_generator::wall::Walls;
use crate::maze_generator::wall::WallsContainer;

pub trait GridCell<'a> {
    fn new(coord: &'a Coord) -> Self;
    fn mark_start(&mut self);
    fn is_start(&self) -> bool;
    fn mark_visited(&mut self);
    fn is_visited(&self) -> bool;
    fn mark_popped(&mut self);
    fn is_popped(&self) -> bool;
    fn get_walls<'b>(&'b self) -> &'b Walls;
    fn get_opposite_wall(wall: u32) -> u32;
    fn get_coord(&self) -> &'a Coord;
}

pub struct Cell<'a> {
    coord: &'a Coord,
    start: bool,
    visited: bool,
    popped: bool,
    walls: Walls
}

impl<'a> GridCell<'a> for Cell<'a> {
    fn new(coord: &'a Coord) -> Self {
        Cell {
            coord,
            start: false,
            visited: false,
            popped: false,
            walls: WallsContainer::new(
                Wall::new(Direction::NORTH),
                Wall::new(Direction::EAST),
                Wall::new(Direction::SOUTH),
                Wall::new(Direction::WEST),
            )
        }
    }
    fn mark_start(&mut self) {
        self.start = true;
    }
    fn is_start(&self) -> bool {
        return self.start == true;
    }
    fn mark_visited(&mut self) {
        self.visited = true;
    }
    fn is_visited(&self) -> bool {
        return self.visited == true;
    }
    fn mark_popped(&mut self) {
        self.popped = true;
    }
    fn is_popped(&self) -> bool {
        return self.popped == true;
    }
    fn get_walls<'b>(&'b self) -> &'b Walls {
        return &self.walls;
    }
    fn get_opposite_wall(wall: u32) -> u32 {
        if wall == 0 {
            return 2;
        } else if wall == 1 {
            return 3;
        } else if wall == 2 {
            return 0;
        }
        return 1;
    }
    fn get_coord(&self) -> &'a Coord {
        return self.coord;
    }
}

impl<'a> to_string::ToString for Cell<'a> {
    fn to_string(&self) -> String {
        return format!(
            "cell coord={} start={} visited={} popped={}",
            self.coord.to_string(),
            self.is_start(),
            self.is_visited(),
            self.is_popped()
        );
    }
}
