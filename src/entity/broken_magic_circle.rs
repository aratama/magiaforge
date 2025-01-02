use crate::constant::*;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct BrokenMagicCircle;

pub fn spawn_broken_magic_circle(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
) {
    commands.spawn((
        Name::new("broken_magic_circle"),
        StateScoped(GameState::InGame),
        BrokenMagicCircle,
        AseSpriteSlice {
            aseprite: aseprite,
            name: "broken_magic_circle".into(),
        },
        Transform::from_translation(position.extend(PAINT_LAYER_Z)),
    ));
}
