use bevy::prelude::*;

use crate::states::GameState;

#[derive(Default, Component, Reflect)]
pub struct Breakable {
    pub life: i32,
    pub amplitude: f32,
}

#[derive(Default, Component, Reflect)]
pub struct BreakableSprite;

fn vibrate_breakabke_sprite(
    time: Res<Time>,
    mut breakable_query: Query<(&mut Breakable, &Children)>,
    mut breakable_sprite_query: Query<&mut Transform, With<BreakableSprite>>,
) {
    for (mut breakable, children) in breakable_query.iter_mut() {
        for child in children {
            if let Ok(mut transform) = breakable_sprite_query.get_mut(*child) {
                transform.translation.x =
                    (time.elapsed_seconds() * 56.0).sin() * breakable.amplitude;
            }
            breakable.amplitude *= 0.9;
        }
    }
}

pub struct BreakablePlugin;

impl Plugin for BreakablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            vibrate_breakabke_sprite.run_if(in_state(GameState::InGame)),
        );
    }
}
