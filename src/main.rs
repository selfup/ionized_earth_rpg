extern crate rand;
use rand::Rng;

use bevy::{app::startup_stage, prelude::*};

const BLOCK_SIZE: i32 = 16;

const BG_COLOR: f32 = 0.04;
const MAIN_SCENE_START_SIZE: i32 = -6;
const MAIN_SCENE_END_SIZE: i32 = 4;
const MAIN_CAMERA_SCALE: f32 = 0.2;

const GRASS_001: &str = "grass-001.png";
const GRASS_002: &str = "grass-002.png";
const BUILDING_BLOCK_001: &str = "building-block-001.png";

const PLAYER_SPRITE_SHEET: &str = "player.png";

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

struct CameraMatcher();

#[derive(Debug, Clone)]
struct Store {
    start: i32,
    end: i32,
    grass_001: Handle<Texture>,
    grass_002: Handle<Texture>,
    blocks: Vec<(i32, i32, i8)>,
    updating: bool,
}

impl Store {
    fn new(start: i32, end: i32, asset_server: Res<AssetServer>) -> Self {
        let grass_001: Handle<Texture> = asset_server.load(GRASS_001);
        let grass_002: Handle<Texture> = asset_server.load(GRASS_002);

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

    fn update_blocks(&mut self, start: i32, end: i32) {
        self.blocks = create_blocks(self.start, self.end).0;

        self.start = start;
        self.end = end;
    }
}

fn create_blocks(start: i32, end: i32) -> (Vec<(i32, i32, i8)>, (i32, i32)) {
    let mut blocks: Vec<(i32, i32, i8)> = vec![];

    for x in start..end {
        let x_buffer = x * BLOCK_SIZE;

        for y in start..end {
            let y_buffer = y * BLOCK_SIZE;

            let mut range = rand::thread_rng();

            let grass_type = range.gen_range(0..16);

            blocks.push((x_buffer, y_buffer, grass_type));
        }
    }

    (blocks, (start, end))
}

#[derive(Debug, Copy, Clone)]
struct Player {
    x: f32,
    y: f32,
    z: f32,

    idle: bool,

    left: bool,
    right: bool,
    up: bool,
    down: bool,

    animation_index: u32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,

            idle: true,
            left: false,
            right: false,
            up: false,
            down: false,

            animation_index: 0,
        }
    }

    fn set_all_to_false(&mut self) {
        self.idle = false;
        self.left = false;
        self.right = false;
        self.up = false;
        self.down = false;
    }
}

fn init_state(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let store = Store::new(MAIN_SCENE_START_SIZE, MAIN_SCENE_END_SIZE, asset_server);

    commands.insert_resource(store);
}

fn camera_setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .with(CameraMatcher());
}

fn setup(
    commands: &mut Commands,
    mut store: ResMut<Store>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if store.updating {
        for block in &store.blocks {
            let material: Handle<Texture>;

            if block.2 == 1 {
                material = store.grass_001.clone();
            } else {
                material = store.grass_002.clone();
            }

            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    block.0 as f32,
                    block.1 as f32,
                    0.0,
                )),
                material: materials.add(material.into()),
                ..Default::default()
            });
        }

        store.updating = false;
    }
}

fn player_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(PLAYER_SPRITE_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = Player::new();

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
    mut store: ResMut<Store>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Player)>,
) {
    let store_start_pow = store.start.pow(2);
    let store_end_pow = store.end.pow(2);

    for (mut timer, mut sprite, mut player) in query.iter_mut() {
        timer.tick(time.delta_seconds_f64() as f32);

        println!("{:?} --- {:?}", player.x, store_start_pow);

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

        let mut velocity = 25.0;

        let x_dir = normalized_input_dir[0];
        let y_dir = normalized_input_dir[1];

        if input_dir.1 == "run" {
            velocity = 50.0;
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
    let mut velocity = 25.0;
    let got_input_dir = get_input_dir(keyboard_input);
    let input_dir = got_input_dir.0;

    if got_input_dir.1 == "run" {
        velocity = 50.0;
    }

    if input_dir.length() > 0. {
        for (mut transform, _camera) in query.iter_mut() {
            let input_dir = (transform.rotation * input_dir).normalize();

            transform.translation += input_dir * (time.delta_seconds_f64() * velocity) as f32;
        }
    }
}

fn get_input_dir(keyboard_input: Res<Input<KeyCode>>) -> (Vec3, &'static str) {
    let mut modifier = "";
    let mut input_dir = Vec3::default();

    if keyboard_input.pressed(KeyCode::W) {
        let up = Vec3::unit_y();

        input_dir += up;
    } else if keyboard_input.pressed(KeyCode::A) {
        let left = Vec3::unit_x();

        input_dir -= left;
    } else if keyboard_input.pressed(KeyCode::S) {
        let down = Vec3::unit_y();

        input_dir -= down;
    } else if keyboard_input.pressed(KeyCode::D) {
        let right = Vec3::unit_x();

        input_dir += right;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        modifier = "run";
    }

    (input_dir, modifier)
}
