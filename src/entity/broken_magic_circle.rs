use crate::constant::*;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct BrokenMagicCircle;

pub fn spawn_broken_magic_circle(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    x: f32,
    y: f32,
) {
    commands.spawn((
        Name::new("broken_magic_circle"),
        StateScoped(GameState::InGame),
        BrokenMagicCircle,
        AseSpriteSlice {
            aseprite: aseprite,
            name: "broken_magic_circle".into(),
        },
        Transform::from_translation(Vec3::new(x, y, PAINT_LAYER_Z)),
    ));
}
