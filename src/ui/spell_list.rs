use crate::constant::UI_PRIMARY;
use crate::constant::UI_PRIMARY_DARKER;
use crate::constant::UI_SECONDARY;
use crate::controller::player::Player;
use crate::language::M18NTtext;
use crate::message::DISCOVERED_SPELLS;
use crate::registry::Registry;
use crate::spell::Spell;
use crate::states::GameState;
use crate::ui::popup::PopUp;
use crate::ui::popup::PopupContent;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

const TILE_SIZE: f32 = 32.0;

const ROWS: usize = 8;

const COLUMNS: usize = 8;

const RIGHT_ON_OPEN: f32 = 60.0;

const RIGHT_ON_CLOSE: f32 = -400.0;

#[derive(Component)]
pub struct SpellList {
    pub open: bool,
    right: f32,
}

#[derive(Component)]
struct SpellListItem {
    spell_type: Option<Spell>,
}

#[derive(Component)]
struct DicoveredSpellCount;

fn setup(mut commands: Commands, registry: Registry) {
    let spells = registry.get_spells();
    let spell_iter = spells.iter().cloned();
    let count = spell_iter.clone().count();
    let mut spells: Vec<Option<Spell>> = spell_iter.map(|s| Some(s)).collect();
    spells.sort_by(|a, b| match (a, b) {
        (Some(a), Some(b)) => {
            let a_props = registry.get_spell_props(a);
            let b_props = registry.get_spell_props(b);
            a_props.rank.cmp(&b_props.rank)
        }
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });
    spells.extend(vec![None; ROWS * COLUMNS - count]);

    commands
        .spawn((
            Name::new("Spell list root"),
            SpellList {
                open: false,
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
                    font: registry.assets.noto_sans_jp.clone(),
                    font_size: 24.0,
                    ..default()
                },
                Label,
            ));

            commands.spawn((
                DicoveredSpellCount,
                Text::new(""),
                TextFont {
                    font: registry.assets.noto_sans_jp.clone(),
                    font_size: 24.0,
                    ..default()
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

                        let icon = spell_type
                            .as_ref()
                            .map(|spell| registry.get_spell_props(&spell).icon.clone())
                            .unwrap_or("unknown".to_string());
                        builder.spawn((
                            SpellListItem {
                                spell_type: spell_type.clone(),
                            },
                            AseUiSlice {
                                aseprite: registry.assets.atlas.clone(),
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

fn update_right(mut query: Query<(&mut SpellList, &mut Node)>) {
    let (mut spell_list, mut node) = query.single_mut();
    spell_list.right += ((if spell_list.open {
        RIGHT_ON_OPEN
    } else {
        RIGHT_ON_CLOSE
    }) - spell_list.right)
        * 0.1;
    node.right = Val::Px(spell_list.right);
}

fn update_icons(
    registry: Registry,
    mut query: Query<(&mut SpellListItem, &mut AseUiSlice)>,
    player_query: Query<&Player>,
) {
    if let Ok(player) = player_query.get_single() {
        for (item, mut aseprite) in query.iter_mut() {
            if let Some(spell_type) = &item.spell_type {
                let props = registry.get_spell_props(&spell_type);
                let discovered = player.discovered_spells.contains(&spell_type);
                aseprite.name = (if discovered {
                    props.icon.clone()
                } else {
                    "unknown".to_string()
                })
                .into()
            } else {
                aseprite.name = "empty".into();
            }
        }
    }
}

fn update_discovered_items_count(
    registry: Registry,
    mut query: Query<&mut Text, With<DicoveredSpellCount>>,
    player_query: Query<&Player>,
) {
    if let Ok(player) = player_query.get_single() {
        for mut text in query.iter_mut() {
            text.0 = format!(
                "{} / {}",
                player.discovered_spells.len(),
                registry.get_spells().iter().count()
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
                    if let Some(spell) = &item.spell_type {
                        if player.discovered_spells.contains(&spell) {
                            popup
                                .set
                                .insert(PopupContent::DiscoveredSpell(spell.clone()));
                            popup.anchor_left = false;
                            popup.anchor_top = true;
                        }
                    }
                }
                Interaction::None => {
                    if let Some(spell) = &item.spell_type {
                        popup
                            .set
                            .remove(&PopupContent::DiscoveredSpell(spell.clone()));
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
