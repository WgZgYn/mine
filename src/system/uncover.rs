use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::entity::board::cell::CellType;
use crate::entity::board::model::BoardModel;
use crate::entity::Coordinate;
use crate::system::event::{EventType, TileEvent};

pub fn handle_tile_event(
    board: Res<BoardModel>,
    mut tile_ev: EventReader<TileEvent>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
) {
    for ev in tile_ev.read() {
        match ev.1 {
            EventType::Uncover => {
                let mut st = HashSet::new();
                board.uncover_tiles(ev.0, &mut st);
                for (mut tile, co) in &mut tiles {
                    if st.contains(co) {
                        tile.index = match board.get(co.y, co.x) {
                            CellType::Empty => 0,
                            CellType::Number(i) => i,
                            CellType::Mine => 10
                        };
                    }
                }
            },
            EventType::Flag => {
                for (mut tile, co) in &mut tiles {
                    if *co == ev.0 {
                        tile.index = 13;
                        break;
                    }
                }
            }
        }
    }
}