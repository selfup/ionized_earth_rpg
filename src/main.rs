extern crate rand;
use rand::Rng;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;

const BLOCK_SIZE: i32 = 16;

const BG_COLOR: f32 = 0.04;
const MAIN_SCENE_START_SIZE: i32 = -20;
const MAIN_SCENE_END_SIZE: i32 = 20;
const MAIN_CAMERA_SCALE: f32 = 0.2;

const GRASS_001: &str = "world/grass-001.png";
const GRASS_002: &str = "world/grass-002.png";
const BUILDING_BLOCK_001: &str = "world/building-block-001.png";
const PLAYER: &str = "characters/player/player-001.png";

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_system(add_random_building_blocks.system())
        .add_system(camera_scale.system())
        .add_system(player_movement.system())
        .add_resource(ClearColor(Color::rgb(BG_COLOR, BG_COLOR, BG_COLOR)))
        .run();
}

struct CameraMatcher();

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .with(CameraMatcher());

    let grass001: Handle<Texture> = asset_server.load(GRASS_001);
    let grass002: Handle<Texture> = asset_server.load(GRASS_002);

    let mut blocks: Vec<(i32, i32, i8)> = vec![];

    for x in MAIN_SCENE_START_SIZE..MAIN_SCENE_END_SIZE {
        let x_buffer = x * BLOCK_SIZE;

        for y in MAIN_SCENE_START_SIZE..MAIN_SCENE_END_SIZE {
            let y_buffer = y * BLOCK_SIZE;

            let mut range = rand::thread_rng();

            let grass_type = range.gen_range(0..4);

            blocks.push((x_buffer, y_buffer, grass_type));
        }
    }

    for block in blocks {
        let material: Handle<Texture>;

        if block.2 == 1 {
            material = grass002.clone();
        } else {
            material = grass001.clone();
        }

        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(block.0 as f32, block.1 as f32, 0.0)),
            material: materials.add(material.into()),
            ..Default::default()
        });
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

fn camera_scale(mut query: Query<(&mut Transform, &mut CameraMatcher)>) {
    for (mut transform, _camera) in query.iter_mut() {
        transform.scale = Vec3::new(MAIN_CAMERA_SCALE, MAIN_CAMERA_SCALE, 1.0);
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraMatcher)>,
) {
    let input_dir = get_input_dir(keyboard_input);

    // TODO: figure out how to center player on screen and move with camera movement

    if input_dir.length() > 0. {
        for (mut transform, _camera) in query.iter_mut() {
            let input_dir = (transform.rotation * input_dir).normalize();

            transform.translation += input_dir * (time.delta_seconds_f64() * 50.0) as f32;
        }
    }
}

fn get_input_dir(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut input_dir = Vec3::default();

    if keyboard_input.pressed(KeyCode::W) {
        let up = Vec3::unit_y();

        input_dir += up;

        return input_dir;
    }

    if keyboard_input.pressed(KeyCode::A) {
        let left = Vec3::unit_x();

        input_dir -= left;

        return input_dir;
    }

    if keyboard_input.pressed(KeyCode::S) {
        let down = Vec3::unit_y();

        input_dir -= down;

        return input_dir;
    }

    if keyboard_input.pressed(KeyCode::D) {
        let right = Vec3::unit_x();

        input_dir += right;

        return input_dir;
    }

    input_dir
}
