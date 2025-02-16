use bevy::prelude::*;

pub fn get_input_dir(keyboard_input: Res<ButtonInput<KeyCode>>) -> (Vec3, &'static str) {
    let mut modifier = "";
    let mut input_dir = Vec3::default();

    if keyboard_input.pressed(KeyCode::KeyW) {
        let up = Vec3::Y;

        input_dir += up;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        let left = Vec3::X;

        input_dir -= left;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        let down = Vec3::Y;

        input_dir -= down;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        let right = Vec3::X;

        input_dir += right;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        modifier = "run";
    }

    (input_dir, modifier)
}
