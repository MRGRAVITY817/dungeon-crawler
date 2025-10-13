use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let x = (i as i32) % SCREEN_WIDTH;
            let y = (i as i32) / SCREEN_WIDTH;
            match tile {
                TileType::Floor => {
                    ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                }
                TileType::Wall => {
                    ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                }
            }
        }
    }
}

/// Convert x,y coordinates to a single index in the tiles vector
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
