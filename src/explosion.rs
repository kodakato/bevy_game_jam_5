use bevy::{prelude::*, sprite::*};
use bevy_rapier2d::prelude::*;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<SpawnExplosionEvent>()
            // Systems
            .add_systems(Update, (spawn_explosion, despawn_explosions));
    }
}

#[derive(Component)]
pub struct ExplosionTag;

#[derive(Component)]
struct ExplosionTimer(pub Timer);

#[derive(Bundle)]
struct ExplosionBundle(
    MaterialMesh2dBundle<ColorMaterial>,
    ExplosionTag,
    Collider,
    ExplosionTimer,
    Sensor,
    ActiveEvents,
);

#[derive(Event)]
pub struct SpawnExplosionEvent(pub Transform);

const EXPLOSION_RADIUS: f32 = 20.0;
const EXPLOSION_TIME: f32 = 0.05;

fn spawn_explosion(
    mut spawn_explosion_er: EventReader<SpawnExplosionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in spawn_explosion_er.read() {
        commands.spawn(ExplosionBundle(
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: EXPLOSION_RADIUS,
                })),
                material: materials.add(Color::srgb(0.2, 0.5, 0.0)),
                transform: event.0,
                ..default()
            },
            ExplosionTag,
            Collider::ball(EXPLOSION_RADIUS),
            ExplosionTimer(Timer::from_seconds(EXPLOSION_TIME, TimerMode::Once)),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

fn despawn_explosions(
    mut commands: Commands,
    mut explosion_q: Query<(Entity, &mut ExplosionTimer), With<ExplosionTag>>,
    time: Res<Time>,
) {
    for (explosion_entity, mut explosion_timer) in explosion_q.iter_mut() {
        explosion_timer.0.tick(time.delta());
        if explosion_timer.0.finished() {
            // Despawn the explosion
            commands.entity(explosion_entity).despawn();
        }
    }
}
