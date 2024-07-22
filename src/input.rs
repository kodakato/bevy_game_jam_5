use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
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
