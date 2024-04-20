use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::constant::*;
use crate::entity::board::model::BoardModel;
use crate::entity::BoardOptions;
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
        .add_plugins(WorldInspectorPlugin::new())
        .init_resource::<BoardModel>()
        .register_type::<BoardModel>()
        .insert_resource(BoardOptions {
            width: 40,
            height: 40,
            mines_count: 500,
        })
        .add_systems(Startup, (setup_camera, setup_board_model, setup_board_view).chain())
        .run();
}

