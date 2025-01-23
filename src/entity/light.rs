use crate::states::GameState;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;

#[derive(Component)]
pub struct FlashLight {
    pub intensity: f32,
    pub duration: u32,
    pub count: u32,
    pub reverse: bool,
}

pub fn spawn_flash_light(
    commands: &mut Commands,
    position: Vec2,
    intensity: f32,
    radius: f32,
    duration: u32,
    reverse: bool,
) {
    commands.spawn((
        StateScoped(GameState::InGame),
        FlashLight {
            intensity,
            duration,
            count: 0,
            reverse,
        },
        Transform::from_translation(position.extend(0.0)),
        PointLight2d {
            intensity,
            radius,
            ..default()
        },
    ));
}

fn flash_ligh_fade_out(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FlashLight, &mut PointLight2d)>,
) {
    for (entity, mut flash, mut light) in query.iter_mut() {
        if flash.count < flash.duration {
            flash.count += 1;
            let t = flash.count as f32 / flash.duration as f32;
            light.intensity = flash.intensity * if flash.reverse { t } else { 1.0 - t };
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct FlashLightPlugin;

impl Plugin for FlashLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flash_ligh_fade_out,)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
    }
}
