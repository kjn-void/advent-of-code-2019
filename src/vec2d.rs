use std::ops::*;

pub const NORTH: Vec2D = Vec2D { x: 0, y: 1 };
pub const SOUTH: Vec2D = Vec2D { x: 0, y: -1 };
pub const WEST: Vec2D = Vec2D { x: -1, y: 0 };
pub const EAST: Vec2D = Vec2D { x: 1, y: 0 };

pub const UP: Vec2D = Vec2D { x: 0, y: -1 };
pub const DOWN: Vec2D = Vec2D { x: 0, y: 1 };
pub const LEFT: Vec2D = Vec2D { x: -1, y: 0 };
pub const RIGHT: Vec2D = Vec2D { x: 1, y: 0 };

pub type Coord = i32;
pub type Distance = u32;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd)]
pub struct Vec2D {
    x: Coord,
    y: Coord,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum Compass {
    North,
    South,
    West,
    East,
}

impl Eq for Compass { }

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    Right,
    Left,
}

impl Turn {
    pub fn to_str(&self) -> &'static str {
        if *self == Turn::Right { "R" } else { "L" }
    }
}

impl Compass {
    pub fn mirror(&self) -> Compass {
        match *self {
            Compass::East => Compass::West,
            Compass::North => Compass::South,
            Compass::South => Compass::North,
            Compass::West => Compass::East,
        }
    }
}

impl Dir {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Dir::Down => "v",
            Dir::Up => "^",
            Dir::Left => "<",
            Dir::Right => ">",
        }
    }

    pub fn turn(&self, turn: Turn) -> Dir {
        if turn == Turn::Right {
            match *self {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            }
        } else {
            match *self {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            }
        }
    }
}

impl Vec2D {
    pub fn x(&self) -> Coord {
        self.x
    }

    pub fn y(&self) -> Coord {
        self.y
    }

    pub fn from(x: i32, y: i32) -> Vec2D {
        Vec2D { x: x, y: y }
    }

    pub fn from_dir(dir: Dir) -> Vec2D {
        match dir {
            Dir::Up => UP,
            Dir::Down => DOWN,
            Dir::Left => LEFT,
            Dir::Right => RIGHT,
        }
    }

    pub fn compass(self: Vec2D, other: Vec2D) -> Compass {
        match other - self {
            Vec2D { x: 0, y: 1 } => Compass::North,
            Vec2D { x: 0, y: -1 } => Compass::South,
            Vec2D { x: -1, y: 0 } => Compass::West,
            Vec2D { x: 1, y: 0 } => Compass::East,
            _ => panic!("Does not result in an unit vector"),
        }
    }

    pub fn turn(self: Vec2D, turn: Turn) -> Vec2D {
        if turn == Turn::Right {
            Vec2D::from(self.y, -self.x)
        } else {
            Vec2D::from(-self.y, self.x)
        }
    }

    pub fn manhattan_distance(&self, pos: Vec2D) -> Distance {
        ((self.x - pos.x).abs() + (self.y - pos.y).abs()) as Distance
    }

    pub fn step_n(&mut self, n: Distance, dir: Dir) -> Vec<Vec2D> {
        let mut path = Vec::new();
        let dir_vec = Vec2D::from_dir(dir);
        for _ in 0..n {
            *self += dir_vec;
            path.push(*self);
        }
        path
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
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
