use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, accelerate_forward)
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

fn accelerate_forward(mut player_q: Query<(&ActionState<PlayerAction>, &mut ExternalImpulse), With<PlayerTag>>) {
    let (action_state, mut ext_impulse) = player_q.single_mut();
    if action_state.pressed(&PlayerAction::Accelerate) {
        println!("Accelerating!");
        ext_impulse.impulse = Vec2::new(0.0, 100.0);
    }
}

// fn rotate_player() {}
