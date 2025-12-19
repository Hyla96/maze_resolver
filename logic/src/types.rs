#[derive(Debug, Clone)]
pub struct Tile {
    pub walkable: bool,
    pub is_goal: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Player {
    pub position: Position,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
}
