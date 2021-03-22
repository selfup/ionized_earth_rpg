use bevy::prelude::*;

use crate::resources::*;

const MAIN_SCENE_END_SIZE: i32 = 4;
const MAIN_SCENE_START_SIZE: i32 = -6;

pub fn init_state(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let store = Store::new(MAIN_SCENE_START_SIZE, MAIN_SCENE_END_SIZE, asset_server);

    commands.insert_resource(store);
}
