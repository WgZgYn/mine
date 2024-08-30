use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::constant::*;
use crate::entity::board::model::BoardModel;
use crate::entity::{BoardOptions, GameState};
// use crate::entity::sprite::SpriteContainer;
use crate::system::event::{TileEvent, UncoverMine};
use system::event::handle::handle_tile_event;
use crate::system::event::handle::handle_uncover_mine_event;
use crate::system::input::handle_input;
use crate::system::setup::*;

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
        // .init_resource::<SpriteContainer>()
        .insert_resource(BoardOptions {
            width: 40,
            height: 40,
            mines_count: 300,
        })
        .add_systems(
            Startup,
            (setup_camera, setup_board_model, setup_board_view).chain(),
        )
        .add_systems(Update, (handle_input, handle_tile_event, handle_uncover_mine_event))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}
