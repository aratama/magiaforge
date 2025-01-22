pub mod life_bar;
pub mod overlay;
pub mod pointer;

use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::config::GameConfig;
use crate::constant::GameConstants;
use crate::constant::HUD_Z_INDEX;
use crate::controller::player::Player;
use crate::controller::player::PlayerDown;
use crate::message::MULTIPLAY_ARENA;
use crate::message::UNKNOWN_LEVEL;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
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

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct PlayerLifeBar;

#[derive(Component)]
pub struct PlayerGold;

fn setup_hud(
    mut commands: Commands,
    assets: Res<GameAssets>,
    ron: Res<Assets<GameConstants>>,
    next: Res<LevelSetup>,
    config: Res<GameConfig>,
) {
    let constants = ron.get(assets.spells.id()).unwrap();

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
            spawn_status_bars(&mut parent, &assets);

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
                            spawn_wand_list(&mut parent, &assets);
                        });

                    // 右下

                    let level_name = match next.next_level {
                        GameLevel::Level(i) => constants
                            .levels
                            .get(i as usize)
                            .map(|l| l.name.clone())
                            .unwrap_or(UNKNOWN_LEVEL.to_string()),
                        GameLevel::MultiPlayArena => MULTIPLAY_ARENA.to_string(),
                    };
                    let name = level_name.get(config.language).to_string();

                    parent.spawn((
                        Text(name),
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                        TextFont {
                            font: assets.noto_sans_jp.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                    ));
                });

            spawn_wand_editor(&mut parent, &assets);

            spawn_inventory_floating(&mut parent, &assets);

            spawn_boss_hitpoint_bar(&mut parent, &assets);

            spawn_drop_area(&mut parent);
        });

    spawn_speech_bubble(&mut commands, &assets);
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

fn spawn_status_bars(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
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
                &assets,
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
                            aseprite: assets.atlas.clone(),
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
                            font: assets.noto_sans_jp.clone(),
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
    player_query: Query<&Life, (With<Player>, Without<Camera2d>)>,
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
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
