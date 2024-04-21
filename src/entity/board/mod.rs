use bevy::prelude::Component;

pub mod cell;
pub mod model;

#[derive(Component, Copy, Clone)]
pub struct Board;
