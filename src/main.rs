use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod constants;
mod entities;
mod resources;
mod systems;
mod utils;

use constants::*;
use systems::*;

fn main() {
    let color: Color = Color::rgb(BG_COLOR, BG_COLOR, BG_COLOR);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system_to_stage(StartupStage::PreStartup, init_state)
        .add_startup_system(camera_setup)
        .add_system(setup)
        .add_startup_system(add_random_building_blocks)
        .add_startup_system(player_setup)
        .add_system(camera_scale)
        .add_system(camera_movement)
        .add_system(animate_player)
        .add_system(player_movement)
        .insert_resource(ClearColor(color))
        .run();
}
