use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AsepriteSlice, AsepriteSliceUiBundle};

use crate::{
    asset::GameAssets,
    spell::SpellType,
    spell_props::{spell_to_props, SpellCast},
    states::GameState,
};

#[derive(Component)]
pub struct SpellInformation(pub Option<SpellType>);

#[derive(Component)]
struct SpellIcon;

#[derive(Component)]
struct SpellName;

#[derive(Component)]
struct SpellDescription;

pub fn spawn_spell_information(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
            SpellInformation(None),
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(8.0)),
                    width: Val::Px(500.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::hsla(0.0, 0.0, 0.2, 0.95).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        SpellIcon,
                        StateScoped(GameState::MainMenu),
                        ImageBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                ..default()
                            },
                            ..default()
                        },
                        AsepriteSliceUiBundle {
                            aseprite: assets.atlas.clone(),
                            slice: "empty".into(),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        StateScoped(GameState::MainMenu),
                        SpellName,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: assets.dotgothic.clone(),
                                ..default()
                            },
                        ),
                    ));
                });

            parent.spawn((
                StateScoped(GameState::MainMenu),
                SpellDescription,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: assets.dotgothic.clone(),
                        ..default()
                    },
                ),
            ));
        });
}

fn update_spell_icon(
    mut query: Query<&mut AsepriteSlice, With<SpellIcon>>,
    spell_info: Query<&SpellInformation>,
) {
    let mut slice = query.single_mut();
    let spell_info = spell_info.single();
    if let SpellInformation(Some(info)) = spell_info {
        let props = spell_to_props(*info);
        *slice = AsepriteSlice::new(props.icon);
    } else {
        *slice = AsepriteSlice::new("empty");
    }
}

fn update_spell_name(
    mut query: Query<&mut Text, With<SpellName>>,
    spell_info: Query<&SpellInformation>,
) {
    let mut text = query.single_mut();
    let spell_info = spell_info.single();
    if let SpellInformation(Some(info)) = spell_info {
        let props = spell_to_props(*info);
        text.sections[0].value = props.name.to_string();
    } else {
        text.sections[0].value = "".to_string();
    }
}

fn update_spell_description(
    mut query: Query<&mut Text, With<SpellDescription>>,
    spell_info: Query<&SpellInformation>,
) {
    let mut text = query.single_mut();
    let spell_info = spell_info.single();
    if let SpellInformation(Some(info)) = spell_info {
        let props = spell_to_props(*info);

        let appendix = match props.category {
            SpellCast::Bullet {
                slice: _,
                collier_radius,
                speed,
                lifetime,
                damage,
                impulse,
                scattering,
                light_intensity: _,
                light_radius: _,
                light_color_hlsa: _,
            } => {
                format!(
                    "大きさ:{}\nダメージ:{}\n射出速度:{}\n持続時間:{}\nノックバック:{}\n拡散:{}",
                    collier_radius,
                    damage,
                    speed,
                    lifetime,
                    impulse * 0.001,
                    scattering
                )
            }
            SpellCast::Heal => {
                format!("回復:{}", 10)
            }
            SpellCast::BulletSpeedUpDown { delta: _ } => format!(""),
        };

        text.sections[0].value = format!(
            "{}\nマナ消費:{}\n詠唱遅延:{}\n{}",
            props.description, props.mana_drain, props.cast_delay, appendix
        )
        .to_string();
    } else {
        text.sections[0].value = "".to_string();
    }
}

pub struct SpellInformationPlugin;

impl Plugin for SpellInformationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_spell_name,
                update_spell_description,
                update_spell_icon,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
