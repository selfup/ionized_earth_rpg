extern crate rand;
use rand::Rng;

use bevy::prelude::*;

use crate::constants::*;

pub fn add_random_building_blocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let building_block_001: Handle<Image> = asset_server.load(BUILDING_BLOCK_001);

    let mut building_blocks: Vec<(i32, i32, i8)> = vec![];

    for x in MAIN_SCENE_START_SIZE..MAIN_SCENE_END_SIZE {
        let x_buffer = x * BLOCK_SIZE;

        for y in MAIN_SCENE_START_SIZE..MAIN_SCENE_END_SIZE {
            let y_buffer = y * BLOCK_SIZE;

            let mut range = rand::thread_rng();

            let grass_type = range.gen_range(0..32);

            building_blocks.push((x_buffer, y_buffer, grass_type));
        }
    }

    for building_block in building_blocks {
        let material: Handle<Image>;

        if building_block.2 == 1 {
            material = building_block_001.clone();

            commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    building_block.0 as f32,
                    building_block.1 as f32,
                    1.0,
                )),
                texture: material,
                ..Default::default()
            });
        }
    }
}
