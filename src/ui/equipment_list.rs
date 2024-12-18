use crate::entity::actor::Actor;
use crate::{
    asset::GameAssets, constant::MAX_SPELLS_IN_WAND, controller::player::Player, states::GameState,
};
use crate::{
    states::GameMenuState,
    ui::floating::{Floating, FloatingContent},
};
use bevy::{prelude::*, ui::Display};
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct EquipmentContainer;

#[derive(Component, Debug, Clone)]
struct EquipmentSprite {
    index: usize,
}

#[derive(Component)]
struct ChargeAlert;

pub fn spawn_equipment_list(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            EquipmentContainer,
            BorderColor(Color::hsla(0.0, 0.0, 1.0, 0.0)),
            Node {
                width: Val::Px(64. + 32. * MAX_SPELLS_IN_WAND as f32),
                height: Val::Px(32.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
        ))
        .with_children(|mut parent| {
            for index in 0..MAX_SPELLS_IN_WAND {
                spawn_equipment_slot(&mut parent, &assets, index);
            }
        });
}

fn spawn_equipment_slot(parent: &mut ChildBuilder, assets: &Res<GameAssets>, index: usize) {
    parent
        .spawn((
            EquipmentSprite { index },
            Interaction::default(),
            BackgroundColor(slot_color(5, index)),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(64.0 + 32. * (index as f32)),
                width: Val::Px(32.),
                height: Val::Px(32.),
                ..default()
            },
            AseUiSlice {
                aseprite: assets.atlas.clone(),
                name: "empty".into(),
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                ChargeAlert,
                AseUiSlice {
                    aseprite: assets.atlas.clone(),
                    name: "charge_alert".into(),
                },
            ));
        });
}

fn update_equipment_sprite(
    mut sprite_query: Query<(&EquipmentSprite, &mut AseUiSlice)>,
    player_query: Query<&Actor, With<Player>>,
    floating_query: Query<&mut Floating>,
) {
    if let Ok(actor) = player_query.get_single() {
        for (sprite, mut slice) in sprite_query.iter_mut() {
            let float = floating_query.single();
            let picked = match float.content {
                Some(FloatingContent::Equipment(index)) => index == sprite.index,
                _ => false,
            };
            if picked {
                slice.name = "empty".into();
            } else if let Some(ref equipment) = actor.equipments[sprite.index] {
                slice.name = equipment.equipment_type.to_props().icon.into();
            } else {
                slice.name = "empty".into();
            }
        }
    }
}

fn interact(
    sprite_query: Query<(&EquipmentSprite, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    state: Res<State<GameMenuState>>,
) {
    if *state == GameMenuState::WandEditOpen {
        let mut floating = floating_query.single_mut();
        for (sprite, interaction) in sprite_query.iter() {
            match interaction {
                Interaction::Pressed => {
                    floating.content = Some(FloatingContent::Equipment(sprite.index));
                }
                Interaction::Hovered => {
                    floating.target = Some(FloatingContent::Equipment(sprite.index));
                }
                _ => {}
            }
        }
    }
}

fn update_alert_visibility(
    player_query: Query<&Actor, With<Player>>,
    slot_query: Query<&EquipmentSprite>,
    mut alert_query: Query<(&Parent, &mut Visibility), With<ChargeAlert>>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();
    if let Ok(actor) = player_query.get_single() {
        for (parent, mut alert) in alert_query.iter_mut() {
            if let Ok(slot) = slot_query.get(parent.get()) {
                match floating.content {
                    Some(FloatingContent::Equipment(index)) if index == slot.index => {
                        *alert = Visibility::Hidden;
                    }
                    _ => {
                        *alert = match actor.equipments[slot.index] {
                            Some(item) if 0 < item.price => Visibility::Inherited,
                            _ => Visibility::Hidden,
                        };
                    }
                }
            } else {
                *alert = Visibility::Hidden;
            }
        }
    }
}

fn slot_color(wand_index: usize, spell_index: usize) -> Color {
    return Color::hsla(
        120.0,
        0.3,
        0.4,
        if (wand_index + spell_index) % 2 == 0 {
            0.1
        } else {
            0.12
        },
    );
}

pub struct EquipmentListPlugin;

impl Plugin for EquipmentListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_equipment_sprite, interact, update_alert_visibility)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
