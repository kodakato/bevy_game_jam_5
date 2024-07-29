use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (follow_player, auto_zoom));
    }
}

pub const PROJECTION_SCALE: f32 = 1.0;

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

use crate::planet::*;

const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 30.0;
const ZOOM_SCALE: f32 = 0.005;

// Automatically adjusts the zoom depending on the distance from the closest planet
fn auto_zoom(
    mut camera_q: Query<&mut OrthographicProjection, With<Camera>>,
    planet_q: Query<(&Transform, &Collider), With<PlanetTag>>,
    player_q: Query<&Transform, With<PlayerTag>>,
) {
    let mut projection = camera_q.single_mut();
    let player_transform = player_q.single();

    // Initialize distance to a large value
    let mut min_distance = f32::MAX;

    // Find the closest planet
    for (planet_transform, planet_collider) in planet_q.iter() {
        let planet_radius = planet_collider
            .as_ball()
            .expect("Can't make collider into ball")
            .radius();
        let current_distance = planet_transform
            .translation
            .distance(player_transform.translation)
            - planet_radius;

        if current_distance < min_distance {
            min_distance = current_distance;
        }
    }

    // Calculate the new scale based on the closest distance
    let new_scale = (min_distance * ZOOM_SCALE).clamp(MIN_ZOOM, MAX_ZOOM);

    // Set the projection scale
    projection.scale = new_scale;
}
