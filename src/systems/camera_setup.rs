use bevy::prelude::*;

use crate::entities::*;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d).insert(CameraMatcher());
}
