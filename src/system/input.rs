use bevy::prelude::*;

use crate::entity::board::model::BoardModel;
use crate::entity::Coordinate;
use crate::system::event::{MouseClickEvent, TileEvent};


pub fn on_cell_click(
    trigger: Trigger<MouseClickEvent>,
    board: Res<BoardModel>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let MouseClickEvent { coordinate: Coordinate { x, y }, left_button } = *event;
    if let Some(event) = board.click(y, x, left_button) {
        commands.trigger(event);
    }
}