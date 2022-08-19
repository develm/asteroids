mod asset_manager;
mod components;
mod player_input;
mod wrap;

mod prelude {
    pub use bevy::prelude::*;
    pub use crate::components::*;
    pub use crate::player::*;
    pub use crate::common::*;
    pub use crate::wrap::*;
    pub use crate::player_input::*;
    pub use crate::asset_manager::*;

    pub const SCALE: Vec3 = Vec3::splat(0.5);
}

use bevy::window::PresentMode;
use prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Asteroids".to_string(),
            width: 1024.0,
            height: 768.0,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetManagerPlugin)
        .add_startup_system(load_game)
        .add_system(player_movement)
        .add_system(wrap_window)
        .add_system(expend)
        .run();
}

fn load_game(
    mut commands: Commands,
    atlas_manager: Res<AtlasManager>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    Player::spawn(&mut commands, &atlas_manager, Vec3::ZERO);
}