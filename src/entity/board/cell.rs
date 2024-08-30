use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum CellType {
    #[default]
    Empty,
    Mine,
    Number(usize),
}

#[derive(Component, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum CellState {
    #[default]
    Covered,
    Uncovered,
    Flag,
}
