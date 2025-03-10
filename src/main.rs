use crate::constant::{APP_TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::entity::board::model::BoardModel;
use crate::entity::{BoardOptions, GameState};
use crate::system::event::handle::{on_tile_event, on_uncover_mine_event};
use crate::system::event::{TileEvent, UncoverMine};
use crate::system::input::on_cell_click;
use crate::system::setup::*;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};

mod constant;
mod entity;
mod system;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: APP_TITLE.into(),
                        present_mode: PresentMode::AutoVsync,
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_event::<TileEvent>()
        .add_event::<UncoverMine>()
        .init_resource::<BoardModel>()
        .init_resource::<GameState>()
        .insert_resource(BoardOptions {
            width: 40,
            height: 40,
            mines_count: 300,
        })
        .add_systems(Startup, (setup_camera, setup_board_model, setup_board_view).chain())
        .add_systems(Update, move_camera)
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(LogDiagnosticsPlugin::default())
        .add_observer(on_cell_click)
        .add_observer(on_uncover_mine_event)
        .add_observer(on_tile_event)
        .run();
}

fn move_camera(
    motion: Res<AccumulatedMouseMotion>,
    scale: Res<AccumulatedMouseScroll>,
    mut transform: Single<&mut Transform, With<Camera>>,
    btn: Res<ButtonInput<KeyCode>>,
    mus: Res<ButtonInput<MouseButton>>,
) {
    if motion.delta != Vec2::ZERO && btn.pressed(KeyCode::AltLeft) || mus.pressed(MouseButton::Middle) {
        // info!("move!");
        transform.translation.x -= motion.delta.x;
        transform.translation.y += motion.delta.y;
    }

    if scale.delta != Vec2::ZERO && btn.pressed(KeyCode::ControlLeft) {
        // info!("scale!");
        if scale.delta.y == -1. {
            transform.scale *= 1.25;
        } else if scale.delta.y == 1. {
            transform.scale *= 0.9;
        }
    }
}