use bevy::prelude::Event;
use crate::entity::Coordinate;

#[derive(Copy, Clone)]
pub enum EventType {
    Uncover,
    Flag,
}

#[derive(Event)]
pub struct TileEvent(pub Coordinate, pub EventType);