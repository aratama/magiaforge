use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::collision::PLAYER_GROUPS;
use crate::collision::SENSOR_GROUPS;
use crate::constant::*;
use crate::controller::player::Player;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::level::world::LevelScoped;
use crate::player_state::PlayerState;
use crate::registry::Registry;
use crate::script::cmd::Cmd;
use crate::script::cmd::CmdEvent;
use crate::se::SEEvent;
use crate::se::TURN_ON;
use crate::se::WARP;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::PointLight2d;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::QueryFilter;
use bevy_rapier2d::prelude::Sensor;

const MAX_POWER: i32 = 360;
const MIN_RADIUS_ON: f32 = 100.0;
const MIN_INTENSITY_ON: f32 = 1.0;
const MIN_FALLOFF_ON: f32 = 10.0;

#[derive(Component)]
pub struct MagicCircle {
    active: bool,
    step: i32,
    light: Entity,
    destination_iid: String,
}

#[derive(Component)]
struct MagicStar;

#[derive(Component)]
pub struct MagicCircleLight;

pub fn spawn_magic_circle(
    commands: &mut Commands,
    registry: &Registry,
    level: &GameLevel,
    position: Vec2,
    destination_iid: &str,
) {
    let light_entity = commands.spawn_empty().id();

    commands
        .spawn((
            Name::new("magic_circle"),
            LevelScoped(level.clone()),
            StateScoped(GameState::InGame),
            MagicCircle {
                active: false,
                step: 0,
                light: light_entity,
                destination_iid: destination_iid.to_string(),
            },
            Transform::from_translation(position.extend(PAINT_LAYER_Z)),
            Sprite {
                color: Color::hsla(0.0, 1.0, 1.0, 0.7),
                ..default()
            },
            AseSpriteSlice {
                aseprite: registry.assets.atlas.clone(),
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
                    aseprite: registry.assets.atlas.clone(),
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
    player_query: Query<&Transform, (With<Player>, With<Witch>)>,
    mut circle_query: Query<(Entity, &mut MagicCircle)>,
    mut writer: EventWriter<SEEvent>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let mut entity = None;
    if let Ok(player_transform) = player_query.get_single() {
        let context = rapier_context.single();
        let position = player_transform.translation.truncate();
        context.intersections_with_shape(
            position,
            0.0,
            &Collider::ball(16.0),
            QueryFilter::default().groups(*PLAYER_GROUPS),
            |e| {
                // PLAYER_GROUPS でクエリを送っているので、プレイヤー自身もクエリに引っかかることに注意
                if circle_query.contains(e) {
                    entity = Some(e);
                    return false;
                }
                true
            },
        );
    }
    for (circle_entity, mut circle) in circle_query.iter_mut() {
        if Some(circle_entity) == entity {
            if !circle.active {
                writer.send(SEEvent::new(TURN_ON));
            }
            circle.active = true;
            circle.step = (circle.step + 1).min(MAX_POWER);
        } else {
            circle.active = false;
            circle.step = (circle.step - 4).max(0);
        }
    }
}

fn warp(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player, &Actor), With<Witch>>,
    mut circle_query: Query<(&mut MagicCircle, &Transform)>,
    mut level: ResMut<GameWorld>,
    mut writer: EventWriter<SEEvent>,
    mut cmd_writer: EventWriter<CmdEvent>,
) {
    let Ok((entity, player, actor)) = player_query.get_single_mut() else {
        return;
    };

    for (mut circle, transform) in circle_query.iter_mut() {
        if circle.step == MAX_POWER {
            writer.send(SEEvent::pos(WARP, transform.translation.truncate()));
            commands.entity(entity).despawn_recursive();

            circle.step = 0;
            let player_state = PlayerState::from_player(&player, &actor);
            level.next_state = Some(player_state);
            cmd_writer.send(CmdEvent(Cmd::Warp {
                destination_iid: circle.destination_iid.clone(),
            }));
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
            slice.name = (if circle.active {
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
