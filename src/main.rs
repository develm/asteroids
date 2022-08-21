use bevy::window::PresentMode;

use prelude::*;

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

    pub const ASSET_SCALING: Vec3 = Vec3::splat(0.5);
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
        .add_system(game_systems::player_movement)
        .add_system(game_systems::wrap_window)
        .add_system(game_systems::expend)
        .add_system(game_systems::auto_move)
        .add_system(game_systems::player_action)
        .add_system(game_systems::destroy_asteroid)
        .add_system(game_systems::destroy_player)
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