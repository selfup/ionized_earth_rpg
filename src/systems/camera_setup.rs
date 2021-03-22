use bevy::prelude::*;

use crate::entities::*;

pub fn camera_setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .with(CameraMatcher());
}
