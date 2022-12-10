use std::collections::VecDeque;
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub direction: Direction,
    pub count: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct State {
    pub head: Coordinate,
    pub tail: Coordinate,
    pub moves: VecDeque<Move>,
}

impl Direction {
    pub fn from_string(direction: &str) -> Direction {
        match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction: {}", direction),
        }
    }
}

impl Move {
    pub fn consume_step(&self) -> Option<Move> {
        if self.count > 1 {
            Some(Move {
                direction: self.direction.clone(),
                count: self.count - 1,
            })
        } else {
            None
        }
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn zero() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }

    pub fn subtract(&self, other: &Coordinate) -> Coordinate {
        Coordinate::new(other.x - self.x, other.y - self.y)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn in_one_radius(&self) -> bool {
        self.x.abs() <= 1 && self.y.abs() <= 1
    }

    pub fn ones_vector(&self) -> Coordinate {
        Coordinate::new(
            if self.x != 0 {
                self.x / self.x.abs()
            } else {
                0
            },
            if self.y != 0 {
                self.y / self.y.abs()
            } else {
                0
            },
        )
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x + other.x, self.y + other.y)
    }
}
