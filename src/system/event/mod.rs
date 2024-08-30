pub mod handle;

use bevy::prelude::Event;
use bevy::utils::HashSet;

use crate::entity::Coordinate;

#[derive(Event)]
pub enum TileEvent {
    Uncover(HashSet<Coordinate>),
    Flag(HashSet<Coordinate>),
    FlagOne(Coordinate),
    Hover(Coordinate),
}

#[derive(Event)]
pub struct UncoverMine(pub Coordinate);