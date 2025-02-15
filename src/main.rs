use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod constants;
mod entities;
mod resources;
mod systems;
mod utils;

use constants::*;
use systems::*;

#[derive(Component)]
struct FuseTime {
    /// track when the bomb should explode (non-repeating timer)
    timer: Timer,
}

fn main() {
    let color: Color = Color::srgb(BG_COLOR, BG_COLOR, BG_COLOR);

    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(PreStartup, init_state)
        .add_systems(
            Startup,
            (
                camera_setup,
                setup,
                add_random_building_blocks,
                player_setup,
            ),
        )
        .add_systems(
            Update,
            (
                camera_scale,
                camera_movement,
                animate_player,
                player_movement,
            ),
        )
        .insert_resource(ClearColor(color))
        .run();
}
