use bevy::prelude::*;

use crate::entity::board::cell::{CellState, CellType};
use crate::entity::board::model::BoardModel;
use crate::entity::{Coordinate, GameState};
use crate::system::event::{TileEvent, UncoverMine};
pub fn on_tile_event(
    trigger: Trigger<TileEvent>,
    mut board: ResMut<BoardModel>,
    mut tiles: Query<(&mut Sprite, &Coordinate)>,
    mut commands: Commands,
) {
    let ev = trigger.event();
    for (mut sprite, co) in &mut tiles {
        if sprite.texture_atlas.is_none() { continue; }
        let tile = sprite.texture_atlas.as_mut().unwrap();
        match ev {
            TileEvent::Uncover(st) => {
                if st.contains(co) {
                    match board.get_state(co.y, co.x) {
                        CellState::Covered => {
                            tile.index = match board.get_type(co.y, co.x) {
                                CellType::Empty => 0,
                                CellType::Number(i) => i as usize,
                                CellType::Mine => {
                                    commands.trigger(UncoverMine(*co));
                                    break;
                                }
                            };
                            board.set_state(co.y, co.x, CellState::Uncovered);
                        }
                        _ => {}
                    }
                }
            }

            TileEvent::Flag(st) => {
                if st.contains(co) {
                    if let CellState::Uncovered = board.get_state(co.y, co.x) { break; }
                    flag_flip(&mut board, &mut tile.index, co.x, co.y);
                }
            }

            TileEvent::FlagOne(st) => {
                if st == co {
                    if let CellState::Uncovered = board.get_state(co.y, co.x) { break; }
                    flag_flip(&mut board, &mut tile.index, co.x, co.y);
                    break;
                }
            }
            _ => {}
        }
    }
}

fn flag_flip(
    board: &mut ResMut<BoardModel>,
    index: &mut usize,
    x: usize, y: usize,
) {
    match board.get_state(y, x) {
        CellState::Covered => {
            *index = 13;
            board.set_state(y, x, CellState::Flag);
        }

        CellState::Flag => {
            *index = 9;
            board.set_state(y, x, CellState::Covered);
        }

        CellState::Uncovered => {}
    }
}

pub fn on_uncover_mine_event(
    mut trigger: Trigger<UncoverMine>,
    mut board: ResMut<BoardModel>,
    mut state: ResMut<GameState>,
    mut tiles: Query<(&mut Sprite, &Coordinate)>,
) {
    let UncoverMine(Coordinate { x, y }) = *trigger.event();
    for (mut sp, &Coordinate { x: c, y: r }) in &mut tiles {
        if let Some(sp) = &mut sp.texture_atlas {
            if (c, r) == (x, y) {
                sp.index = 11;
                continue;
            }

            if CellType::Mine == board.get_type(r, c) {
                sp.index = 10;
            }

            if CellState::Flag == board.get_state(r, c) && CellType::Mine != board.get_type(r, c) {
                sp.index = 12;
            }
        }
    }
    info!("Game Over!");
    *state = GameState::Ending;
}