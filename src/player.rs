use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, spawn_player)
            ;
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
);

struct PlayerSize {
    height: f32,
    width: f32,
}

const PLAYER_SIZE: PlayerSize = PlayerSize {
    height: 100.0,
    width: 50.0,
};
pub const PLAYER_MAX_VELOCITY: f32 = 20.0;

use crate::input::PlayerAction;

fn spawn_player(mut commands: Commands,
                asset_server: Res<AssetServer>,) {
    let texture_handle: Handle<Image> = asset_server.load("textures/ship.png");
    let transform = Transform::from_xyz(-150.0, 0.0, 0.0);

    let input_map = InputMap::new([(PlayerAction::Accelerate, KeyCode::Space)]);

    commands.spawn(PlayerBundle(
        PlayerTag,
        SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE.width, PLAYER_SIZE.height)),
                ..default()
            },
            texture: texture_handle,
            transform,
            ..default()
        },
        Collider::capsule_y(
            PLAYER_SIZE.height / 2.0 - PLAYER_SIZE.width / 2.0,
            PLAYER_SIZE.width / 2.0
        ),
        InputManagerBundle::with_map(input_map),
        RigidBody::Dynamic,
        Velocity::default(),
        ExternalImpulse::default(),
        ColliderMassProperties::Mass(20.0),
    ));
}


