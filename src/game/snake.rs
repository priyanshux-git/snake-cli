use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

pub struct Snake {
    pub body: VecDeque<(u16, u16)>,
    pub hashed_body: HashSet<(u16, u16)>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: VecDeque::from([(0, 0), (1, 0)]),
            hashed_body: HashSet::from([(0, 0), (1, 0)]),
            direction: Direction::Right,
        }
    }
}
