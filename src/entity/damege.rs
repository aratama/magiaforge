use crate::asset::GameAssets;
use crate::constant::DAMAGE_NUMBER_LAYER_Z;
use crate::entity::actor::ActorEvent;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct DamageParticle {
    origin: f32,
    velocity: f32,
    v: f32,
    lifetime: usize,
    bounce: u8,
}

fn spawn_damage_number(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut reader: EventReader<ActorEvent>,
) {
    for event in reader.read() {
        match event {
            ActorEvent::Damaged {
                damage, position, ..
            } => {
                if 0 < *damage {
                    let origin = position.extend(DAMAGE_NUMBER_LAYER_Z);
                    commands
                        .spawn((
                            Name::new("Damage Number"),
                            StateScoped(GameState::InGame),
                            DamageParticle {
                                lifetime: 80,
                                origin: origin.y,
                                velocity: 0.0,
                                v: 10.0,
                                bounce: 2,
                            },
                            Transform::from_translation(origin),
                            Visibility::default(),
                        ))
                        .with_children(|builder| {
                            for y in -1..2 {
                                for x in -1..2 {
                                    let center = x == 0 && y == 0;
                                    let dz = if center { 0.0001 } else { 0.0 };
                                    let color = if center { Color::WHITE } else { Color::BLACK };
                                    builder.spawn((
                                        Text2d(damage.to_string()),
                                        TextColor(color),
                                        TextFont {
                                            font: assets.noto_sans_jp.clone(),
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        Transform::from_xyz(x as f32, y as f32, dz)
                                            .with_scale(Vec3::new(0.25, 0.25, 1.0)),
                                    ));
                                }
                            }
                        });
                }
            }
            ActorEvent::FatalFall { .. } => {}
        }
    }
}

fn update_damage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageParticle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in query.iter_mut() {
        particle.velocity -= 0.05;
        particle.v += particle.velocity;
        if particle.v <= 0.0 {
            particle.v = 0.0;
            if 0 < particle.bounce {
                particle.velocity = particle.velocity * -0.5;
                particle.bounce -= 1;
            } else {
                particle.velocity = 0.0;
            }
        }
        transform.translation.y = particle.origin + particle.v;
        transform.translation.z -= 0.0001;

        particle.lifetime -= 1;
        if particle.lifetime == 0 {
            commands.entity(entity).despawn_recursive();
            // info!("despawn {} {}", file!(), line!());
        }
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (spawn_damage_number, update_damage).in_set(FixedUpdateGameActiveSet),
        );
    }
}
