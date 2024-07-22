use bevy::prelude::*;

mod camera;
mod player;
mod input;
mod debug;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(debug::DebugPlugin)
        .run()
}
