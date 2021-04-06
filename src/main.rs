use bevy::prelude::*;

mod constants;
mod entities;
mod resources;
mod systems;
mod utils;

use constants::*;
use systems::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, init_state.system())
        .add_startup_system(camera_setup.system())
        .add_system(setup.system())
        .add_startup_system(add_random_building_blocks.system())
        .add_startup_system(player_setup.system())
        .add_system(camera_scale.system())
        .add_system(camera_movement.system())
        .add_system(animate_player.system())
        .add_system(player_movement.system())
        .insert_resource(ClearColor(Color::rgb(BG_COLOR, BG_COLOR, BG_COLOR)))
        .run();
}
