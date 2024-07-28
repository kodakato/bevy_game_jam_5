use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<SpawnProjectileEvent>()
            // Systems
            .add_systems(
                Update,
                (spawn_projectile, accelerate_projectiles, hit_object),
            );
    }
}

#[derive(Event)]
pub struct SpawnProjectileEvent(pub Transform, pub Velocity);

#[derive(Default, Bundle)]
pub struct ProjectileBundle(
    RigidBody,
    Collider,
    Sensor,
    Velocity,
    SpriteBundle,
    ProjectileTag,
    ExternalImpulse,
    ColliderMassProperties,
    ActiveEvents,
);

#[derive(Component, Default)]
pub struct ProjectileTag;

struct ProjectileSize {
    height: f32,
    width: f32,
}

const PROJECTILE_SIZE: ProjectileSize = ProjectileSize {
    height: 10.0,
    width: 5.0,
};

fn spawn_projectile(
    mut commands: Commands,
    mut spawn_projectile_er: EventReader<SpawnProjectileEvent>,
) {
    for event in spawn_projectile_er.read() {
        let projectile_bundle = ProjectileBundle(
            RigidBody::Dynamic,
            Collider::capsule_y(PROJECTILE_SIZE.height / 2.0, PROJECTILE_SIZE.width / 2.0),
            Sensor,
            event.1,
            SpriteBundle {
                transform: event.0,
                ..default()
            },
            ProjectileTag,
            ExternalImpulse::default(),
            ColliderMassProperties::Mass(1.0),
            ActiveEvents::COLLISION_EVENTS,
        );

        commands.spawn(projectile_bundle);
    }
}

const PROJECTILE_ACCELERATION: f32 = 400.0;

fn accelerate_projectiles(
    mut projectile_q: Query<(&mut ExternalImpulse, &Transform), With<ProjectileTag>>,
) {
    for (mut ext_impulse, transform) in projectile_q.iter_mut() {
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        let direction = Vec2::new(-rotation.sin(), rotation.cos());

        ext_impulse.impulse += direction * PROJECTILE_ACCELERATION;
    }
}

use crate::{explosion::*, player::*};

fn hit_object(
    projectile_q: Query<(Entity, &Transform), With<ProjectileTag>>,
    mut collision_er: EventReader<CollisionEvent>,
    object_q: Query<Entity, Without<PlayerTag>>,
    mut spawn_explosion_ew: EventWriter<SpawnExplosionEvent>,
    mut commands: Commands,
) {
    for event in collision_er.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let projectile_entity;
                let projectile_transform;

                if let Ok((entity, transform)) = projectile_q.get(*entity1) {
                    if object_q.get(*entity2).is_ok() {
                        projectile_entity = entity;
                        projectile_transform = transform;
                    } else {
                        continue;
                    }
                } else if let Ok((entity, transform)) = projectile_q.get(*entity2) {
                    if object_q.get(*entity1).is_ok() {
                        projectile_entity = entity;
                        projectile_transform = transform;
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }

                // Spawn explosion at the position of the projectile
                spawn_explosion_ew.send(SpawnExplosionEvent(*projectile_transform));

                // Despawn the projectile
                commands.entity(projectile_entity).despawn();
            }
            _ => continue,
        }
    }
}
