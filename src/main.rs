mod asset_manager;

use bevy::prelude::*;
use crate::asset_manager::{AssetManagerPlugin, AtlasManager};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetManagerPlugin)
        .add_startup_system(load_game)
        .run();
}

fn load_game(
    mut commands: Commands,
    atlas_manager: Res<AtlasManager>,
) {

}