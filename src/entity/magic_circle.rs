use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::hud::overlay::OverlayEvent;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::physics::identify;
use crate::physics::IdentifiedCollisionEvent;
use crate::player_state::PlayerState;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::Sensor;

const MAX_POWER: i32 = 360;
const MIN_RADIUS_ON: f32 = 100.0;
const MIN_INTENSITY_ON: f32 = 1.0;
const MIN_FALLOFF_ON: f32 = 10.0;

#[derive(Debug, Clone, Copy)]
pub enum MagicCircleDestination {
    NextLevel,
    Home,
    MultiplayArena,
}

#[derive(Component)]
pub struct MagicCircle {
    players: i32,
    step: i32,
    light: Entity,
    destination: MagicCircleDestination,
}

#[derive(Component)]
struct MagicStar;

#[derive(Component)]
pub struct MagicCircleLight;

pub fn spawn_magic_circle(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    destination: MagicCircleDestination,
) {
    let light_entity = commands.spawn_empty().id();

    commands
        .spawn((
            Name::new("magic_circle"),
            StateScoped(GameState::InGame),
            MagicCircle {
                players: 0,
                step: 0,
                light: light_entity,
                destination,
            },
            Transform::from_translation(position.extend(PAINT_LAYER_Z)),
            Sprite {
                color: Color::hsla(0.0, 1.0, 1.0, 0.7),
                ..default()
            },
            AseSpriteSlice {
                aseprite: assets.atlas.clone(),
                name: "magic_circle0".into(),
            },
            (
                Collider::cuboid(TILE_HALF, TILE_HALF),
                Sensor,
                *SENSOR_GROUPS,
                ActiveEvents::COLLISION_EVENTS,
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("magic_circle_star"),
                MagicStar,
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "magic_star0".into(),
                },
                Sprite {
                    color: Color::hsla(0.0, 1.0, 1.0, 0.7),
                    ..default()
                },
            ));
        });

    // 光源をスプライトの子にすると、画面外に出た時に光が消えてしまうことに注意
    commands.entity(light_entity).insert((
        Name::new("magic_circle_light"),
        MagicCircleLight,
        Transform::from_translation(position.extend(0.0)),
        PointLight2d {
            color: Color::hsla(240.0, 1.0, 0.5, 1.0),
            radius: MIN_RADIUS_ON,
            intensity: MIN_INTENSITY_ON,
            falloff: MIN_FALLOFF_ON,
            ..default()
        },
    ));
}

fn power_on_circle(
    player_query: Query<&Player>,
    mut circle_query: Query<&mut MagicCircle>,
    mut events: EventReader<CollisionEvent>,
    mut writer: EventWriter<SEEvent>,
) {
    for event in events.read() {
        match identify(&event, &circle_query, &player_query) {
            IdentifiedCollisionEvent::Started(circle_entity, _) => {
                let mut circle = circle_query.get_mut(circle_entity).unwrap();
                circle.players += 1;
                writer.send(SEEvent::new(SE::TurnOn));
            }
            IdentifiedCollisionEvent::Stopped(circle_entity, _) => {
                let mut circle = circle_query.get_mut(circle_entity).unwrap();
                circle.players -= 1;
            }
            _ => {}
        }
    }
}

fn warp(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player, &Actor, &Life)>,
    mut circle_query: Query<(&mut MagicCircle, &Transform)>,
    mut next: ResMut<LevelSetup>,
    mut writer: EventWriter<SEEvent>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
) {
    for (mut circle, transform) in circle_query.iter_mut() {
        if circle.step < MAX_POWER {
            if 0 < circle.players {
                circle.step = (circle.step + 1).min(MAX_POWER);
            } else {
                circle.step = (circle.step - 1).max(0);
            }
        } else if circle.step == MAX_POWER {
            if let Ok((entity, player, actor, player_life)) = player_query.get_single_mut() {
                writer.send(SEEvent::pos(SE::Warp, transform.translation.truncate()));
                commands.entity(entity).despawn_recursive();

                let player_state = PlayerState::from_player(&player, &actor, &player_life);

                match circle.destination {
                    MagicCircleDestination::NextLevel => {
                        match next.next_level {
                            GameLevel::Level(level) => {
                                next.next_level = GameLevel::Level((level + 1) % LEVELS);
                                next.next_state = Some(player_state);
                            }
                            GameLevel::MultiPlayArena => {
                                next.next_level = GameLevel::Level(1);
                                next.next_state = Some(player_state);
                            }
                        };
                    }
                    MagicCircleDestination::Home => {
                        next.next_level = GameLevel::Level(0);
                        next.next_state = Some(player_state);
                    }
                    MagicCircleDestination::MultiplayArena => {
                        next.next_level = GameLevel::MultiPlayArena;
                        next.next_state = Some(player_state);
                    }
                }
            }
            circle.step += 1;
        } else if circle.step == MAX_POWER + 120 {
            overlay_event_writer.send(OverlayEvent::Close(GameState::Warp));
            circle.step += 1;
        } else {
            circle.step += 1;
        }
    }
}

fn update_circle_color(
    mut circle_query: Query<&MagicCircle>,
    mut light_query: Query<&mut PointLight2d, With<MagicCircleLight>>,
) {
    for circle in circle_query.iter_mut() {
        if let Ok(mut light) = light_query.get_mut(circle.light) {
            if circle.step == 0 {
                light.color = Color::hsla(240.0, 1.0, 0.5, 1.0);
            } else if circle.step < MAX_POWER {
                light.color = Color::hsla(240.0, 0.0, 1.0, 1.0);
            } else {
                light.color = Color::hsla(240.0, 1.0, 0.5, 1.0);
            }
        }
    }
}

fn change_slice(mut circle_query: Query<(&MagicCircle, &mut AseSpriteSlice)>) {
    for (circle, mut slice) in circle_query.iter_mut() {
        let ratio = (circle.step as f32 / MAX_POWER as f32 * 10.0).ceil() as i32 * 10;
        slice.name = format!("magic_circle{}", ratio.max(0).min(100)).to_string();
    }
}

fn change_star_slice(
    circle_query: Query<&MagicCircle>,
    mut star_query: Query<(&Parent, &mut AseSpriteSlice), With<MagicStar>>,
) {
    for (parent, mut slice) in star_query.iter_mut() {
        if let Ok(circle) = circle_query.get(parent.get()) {
            slice.name = (if 0 < circle.players {
                "magic_star1"
            } else {
                "magic_star0"
            })
            .to_string();
        }
    }
}

pub struct MagicCirclePlugin;

impl Plugin for MagicCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                power_on_circle,
                warp,
                update_circle_color,
                change_slice,
                change_star_slice,
            )
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
