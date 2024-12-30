use super::popup::{PopUp, PopupContent};
use crate::{
    asset::GameAssets,
    camera::GameCamera,
    constant::{UI_PRIMARY, UI_PRIMARY_DARKER, UI_SECONDARY},
    controller::{message_rabbit::SpellListRabbit, player::Player},
    language::M18NTtext,
    message::DISCOVERED_SPELLS,
    spell::SpellType,
    states::GameState,
};
use bevy::{prelude::*, text::FontSmoothing};
use bevy_aseprite_ultra::prelude::*;
use strum::IntoEnumIterator;

const TILE_SIZE: f32 = 32.0;

const ROWS: usize = 8;

const COLUMNS: usize = 8;

const RIGHT_ON_OPEN: f32 = 60.0;

const RIGHT_ON_CLOSE: f32 = -400.0;

#[derive(Component)]
struct SpellList {
    right: f32,
}

#[derive(Component)]
struct SpellListItem {
    spell_type: Option<SpellType>,
}

#[derive(Component)]
struct DicoveredSpellCount;

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    let mut spells: Vec<Option<SpellType>> = SpellType::iter().map(|s| Some(s)).collect();
    spells.extend(vec![None; ROWS * COLUMNS - SpellType::iter().count()]);

    commands
        .spawn((
            Name::new("Spell list root"),
            SpellList {
                right: RIGHT_ON_CLOSE,
            },
            StateScoped(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(150.0),
                right: Val::Px(RIGHT_ON_CLOSE),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(UI_SECONDARY),
        ))
        .with_children(|commands| {
            commands.spawn((
                M18NTtext(DISCOVERED_SPELLS.to_string()),
                TextFont {
                    font: assets.dotgothic.clone(),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                Label,
            ));

            commands.spawn((
                DicoveredSpellCount,
                Text::new(""),
                TextFont {
                    font: assets.dotgothic.clone(),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                Label,
            ));

            commands
                .spawn((Node {
                    width: Val::Px(TILE_SIZE * COLUMNS as f32),
                    height: Val::Px(TILE_SIZE * ROWS as f32),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::min_content(); COLUMNS],
                    grid_template_rows: vec![GridTrack::min_content(); ROWS],
                    ..default()
                },))
                .with_children(|builder| {
                    for (i, spell_type) in spells.iter().enumerate() {
                        let x = i % COLUMNS;
                        let y = i / COLUMNS;
                        let props = spell_type.map(|s| s.to_props());
                        let icon = props.map(|s| s.icon).unwrap_or("unknown");
                        builder.spawn((
                            SpellListItem {
                                spell_type: *spell_type,
                            },
                            AseUiSlice {
                                aseprite: assets.atlas.clone(),
                                name: icon.into(),
                            },
                            Node {
                                width: Val::Px(TILE_SIZE),
                                height: Val::Px(TILE_SIZE),
                                ..default()
                            },
                            BackgroundColor(if (x + y) % 2 == 0 {
                                UI_PRIMARY
                            } else {
                                UI_PRIMARY_DARKER
                            }),
                            Interaction::default(),
                        ));
                    }
                });
        });
}

fn update_right(
    mut query: Query<(&mut SpellList, &mut Node)>,
    camera_query: Query<&GameCamera>,
    spell_list_rabbit_query: Query<&SpellListRabbit>,
) {
    let camera = camera_query.single();

    let open = if let Some(target) = camera.target {
        spell_list_rabbit_query.contains(target)
    } else {
        false
    };

    let (mut spell_list, mut node) = query.single_mut();
    spell_list.right +=
        ((if open { RIGHT_ON_OPEN } else { RIGHT_ON_CLOSE }) - spell_list.right) * 0.1;
    node.right = Val::Px(spell_list.right);
}

fn update_icons(
    mut query: Query<(&mut SpellListItem, &mut AseUiSlice)>,
    player_query: Query<&Player>,
) {
    if let Ok(player) = player_query.get_single() {
        for (item, mut aseprite) in query.iter_mut() {
            if let Some(spell_type) = item.spell_type {
                let props = spell_type.to_props();
                let discovered = player.discovered_spells.contains(&spell_type);
                aseprite.name = (if discovered { props.icon } else { "unknown" }).into()
            }
        }
    }
}

fn update_discovered_items_count(
    mut query: Query<&mut Text, With<DicoveredSpellCount>>,
    player_query: Query<&Player>,
) {
    if let Ok(player) = player_query.get_single() {
        for mut text in query.iter_mut() {
            text.0 = format!(
                "{} / {}",
                player.discovered_spells.len(),
                SpellType::iter().count()
            );
        }
    }
}

fn interaction(
    mut interaction_query: Query<(&SpellListItem, &Interaction), Changed<Interaction>>,
    mut popup_query: Query<&mut PopUp>,
    player_query: Query<&Player>,
) {
    if let Ok(player) = player_query.get_single() {
        let mut popup = popup_query.single_mut();
        for (item, interaction) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {}
                Interaction::Hovered => {
                    if let Some(spell) = item.spell_type {
                        if player.discovered_spells.contains(&spell) {
                            popup.set.insert(PopupContent::DiscoveredSpell(spell));
                            popup.anchor_left = false;
                            popup.anchor_top = true;
                        }
                    }
                }
                Interaction::None => {
                    if let Some(spell) = item.spell_type {
                        popup.set.remove(&PopupContent::DiscoveredSpell(spell));
                    }
                }
            }
        }
    }
}

pub struct SpellListPlugin;

impl Plugin for SpellListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup);
        app.add_systems(
            Update,
            (
                update_right,
                update_icons,
                interaction,
                update_discovered_items_count,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
