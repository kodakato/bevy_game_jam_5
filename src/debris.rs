use bevy::{prelude::*, sprite::*};
use bevy_rapier2d::prelude::*;

pub struct DebrisPlugin;

impl Plugin for DebrisPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<SpawnDebrisEvent>()
            // Systems
            .add_systems(Update, (spawn_debris, collect_debris));
    }
}

#[derive(Component, Default)]
pub struct DebrisTag;

#[derive(Event)]
pub struct SpawnDebrisEvent(pub Transform);

#[derive(Bundle)]
pub struct DebrisBundle(
    MaterialMesh2dBundle<ColorMaterial>,
    DebrisTag,
    Collider,
    RigidBody,
    Velocity,
    ActiveEvents,
    ColliderMassProperties,
);

const DEBRIS_SIZE: f32 = 10.0;

use rand::*;

fn spawn_debris(
    mut spawn_debris_er: EventReader<SpawnDebrisEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in spawn_debris_er.read() {
        let velocity = (
            rand::thread_rng().gen_range(30..150),
            rand::thread_rng().gen_range(30..150),
        );
        commands.spawn(DebrisBundle(
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: DEBRIS_SIZE,
                })),
                material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
                transform: event.0,
                ..default()
            },
            DebrisTag,
            Collider::ball(DEBRIS_SIZE),
            RigidBody::Dynamic,
            Velocity {
                linvel: Vec2::new(velocity.0 as f32, velocity.1 as f32),
                ..default()
            },
            ActiveEvents::COLLISION_EVENTS,
            ColliderMassProperties::Mass(10.0),
        ));
    }
}

use crate::player::*;
use crate::Score;

// Player touches debris, despawn debris and increase score
fn collect_debris(
    mut commands: Commands,
    mut collision_er: EventReader<CollisionEvent>,
    player_q: Query<Entity, With<PlayerTag>>,
    debris_q: Query<Entity, With<DebrisTag>>,
    mut score: ResMut<Score>,
) {
    for event in collision_er.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let (_, debris_entity) = if let Ok(player) = player_q.get(*entity1) {
                    if let Ok(debris) = debris_q.get(*entity2) {
                        (player, debris)
                    } else {
                        continue;
                    }
                } else if let Ok(player) = player_q.get(*entity2) {
                    if let Ok(debris) = debris_q.get(*entity1) {
                        (player, debris)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };

                // Despawn the debris
                commands.entity(debris_entity).despawn();

                // Increase the player's score
                score.0 += 1;
            }
            _ => continue,
        }
    }
}
