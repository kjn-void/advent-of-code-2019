use std::ops::*;

pub const NORTH: Vec2D = Vec2D{x: 0, y: 1};
pub const SOUTH: Vec2D = Vec2D{x: 0, y: -1};
pub const WEST: Vec2D = Vec2D{x: -1, y: 0};
pub const EAST: Vec2D = Vec2D{x: 1, y: 0};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Vec2D {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Compass {
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq)]
pub enum Turn {
    Clockwise,
    Counterclockwise,
}

impl Vec2D {
    pub fn from(x: i32, y: i32) -> Vec2D {
        Vec2D{x: x, y: y}
    }

    pub fn compass(self: Vec2D, other: Vec2D) -> Compass {
        match other - self {
            Vec2D{x: 0, y: 1} => Compass::North,
            Vec2D{x: 0, y: -1} => Compass::South,
            Vec2D{x: -1, y: 0} => Compass::West,
            Vec2D{x: 1, y: 0} => Compass::East,
            _ => panic!("Does not result in an unit vector")
        }
    }

    pub fn from_dir(dir: Dir) -> Vec2D {
        match dir {
            Dir::Up => Vec2D::from(1, 0),
            Dir::Down => Vec2D::from(-1, 0),
            Dir::Left => Vec2D::from(0, -1),
            Dir::Right => Vec2D::from(0, 1),
        }
    }

    pub fn turn(self: Vec2D, turn: Turn) -> Vec2D {
        if turn == Turn::Clockwise {
            Vec2D::from(self.y, -self.x)
        } else {
            Vec2D::from(-self.y, self.x)
        }
    }
} 

impl Default for Vec2D {
    fn default() -> Vec2D {
        Vec2D::from(0, 0)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2D{ x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2D{ x: self.x - other.x, y: self.y - other.y }
    }
}