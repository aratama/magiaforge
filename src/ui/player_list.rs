use crate::{
    asset::GameAssets,
    controller::{player::Player, remote::RemotePlayer},
    states::GameState,
};
use bevy::prelude::*;
use bevy_simple_websocket::{ReadyState, WebSocketState};

#[derive(Component)]
struct ReadyStateLabel;

#[derive(Component)]
struct PlayerList;

#[derive(Component)]
struct PlayersLabel;

#[derive(Component)]
struct SelfPlayerLabel;

#[derive(Component)]
struct RemotePlayerListItem;

fn spawn_player_list(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(1100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ReadyStateLabel,
                TextBundle::from_section(
                    "Offline",
                    TextStyle {
                        font: assets.dotgothic.clone(),
                        color: Color::WHITE,
                        font_size: 20.0,
                        ..default()
                    },
                ),
            ));

            parent.spawn((
                PlayersLabel,
                TextBundle::from_section(
                    "[ 1 Players ]",
                    TextStyle {
                        font: assets.dotgothic.clone(),
                        color: Color::WHITE,
                        font_size: 20.0,
                        ..default()
                    },
                ),
            ));

            parent.spawn((
                PlayerList,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

/// プレイヤーリストを更新
fn update_player_list(
    mut commands: Commands,
    player_query: Query<&Player>,
    remote_query: Query<(Entity, &RemotePlayer)>,
    mut remote_player_items_query: Query<(Entity, &RemotePlayerListItem, &mut Text)>,
    mut list_query: Query<Entity, With<PlayerList>>,
    assets: Res<GameAssets>,
) {
    let parent = list_query.single_mut();

    let mut players = Vec::<(String, i32, Color)>::new();
    if let Ok(player) = player_query.get_single() {
        players.push((
            player.name.clone(),
            player.golds,
            Color::hsl(120.0, 1.0, 0.5),
        ));
    }
    for (_, remote_player) in remote_query.iter() {
        players.push((
            remote_player.name.clone(),
            remote_player.golds,
            Color::WHITE,
        ));
    }

    players.sort_by(|a, b| b.1.cmp(&a.1));

    // 必要な個数だけListItemを生成
    let diff = players.len() as i32 - remote_player_items_query.iter().len() as i32;
    if 0 < diff {
        for _ in 0..diff {
            commands.entity(parent).with_children(|parent| {
                parent.spawn((
                    RemotePlayerListItem,
                    TextBundle::from_section(
                        "(anonymous)".to_string(),
                        TextStyle {
                            font: assets.dotgothic.clone(),
                            color: Color::WHITE,
                            font_size: 20.0,
                            ..default()
                        },
                    ),
                    Label,
                ));
            });
        }
    } else if diff < 0 {
        for i in players.len()..remote_player_items_query.iter().len() {
            if let Some((item_entity, _, _)) = remote_player_items_query.iter().nth(i) {
                commands.entity(item_entity).despawn_recursive();
            }
        }
    } else {
        for (i, (name, golds, color)) in players.iter().enumerate() {
            let (_, _, mut text) = remote_player_items_query.iter_mut().nth(i).unwrap();
            text.sections[0].value = format_remote_player_name(&name, *golds);
            text.sections[0].style.color = *color;
        }
    }
}

fn format_remote_player_name(name: &str, golds: i32) -> String {
    if name.is_empty() {
        format!("(anonymous) ({})", golds).to_string()
    } else {
        format!("{} ({})", name, golds).to_string()
    }
}

fn update_players(
    remote_query: Query<(Entity, &RemotePlayer)>,
    mut players_label_query: Query<&mut Text, With<PlayersLabel>>,
) {
    // プレイヤー数を更新
    let mut players_label = players_label_query.single_mut();
    players_label.sections[0].value = format!("[ {} Players ]", 1 + remote_query.iter().count(),);
}

/// WebSocketの状態のラベルテキストを更新
fn update_ready_state_label(
    mut ready_state_label_query: Query<&mut Text, With<ReadyStateLabel>>,
    state: Res<WebSocketState>,
) {
    let mut ready_state_label = ready_state_label_query.single_mut();
    ready_state_label.sections[0].value = format!(
        "{}",
        if state.ready_state == ReadyState::OPEN {
            "Online"
        } else {
            "Offline"
        }
    );
}

pub struct PlayerListPlugin;

impl Plugin for PlayerListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player_list);
        app.add_systems(
            Update,
            update_player_list.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            update_ready_state_label.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(Update, update_players.run_if(in_state(GameState::InGame)));
    }
}
