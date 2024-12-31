use super::counter::CounterAnimated;
use super::falling::Falling;
use super::point_light::WithPointLight;
use crate::asset::GameAssets;
use crate::constant::ENEMY_GROUP;
use crate::constant::ENTITY_GROUP;
use crate::constant::RABBIT_GROUP;
use crate::constant::SENSOR_GROUP;
use crate::constant::WALL_GROUP;
use crate::constant::WITCH_GROUP;
use crate::entity::life::Life;
use crate::entity::life::LifeBeingSprite;
use crate::entity::EntityDepth;
use crate::level::tile::Tile;
use crate::page::in_game::Interlevel;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct Fireball;

#[derive(Default, Component, Reflect)]
pub struct Fire;

pub fn spawn_fireball(commands: &mut Commands, assets: &Res<GameAssets>, from: Vec2, to: Vec2) {
    commands
        .spawn((
            Name::new("fireball"),
            StateScoped(GameState::InGame),
            Fireball,
            EntityDepth,
            Visibility::default(),
            Transform::from_translation(from.extend(0.0)),
            WithPointLight {
                radius: 64.0,
                intensity: 1.0,
                falloff: 10.0,
                color: Color::hsl(42.0, 1.0, 0.71),
                animation_offset: rand::random::<u32>() % 1000,
                speed: 0.43,
                amplitude: 0.1,
            },
            (
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 0.0,
                    angular_damping: 0.0,
                },
                Collider::ball(6.0),
                CollisionGroups::new(ENTITY_GROUP, ENTITY_GROUP | WALL_GROUP | RABBIT_GROUP),
                Velocity::linear((to - from).normalize_or_zero() * 200.0),
                ExternalImpulse::default(),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                Falling::new(2.0, -0.1),
                LifeBeingSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: assets.fireball.clone(),
                    animation: "default".into(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.01),
            ));
        });
}

fn fall(
    mut commands: Commands,
    assets: Res<GameAssets>,
    child_query: Query<(&Parent, &Falling)>,
    parent_query: Query<(Entity, &Transform), With<Fireball>>,
    interlevel: Res<Interlevel>,
) {
    for (parent, falling) in child_query.iter() {
        if falling.just_landed {
            if let Ok((entity, parent_transform)) = parent_query.get(parent.get()) {
                commands.entity(entity).despawn_recursive();
                if let Some(ref level) = interlevel.chunk {
                    let position = parent_transform.translation.truncate();
                    let tile = level.get_tile_by_coords(position);
                    if tile != Tile::Wall && tile != Tile::Blank {
                        spawn_fire(&mut commands, &assets, position);
                    }
                }
            }
        }
    }
}

fn spawn_fire(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands.spawn((
        Name::new("fire"),
        StateScoped(GameState::InGame),
        Fire,
        EntityDepth,
        Visibility::default(),
        Transform::from_translation(position.extend(0.0)),
        CounterAnimated,
        AseSpriteAnimation {
            aseprite: assets.fire.clone(),
            animation: "default".into(),
            ..default()
        },
        WithPointLight {
            radius: 128.0,
            intensity: 1.0,
            falloff: 10.0,
            color: Color::hsl(42.0, 1.0, 0.71),
            animation_offset: rand::random::<u32>() % 1000,
            speed: 0.43,
            amplitude: 0.1,
        },
        (
            Sensor,
            Collider::ball(8.0),
            CollisionGroups::new(
                SENSOR_GROUP,
                ENTITY_GROUP | WITCH_GROUP | ENEMY_GROUP | RABBIT_GROUP,
            ),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ),
    ));
}

fn despown(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform), With<Fire>>,
    mut writer: EventWriter<SEEvent>,
) {
    for (entity, breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();
            writer.send(SEEvent::pos(SE::Break, position));
        }
    }
}

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (fall, despown)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Fireball>();
    }
}
