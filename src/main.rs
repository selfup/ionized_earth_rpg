use bevy::{app::startup_stage, prelude::*};

extern crate rand;
use rand::Rng;

mod constants;
mod entities;
mod resources;
mod systems;
mod utils;

use constants::*;
use entities::*;
use resources::*;
use systems::*;
use utils::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(startup_stage::PRE_STARTUP, init_state.system())
        .add_startup_system(camera_setup.system())
        .add_system(setup.system())
        .add_startup_system(add_random_building_blocks.system())
        .add_startup_system(player_setup.system())
        .add_system(camera_scale.system())
        .add_system(camera_movement.system())
        .add_system(animate_sprite_system.system())
        .add_system(player_movement.system())
        .add_resource(ClearColor(Color::rgb(BG_COLOR, BG_COLOR, BG_COLOR)))
        .run();
}

fn animate_sprite_system(
    time: Res<Time>,
    mut store: ResMut<Store>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Player)>,
) {
    let store_start_pow = store.start.pow(2);
    let store_end_pow = store.end.pow(2);

    for (mut timer, mut sprite, mut player) in query.iter_mut() {
        timer.tick(time.delta_seconds_f64() as f32);

        if (player.x - 16.0) as i32 == store_start_pow || (player.x + 16.0) as i32 == store_end_pow
        {
            store.updating = true;

            let new_start = store.start - 2;
            let new_end = store.end + 2;

            store.update_blocks(new_start, new_end);
        } else if (player.y - 16.0) as i32 == store_start_pow
            || (player.y + 16.0) as i32 == store_end_pow
        {
            store.updating = true;

            let new_start = store.start - 2;
            let new_end = store.end + 2;

            store.update_blocks(new_start, new_end);
        }

        /*
            [
                0,1,2,3,4, <- idle
                5,6,7,8,9, <- left
                10,11,12,13,14, <- right
                15,16,17,18,19 <- up
                20,21,22,23,24, <- down
            ]
        */

        if player.idle {
            if player.animation_index < 4 {
                player.animation_index += 1;
            } else {
                player.animation_index = 0;
            }
        } else if player.left {
            if player.animation_index < 9 {
                player.animation_index += 1;
            } else {
                player.animation_index = 5;
            }
        } else if player.right {
            if player.animation_index < 14 {
                player.animation_index += 1;
            } else {
                player.animation_index = 10;
            }
        } else if player.up {
            if player.animation_index < 19 {
                player.animation_index += 1;
            } else {
                player.animation_index = 15;
            }
        } else if player.down {
            if player.animation_index < 24 {
                player.animation_index += 1;
            } else {
                player.animation_index = 20;
            }
        }

        if timer.finished() {
            sprite.index = player.animation_index;
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Handle<TextureAtlas>, &mut Player)>,
) {
    let input_dir = get_input_dir(keyboard_input);

    for (mut transform, _handle_texture_atlas, mut player) in query.iter_mut() {
        let normalized_input_dir = (transform.rotation * input_dir.0).normalize();

        let mut velocity = NORMAL_VELOCITY;

        let x_dir = normalized_input_dir[0];
        let y_dir = normalized_input_dir[1];

        if input_dir.1 == "run" {
            velocity = RUNNING_VELOCITY;
        }

        if x_dir == -1.0 {
            player.x -= (1.0 * time.delta_seconds_f64() * velocity) as f32;
            player.set_all_to_false();
            player.left = true;
        } else if x_dir == 1.0 {
            player.x += (1.0 * time.delta_seconds_f64() * velocity) as f32;
            player.set_all_to_false();
            player.right = true;
        } else if y_dir == -1.0 {
            player.y -= (1.0 * time.delta_seconds_f64() * velocity) as f32;
            player.set_all_to_false();
            player.down = true;
        } else if y_dir == 1.0 {
            player.y += (1.0 * time.delta_seconds_f64() * velocity) as f32;
            player.set_all_to_false();
            player.up = true;
        } else {
            player.set_all_to_false();
            player.idle = true;
        }

        transform.translation = Vec3::new(player.x, player.y, player.z);
    }
}

fn add_random_building_blocks(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let building_block_001: Handle<Texture> = asset_server.load(BUILDING_BLOCK_001);

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
        let material: Handle<Texture>;

        if building_block.2 == 1 {
            material = building_block_001.clone();

            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    building_block.0 as f32,
                    building_block.1 as f32,
                    0.0,
                )),
                material: materials.add(material.into()),
                ..Default::default()
            });
        }
    }
}
