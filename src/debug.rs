use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
                .add_plugins(RapierDebugRenderPlugin::default())
            ;

        }
    }
}
