use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::command::GameCommand;
use crate::constant::*;
use crate::controller::enemy::Enemy;
use crate::controller::player::Player;
use crate::curve::jump_curve;
use crate::entity::actor::{Actor, ActorFireState, ActorState};
use crate::entity::life::Life;
use crate::entity::EntityDepth;
use crate::hud::life_bar::LifeBarResource;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::{Wand, WandType};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

use super::slime::spawn_slime;

#[derive(Component)]
pub enum HugeSlime {
    Growl { animation: u32 },
    Approach { animation: u32, up_velocity: f32 },
    Summon { animation: u32 },
}

#[derive(Component)]
pub struct HugeSlimeSprite;

#[derive(Component)]
pub struct SlimeSeed {
    animation: u32,
    from: Vec2,
    to: Vec2,
    speed: u32,
}

#[derive(Component)]
pub struct SlimeSeedSprite;

pub fn spawn_huge_slime(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    let mut slots = [None; MAX_SPELLS_IN_WAND];
    slots[0] = Some(SpellType::MagicBolt);

    commands
        .spawn((
            Name::new("huge slime"),
            StateScoped(GameState::InGame),
            Enemy,
            Life {
                life: 200,
                max_life: 200,
                amplitude: 0.0,
            },
            HugeSlime::Growl { animation: 0 },
            // HugeSlime::Summon { animation: 0 },
            Actor {
                uuid: Uuid::new_v4(),
                spell_delay: 0,
                mana: 1000,
                max_mana: 1000,
                pointer: Vec2::ZERO,
                intensity: 0.0,
                move_direction: Vec2::ZERO,
                move_force: 0.0,
                fire_state: ActorFireState::Idle,
                group: ENEMY_GROUP,
                filter: ENTITY_GROUP | WALL_GROUP | WITCH_GROUP,
                current_wand: 0,
                effects: default(),
                wands: [
                    Some(Wand {
                        wand_type: WandType::CypressWand,
                        slots,
                        index: 0,
                    }),
                    None,
                    None,
                    None,
                ],
            },
            ActorState::default(),
            EntityDepth,
            AseSpriteAnimation {
                aseprite: assets.huge_slime_shadow.clone(),
                animation: Animation::default().with_tag("idle"),
            },
            Transform::from_translation(position.extend(PAINT_LAYER_Z)),
            (
                RigidBody::Dynamic,
                Velocity::zero(),
                Collider::ball(24.0),
                GravityScale(0.0),
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 40.0,
                    angular_damping: 1.0,
                },
                ExternalForce::default(),
                ExternalImpulse::default(),
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(
                    ENEMY_GROUP,
                    ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | WITCH_BULLET_GROUP | ENEMY_GROUP,
                ),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                HugeSlimeSprite,
                AseSpriteAnimation {
                    aseprite: assets.huge_slime.clone(),
                    animation: Animation::default().with_tag("idle"),
                },
                Transform::from_xyz(0.0, 0.0, ENTITY_LAYER_Z),
            ));
        });
}

fn update_huge_slime_growl(
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform), Without<Player>>,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<GameCommand>,
) {
    for parent in sprite_query.iter_mut() {
        let (mut huge_slime, transform) = huge_slime_query.get_mut(**parent).unwrap();
        if let HugeSlime::Growl { ref mut animation } = *huge_slime {
            *animation += 1;
            if *animation == 120 {
                se_writer.send(GameCommand::SEGrowl(Some(transform.translation.truncate())));
            }
            if 300 <= *animation {
                *animation = 0;
                *huge_slime = HugeSlime::Approach {
                    animation: 0,
                    up_velocity: 0.0,
                };
            }
        }
    }
}

fn update_huge_slime_approach(
    player_query: Query<&Transform, With<Player>>,
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform, &mut ExternalForce), Without<Player>>,
    mut huge_slime_sprite_query: Query<
        (&Parent, &mut Transform),
        (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>),
    >,
    mut camera_query: Query<
        (&mut GameCamera, &Transform),
        (
            Without<HugeSlimeSprite>,
            Without<HugeSlime>,
            Without<Player>,
        ),
    >,
    mut se_writer: EventWriter<GameCommand>,
) {
    const GRAVITY: f32 = 0.2;
    const JUMP_POWER: f32 = 3.0;
    const JUMP_TIMESPAN: u32 = 60;

    for (parent, mut offset) in huge_slime_sprite_query.iter_mut() {
        let (mut huge_slime, transform, mut force) = huge_slime_query.get_mut(**parent).unwrap();

        if let HugeSlime::Approach {
            ref mut animation,
            ref mut up_velocity,
        } = *huge_slime
        {
            *animation += 1;

            let next = (offset.translation.y + *up_velocity as f32).max(0.0);
            if 0.0 < offset.translation.y && next == 0.0 {
                se_writer.send(GameCommand::SEDrop(Some(transform.translation.truncate())));
                let (mut camera, camera_transform) = camera_query.single_mut();
                let distance = camera_transform.translation.distance(transform.translation);
                let max_range = 320.0; // 振動が起きる最大距離
                camera.vibration = (20.0 * (max_range - distance).max(0.0) / max_range).min(10.0);
            }
            // 重力に従って落下
            *up_velocity -= GRAVITY;
            offset.translation.y = next;

            // プレイヤーがいる場合はジャンプしながら接近
            if let Ok(player) = player_query.get_single() {
                // 60フレームに一度ジャンプ
                if *animation % JUMP_TIMESPAN == 0 {
                    *up_velocity = JUMP_POWER;
                }

                // 空中にいる場合は移動の外力が働く
                if next == 0.0 {
                    force.force = Vec2::ZERO;
                } else {
                    let direction = (player.translation - transform.translation)
                        .normalize()
                        .truncate();
                    force.force = direction * 4000000.0;
                };
            } else {
                force.force = Vec2::ZERO;
            }

            // 6秒ごとに召喚フェイズに移行
            if *animation == 360 {
                *huge_slime = HugeSlime::Summon { animation: 0 };
            }
        }
    }
}

fn update_huge_slime_summon(
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_query: Query<&Transform, With<Player>>,
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform), Without<Player>>,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<GameCommand>,
) {
    for parent in sprite_query.iter_mut() {
        let (mut huge_slime, transform) = huge_slime_query.get_mut(**parent).unwrap();

        if let HugeSlime::Summon { ref mut animation } = *huge_slime {
            *animation += 1;

            if let Ok(player) = player_query.get_single() {
                if *animation == 60 {
                    let slimes = 8;
                    for i in 0..slimes {
                        let t = std::f32::consts::PI * 2.0 / slimes as f32; // 等間隔に配置した場合の角度
                        let a = rand::random::<f32>() * 3.0; // 起点は適当にばらけさせる
                        let angle = a + t * i as f32 + t * 0.5 * rand::random::<f32>(); // 少しランダムにずらす
                        let offset = Vec2::from_angle(angle) * 100.0; // 100ピクセルの演習場にばらまく
                        let to = player.translation.truncate() + offset;
                        commands
                            .spawn((
                                StateScoped(GameState::InGame),
                                SlimeSeed {
                                    animation: 0,
                                    from: transform.translation.truncate(),
                                    to,
                                    speed: 60 + rand::random::<u32>() % 30,
                                },
                                AseSpriteSlice {
                                    aseprite: assets.atlas.clone(),
                                    name: "slime_shadow".into(),
                                },
                                Transform::from_translation(transform.translation),
                            ))
                            .with_child((
                                SlimeSeedSprite,
                                AseSpriteAnimation {
                                    aseprite: assets.slime.clone(),
                                    animation: Animation::default().with_tag("idle"),
                                },
                            ));
                    }

                    se_writer.send(GameCommand::SEPuyon(Some(transform.translation.truncate())));
                }
            }

            if 120 <= *animation {
                *animation = 0;
                *huge_slime = HugeSlime::Growl { animation: 0 };
            }
        }
    }
}

fn update_slime_seed(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SlimeSeed, &mut Transform)>,
    assets: Res<GameAssets>,
    life_bar_locals: Res<LifeBarResource>,
    mut se_writer: EventWriter<GameCommand>,
) {
    for (entity, mut seed, mut transform) in query.iter_mut() {
        seed.animation += 1;
        transform.translation = seed
            .from
            .lerp(seed.to, seed.animation as f32 / seed.speed as f32)
            .extend(ENTITY_LAYER_Z);
        if seed.animation == seed.speed {
            commands.entity(entity).despawn_recursive();
            spawn_slime(
                &mut commands,
                &assets,
                seed.to,
                &life_bar_locals,
                30 + rand::random::<u32>() % 30,
            );
            se_writer.send(GameCommand::SEBicha(Some(seed.to)));
        }
    }
}

fn update_slime_seed_sprite(
    parent_query: Query<&SlimeSeed>,
    mut query: Query<(&Parent, &mut Transform), With<SlimeSeedSprite>>,
) {
    for (parent, mut transform) in query.iter_mut() {
        if let Ok(seed) = parent_query.get(parent.get()) {
            transform.translation.y = jump_curve(seed.speed as f32, 100.0, seed.animation as f32);
        }
    }
}

pub struct HugeSlimePlugin;

impl Plugin for HugeSlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_huge_slime_growl,
                update_huge_slime_approach,
                update_huge_slime_summon,
                update_slime_seed,
                update_slime_seed_sprite,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}