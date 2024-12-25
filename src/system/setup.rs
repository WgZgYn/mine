use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::constant::TILE_SIZE;
use crate::entity::board::model::BoardModel;
use crate::entity::board::Board;
use crate::entity::{BoardOptions, Coordinate};
use crate::system::event::MouseClickEvent;

pub fn setup_camera(mut commands: Commands, options: Res<BoardOptions>) {
    commands.spawn((
        Camera2d,
        {
            let mut projection = OrthographicProjection::default_2d();
            projection.scaling_mode = ScalingMode::AutoMin {
                min_width: (options.width as f32) * TILE_SIZE,
                min_height: (options.height as f32) * TILE_SIZE,
            };
            projection
        },
    ));
}

pub fn setup_board_model(options: Res<BoardOptions>, mut grid: ResMut<BoardModel>) {
    *grid = BoardModel::new(options.width, options.height, options.mines_count);
}

pub fn setup_board_view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid: Res<BoardModel>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("texture.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE as u32), 4, 4, None, None);
    let handle = texture_atlases.add(layout);
    let (w, h) = grid.size();
    let board_size = Vec2::new(w as f32, h as f32) * TILE_SIZE;
    let offset = (board_size - TILE_SIZE) / 2.0;

    commands
        .spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(board_size),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
            Board,
        ))
        .with_children(|sub_commands| {
            for y in 0..h {
                for x in 0..w {
                    let sp = (
                        Sprite::from_atlas_image(
                            texture.clone(),
                            TextureAtlas {
                                layout: handle.clone(),
                                index: 9,
                            },
                        ),
                        Transform::from_xyz(
                            TILE_SIZE * (x as f32) - offset.x,
                            offset.y - TILE_SIZE * (y as f32),
                            2.0,
                        ),
                        Coordinate::new(x, y)
                    );
                    sub_commands.spawn(sp).observe(|trigger: Trigger<Pointer<Down>>, query: Query<&Coordinate>, mut commands: Commands| {
                        if trigger.event.button == PointerButton::Middle {
                            return;
                        }
                        let left_button = trigger.event.button == PointerButton::Primary;
                        if let Ok(&coordinate) = query.get(trigger.entity()) {
                            commands.trigger(MouseClickEvent {
                                coordinate,
                                left_button,
                            });
                        }
                    });
                }
            }
        });
}
