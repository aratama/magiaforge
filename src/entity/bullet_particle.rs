use crate::constant::PARTICLE_LAYER_Z;
use crate::states::GameState;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Resource)]
pub struct BulletParticleResource {
    shape: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct BulletParticle {
    velocity: Vec2,
    lifetime: u16,
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

#[derive(Debug, Clone)]
pub struct SpawnParticle {
    pub scale: f32,
    pub count: u16,
    pub velocity_base: f32,
    pub velocity_random: f32,
    pub lifetime_base: u16,
    pub lifetime_random: u16,
}

impl Default for SpawnParticle {
    fn default() -> Self {
        Self {
            scale: 1.0,
            count: 10,
            velocity_base: 0.4,
            velocity_random: 0.2,
            lifetime_base: 15,
            lifetime_random: 10,
        }
    }
}

pub fn spawn_particle_system(
    commands: &mut Commands,
    position: Vec2,
    resource: &Res<BulletParticleResource>,
    spawn: &SpawnParticle,
) {
    for _ in 0..spawn.count {
        let angle = rand::random::<f32>() * PI * 2.0;
        let velocity = Vec2::from_angle(angle)
            * (spawn.velocity_base + rand::random::<f32>() * spawn.velocity_random);
        let lifetime = spawn.lifetime_base + rand::random::<u16>() % spawn.lifetime_random;
        commands.spawn((
            Name::new("bullet_particle"),
            BulletParticle { velocity, lifetime },
            Mesh2d(resource.shape.clone()),
            MeshMaterial2d(resource.material.clone()),
            Transform::from_translation(position.extend(PARTICLE_LAYER_Z)).with_scale(Vec3::new(
                spawn.scale,
                spawn.scale,
                1.0,
            )),
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
