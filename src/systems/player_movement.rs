use bevy::prelude::*;

use crate::constants::*;
use crate::entities::*;
use crate::utils::*;

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Handle<TextureAtlas>, &mut Player)>,
) {
    let input_dir = get_input_dir(keyboard_input);

    for (mut transform, _handle_texture_atlas, mut player) in query.iter_mut() {
        let normalized_input_dir = (transform.rotation * input_dir.0).normalize();

        let mut velocity = NORMAL_VELOCITY;

        let x_dir = normalized_input_dir[0];
        let y_dir = normalized_input_dir[1];

        if input_dir.1 == "run" {
            velocity = RUNNING_VELOCITY;
        }

        player.set_all_to_false();

        if x_dir == -1.0 {
            player.x -= (1.0 * time.delta_seconds_f64() * velocity) as f32;

            player.left = true;
        } else if x_dir == 1.0 {
            player.x += (1.0 * time.delta_seconds_f64() * velocity) as f32;

            player.right = true;
        } else if y_dir == -1.0 {
            player.y -= (1.0 * time.delta_seconds_f64() * velocity) as f32;

            player.down = true;
        } else if y_dir == 1.0 {
            player.y += (1.0 * time.delta_seconds_f64() * velocity) as f32;

            player.up = true;
        } else {
            player.idle = true;
        }

        transform.translation = Vec3::new(player.x, player.y, player.z);
    }
}
