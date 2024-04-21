use bevy::prelude::{Component, Resource};

pub mod board;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}


impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y}
    }
}

#[derive(Resource)]
pub struct BoardOptions {
    pub width: usize,
    pub height: usize,
    pub mines_count: usize,
}
