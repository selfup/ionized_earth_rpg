use bevy::prelude::*;

use crate::{entities::*, GameTime};

const PLAYER_SPRITE_SHEET: &str = "player.png";

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player = Player::new();

    let texture = asset_server.load(PLAYER_SPRITE_SHEET);
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
    let texture_atlas_layout = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            Transform::from_scale(Vec3::splat(1.0)),
        ))
        .insert(player)
        .insert(GameTime {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        });
}
