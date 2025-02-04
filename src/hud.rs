pub mod life_bar;
pub mod overlay;
pub mod pointer;
pub mod tutorial;

use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::constant::HUD_Z_INDEX;
use crate::controller::player::Player;
use crate::controller::player::PlayerDown;
use crate::language::M18NTtext;
use crate::level::world::GameWorld;
use crate::registry::Registry;
use crate::states::GameState;
use crate::ui::bar::spawn_status_bar;
use crate::ui::bar::StatusBar;
use crate::ui::boss_hitpoint_bar::spawn_boss_hitpoint_bar;
use crate::ui::floating::spawn_inventory_floating;
use crate::ui::floating::Floating;
use crate::ui::speech_bubble::spawn_speech_bubble;
use crate::ui::wand_editor::spawn_wand_editor;
use crate::ui::wand_list::spawn_wand_list;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseUiSlice;
use tutorial::spawn_tutorial_text;

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct PlayerLifeBar;

#[derive(Component)]
pub struct PlayerGold;

#[derive(Component)]
pub struct LevelName;

fn setup_hud(mut commands: Commands, registry: Registry) {
    commands
        .spawn((
            Name::new("hud_root"),
            StateScoped(GameState::InGame),
            GlobalZIndex(HUD_Z_INDEX),
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                row_gap: Val::Px(4.),
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                padding: UiRect {
                    left: Val::Px(8.),
                    right: Val::Px(8.),
                    top: Val::Px(8.),
                    bottom: Val::Px(8.),
                },
                ..default()
            },
        ))
        .with_children(|mut parent| {
            // 左上
            spawn_status_bars(&mut parent, &registry);

            // 下半分
            parent
                .spawn((
                    GlobalZIndex(HUD_Z_INDEX),
                    Node {
                        width: Val::Percent(100.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // 左下
                    parent
                        .spawn((
                            GlobalZIndex(HUD_Z_INDEX),
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                row_gap: Val::Px(4.),
                                ..default()
                            },
                        ))
                        .with_children(|mut parent| {
                            spawn_wand_list(&mut parent, &registry);
                        });

                    // 右下
                    parent.spawn((
                        LevelName,
                        M18NTtext::empty(),
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                        TextFont {
                            font: registry.assets.noto_sans_jp.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                    ));
                });

            spawn_wand_editor(&mut parent, &registry);

            spawn_inventory_floating(&mut parent, &registry);

            spawn_boss_hitpoint_bar(&mut parent, &registry);

            spawn_drop_area(&mut parent);

            spawn_tutorial_text(&mut parent);
        });

    spawn_speech_bubble(&mut commands, &registry);
}

#[derive(Component)]
pub struct DropArea {
    pub hover: bool,
}

fn spawn_drop_area(parent: &mut ChildBuilder) {
    parent.spawn((
        DropArea { hover: false },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(540.0),
            top: Val::Px(260.0),
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor(Color::hsva(0.0, 0.0, 1.0, 0.2)),
        Interaction::default(),
    ));
}

fn drop_area_interaction(
    mut quary: Query<(&mut DropArea, &Interaction, &mut BorderColor), Changed<Interaction>>,
) {
    for (mut area, interaction, mut border) in quary.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                area.hover = true;
                border.0.set_alpha(0.5);
            }
            Interaction::None => {
                area.hover = false;
                border.0.set_alpha(0.2);
            }
            _ => {}
        }
    }
}

fn drop_area_visibility(
    mut droparea_quary: Query<&mut Node, With<DropArea>>,
    floating_query: Query<&Floating, Changed<Floating>>,
) {
    if let Ok(floating) = floating_query.get_single() {
        let mut node = droparea_quary.single_mut();
        node.display = match floating.content {
            Some(_) => Display::Flex,
            _ => Display::None,
        };
    }
}

fn spawn_status_bars(parent: &mut ChildBuilder, registry: &Registry) {
    parent
        .spawn((Node {
            display: Display::Flex,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            row_gap: Val::Px(8.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..default()
        },))
        .with_children(|mut parent| {
            spawn_status_bar(
                &mut parent,
                registry,
                PlayerLifeBar,
                0,
                0,
                Color::hsla(110., 0.7, 0.7, 0.9),
            );

            parent
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        AseUiSlice {
                            aseprite: registry.assets.atlas.clone(),
                            name: "gold_icon".to_string(),
                        },
                        Transform::from_scale(Vec3::new(2.0, 2.0, 1.0)),
                    ));

                    parent.spawn((
                        PlayerGold,
                        Name::new("golds"),
                        GlobalZIndex(HUD_Z_INDEX),
                        Text::new(""),
                        TextColor(Color::hsla(57.0, 1.0, 0.5, 0.7)),
                        TextFont {
                            font: registry.assets.noto_sans_jp.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        HUD,
                    ));
                });
        });
}

fn update_life_in_hud(
    // ライフバーのみ、変身中も現在の姿のライフを表示します
    // なお、そのほかのゴールドやスペルは変身前の状態を表示しています
    player_query: Query<&Actor, (With<Player>, Without<Camera2d>)>,
    mut player_life_query: Query<&mut StatusBar, With<PlayerLifeBar>>,
    player_down_query: Query<&PlayerDown>,
) {
    let mut player_life = player_life_query.single_mut();
    if let Ok(actor_life) = player_query.get_single() {
        player_life.value = actor_life.life;
        player_life.max_value = actor_life.max_life;
    } else if player_down_query.is_empty() {
        // ワープでプレイヤーがいない
    } else {
        player_life.value = 0;
    }
}

fn update_golds_in_hud(
    player_query: Query<&Actor, (With<Player>, With<Witch>, Without<Camera2d>)>,
    mut player_gold_query: Query<&mut Text, With<PlayerGold>>,
) {
    let mut player_gold = player_gold_query.single_mut();
    if let Ok(actor) = player_query.get_single() {
        player_gold.0 = format!("{}", actor.golds);
    }
}

fn update_level_name_in_hud(
    world: Res<GameWorld>,
    registry: Registry,
    mut level_name_query: Query<&mut M18NTtext, With<LevelName>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let player_position = player_transform.translation.truncate();

    let Some(chunk) = world.find_chunk_by_position(player_position) else {
        return;
    };

    let level_props = registry.get_level(&chunk.level);

    let mut level_name = level_name_query.single_mut();
    level_name.0 = level_props.get_name();
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        app.add_systems(
            Update,
            (
                update_life_in_hud,
                update_golds_in_hud,
                drop_area_interaction,
                drop_area_visibility,
                update_level_name_in_hud,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
