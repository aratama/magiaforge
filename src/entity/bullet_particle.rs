use bevy::prelude::*;
use std::f32::consts::PI;

use crate::states::GameState;

#[derive(Resource)]
pub struct BulletParticleResource {
    shape: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct BulletParticle {
    velocity: Vec2,
    lifetime: usize,
}

fn setup_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(1.0, 1.0));
    let material = materials.add(Color::WHITE);
    commands.insert_resource(BulletParticleResource { shape, material });
}

pub fn spawn_particle_system(
    commands: &mut Commands,
    position: Vec2,
    resource: &Res<BulletParticleResource>,
) {
    for _ in 0..10 {
        let angle = rand::random::<f32>() * PI * 2.0;
        let velocity = Vec2::from_angle(angle) * (0.4 + rand::random::<f32>() * 0.2);
        let lifetime = 15 + rand::random::<usize>() % 10;
        commands.spawn((
            BulletParticle { velocity, lifetime },
            Mesh2d(resource.shape.clone()),
            MeshMaterial2d(resource.material.clone()),
            Transform::from_translation(position.extend(11.0)),
        ));
    }
}

fn update_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BulletParticle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in query.iter_mut() {
        particle.lifetime -= 1;
        if particle.lifetime == 0 {
            commands.entity(entity).despawn();
        } else {
            transform.translation += particle.velocity.extend(0.0);
        }
    }
}

pub struct BulletParticlePlugin;

impl Plugin for BulletParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bullet);
        app.add_systems(Update, update_particles.run_if(in_state(GameState::InGame)));
    }
}
