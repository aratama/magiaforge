use crate::actor::Actor;
use crate::controller::player::Player;
use crate::language::Dict;
use crate::language::M18NTtext;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
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
            bottom: Val::Px(16.),
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            ..default()
        },))
        .with_child((TUtorialText, M18NTtext::new(&Dict::empty())));
}

pub fn update(
    registry: Registry,
    level: Res<LevelSetup>,
    actor_query: Query<(&Actor, &Player)>,
    mut query: Query<&mut M18NTtext, With<TUtorialText>>,
    new_spell_query: Query<&NewSpell>,
    menu_state: Res<State<GameMenuState>>,
) {
    let props = registry.game();

    if let Ok((actor, player)) = actor_query.get_single() {
        for mut text in query.iter_mut() {
            if let Some(level) = &level.level {
                if *level != GameLevel::new("warehouse") {
                    text.0 = Dict::empty();
                } else if 3 <= player.broken_chests {
                    text.0 = props.tutorial_magic_circle.clone();
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
        }
    }
}

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(GameState::InGame)));
    }
}
