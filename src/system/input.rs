use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::vec2;
use bevy::prelude::*;
use crate::constant::TILE_SIZE;
use crate::entity::{BoardOptions, Coordinate};
use crate::system::event::{EventType, TileEvent};

pub fn handle_input(
    windows: Query<&mut Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_ev: EventWriter<TileEvent>,
    options: Res<BoardOptions>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    let (w, h) = (options.width, options.height);

    for event in button_evr.read() {
        // 捕获松开鼠标的事件
        if event.state == ButtonState::Released {
            // 将鼠标点击的位置转换为相机中的实际位置
            let position = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor));

            let position = match position {
                None => break,
                Some(p) => p
            };

            if position.x.abs() < TILE_SIZE * w as f32 / 2. && position.y.abs() < TILE_SIZE * h as f32 / 2. {
                let lb = position / TILE_SIZE + vec2(w as f32 / 2., h as f32 / 2.);
                let co = Coordinate::new(lb.x as usize, (h as f32 - lb.y) as usize);
                println!("x, y: {:?}", co);
                match event.button {
                    MouseButton::Left => { tile_ev.send(TileEvent(co, EventType::Uncover)); }
                    MouseButton::Right => { tile_ev.send(TileEvent(co, EventType::Flag)) }
                    _ => {}
                }
            }
        }
    }
}


// fn in_transform(transform: &GlobalTransform, position: Vec2) -> bool {
//     let inx =
//         transform.translation().x - 8.0 < position.x &&
//             transform.translation().x + 8.0 > position.x;
//     let iny =
//         transform.translation().y - 8.0 < position.y &&
//             transform.translation().y + 8.0 > position.y;
//     inx && iny
// }