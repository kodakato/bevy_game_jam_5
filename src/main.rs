use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_rapier2d::prelude::*;

mod camera;
mod debris;
mod debug;
mod explosion;
mod input;
mod level;
mod planet;
mod player;
mod projectile;
mod satellite;
mod ui;

fn main() -> AppExit {
    let mut rapier_config = RapierConfiguration::new(100.0);
    rapier_config.gravity = Vec2::ZERO;
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .insert_resource(rapier_config)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(planet::PlanetPlugin)
        .add_plugins(satellite::SatellitePlugin)
        .add_plugins(explosion::ExplosionPlugin)
        .add_plugins(debris::DebrisPlugin)
        .add_plugins(ui::UiPlugin)
        .insert_resource(Score(0))
        .run()
}

#[derive(Resource)]
struct Score(u32);
