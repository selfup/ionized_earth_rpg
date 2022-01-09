use bevy::prelude::*;

use crate::resources::*;

pub fn setup(mut commands: Commands, mut store: ResMut<Store>) {
    if store.updating {
        for block in &store.blocks {
            let material: Handle<Image>;

            if block.2 == 1 {
                material = store.grass_001.clone();
            } else {
                material = store.grass_002.clone();
            }

            commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    block.0 as f32,
                    block.1 as f32,
                    0.0,
                )),
                texture: material,
                ..Default::default()
            });
        }

        store.updating = false;
    }
}
