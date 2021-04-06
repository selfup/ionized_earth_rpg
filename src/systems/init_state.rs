use bevy::prelude::*;

use crate::constants::{MAIN_SCENE_END_SIZE, MAIN_SCENE_START_SIZE};
use crate::resources::*;

pub fn init_state(mut commands: Commands, asset_server: Res<AssetServer>) {
    let store = Store::new(MAIN_SCENE_START_SIZE, MAIN_SCENE_END_SIZE, asset_server);

    commands.insert_resource(store);
}
