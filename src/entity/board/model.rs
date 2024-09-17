use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::constant::TOWARDS;
use crate::entity::board::cell::{CellState, CellType};
use crate::entity::Coordinate;
use crate::system::event::TileEvent;

#[derive(Resource, Default)]
pub struct BoardModel {
    board: Vec<Vec<CellType>>,
    state: Vec<Vec<CellState>>,
    height: usize,
    width: usize,
}

impl BoardModel {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        let mut res = Self {
            board: vec![vec![CellType::Empty; width]; height],
            state: vec![vec![CellState::Covered; width]; height],
            height,
            width,
        };
        res.init(mines);
        res
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_type(&self, r: usize, c: usize) -> CellType {
        self.board[r][c]
    }
    pub fn get_state(&self, r: usize, c: usize) -> CellState {
        self.state[r][c]
    }
    pub fn set_state(&mut self, r: usize, c: usize, state: CellState) {
        self.state[r][c] = state;
    }
    pub fn uncover_tiles(&self, r: usize, c: usize, visit: &mut HashSet<Coordinate>) {
        let tiles = Coordinate::new(c, r);
        if visit.contains(&tiles) || self.get_state(r, c) == CellState::Uncovered {
            return;
        }
        match self.get_type(r, c) {
            CellType::Empty => {
                visit.insert(tiles);
            }
            _ => {
                visit.insert(tiles);
                return;
            }
        }
        let (w, h) = self.size();
        for (x, y) in TOWARDS {
            let x = c as i32 + x;
            let y = r as i32 + y;
            if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
                continue;
            }
            self.uncover_tiles(y as usize, x as usize, visit);
        }
    }

    pub fn click(&self, r: usize, c: usize, tile_ev: &mut EventWriter<TileEvent>, left_click: bool) {
        let mut st = HashSet::new();

        match (left_click, self.get_state(r, c)) {
            // it will send TileEvent::Uncover
            (true, CellState::Covered) => {
                self.uncover_tiles(r, c, &mut st);
                tile_ev.send(TileEvent::Uncover(st));
            }

            // it will send TileEvent::FlagOne
            (false, CellState::Covered | CellState::Flag) => {
                tile_ev.send(TileEvent::FlagOne(Coordinate::new(c, r)));
            }

            // it will send TileEvent::Uncover or TileEvent::Flag
            (true, CellState::Uncovered) => {
                if let CellType::Number(i) = self.get_type(r, c) {
                    let (covered, f) = self.count_state(r, c);
                    println!("covered: {:?}, flag: {}, i: {}", covered, f, i);
                    if i == f {
                        for co in covered {
                            self.uncover_tiles(co.y, co.x, &mut st);
                        }
                        tile_ev.send(TileEvent::Uncover(st));
                    } else if covered.len() + f == i {
                        tile_ev.send(TileEvent::Flag(covered));
                    }
                }
            }
            
            _ => {}
        }
    }

    fn count_state(&self, r: usize, c: usize) -> (HashSet<Coordinate>, usize) {
        let (mut covered, mut flag) = (HashSet::new(), 0);
        for i in (r.max(1)-1)..=(r+1).min(self.height-1) {
            for j in (c.max(1)-1)..=(c+1).min(self.width-1) {
                match self.get_state(i, j) {
                    CellState::Covered => { covered.insert(Coordinate::new(j, i)); },
                    CellState::Flag => { flag += 1; },
                    _ => {}
                }
            }
        }
        (covered, flag)
    }

    fn init_mines(&mut self, mut count: usize) {
        // assert!(count <= self.height * self.width);
        // if count >= self.width * self.height / 2 {
        //     info!(
        //         "the amount of the mines {} is more than half of the cells",
        //         count
        //     );
        // }
        while count > 0 {
            let r = rand::random::<usize>() % self.height;
            let c = rand::random::<usize>() % self.width;
            if self.board[r][c] == CellType::Empty {
                count -= 1;
                self.board[r][c] = CellType::Mine;
            }
        }
        // info!("the board was just initialized");
    }
    fn count_mines(&mut self) {
        fn count_mine(grid: &Vec<Vec<CellType>>, r: usize, c: usize) -> usize {
            let mut res = 0;
            for i in r.max(1) - 1..=(r + 1).min(grid.len() - 1) {
                for j in c.max(1) - 1..=(c + 1).min(grid[i].len() - 1) {
                    if grid[i][j] == CellType::Mine {
                        res += 1;
                    }
                }
            }
            res
        }

        for r in 0..self.height {
            for c in 0..self.width {
                if self.board[r][c] == CellType::Empty {
                    let count = count_mine(&self.board, r, c);
                    if count != 0 {
                        self.board[r][c] = CellType::Number(count);
                    }
                }
            }
        }
    }
    fn init(&mut self, mines: usize) {
        self.init_mines(mines);
        self.count_mines();
    }

    pub fn is_mine_at(&self, r: usize, c: usize) -> bool {
        self.board[r][c] == CellType::Mine
    }

    pub fn print(&self) {
        let board = self
            .board
            .iter()
            .map(|v| {
                v.iter()
                    .map(|c| match c {
                        CellType::Empty => "0".to_string(),
                        CellType::Mine => "-1".to_string(),
                        CellType::Number(i) => i.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join("\t")
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", board);
    }
}
