use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::constant::*;
use crate::entity::board::model::BoardModel;
use crate::entity::BoardOptions;
use crate::system::event::TileEvent;
use crate::system::input::handle_input;
use crate::system::setup::*;
use crate::system::uncover::handle_tile_event;

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
        .add_plugins(WorldInspectorPlugin::new())
        .add_event::<TileEvent>()
        .init_resource::<BoardModel>()
        .register_type::<BoardModel>()
        .insert_resource(BoardOptions {
            width: 20,
            height: 20,
            mines_count: 50,
        })
        .add_systems(Startup, (setup_camera, setup_board_model, setup_board_view).chain())
        .add_systems(Update, (handle_input, handle_tile_event))
        .run();
}

