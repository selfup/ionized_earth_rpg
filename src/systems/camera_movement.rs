use bevy::prelude::*;

use crate::constants::*;
use crate::entities::*;
use crate::utils::*;

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraMatcher)>,
) {
    let mut velocity = NORMAL_VELOCITY;
    let got_input_dir = get_input_dir(keyboard_input);
    let input_dir = got_input_dir.0;

    if got_input_dir.1 == "run" {
        velocity = RUNNING_VELOCITY;
    }

    if input_dir.length() > 0. {
        for (mut transform, _camera) in query.iter_mut() {
            let input_dir = (transform.rotation * input_dir).normalize();

            transform.translation += input_dir * (time.delta_secs_f64() * velocity) as f32;
        }
    }
}
