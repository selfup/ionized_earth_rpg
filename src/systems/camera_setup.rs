use bevy::prelude::*;

use crate::entities::*;

pub fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraMatcher());
}
