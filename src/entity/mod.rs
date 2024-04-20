use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};

pub mod board;

#[derive(Component)]
struct Bound {
    position: Vec2,
    scale: Vec2,
}

#[derive(Component)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn new(r: usize, c: usize) -> Self {
        Self { row: r, col: c }
    }
}

#[derive(Resource)]
pub struct BoardOptions {
    pub width: usize,
    pub height: usize,
    pub mines_count: usize,
}
