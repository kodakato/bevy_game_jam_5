use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<SpawnProjectileEvent>()
            // Systems
            .add_systems(Update, (spawn_projectile, accelerate_projectiles));
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
        );

        commands.spawn(projectile_bundle);
    }
}

const PROJECTILE_ACCELERATION: f32 = 3000.0;

fn accelerate_projectiles(
    mut projectile_q: Query<(&mut ExternalImpulse, &Transform), With<ProjectileTag>>,
) {
    for (mut ext_impulse, transform) in projectile_q.iter_mut() {
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        let direction = Vec2::new(-rotation.sin(), rotation.cos());

        ext_impulse.impulse = direction * PROJECTILE_ACCELERATION;
    }
}
