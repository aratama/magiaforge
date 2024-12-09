use crate::states::GameState;
use bevy::{prelude::*, text::FontSmoothing};

#[derive(Component)]
struct DamageParticle {
    lifetime: usize,
}

pub fn spawn_damage_number(commands: &mut Commands, damage: i32, position: Vec2) {
    commands.spawn((
        Name::new("Damage Number"),
        StateScoped(GameState::InGame),
        DamageParticle { lifetime: 40 },
        Text2d(format!("{}", damage).to_string()),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 8.0,
            font_smoothing: FontSmoothing::None,
            ..default()
        },
        Transform::from_translation(position.extend(11.0)),
    ));
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
        app.add_systems(Update, update_damage.run_if(in_state(GameState::InGame)));
    }
}
