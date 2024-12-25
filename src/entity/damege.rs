use crate::asset::GameAssets;
use crate::constant::DAMAGE_NUMBER_LAYER_Z;
use crate::entity::actor::ActorEvent;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::text::FontSmoothing;

#[derive(Component)]
struct DamageParticle {
    lifetime: usize,
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
                    commands.spawn((
                        Name::new("Damage Number"),
                        StateScoped(GameState::InGame),
                        DamageParticle { lifetime: 40 },
                        Text2d(damage.to_string()),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: assets.dotgothic.clone(),
                            font_size: 32.0,
                            font_smoothing: FontSmoothing::AntiAliased,
                            ..default()
                        },
                        Transform::from_translation(position.extend(DAMAGE_NUMBER_LAYER_Z))
                            .with_scale(Vec3::new(0.25, 0.25, 1.0)),
                    ));
                }
            }
        }
    }
}

fn update_damage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageParticle, &mut Transform)>,
) {
    for (entity, mut particle, mut _transform) in query.iter_mut() {
        particle.lifetime -= 1;
        if particle.lifetime == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_damage_number, update_damage).run_if(in_state(GameState::InGame)),
        );
    }
}
