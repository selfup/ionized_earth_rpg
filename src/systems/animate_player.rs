use std::time::Duration;

use bevy::prelude::*;

use crate::entities::*;

/*
    [
        0,1,2,3,4, <- idle
        5,6,7,8,9, <- left
        10,11,12,13,14, <- right
        15,16,17,18,19 <- up
        20,21,22,23,24, <- down
    ]
*/

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Player)>,
) {
    for (mut timer, mut sprite, mut player) in query.iter_mut() {
        let duration: Duration = time.delta();

        timer.tick(duration);

        if player.idle {
            animation_index_bounds_calc(&mut player, 4, 0);
        } else if player.left {
            animation_index_bounds_calc(&mut player, 9, 5);
        } else if player.right {
            animation_index_bounds_calc(&mut player, 14, 10);
        } else if player.up {
            animation_index_bounds_calc(&mut player, 19, 15);
        } else if player.down {
            animation_index_bounds_calc(&mut player, 24, 20);
        }

        if timer.finished() {
            sprite.index = player.animation_index;
        }
    }
}

fn animation_index_bounds_calc(player: &mut Player, upper_bound: usize, lower_bound: usize) {
    let idx: usize = player.animation_index;

    if idx < upper_bound && idx >= lower_bound {
        player.animation_index += 1;
    } else {
        player.animation_index = lower_bound;
    }
}
