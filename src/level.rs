use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, build_level)
        ;
    }
}

use crate::planet::*;

pub fn build_level(
    mut spawn_planet_ew: EventWriter<SpawnPlanetEvent>,
    ) {

    let transform = Transform::from_xyz(0.0, 0.0, 0.0);
    
    // Spawn planet
    spawn_planet_ew.send(SpawnPlanetEvent(transform));
}
