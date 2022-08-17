mod asset_manager;

use bevy::prelude::*;
use crate::asset_manager::AssetManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetManagerPlugin)
        .run();
}