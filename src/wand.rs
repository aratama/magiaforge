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

pub fn setup(mut commands: Commands) {
    commands.spawn((
        WandList,
        NodeBundle {
            style: Style {
                display: Display::Flex,
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
                parent.spawn((
                    WandSlot,
                    AsepriteSliceUiBundle {
                        aseprite: assets.asset.clone(),
                        slice: "wand".into(),
                        ..default()
                    },
                ));
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
        app.add_systems(OnEnter(GameState::InGame), setup);
        app.add_systems(Update, update.run_if(in_state(GameState::InGame)));
    }
}
