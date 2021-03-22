use bevy::prelude::*;

use crate::entities::*;

const PLAYER_SPRITE_SHEET: &str = "player.png";

pub fn player_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(PLAYER_SPRITE_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = Player::new();

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(player);
}
