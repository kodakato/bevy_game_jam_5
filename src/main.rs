use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod debug;
mod input;
mod player;
mod projectile;
mod planet;
mod level;

fn main() -> AppExit {
    let mut rapier_config = RapierConfiguration::new(100.0);
    rapier_config.gravity = Vec2::ZERO;
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(rapier_config)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(projectile::ProjectilePlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(planet::PlanetPlugin)
        .run()
}
