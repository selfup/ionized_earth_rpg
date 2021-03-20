use bevy::prelude::*;

pub fn util(keyboard_input: Res<Input<KeyCode>>) -> (Vec3, &'static str) {
    let mut modifier = "";
    let mut input_dir = Vec3::default();

    if keyboard_input.pressed(KeyCode::W) {
        let up = Vec3::unit_y();

        input_dir += up;
    } else if keyboard_input.pressed(KeyCode::A) {
        let left = Vec3::unit_x();

        input_dir -= left;
    } else if keyboard_input.pressed(KeyCode::S) {
        let down = Vec3::unit_y();

        input_dir -= down;
    } else if keyboard_input.pressed(KeyCode::D) {
        let right = Vec3::unit_x();

        input_dir += right;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        modifier = "run";
    }

    (input_dir, modifier)
}
