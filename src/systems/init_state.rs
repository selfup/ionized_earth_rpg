use bevy::prelude::*;

use super::super::resources::store;

const MAIN_SCENE_END_SIZE: i32 = 4;
const MAIN_SCENE_START_SIZE: i32 = -6;

pub fn system(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let store = store::Resource::new(MAIN_SCENE_START_SIZE, MAIN_SCENE_END_SIZE, asset_server);

    commands.insert_resource(store);
}
