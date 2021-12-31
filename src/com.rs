use bevy::prelude::*;

pub const ARENA: (u32, u32) = (11, 11);

pub struct SnakeHead {
    pub direction: Direction,
}

pub struct SnakeSegment;

pub struct Food;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position(pub i32, pub i32);

pub struct Size(pub Vec2);

impl Size {
    pub fn square(x: f32) -> Self {
        Size(Vec2::new(x, x))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        }
    }
}

