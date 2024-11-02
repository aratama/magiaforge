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
struct RemotePlayerListItem(Entity);

fn spawn_player_list(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(1180.0),
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
                    "Players (1)",
                    TextStyle {
                        font: assets.dotgothic.clone(),
                        color: Color::WHITE,
                        font_size: 20.0,
                        ..default()
                    },
                ),
            ));

            parent.spawn((
                SelfPlayerLabel,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: assets.dotgothic.clone(),
                        color: Color::hsl(120.0, 1.0, 0.5),
                        font_size: 20.0,
                        ..default()
                    },
                ),
                Label,
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

fn update_player_list(
    mut commands: Commands,
    remote_query: Query<(Entity, &RemotePlayer)>,
    remote_player_items_query: Query<(Entity, &RemotePlayerListItem)>,
    mut list_query: Query<Entity, With<PlayerList>>,
    assets: Res<GameAssets>,
) {
    // プレイヤーリストを更新
    // ListItemが存在しないRemotePlayerを追加
    let list = list_query.single_mut();
    for (remote_entity, remote) in remote_query.iter() {
        if remote_player_items_query
            .iter()
            .find(|(_, i)| i.0.index() == remote_entity.index())
            .is_none()
        {
            commands.entity(list).with_children(|parent| {
                parent.spawn((
                    RemotePlayerListItem(remote_entity),
                    TextBundle::from_section(
                        if remote.name.is_empty() {
                            "(no name)".to_string()
                        } else {
                            remote.name.clone()
                        },
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
    }

    // RemotePlayerが存在しないListItemを削除
    for (item_entity, item) in remote_player_items_query.iter() {
        if remote_query.get(item.0).is_err() {
            commands.entity(item_entity).despawn_recursive();
        }
    }
}

/// プレイヤーリストの自分の名前を更新
fn update_self_label(
    player_query: Query<&Player>,
    mut self_player_query: Query<&mut Text, With<SelfPlayerLabel>>,
) {
    if let Ok(player) = player_query.get_single() {
        let mut self_player_label = self_player_query.single_mut();
        self_player_label.sections[0].value = if player.name.is_empty() {
            "(no name)".to_string()
        } else {
            player.name.clone()
        };
    }
}

fn update_players(
    remote_query: Query<(Entity, &RemotePlayer)>,
    mut players_label_query: Query<&mut Text, With<PlayersLabel>>,
) {
    // プレイヤー数を更新
    let mut players_label = players_label_query.single_mut();
    players_label.sections[0].value = format!("Players ({})", 1 + remote_query.iter().count(),);
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
        app.add_systems(
            Update,
            update_self_label.run_if(in_state(GameState::InGame)),
        );
    }
}
