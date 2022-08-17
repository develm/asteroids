mod asset_manager;
mod components;

use bevy::prelude::*;
use crate::asset_manager::{AssetManagerPlugin, AtlasManager};
use crate::components::Player;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Asteroids".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetManagerPlugin)
        .add_startup_system(load_game)
        .run();
}

fn load_game(
    mut commands: Commands,
    atlas_manager: Res<AtlasManager>,
) {
    commands.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: atlas_manager.texture_atlas.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 220.0, 0.0)),
            sprite: TextureAtlasSprite::new(*atlas_manager.texture_index.get("spaceship").expect("Could not find texture")),
            ..Default::default()

        })
        .insert(Player);
}