use bevy::{
    prelude::*,
    ui::{Display, Style},
};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    asset::GameAssets,
    controller::player::{self, Player},
    entity::{actor::Actor, witch::Witch},
    states::GameState,
};

pub enum Spell {
    MagicBolt,
    SlimeCharge,
}

pub enum WandType {
    CypressWand,
}

pub struct Wand {
    pub slots: Vec<Option<Spell>>,
}

#[derive(Component)]
pub struct WandList;

#[derive(Component)]
pub struct WandSlot;

pub fn spawn_wand_list(parent: &mut ChildBuilder) {
    parent.spawn((
        WandList,
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.),
                ..default()
            },
            ..default()
        },
    ));
}

fn update(
    mut commands: Commands,
    player_query: Query<&Witch, With<Player>>,
    wand_slots_query: Query<(Entity, &WandSlot)>,
    mut wand_list_query: Query<(Entity, &WandList)>,
    assets: Res<GameAssets>,
) {
    if let Ok(witch) = player_query.get_single() {
        let (wand_list, _) = wand_list_query.single_mut();
        for _ in wand_slots_query.iter().count()..witch.wands.len() {
            commands.entity(wand_list).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(2.),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            WandSlot,
                            ImageBundle {
                                style: Style {
                                    width: Val::Px(64.),
                                    height: Val::Px(32.),
                                    ..default()
                                },
                                ..default()
                            },
                            AsepriteSliceUiBundle {
                                aseprite: assets.asset.clone(),
                                slice: "wand".into(),
                                ..default()
                            },
                        ));

                        for _ in 0..5 {
                            parent.spawn((
                                ImageBundle {
                                    style: Style {
                                        width: Val::Px(32.),
                                        height: Val::Px(32.),
                                        border: UiRect::all(Val::Px(1.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                BorderColor(Color::WHITE),
                                AsepriteSliceUiBundle {
                                    aseprite: assets.asset.clone(),
                                    slice: "bullet".into(),
                                    ..default()
                                },
                            ));
                        }
                    });
            });
        }
        for _ in witch.wands.len()..wand_slots_query.iter().count() {
            wand_slots_query.iter().last().map(|(entity, _)| {
                commands.entity(entity).despawn_recursive();
            });
        }
    }
}

pub struct WandListPlugin;

impl Plugin for WandListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(GameState::InGame)));
    }
}
