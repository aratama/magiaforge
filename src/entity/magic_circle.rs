use crate::{
    asset::GameAssets, command::GameCommand, config::GameConfig, constant::*,
    controller::player::Player, states::GameState, world::NextLevel,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, CollisionGroups, Sensor};

const MAX_POWER: i32 = 360;
const MIN_RADIUS_ON: f32 = 100.0;
const MIN_INTENSITY_ON: f32 = 1.0;
const MIN_FALLOFF_ON: f32 = 10.0;

#[derive(Debug, Clone, Copy)]
pub enum MagicCircleDestination {
    NextLevel,
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
    x: f32,
    y: f32,
    destination: MagicCircleDestination,
) {
    let light_entity = commands.spawn_empty().id();

    info!("spawn cirlce to {:?}", destination);

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
            AsepriteSliceBundle {
                aseprite: assets.atlas.clone(),
                slice: "magic_circle0".into(),
                transform: Transform::from_translation(Vec3::new(x, y, PAINT_LAYER_Z)),
                sprite: Sprite {
                    color: Color::hsla(0.0, 1.0, 1.0, 0.7),
                    ..default()
                },
                ..default()
            },
            Collider::cuboid(TILE_HALF, TILE_HALF),
            Sensor,
            CollisionGroups::new(MAGIC_CIRCLE_GROUP, WITCH_GROUP),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("magic_circle_star"),
                StateScoped(GameState::InGame),
                MagicStar,
                AsepriteSliceBundle {
                    aseprite: assets.atlas.clone(),
                    slice: "magic_star0".into(),
                    sprite: Sprite {
                        color: Color::hsla(0.0, 1.0, 1.0, 0.7),
                        ..default()
                    },
                    ..default()
                },
            ));
        });

    // 光源をスプライトの子にすると、画面外に出た時に光が消えてしまうことに注意
    commands.entity(light_entity).insert((
        Name::new("magic_circle_light"),
        MagicCircleLight,
        StateScoped(GameState::InGame),
        PointLight2dBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            point_light: PointLight2d {
                color: Color::hsla(240.0, 1.0, 0.5, 1.0),
                radius: MIN_RADIUS_ON,
                intensity: MIN_INTENSITY_ON,
                falloff: MIN_FALLOFF_ON,
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
    mut writer: EventWriter<GameCommand>,
) {
    for event in events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if process_collision_start_event(a, b, &player_query, &mut circle_query)
                    || process_collision_start_event(b, a, &player_query, &mut circle_query)
                {
                    writer.send(GameCommand::SETurnOn(None));
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
    mut circle_query: Query<(&mut MagicCircle, &Transform)>,
    mut next: ResMut<NextLevel>,
    mut writer: EventWriter<GameCommand>,
    mut config: ResMut<GameConfig>,
) {
    for (mut circle, transform) in circle_query.iter_mut() {
        if circle.step < MAX_POWER {
            if 0 < circle.players {
                circle.step = (circle.step + 1).min(MAX_POWER);
            } else {
                circle.step = (circle.step - 1).max(0);
            }
        } else if circle.step == MAX_POWER {
            if let Ok(entity) = player_query.get_single_mut() {
                writer.send(GameCommand::SEWarp(Some(transform.translation.truncate())));
                commands.entity(entity).despawn_recursive();

                info!("circle.destination {:?}", circle.destination);

                match circle.destination {
                    MagicCircleDestination::NextLevel => {
                        *next = match *next {
                            NextLevel::None => NextLevel::Level(1),
                            NextLevel::Level(level) => NextLevel::Level((level + 1) % LEVELS),
                            NextLevel::MultiPlayArena => NextLevel::Level(1),
                        };
                        config.online = false;
                        info!("next level: {:?}", *next);
                    }
                    MagicCircleDestination::MultiplayArena => {
                        *next = NextLevel::MultiPlayArena;
                        config.online = true;
                        info!("next level: {:?}", *next);
                    }
                }
            }
            circle.step += 1;
        } else if circle.step == MAX_POWER + 120 {
            writer.send(GameCommand::StateWarp);
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

fn process_collision_start_event(
    a: &Entity,
    b: &Entity,
    players: &Query<&Player>,
    circle_query: &mut Query<&mut MagicCircle>,
) -> bool {
    if players.contains(*a) {
        if let Ok(mut circle) = circle_query.get_mut(*b) {
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
            circle.players -= 1;
            return true;
        }
    }
    false
}

fn change_slice(mut circle_query: Query<(&MagicCircle, &mut AsepriteSlice)>) {
    for (circle, mut slice) in circle_query.iter_mut() {
        let ratio = (circle.step as f32 / MAX_POWER as f32 * 10.0).ceil() as i32 * 10;
        *slice = AsepriteSlice::new(format!("magic_circle{}", ratio.max(0).min(100)).as_str());
    }
}

fn change_star_slice(
    circle_query: Query<&MagicCircle>,
    mut star_query: Query<(&Parent, &mut AsepriteSlice), With<MagicStar>>,
) {
    for (parent, mut slice) in star_query.iter_mut() {
        if let Ok(circle) = circle_query.get(parent.get()) {
            *slice = AsepriteSlice::new(if 0 < circle.players {
                "magic_star1"
            } else {
                "magic_star0"
            });
        }
    }
}

pub struct MagicCirclePlugin;

impl Plugin for MagicCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (power_on_circle, warp).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            (update_circle_color, change_slice, change_star_slice)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
