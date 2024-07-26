use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, build_level);
    }
}

use crate::planet::*;
use crate::satellite::*;

pub fn build_level(
    mut spawn_planet_ew: EventWriter<SpawnPlanetEvent>,
    mut spawn_satellite_ew: EventWriter<SpawnSatelliteEvent>,
) {
    let transform = Transform::from_xyz(8000.0, 0.0, 0.0);

    // Spawn planet
    spawn_planet_ew.send(SpawnPlanetEvent(transform, 3000.0));
    spawn_planet_ew.send(SpawnPlanetEvent(
        Transform::from_xyz(-9000.0, 5000.0, 0.0),
        1000.0,
    ));

    // Spawn satellite
    spawn_satellite_ew.send(SpawnSatelliteEvent {
        orbit_centre: Vec2::new(transform.translation.x, transform.translation.y),
        satellite_transform: Transform::from_xyz(-2000.0, 0.0, 0.0),
        radius: 6000.0,
        angular_velocity: 0.1,
    });
}
