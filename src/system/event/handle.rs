use bevy::prelude::*;

use crate::entity::board::cell::{CellState, CellType};
use crate::entity::board::model::BoardModel;
use crate::entity::{Coordinate, GameState};
use crate::system::event::{TileEvent, UncoverMine};

pub fn handle_tile_event(
    mut board: ResMut<BoardModel>,
    mut tile_ev: EventReader<TileEvent>,
    mut tile_mine: EventWriter<UncoverMine>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
) {
    for ev in tile_ev.read() {
        match ev {
            TileEvent::Uncover(st) => {
                for (mut tile, co) in &mut tiles {
                    if st.contains(co) {
                        match board.get_state(co.y, co.x) {
                            CellState::Covered => {
                                tile.index = match board.get_type(co.y, co.x) {
                                    CellType::Empty => 0,
                                    CellType::Number(i) => i,
                                    CellType::Mine => {
                                        tile_mine.send(UncoverMine(*co));
                                        break;
                                    },
                                };
                                board.set_state(co.y, co.x, CellState::Uncovered);
                            }
                            _ => {}
                        }
                    }
                }
            }

            TileEvent::Flag(st) => {
                for (mut tile, co) in &mut tiles {
                    if st.contains(co) {
                        if let CellState::Uncovered = board.get_state(co.y, co.x) { break; }
                        flag_flip(&mut board, &mut tile.index, co.x, co.y);
                    }
                }
            }

            TileEvent::FlagOne(st) => {
                for (mut tile, co) in &mut tiles {
                    if st == co {
                        if let CellState::Uncovered = board.get_state(co.y, co.x) { break; }
                        flag_flip(&mut board, &mut tile.index, co.x, co.y);
                        break;
                    }
                }
            }
            _ => {}
        }
    }
}

fn flag_flip(
    board: &mut ResMut<BoardModel>,
    index: &mut usize,
    x: usize, y: usize
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

        CellState::Uncovered => {},
    }
}

pub fn handle_uncover_mine_event(
    mut board: ResMut<BoardModel>,
    mut state: ResMut<GameState>,
    mut tile_mine: EventReader<UncoverMine>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
) {
    for &UncoverMine(co) in tile_mine.read() {
        for (mut sp, &Coordinate{ x: c, y: r }) in &mut tiles {
            if Coordinate::new(c, r) == co {
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
        info!("Game Over!");
        *state = GameState::Ending;
        break;
    }

}