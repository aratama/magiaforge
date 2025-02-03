use crate::actor::Actor;
use crate::controller::player::Player;
use crate::language::Dict;
use crate::language::M18NTtext;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::spell::Spell;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::new_spell::NewSpell;
use bevy::prelude::*;

#[derive(Component)]
pub struct TUtorialText;

pub fn spawn_tutorial_text(builder: &mut ChildBuilder) {
    builder
        .spawn((Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(64.),
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            ..default()
        },))
        .with_child((TUtorialText, M18NTtext::new(&Dict::empty())));
}

pub fn update(
    registry: Registry,
    setup: Res<GameWorld>,
    actor_query: Query<(&Actor, &Player, &Transform)>,
    mut query: Query<&mut M18NTtext, With<TUtorialText>>,
    new_spell_query: Query<&NewSpell>,
    menu_state: Res<State<GameMenuState>>,
) {
    let props = registry.game();

    let mut text = query.single_mut();

    let Ok((actor, player, player_transform)) = actor_query.get_single() else {
        text.0 = Dict::empty();
        return;
    };

    let player_position = player_transform.translation.truncate();

    let Some(chunk) = setup.find_chunk_by_position(player_position) else {
        text.0 = Dict::empty();
        return;
    };

    let level = &chunk.level;

    if *level != GameLevel::new("Inlet") {
        text.0 = Dict::empty();
    } else if 3 <= player.broken_chests {
        text.0 = Dict::empty();
    } else if actor.contains_in_slot(&Spell::new("MagicBolt"))
        && *menu_state == GameMenuState::Closed
    {
        text.0 = props.tutorial_cast.clone();
    } else if actor.contains_in_slot(&Spell::new("MagicBolt")) {
        text.0 = props.tutorial_close_inventory.clone();
    } else if *menu_state == GameMenuState::WandEditOpen {
        text.0 = props.tutorial_slot.clone();
    } else if !new_spell_query.is_empty() {
        text.0 = Dict::empty();
    } else if player.discovered_spells.contains(&Spell::new("MagicBolt")) {
        text.0 = props.tutorial_inventory.clone();
    } else {
        text.0 = props.tutorial_move.clone();
    }
}

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(GameState::InGame)));
    }
}
