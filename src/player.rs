use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, spawn_player);
    }
}

#[derive(Default, Component)]
pub struct PlayerTag;

#[derive(Default, Bundle)]
pub struct PlayerBundle(
    PlayerTag,
    SpriteBundle,
    Collider,
    InputManagerBundle<PlayerAction>,
    RigidBody,
    Velocity,
    ExternalImpulse,
    ColliderMassProperties,
    Damping,
    ExternalForce,
);

struct PlayerSize {
    height: f32,
    width: f32,
}

const PLAYER_SIZE: PlayerSize = PlayerSize {
    height: 100.0,
    width: 50.0,
};

use crate::input::PlayerAction;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("textures/ship.png");
    let transform = Transform::from_xyz(-150.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(0.0));

    let input_map = InputMap::new([
        (PlayerAction::Accelerate, KeyCode::KeyW),
        (PlayerAction::Left, KeyCode::KeyA),
        (PlayerAction::Right, KeyCode::KeyD),
        (PlayerAction::Shoot, KeyCode::Space),
    ]);

    let cuboid_height = PLAYER_SIZE.height - PLAYER_SIZE.width;

    // Create a combined collider using a cuboid and a ball
    let colliders = Collider::compound(vec![
        (
            Vec2::new(0.0, -cuboid_height / 2.0).into(),
            0.0,
            Collider::cuboid(PLAYER_SIZE.width / 2.0, cuboid_height / 2.0),
        ),
        (
            Vec2::new(0.0, 0.0).into(),
            0.0,
            Collider::capsule_y(
                PLAYER_SIZE.height / 2.0 - PLAYER_SIZE.width / 2.0,
                PLAYER_SIZE.width / 2.0,
            ),
        ),
    ]);

    commands.spawn(PlayerBundle(
        PlayerTag,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE.width, PLAYER_SIZE.height)),
                ..default()
            },
            texture: texture_handle,
            transform,
            ..default()
        },
        colliders,
        InputManagerBundle::with_map(input_map),
        RigidBody::Dynamic,
        Velocity::default(),
        ExternalImpulse::default(),
        ColliderMassProperties::MassProperties(MassProperties {
            mass: 20.0,
            local_center_of_mass: Vec2::new(0.0, 0.0),
            principal_inertia: 7000.0,
        }),
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.4,
        },
        ExternalForce::default(),
    ));
}
