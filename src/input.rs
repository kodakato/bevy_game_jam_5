use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, (player_movement, shoot_projectile));
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Accelerate,
    //Rotate
    Left,
    Right,
    Shoot,
}

use crate::player::*;
use bevy_rapier2d::prelude::*;

const IMPULSE: f32 = 1000.0;

fn player_movement(
    mut player_q: Query<
        (&ActionState<PlayerAction>, &mut ExternalImpulse, &Transform),
        With<PlayerTag>,
    >,
) {
    let (action_state, mut ext_impulse, transform) = player_q.single_mut();
    if action_state.pressed(&PlayerAction::Accelerate) {
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        let direction = Vec2::new(-rotation.sin(), rotation.cos());

        ext_impulse.impulse = direction * 100.0;
    }

    if action_state.pressed(&PlayerAction::Left) {
        ext_impulse.torque_impulse = IMPULSE;
    }

    if action_state.pressed(&PlayerAction::Right) {
        ext_impulse.torque_impulse = -IMPULSE;
    }
}

use crate::projectile::*;

pub const PROJECTILE_OFFSET: f32 = 60.0;

fn shoot_projectile(
    mut spawn_projectile_ew: EventWriter<SpawnProjectileEvent>,
    player_q: Query<(&ActionState<PlayerAction>, &Transform, &Velocity), With<PlayerTag>>,
) {
    let (action_state, transform, velocity) = player_q.single();

    if !action_state.just_pressed(&PlayerAction::Shoot) {
        return;
    }

    // Calculate the offset position for the top of the player sprite
    let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
    let direction = Vec2::new(-rotation.sin(), rotation.cos());
    let offset = direction * PROJECTILE_OFFSET;

    // Calculate the spawn position for the projectile
    let player_position = transform.translation;
    let spawn_position = Vec3::new(
        player_position.x + offset.x,
        player_position.y + offset.y,
        0.0,
    );

    let projectile_transform = Transform {
        translation: spawn_position,
        rotation: transform.rotation,
        ..default()
    };
    
    let velocity = Velocity::linear(velocity.linvel); // Preserve only linvel

    spawn_projectile_ew.send(SpawnProjectileEvent(projectile_transform, velocity));
}

