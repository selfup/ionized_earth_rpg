use bevy::prelude::*;

use crate::entities::*;

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Player)>,
) {
    for (mut timer, mut sprite, mut player) in query.iter_mut() {
        timer.tick(time.delta());

        /*
            [
                0,1,2,3,4, <- idle
                5,6,7,8,9, <- left
                10,11,12,13,14, <- right
                15,16,17,18,19 <- up
                20,21,22,23,24, <- down
            ]
        */

        if player.idle {
            if player.animation_index < 4 {
                player.animation_index += 1;
            } else {
                player.animation_index = 0;
            }
        } else if player.left {
            if player.animation_index < 9 {
                player.animation_index += 1;
            } else {
                player.animation_index = 5;
            }
        } else if player.right {
            if player.animation_index < 14 {
                player.animation_index += 1;
            } else {
                player.animation_index = 10;
            }
        } else if player.up {
            if player.animation_index < 19 {
                player.animation_index += 1;
            } else {
                player.animation_index = 15;
            }
        } else if player.down {
            if player.animation_index < 24 {
                player.animation_index += 1;
            } else {
                player.animation_index = 20;
            }
        }

        if timer.finished() {
            sprite.index = player.animation_index as usize;
        }
    }
}
