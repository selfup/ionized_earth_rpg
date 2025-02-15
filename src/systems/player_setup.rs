use bevy::prelude::*;

use crate::entities::*;

const PLAYER_SPRITE_SHEET: &str = "player.png";

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player = Player::new();

    let animation_indices = AnimationIndices { first: 1, last: 6 };

    let texture = asset_server.load(PLAYER_SPRITE_SHEET);
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
    let texture_atlas_layout = texture_atlases.add(texture_atlas);

    // commands.spawn(Camera2d);
    commands
        .spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            Transform::from_scale(Vec3::splat(6.0)),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(player);
}
