use bevy::prelude::*;

use crate::resources::*;

pub fn setup(
    commands: &mut Commands,
    mut store: ResMut<Store>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if store.updating {
        for block in &store.blocks {
            let material: Handle<Texture>;

            if block.2 == 1 {
                material = store.grass_001.clone();
            } else {
                material = store.grass_002.clone();
            }

            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    block.0 as f32,
                    block.1 as f32,
                    0.0,
                )),
                material: materials.add(material.into()),
                ..Default::default()
            });
        }

        store.updating = false;
    }
}
