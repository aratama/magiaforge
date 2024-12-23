use crate::asset::GameAssets;
use crate::controller::player::Player;
use crate::controller::remote::RemotePlayer;
use crate::entity::actor::Actor;
use crate::page::in_game::CurrentLevel;
use crate::page::in_game::GameLevel;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_simple_websocket::ReadyState;
use bevy_simple_websocket::WebSocketState;

#[derive(Component)]
struct PlayerListRoot;

#[derive(Component)]
struct ReadyStateLabel;

#[derive(Component)]
struct PlayerList;

#[derive(Component)]
struct PlayersLabel;

#[derive(Component)]
struct RemotePlayerListItem;

fn spawn_player_list(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            Name::new("player_list_root"),
            PlayerListRoot,
            StateScoped(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(1100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                display: Display::None,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ReadyStateLabel,
                Text::new("Offline"),
                TextColor(Color::WHITE),
                TextFont {
                    font: assets.dotgothic.clone(),
                    font_size: 20.0,
                    ..default()
                },
            ));

            parent.spawn((
                PlayersLabel,
                Text::new("[ 1 Players ]"),
                TextColor(Color::WHITE),
                TextFont {
                    font: assets.dotgothic.clone(),
                    font_size: 20.0,
                    ..default()
                },
            ));

            parent.spawn((
                PlayerList,
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
            ));
        });
}

fn update_player_list_visibility(
    mut player_list_query: Query<&mut Node, With<PlayerListRoot>>,
    current: Res<CurrentLevel>,
) {
    let mut player_list_root = player_list_query.single_mut();

    player_list_root.display =
        if current.level != None && current.level == Some(GameLevel::MultiPlayArena) {
            Display::Flex
        } else {
            Display::None
        };
}

/// プレイヤーリストを更新
fn update_player_list(
    mut commands: Commands,
    player_query: Query<(&Player, &Actor)>,
    remote_query: Query<(Entity, &RemotePlayer)>,
    mut remote_player_items_query: Query<(
        Entity,
        &RemotePlayerListItem,
        &mut Text,
        &mut TextColor,
    )>,
    mut list_query: Query<Entity, With<PlayerList>>,
    assets: Res<GameAssets>,
) {
    let parent = list_query.single_mut();
    let mut players = Vec::<(String, u32, Color)>::new();
    if let Ok((player, actor)) = player_query.get_single() {
        players.push((
            player.name.clone(),
            actor.golds,
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
                    Text::new("(anonymous)"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: assets.dotgothic.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    Label,
                ));
            });
        }
    } else if diff < 0 {
        for i in players.len()..remote_player_items_query.iter().len() {
            if let Some((item_entity, _, _, _)) = remote_player_items_query.iter().nth(i) {
                commands.entity(item_entity).despawn_recursive();
            }
        }
    } else {
        for (i, (name, golds, color)) in players.iter().enumerate() {
            let (_, _, mut text, mut text_color) =
                remote_player_items_query.iter_mut().nth(i).unwrap();
            text.0 = format_remote_player_name(&name, *golds);
            text_color.0 = *color;
        }
    }
}

fn format_remote_player_name(name: &str, golds: u32) -> String {
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
    if let Ok(mut players_label) = players_label_query.get_single_mut() {
        players_label.0 = format!("[ {} Players ]", 1 + remote_query.iter().count());
    }
}

/// WebSocketの状態のラベルテキストを更新
fn update_ready_state_label(
    mut ready_state_label_query: Query<&mut Text, With<ReadyStateLabel>>,
    state: Res<WebSocketState>,
) {
    if let Ok(mut ready_state_label) = ready_state_label_query.get_single_mut() {
        ready_state_label.0 = format!(
            "{}",
            if state.ready_state == ReadyState::OPEN {
                "Online"
            } else {
                "Offline"
            }
        );
    }
}

pub struct PlayerListPlugin;

impl Plugin for PlayerListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player_list);
        app.add_systems(
            Update,
            (
                update_player_list,
                update_player_list_visibility,
                update_ready_state_label,
                update_players,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
