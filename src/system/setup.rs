use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::constant::TILE_SIZE;
use crate::entity::board::model::BoardModel;
use crate::entity::{BoardOptions, Coordinate};
use crate::entity::board::Board;

pub fn setup_camera(mut commands: Commands, options: Res<BoardOptions>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: (options.width as f32) * TILE_SIZE,
        min_height: (options.height as f32) * TILE_SIZE,
    };
    commands.spawn(camera);
}

pub fn setup_board_model(options: Res<BoardOptions>, mut grid: ResMut<BoardModel>) {
    *grid = BoardModel::new(options.width, options.height, options.mines_count);
    grid.print()
}

// pub fn setup_texture(
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let texture = asset_server.load("texture.png");
//     let texture_atlas = TextureAtlas::from_grid(texture, Vec2::ONE * TILE_SIZE, 4, 4, None, None);
//     let handle = texture_atlases.add(texture_atlas);
// }

pub fn setup_board_view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid: Res<BoardModel>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("texture.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::ONE * TILE_SIZE, 4, 4, None, None);
    let handle = texture_atlases.add(texture_atlas);

    let (w, h) = grid.size();
    let board_size = Vec2::new(w as f32, h as f32) * TILE_SIZE;
    let offset = (board_size - TILE_SIZE) / 2.0;

    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(board_size),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }, Board))
        .with_children(|commands| {
            for y in 0..h {
                for x in 0..w {
                    commands.spawn((
                        SpriteSheetBundle {
                            texture_atlas: handle.clone(),
                            sprite: TextureAtlasSprite {
                                index: 9,
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                TILE_SIZE * (x as f32) - offset.x,
                                offset.y - TILE_SIZE * (y as f32),
                                2.0,
                            ),
                            ..default()
                        },
                        Coordinate::new(x, y)
                    ));
                }
            }
        });
}
