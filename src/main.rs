extern crate rand;
use rand::Rng;

use bevy::prelude::*;

const BLOCK_SIZE: i32 = 16;

const BG_COLOR: f32 = 0.04;
const MAIN_SCENE_START_SIZE: i32 = -20;
const MAIN_SCENE_END_SIZE: i32 = 20;
const MAIN_CAMERA_SCALE: f32 = 0.2;

const PLAYER_IDLE: &str = "player-idle-sheet.png";
const GRASS_001: &str = "world/grass-001.png";
const GRASS_002: &str = "world/grass-002.png";
const BUILDING_BLOCK_001: &str = "world/building-block-001.png";

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(add_random_building_blocks.system())
        .add_startup_system(player_setup.system())
        .add_system(camera_scale.system())
        .add_system(camera_movement.system())
        .add_system(animate_sprite_system.system())
        .add_system(player_movement.system())
        .add_resource(ClearColor(Color::rgb(BG_COLOR, BG_COLOR, BG_COLOR)))
        .run();
}

struct CameraMatcher();

#[derive(Debug, Copy, Clone)]
struct Player {
    x: f32,
    y: f32,
    z: f32,
}

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

fn player_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(PLAYER_IDLE);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = Player {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(player);
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds_f64() as f32);

        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let input_dir = get_input_dir(keyboard_input);

    for (mut transform, mut player) in query.iter_mut() {
        let input_dir = (transform.rotation * input_dir).normalize();

        let velocity = 50.0;
        let x_dir = input_dir[0];
        let y_dir = input_dir[1];

        if x_dir == -1.0 {
            player.x -= (1.0 * time.delta_seconds_f64() * velocity) as f32;
        }

        if x_dir == 1.0 {
            player.x += (1.0 * time.delta_seconds_f64() * velocity) as f32;
        }

        if y_dir == -1.0 {
            player.y -= (1.0 * time.delta_seconds_f64() * velocity) as f32;
        }

        if y_dir == 1.0 {
            player.y += (1.0 * time.delta_seconds_f64() * velocity) as f32;
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

fn camera_scale(mut query: Query<(&mut Transform, &mut CameraMatcher)>) {
    for (mut transform, _camera) in query.iter_mut() {
        transform.scale = Vec3::new(MAIN_CAMERA_SCALE, MAIN_CAMERA_SCALE, 1.0);
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraMatcher)>,
) {
    let input_dir = get_input_dir(keyboard_input);

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
