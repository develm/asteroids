use bevy::window::PresentMode;

use prelude::*;
use crate::game_systems::GameSystems;

mod asset_manager;
mod components;
mod game_systems;
mod util;

mod prelude {
    pub use bevy::prelude::*;

    pub use crate::asset_manager::*;
    pub use crate::asteroid::*;
    pub use crate::common::*;
    pub use crate::components::*;
    pub use crate::player::*;
    pub use crate::util::*;
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Asteroids".to_string(),
            width: 1024.0,
            height: 768.0,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetManagerPlugin)
        .add_startup_system(load_game)
        .add_plugin(GameSystems)
        .run();
}

fn load_game(
    mut commands: Commands,
    atlas_manager: Res<AtlasManager>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    Player::spawn(&mut commands, &atlas_manager, Vec3::ZERO);
    Asteroid::spawn(&mut commands, &atlas_manager, Vec3::new(-200.0, 200.0, 99.0));
    Asteroid::spawn(&mut commands, &atlas_manager, Vec3::new(-400.0, 700.0, 99.0));
    Asteroid::spawn(&mut commands, &atlas_manager, Vec3::new(800.0, -300.0, 99.0));
}