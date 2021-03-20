use bevy::prelude::*;

pub fn system(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .with(super::super::entities::camera_matcher::CameraMatcher());
}
