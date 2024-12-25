use crate::asset::GameAssets;
use crate::constant::MAX_SPELLS_IN_WAND;
use crate::constant::MAX_WANDS;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::states::GameState;
use crate::ui::spell_in_wand::spawn_wand_spell_slot;
use crate::ui::wand_sprite::spawn_wand_sprite_in_list;
use bevy::prelude::*;
use bevy::ui::Display;
use bevy::ui::Node;
use bevy_aseprite_ultra::prelude::AseUiSlice;

#[derive(Component)]
pub struct WandList;

#[derive(Component)]
pub struct WandSlot {
    wand_index: usize,
}

#[derive(Component)]
struct TriggerMarker;

pub fn spawn_wand_list(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
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
                spawn_wand_and_spell_slot(parent, &assets, wand_index);
            }
        });
}

fn spawn_wand_and_spell_slot(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
) {
    parent
        .spawn((
            WandSlot { wand_index },
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ))
        .with_children(|mut parent| {
            spawn_wand_sprite_in_list(&mut parent, &assets);

            for spell_index in 0..MAX_SPELLS_IN_WAND {
                spawn_wand_spell_slot(&mut parent, &assets, wand_index, spell_index);
            }

            parent.spawn((
                TriggerMarker,
                AseUiSlice {
                    aseprite: assets.atlas.clone(),
                    name: if wand_index == MAX_WANDS - 1 {
                        "rt".to_string()
                    } else {
                        "lt".to_string()
                    },
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    width: Val::Px(18.),
                    height: Val::Px(14.),
                    ..default()
                },
                ZIndex(100),
            ));
        });
}

fn update_trigger_marker(
    mut marker_query: Query<(&Parent, &mut Node), With<TriggerMarker>>,
    slot_query: Query<&WandSlot>,
    actor_query: Query<&Actor, With<Player>>,
) {
    if let Ok(actor) = actor_query.get_single() {
        for (parent, mut node) in marker_query.iter_mut() {
            let slot = slot_query.get(parent.get()).unwrap();
            node.display =
                if slot.wand_index == actor.current_wand || slot.wand_index == MAX_WANDS - 1 {
                    Display::Flex
                } else {
                    Display::None
                };
        }
    }
}

pub struct WandListPlugin;

impl Plugin for WandListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_trigger_marker).run_if(in_state(GameState::InGame)),
        );
    }
}
