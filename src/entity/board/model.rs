use bevy::log::info;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::*;

use crate::entity::board::cell::{CellState, CellType};

#[derive(Resource, Debug, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct BoardModel {
    data: Vec<Vec<CellType>>,
    covered: Vec<Vec<CellState>>,
    height: usize,
    width: usize,
}

impl BoardModel {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        let mut res = Self {
            data: vec![vec![CellType::Empty; width]; height],
            covered: vec![vec![CellState::Covered; width]; height],
            height,
            width,
        };
        res.init(mines);
        res
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    fn init_mines(&mut self, mut count: usize) {
        assert!(count <= self.height * self.width);
        if count >= self.width * self.height / 2 {
            info!(
                "the amount of the mines {} is more than half of the cells",
                count
            );
        }
        while count > 0 {
            let r = rand::random::<usize>() % self.height;
            let c = rand::random::<usize>() % self.width;
            if self.data[r][c] == CellType::Empty {
                count -= 1;
                self.data[r][c] = CellType::Mine;
            }
        }
        info!("the board was just initialized");
    }
    fn count_mines(&mut self) {
        // the (r, c) should be empty.
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
                if self.data[r][c] == CellType::Empty {
                    let count = count_mine(&self.data, r, c);
                    if count != 0 {
                        self.data[r][c] = CellType::Number(count);
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
        self.data[r][c] == CellType::Mine
    }

    pub fn print(&self) {
        let board = self
            .data
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
