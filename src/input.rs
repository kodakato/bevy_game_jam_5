use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, player_movement)
        ;
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Accelerate,
    //Rotate
    Left,
    Right,
}

use crate::player::*;
use bevy_rapier2d::prelude::*;

fn player_movement(mut player_q: Query<(&ActionState<PlayerAction>, &mut ExternalImpulse, &Transform), With<PlayerTag>>) {
    let (action_state, mut ext_impulse, transform) = player_q.single_mut();
    if action_state.pressed(&PlayerAction::Accelerate) {
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        let direction = Vec2::new(-rotation.sin(), rotation.cos());

        ext_impulse.impulse = direction * 100.0;
    }

    const IMPULSE: f32 = 1000.0;

    if action_state.pressed(&PlayerAction::Left) {
        ext_impulse.torque_impulse = IMPULSE;
    }

    if action_state.pressed(&PlayerAction::Right) {
        ext_impulse.torque_impulse = -IMPULSE;
    }
}


