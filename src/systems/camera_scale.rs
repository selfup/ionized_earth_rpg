use bevy::prelude::*;

const MAIN_CAMERA_SCALE: f32 = 0.2;

pub fn system(
    mut query: Query<(
        &mut Transform,
        &mut super::super::entities::camera_matcher::CameraMatcher,
    )>,
) {
    for (mut transform, _camera) in query.iter_mut() {
        transform.scale = Vec3::new(MAIN_CAMERA_SCALE, MAIN_CAMERA_SCALE, 1.0);
    }
}
