extern crate rand;
use rand::Rng;

use bevy::prelude::*;

use crate::constants::*;

#[derive(Debug, Clone, Resource)]
pub struct Store {
    pub start: i32,
    pub end: i32,
    pub grass_001: Handle<Image>,
    pub grass_002: Handle<Image>,
    pub blocks: Vec<(i32, i32, i8)>,
    pub updating: bool,
}

impl Store {
    pub fn new(start: i32, end: i32, asset_server: Res<AssetServer>) -> Self {
        let grass_001: Handle<Image> = asset_server.load(GRASS_001);
        let grass_002: Handle<Image> = asset_server.load(GRASS_002);

        let blocks = create_blocks(start, end);

        Self {
            start: blocks.1 .0,
            end: blocks.1 .1,
            grass_001,
            grass_002,
            blocks: blocks.0,
            updating: true,
        }
    }

    #[allow(dead_code)]
    pub fn update_blocks(&mut self, start: i32, end: i32) {
        self.blocks = create_blocks(self.start, self.end).0;

        self.start = start;
        self.end = end;
    }
}

pub fn create_blocks(start: i32, end: i32) -> (Vec<(i32, i32, i8)>, (i32, i32)) {
    let mut blocks: Vec<(i32, i32, i8)> = vec![];

    for x in start..end {
        let x_buffer = x * BLOCK_SIZE;

        for y in start..end {
            let y_buffer = y * BLOCK_SIZE;

            let mut range = rand::thread_rng();

            let grass_type = range.gen_range(0..2);

            blocks.push((x_buffer, y_buffer, grass_type));
        }
    }

    (blocks, (start, end))
}
