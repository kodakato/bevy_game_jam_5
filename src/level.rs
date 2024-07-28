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

fn build_level(
    mut spawn_planet_ew: EventWriter<SpawnPlanetEvent>,
    mut spawn_satellite_ew: EventWriter<SpawnSatelliteEvent>,
) {
    // Spawn Moon
    let (moon_transform, moon_radius) = (Transform::from_xyz(-100.0, -1080.0, 0.0), 1000.0);
    spawn_planet_ew.send(SpawnPlanetEvent(moon_transform, moon_radius));

    // Spawn Planet
    let (planet_transform, planet_radius) = (Transform::from_xyz(0.0, 20000.0, 0.0), 10000.0);
    spawn_planet_ew.send(SpawnPlanetEvent(planet_transform, planet_radius));

    spawn_satellite_ew.send(SpawnSatelliteEvent {
        orbit_centre: moon_transform,
        radius: 1500.0,
        angular_velocity: 1.0,
        ..default()
    });

    spawn_satellite_ew.send(SpawnSatelliteEvent {
        orbit_centre: moon_transform,
        radius: 1800.0,
        angular_velocity: 0.8,
        ..default()
    });
}
