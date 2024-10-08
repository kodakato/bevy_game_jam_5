use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<SpawnSatelliteEvent>()
            // Systems
            .add_systems(
                Update,
                (spawn_satellite, orbit_satellite, despawn_satellite),
            );
    }
}

#[derive(Component, Default)]
pub struct Satellite {
    pub orbit_center: Transform,
    pub radius: f32,
    pub angular_velocity: f32, // Radians per second
}
#[derive(Event, Default)]
pub struct SpawnSatelliteEvent {
    pub orbit_centre: Transform,
    pub satellite_transform: Transform,
    pub radius: f32,
    pub angular_velocity: f32,
}

#[derive(Component, Default)]
pub struct Health(pub u32);

#[derive(Bundle, Default)]
struct SatelliteBundle(
    Satellite,
    MaterialMesh2dBundle<ColorMaterial>,
    Collider,
    RigidBody,
    Health,
);

const SATELLITE_RADIUS: f32 = 100.0;

fn spawn_satellite(
    mut spawn_satellite_er: EventReader<SpawnSatelliteEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in spawn_satellite_er.read() {
        commands.spawn((
            Satellite {
                orbit_center: event.orbit_centre,
                radius: event.radius,
                angular_velocity: event.angular_velocity,
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: SATELLITE_RADIUS,
                })),
                material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                transform: event.satellite_transform,
                ..default()
            },
            Collider::ball(SATELLITE_RADIUS),
            RigidBody::KinematicPositionBased,
            Health(2),
        ));
    }
}

fn orbit_satellite(time: Res<Time>, mut query: Query<(&mut Transform, &Satellite)>) {
    for (mut transform, satellite) in query.iter_mut() {
        // Calculate the new angle based on the angular velocity
        let angle = satellite.angular_velocity * time.elapsed_seconds();

        // Calculate the new position based on the angle
        let new_position = Vec2::new(
            satellite.orbit_center.translation.x + satellite.radius * angle.cos(),
            satellite.orbit_center.translation.y + satellite.radius * angle.sin(),
        );

        // Update the satellite's position
        transform.translation = new_position.extend(transform.translation.z);
    }
}

use crate::debris::SpawnDebrisEvent;
use rand::*;

fn despawn_satellite(
    satellite_q: Query<(Entity, &Health, &Transform), With<Satellite>>,
    mut commands: Commands,
    mut spawn_debris_ew: EventWriter<SpawnDebrisEvent>,
) {
    for (entity, health, transform) in satellite_q.iter() {
        if health.0 < 1 {
            commands.entity(entity).despawn();

            let amount = rand::thread_rng().gen_range(3..15);
            for _ in 0..amount {
                spawn_debris_ew.send(SpawnDebrisEvent(*transform));
            }
        }
    }
}
