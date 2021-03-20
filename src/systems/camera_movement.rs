use bevy::prelude::*;

use super::super::utils;

pub fn system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Transform,
        &mut super::super::entities::camera_matcher::CameraMatcher,
    )>,
) {
    let mut velocity = 25.0;
    let got_input_dir = utils::get_input_dir::util(keyboard_input);
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
