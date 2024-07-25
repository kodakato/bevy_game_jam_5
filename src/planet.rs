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
            .add_systems(Update, spawn_planet);
    }
}

#[derive(Component, Default)]
pub struct PlanetTag;

#[derive(Event)]
pub struct SpawnPlanetEvent(pub Transform);

pub fn spawn_planet(
    mut spawn_planet_er: EventReader<SpawnPlanetEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in spawn_planet_er.read() {
        commands.spawn((
            Collider::ball(30.0),
            PlanetTag,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 30.0 })),
                material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                transform: event.0,
                ..default()
            },
        ));
    }
}
