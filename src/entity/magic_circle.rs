use crate::{
    asset::GameAssets, audio::play_se, config::GameConfig, constant::*, controller::player::Player,
    hud::overlay::OverlayNextState, states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_kira_audio::Audio;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, CollisionGroups, Sensor};

const MAX_POWER: i32 = 300;

#[derive(Default, Component)]
pub struct MagicCircle {
    players: i32,
    power: i32,

    sleep_to_close: i32,
    warped: bool,
}

pub fn spawn_magic_circle(commands: &mut Commands, aseprite: Handle<Aseprite>, x: f32, y: f32) {
    commands.spawn((
        Name::new("magic_circle"),
        StateScoped(GameState::InGame),
        MagicCircle {
            players: 0,
            power: 0,
            sleep_to_close: 0,
            warped: false,
        },
        AsepriteSliceBundle {
            aseprite: aseprite,
            slice: "magic_circle".into(),
            transform: Transform::from_translation(Vec3::new(x, y, PAINT_LAYER_Z)),
            sprite: Sprite {
                color: Color::hsla(0.0, 1.0, 1.0, 0.3),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(TILE_HALF, TILE_HALF),
        Sensor,
        CollisionGroups::new(WALL_GROUP, ENEMY_GROUP),
        ActiveEvents::COLLISION_EVENTS,
    ));

    // 光源をスプライトの子にすると、画面外に出た時に光が消えてしまうことに注意
    commands.spawn((
        Name::new("magic_circle_light"),
        StateScoped(GameState::InGame),
        PointLight2dBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            point_light: PointLight2d {
                color: Color::hsla(240.0, 1.0, 0.5, 1.0),
                radius: 100.0,
                intensity: 1.0,
                falloff: 10.0,
                ..default()
            },
            ..default()
        },
    ));
}

fn power_on_circle(
    player_query: Query<&Player>,
    mut circle_query: Query<&mut MagicCircle>,
    mut events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
) {
    for event in events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if process_collision_start_event(a, b, &player_query, &mut circle_query)
                    || process_collision_start_event(b, a, &player_query, &mut circle_query)
                {
                    play_se(&audio, &config, assets.menu_open.clone());
                }
            }
            CollisionEvent::Stopped(a, b, _) => {
                let _ = process_collision_end_event(a, b, &player_query, &mut circle_query)
                    || process_collision_end_event(b, a, &player_query, &mut circle_query);
            }
        }
    }
}

fn warp(
    mut commands: Commands,
    mut player_query: Query<Entity, With<Player>>,
    mut circle_query: Query<&mut MagicCircle>,
    mut overlay_next_state: ResMut<OverlayNextState>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
) {
    for mut circle in circle_query.iter_mut() {
        if 0 < circle.players {
            circle.power += 1;
        } else {
            circle.power = (circle.power - 4).max(0);
        }

        if let Ok(entity) = player_query.get_single_mut() {
            if MAX_POWER <= circle.power {
                circle.warped = true;
                circle.sleep_to_close = 0;
                circle.power = 0;
                play_se(&audio, &config, assets.warp.clone());
                commands.entity(entity).despawn_recursive();
            }
        }

        if circle.warped && player_query.is_empty() {
            circle.sleep_to_close += 1;
            if 180 <= circle.sleep_to_close {
                *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
            }
        }
    }
}

fn update_circle_color(mut circle_query: Query<(&MagicCircle, &mut Sprite)>) {
    for (circle, mut sprite) in circle_query.iter_mut() {
        let satulation = if 0 < circle.power { 1.0 } else { 0.0 };

        let lightness = 0.25
            + if 0 < circle.power {
                0.25 + 0.5 * (circle.power as f32 / MAX_POWER as f32).min(1.0)
            } else {
                0.5
            };

        sprite.color = Color::hsla(240.0, satulation, lightness, 1.0)
    }
}

fn process_collision_start_event(
    a: &Entity,
    b: &Entity,
    players: &Query<&Player>,
    circle_query: &mut Query<&mut MagicCircle>,
) -> bool {
    if players.contains(*a) {
        if let Ok(mut circle) = circle_query.get_mut(*b) {
            // info!("player on magic circle");
            circle.players += 1;
            return true;
        }
    }
    false
}

fn process_collision_end_event(
    a: &Entity,
    b: &Entity,
    players: &Query<&Player>,
    circle_query: &mut Query<&mut MagicCircle>,
) -> bool {
    if players.contains(*a) {
        if let Ok(mut circle) = circle_query.get_mut(*b) {
            // info!("player out magic circle");
            circle.players -= 1;
            return true;
        }
    }
    false
}

pub struct MagicCirclePlugin;

impl Plugin for MagicCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            power_on_circle.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(FixedUpdate, warp.run_if(in_state(GameState::InGame)));
        app.add_systems(
            Update,
            update_circle_color.run_if(in_state(GameState::InGame)),
        );
    }
}
