pub mod life_bar;
pub mod overlay;
pub mod pointer;

use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::constant::HUD_Z_INDEX;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::level::{level_to_name, GameLevel, NextLevel};
use crate::states::GameState;
use crate::ui::bar::{spawn_status_bar, StatusBar};
use crate::ui::equipment_list::spawn_equipment_list;
use crate::ui::floating::spawn_inventory_floating;
use crate::ui::wand_editor::spawn_wand_editor;
use crate::ui::wand_list::spawn_wand_list;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use iyes_perf_ui::entries::PerfUiBundle;

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct PlayerLifeBar;

#[derive(Component)]
pub struct PlayerManaBar;

#[derive(Component)]
pub struct PlayerGold;

fn setup_hud(
    mut commands: Commands,
    assets: Res<GameAssets>,
    next: Res<NextLevel>,
    config: Res<GameConfig>,
) {
    commands
        .spawn((
            Name::new("hud_root"),
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
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
                z_index: ZIndex::Global(HUD_Z_INDEX),
                ..default()
            },
        ))
        .with_children(|mut parent| {
            // 左上
            spawn_status_bars(&mut parent);

            // 下半分
            parent
                .spawn((NodeBundle {
                    z_index: ZIndex::Global(HUD_Z_INDEX),
                    style: Style {
                        width: Val::Percent(100.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    // 左下
                    parent
                        .spawn((NodeBundle {
                            z_index: ZIndex::Global(HUD_Z_INDEX),
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                row_gap: Val::Px(4.),
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|mut parent| {
                            spawn_wand_list(&mut parent, &assets);

                            spawn_equipment_list(&mut parent, &assets);
                        });

                    // 右下

                    let level: GameLevel = match *next {
                        NextLevel::None => GameLevel::Level(0),
                        NextLevel::Level(i, _) => GameLevel::Level(i),
                        NextLevel::MultiPlayArena(_) => GameLevel::MultiPlayArena,
                    };
                    let name = level_to_name(level).get(config.language).to_string();

                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            name,
                            TextStyle {
                                color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                                font: assets.dotgothic.clone(),
                                font_size: 12.0,
                                ..default()
                            },
                        ),
                        style: Style { ..default() },
                        ..default()
                    });
                });

            spawn_wand_editor(&mut parent, &assets);

            spawn_inventory_floating(&mut parent, &assets);
        });

    #[cfg(feature = "debug")]
    commands.spawn(PerfUiBundle::default());
}

fn spawn_status_bars(parent: &mut ChildBuilder) {
    parent
        .spawn((NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                row_gap: Val::Px(4.),
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                ..default()
            },
            ..default()
        },))
        .with_children(|mut parent| {
            spawn_status_bar(
                &mut parent,
                PlayerLifeBar,
                0,
                0,
                Color::hsla(110., 0.5, 0.4, 0.9),
            );

            spawn_status_bar(
                &mut parent,
                PlayerManaBar,
                0,
                0,
                Color::hsla(240., 0.5, 0.4, 0.9),
            );

            parent.spawn((
                PlayerGold,
                Name::new("golds"),
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            color: Color::hsla(0.0, 0.0, 1.0, 0.35),
                            font_size: 18.0,
                            ..default()
                        },
                    ),
                    style: Style { ..default() },
                    z_index: ZIndex::Global(HUD_Z_INDEX),
                    ..default()
                },
                HUD,
            ));
        });
}

fn update_hud(
    player_query: Query<(&Player, &Actor), Without<Camera2d>>,
    mut player_life_query: Query<&mut StatusBar, With<PlayerLifeBar>>,
    mut player_mana_query: Query<&mut StatusBar, (With<PlayerManaBar>, Without<PlayerLifeBar>)>,
    mut player_gold_query: Query<&mut Text, (With<PlayerGold>,)>,
) {
    if let Ok((player, actor)) = player_query.get_single() {
        let mut player_life = player_life_query.single_mut();
        let mut player_mana = player_mana_query.single_mut();
        let mut player_gold = player_gold_query.single_mut();

        player_life.value = actor.life;
        player_life.max_value = actor.max_life;

        player_mana.value = actor.mana / 10;
        player_mana.max_value = actor.max_mana / 10;

        player_gold.sections[0].value = format!("Golds: {}", player.golds);
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        app.add_systems(Update, update_hud.run_if(in_state(GameState::InGame)));
    }
}
