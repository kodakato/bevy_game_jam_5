use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player);
    }
}

pub const PROJECTION_SCALE: f32 = 0.1;

use crate::input::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Default)]
pub struct PlayerCameraTag;

#[derive(Bundle)]
struct CameraBundle(
    PlayerCameraTag,
    Camera2dBundle,
    InputManagerBundle<CameraAction>,
    Velocity,
);

fn spawn_camera(mut commands: Commands) {
    let input_map = InputMap::new([
        (CameraAction::ZoomIn, KeyCode::KeyM),
        (CameraAction::ZoomOut, KeyCode::KeyN),
    ]);

    let camera_bundle = CameraBundle(
        PlayerCameraTag,
        Camera2dBundle::default(),
        InputManagerBundle::with_map(input_map),
        Velocity::default(),
    );

    commands.spawn(camera_bundle);
}

use crate::player::*;
use bevy_rapier2d::prelude::*;

fn follow_player(
    mut camera_q: Query<&mut Transform, (With<PlayerCameraTag>, Without<PlayerTag>)>,
    player_q: Query<&Transform, With<PlayerTag>>,
) {
    if player_q.is_empty() {
        return;
    }

    let mut camera_transform = camera_q.single_mut();
    let player_transform = player_q.single();

    camera_transform.translation = player_transform.translation;
}
