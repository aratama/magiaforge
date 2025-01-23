use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::constant::MAX_SPELLS_IN_WAND;
use crate::constant::MAX_WANDS;
use crate::controller::player::Player;
use crate::registry::Registry;
use crate::states::GameState;
use crate::ui::spell_in_wand::spawn_wand_spell_slot;
use bevy::prelude::*;
use bevy::ui::Display;
use bevy::ui::Node;
use bevy_aseprite_ultra::prelude::AseUiSlice;

#[derive(Component)]
pub struct WandList;

#[derive(Component)]
struct WandSlot;

#[derive(Component)]
pub struct WandTriggerSprite(usize);

pub fn spawn_wand_list(parent: &mut ChildBuilder, registry: &Registry) {
    parent
        .spawn((
            WandList,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.),
                ..default()
            },
        ))
        .with_children(|parent| {
            for wand_index in 0..MAX_WANDS {
                spawn_wand_and_spell_slot(parent, &registry, wand_index);
            }
        });
}

fn spawn_wand_and_spell_slot(parent: &mut ChildBuilder, registry: &Registry, wand_index: usize) {
    parent
        .spawn((
            WandSlot,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            parent.spawn((
                WandTriggerSprite(wand_index),
                Node {
                    width: Val::Px(32.),
                    height: Val::Px(32.),
                    ..default()
                },
                AseUiSlice {
                    aseprite: registry.assets.atlas.clone(),
                    name: "empty".into(),
                },
            ));

            for spell_index in 0..MAX_SPELLS_IN_WAND {
                spawn_wand_spell_slot(&mut parent, &registry, wand_index, spell_index);
            }
        });
}

fn update_trigger_sprite(
    player_query: Query<&Actor, (With<Player>, With<Witch>)>,
    mut slot_query: Query<(&WandTriggerSprite, &mut AseUiSlice)>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (trigger, mut aseprite) in slot_query.iter_mut() {
            aseprite.name = match (trigger.0, actor.current_wand) {
                (0, 0) => "lt0_on",
                (0, _) => "lt0_off",
                (1, 1) => "lt1_on",
                (1, _) => "lt1_off",
                (2, 2) => "lt2_on",
                (2, _) => "lt2_off",
                _ => "rt_on",
            }
            .into();
        }
    }
}

pub struct WandListPlugin;

impl Plugin for WandListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_trigger_sprite.run_if(in_state(GameState::InGame)),
        );
    }
}
