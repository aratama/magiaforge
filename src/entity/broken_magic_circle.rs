use crate::constant::*;
use crate::level::world::GameLevel;
use crate::level::world::LevelScoped;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct BrokenMagicCircle;

pub fn spawn_broken_magic_circle(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    level: &GameLevel,
    position: Vec2,
) {
    commands.spawn((
        Name::new("broken_magic_circle"),
        LevelScoped(level.clone()),
        StateScoped(GameState::InGame),
        BrokenMagicCircle,
        AseSpriteSlice {
            aseprite: aseprite,
            name: "broken_magic_circle".into(),
        },
        Transform::from_translation(position.extend(PAINT_LAYER_Z)),
    ));
}
