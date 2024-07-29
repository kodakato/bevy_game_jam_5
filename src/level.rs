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
use rand::*;

const SATELLITE_AMOUNT: usize = 20;
const MIN_DISTANCE: f32 = 500.0; // Minimum additional distance from the planet's radius
const MAX_DISTANCE: f32 = 6000.0;

fn build_level(
    mut spawn_planet_ew: EventWriter<SpawnPlanetEvent>,
    mut spawn_satellite_ew: EventWriter<SpawnSatelliteEvent>,
) {
    // Spawn Moon
    let (moon_transform, moon_radius) = (Transform::from_xyz(-100.0, -1080.0, 0.0), 1000.0);
    spawn_planet_ew.send(SpawnPlanetEvent(moon_transform, moon_radius));

    // Spawn Planet
    let (planet_transform, planet_radius) = (Transform::from_xyz(0.0, 14000.0, 0.0), 8000.0);
    spawn_planet_ew.send(SpawnPlanetEvent(planet_transform, planet_radius));

    let mut rng = rand::thread_rng();

    // Spawn satellites around the planet
    for _ in 0..SATELLITE_AMOUNT {
        let additional_distance = rng.gen_range(MIN_DISTANCE..MAX_DISTANCE);
        let distance = planet_radius + additional_distance;
        let angular_velocity = 0.1 / (additional_distance / planet_radius); // Higher velocity for closer satellites

        let initial_angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let initial_x = planet_transform.translation.x + distance * initial_angle.cos();
        let initial_y = planet_transform.translation.y + distance * initial_angle.sin();

        spawn_satellite_ew.send(SpawnSatelliteEvent {
            orbit_centre: planet_transform,
            satellite_transform: Transform::from_xyz(
                initial_x,
                initial_y,
                planet_transform.translation.z,
            ),
            radius: distance,
            angular_velocity,
        });
    }
}
