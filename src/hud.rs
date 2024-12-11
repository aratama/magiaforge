pub mod life_bar;
pub mod overlay;
pub mod pointer;

use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::constant::HUD_Z_INDEX;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::life::Life;
use crate::level::{level_to_name, GameLevel, NextLevel};
use crate::states::GameState;
use crate::ui::bar::{spawn_status_bar, StatusBar};
use crate::ui::equipment_list::spawn_equipment_list;
use crate::ui::floating::spawn_inventory_floating;
use crate::ui::wand_editor::spawn_wand_editor;
use crate::ui::wand_list::spawn_wand_list;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseUiSlice;

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

                            spawn_equipment_list(&mut parent, &assets);
                        });

                    // 右下

                    let level: GameLevel = match *next {
                        NextLevel::None => GameLevel::Level(0),
                        NextLevel::Level(i, _) => GameLevel::Level(i),
                        NextLevel::MultiPlayArena(_) => GameLevel::MultiPlayArena,
                    };
                    let name = level_to_name(level).get(config.language).to_string();

                    parent.spawn((
                        Text(name),
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                        TextFont {
                            font: assets.dotgothic.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                    ));
                });

            spawn_wand_editor(&mut parent, &assets);

            spawn_inventory_floating(&mut parent, &assets);
        });
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
                PlayerLifeBar,
                0,
                0,
                Color::hsla(110., 0.7, 0.7, 0.9),
            );

            spawn_status_bar(
                &mut parent,
                PlayerManaBar,
                0,
                0,
                Color::hsla(240., 0.7, 0.7, 0.9),
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
                            font_size: 18.0,
                            ..default()
                        },
                        HUD,
                    ));
                });
        });
}

fn update_hud(
    player_query: Query<(&Player, &Actor, &Life), Without<Camera2d>>,
    mut player_life_query: Query<&mut StatusBar, With<PlayerLifeBar>>,
    mut player_mana_query: Query<&mut StatusBar, (With<PlayerManaBar>, Without<PlayerLifeBar>)>,
    mut player_gold_query: Query<&mut Text, (With<PlayerGold>,)>,
) {
    if let Ok((player, actor, actor_life)) = player_query.get_single() {
        let mut player_life = player_life_query.single_mut();
        let mut player_mana = player_mana_query.single_mut();
        let mut player_gold = player_gold_query.single_mut();

        player_life.value = actor_life.life;
        player_life.max_value = actor_life.max_life;

        player_mana.value = actor.mana / 10;
        player_mana.max_value = actor.max_mana / 10;

        player_gold.0 = format!("{}", player.golds);
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        app.add_systems(Update, update_hud.run_if(in_state(GameState::InGame)));
    }
}
