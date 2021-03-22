use bevy::prelude::*;

use crate::entities::*;

use crate::constants::MAIN_CAMERA_SCALE;

pub fn camera_scale(mut query: Query<(&mut Transform, &mut CameraMatcher)>) {
    for (mut transform, _camera) in query.iter_mut() {
        transform.scale = Vec3::new(MAIN_CAMERA_SCALE, MAIN_CAMERA_SCALE, 1.0);
    }
}
