use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<SpawnPlanetEvent>()
            // Systems
            .add_systems(Update, (spawn_planet, apply_gravitational_forces));
    }
}

#[derive(Component, Default)]
pub struct PlanetTag;

#[derive(Event)]
pub struct SpawnPlanetEvent(pub Transform, pub f32);

fn spawn_planet(
    mut spawn_planet_er: EventReader<SpawnPlanetEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in spawn_planet_er.read() {
        let radius = event.1;
        let mass = radius.powf(2.5);
        commands.spawn((
            Collider::ball(radius),
            RigidBody::KinematicVelocityBased,
            PlanetTag,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius })),
                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                transform: event.0,
                ..default()
            },
            ColliderMassProperties::Mass(mass),
        ));
    }
}

// Attracts all other non planet objects
fn apply_gravitational_forces(
    mut non_planet_q: Query<(&mut ExternalImpulse, &Transform), Without<PlanetTag>>,
    planet_q: Query<(&Transform, &ColliderMassProperties), With<PlanetTag>>,
) {
    const G: f32 = 3.0;

    for (mut external_impulse, non_planet_transform) in non_planet_q.iter_mut() {
        let non_planet_position = non_planet_transform.translation.truncate();

        for (planet_transform, planet_mass_props) in planet_q.iter() {
            let planet_position = planet_transform.translation.truncate();
            let planet_mass = match planet_mass_props {
                ColliderMassProperties::Mass(mass) => *mass,
                _ => 0.0, // Handle other cases if needed
            };

            let direction = planet_position - non_planet_position;
            let distance_squared = direction.length_squared();

            if distance_squared > 0.0 {
                let force_magnitude = G * planet_mass / distance_squared;
                let impulse = direction.normalize() * force_magnitude;

                // Apply the gravitational impulse
                external_impulse.impulse += impulse;
            }
        }
    }
}
