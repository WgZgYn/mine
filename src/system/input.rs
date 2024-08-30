use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::vec2;
use bevy::prelude::*;

use crate::constant::TILE_SIZE;
use crate::entity::{BoardOptions, Coordinate};
use crate::entity::board::model::BoardModel;
use crate::system::event::TileEvent;

pub fn handle_input(
    windows: Query<&mut Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_ev: EventWriter<TileEvent>,
    options: Res<BoardOptions>,
    board: Res<BoardModel>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    let (w, h) = (options.width, options.height);

    for event in button_evr.read() {
        if event.state == ButtonState::Released {
            let position = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor));

            let position = match position {
                None => break,
                Some(p) => p,
            };

            if position.x.abs() < TILE_SIZE * w as f32 / 2.
                && position.y.abs() < TILE_SIZE * h as f32 / 2.
            {
                let lb = position / TILE_SIZE + vec2(w as f32 / 2., h as f32 / 2.);
                let co = Coordinate::new(lb.x as usize, (h as f32 - lb.y) as usize);
                info!("{:?}", co);
                match event.button {
                    MouseButton::Left => board.click(co.y, co.x, &mut tile_ev, true),
                    MouseButton::Right => board.click(co.y, co.x, &mut tile_ev, false),
                    _ => {}
                }
            }
        }
    }
}