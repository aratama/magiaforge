use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::{Actor, ActorFireState, ActorGroup, ActorState};
use crate::entity::bullet::HomingTarget;
use crate::entity::impact::SpawnImpact;
use crate::entity::life::Life;
use crate::entity::slime_seed::SpawnSlimeSeed;
use crate::entity::EntityDepth;
use crate::inventory::Inventory;
use crate::se::{SEEvent, SE};
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::{Wand, WandSpell, WandType};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

const HUGE_SLIME_COLLIDER_RADIUS: f32 = 24.0;

const IMPACT_MARGIN: f32 = 16.0;

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct HugeSlime {
    up_velocity: f32,
    state: HugeSlimeState,
    animation: u32,
    promoted: bool,
}

#[derive(Clone)]
pub enum HugeSlimeState {
    Growl,
    Approach,
    Summon,
    Promote,
}

#[derive(Component)]
pub struct HugeSlimeSprite;

pub fn spawn_huge_slime(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    let mut slots = [None; MAX_SPELLS_IN_WAND];
    slots[0] = Some(WandSpell {
        spell_type: SpellType::MagicBolt,
        price: 0,
    });

    commands
        .spawn((
            Name::new("スライムの王 エミルス"),
            StateScoped(GameState::InGame),
            Boss,
            DespawnHugeSlime,
            Life {
                life: 1200,
                max_life: 1200,
                amplitude: 0.0,
            },
            HomingTarget,
            HugeSlime {
                up_velocity: 0.0,
                state: HugeSlimeState::Growl,
                animation: 0,
                promoted: false,
            },
            Actor {
                uuid: Uuid::new_v4(),
                spell_delay: 0,
                pointer: Vec2::ZERO,
                intensity: 0.0,
                move_direction: Vec2::ZERO,
                move_force: 0.0,
                fire_state: ActorFireState::Idle,
                current_wand: 0,
                effects: default(),
                actor_group: ActorGroup::Enemy,
                golds: 0,
                inventory: Inventory::new(),
                equipments: [None; MAX_ITEMS_IN_EQUIPMENT],
                wands: [
                    Some(Wand {
                        wand_type: WandType::CypressWand,
                        price: 0,
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
                Collider::ball(HUGE_SLIME_COLLIDER_RADIUS),
                GravityScale(0.0),
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    // linear_damping: 40.0,
                    linear_damping: 20.0,
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

fn update_huge_slime(
    player_query: Query<&Transform, With<Player>>,
    mut slime_query: Query<(&mut HugeSlime, &Transform, &mut Actor), Without<Player>>,
    mut sprite_query: Query<
        (&Parent, &mut Transform),
        (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>),
    >,
    mut impact_writer: EventWriter<SpawnImpact>,
) {
    const GRAVITY: f32 = 0.2;
    for (parent, mut offset) in sprite_query.iter_mut() {
        let (mut huge_slime, transform, mut actor) = slime_query.get_mut(parent.get()).unwrap();
        huge_slime.up_velocity -= GRAVITY;
        let next = (offset.translation.y + huge_slime.up_velocity as f32).max(0.0);

        actor.move_force = 0.0;

        // プレイヤーがいる場合はジャンプしながら接近
        // 空中にいる場合は移動の外力が働く

        if let Ok(player_transform) = player_query.get_single() {
            if 0.0 < offset.translation.y {
                let direction = (player_transform.translation.truncate()
                    - transform.translation.truncate())
                .normalize_or_zero();

                // スライムを移動するのに、ExternalForceを直接操作しないこと
                // 直接操作すると、実行順序の関係で移動したりしなかったりという不安定なバグになります
                // ExternalForce は Actor の apply_external_force を通じて設定します
                actor.move_direction = direction;
                actor.move_force = 4000000.0;
            };
        }

        // 着地判定
        if 0.0 < offset.translation.y && next == 0.0 {
            impact_writer.send(SpawnImpact {
                owner: parent.get(),
                position: transform.translation.truncate(),
                radius: HUGE_SLIME_COLLIDER_RADIUS + IMPACT_MARGIN,
                impulse: 30000.0,
            });
        }

        offset.translation.y = next;

        huge_slime.animation += 1;
    }
}

fn update_huge_slime_growl(
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform), Without<Player>>,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<SEEvent>,
) {
    for parent in sprite_query.iter_mut() {
        let (mut huge_slime, transform) = huge_slime_query.get_mut(parent.get()).unwrap();
        if let HugeSlimeState::Growl = huge_slime.state {
            if huge_slime.animation == 120 {
                se_writer.send(SEEvent::pos(SE::Growl, transform.translation.truncate()));
            } else if 300 <= huge_slime.animation {
                huge_slime.state = HugeSlimeState::Approach;
                huge_slime.animation = 0;
            }
        }
    }
}

fn update_huge_slime_approach(
    player_query: Query<(&mut Life, &Transform, &mut ExternalImpulse), With<Player>>,
    mut huge_slime_query: Query<&mut HugeSlime, Without<Player>>,
    mut huge_slime_sprite_query: Query<
        &Parent,
        (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>),
    >,
) {
    const JUMP_POWER: f32 = 3.0;

    for parent in huge_slime_sprite_query.iter_mut() {
        let mut huge_slime = huge_slime_query.get_mut(parent.get()).unwrap();
        let timespan = if huge_slime.promoted { 35 } else { 60 };
        if let HugeSlimeState::Approach = huge_slime.state.clone() {
            // プレイヤーがいる場合はジャンプしながら接近
            if let Ok(_) = player_query.get_single() {
                // 60フレームに一度ジャンプ
                if huge_slime.animation % timespan == 0 {
                    huge_slime.up_velocity = JUMP_POWER;
                }
            }

            // 6秒ごとに召喚フェイズに移行
            if huge_slime.animation == 360 {
                huge_slime.state = HugeSlimeState::Summon;
                huge_slime.animation = 0;
            }
        }
    }
}

fn update_huge_slime_summon(
    player_query: Query<&Transform, With<Player>>,
    mut huge_slime_query: Query<(Entity, &mut HugeSlime, &Transform), Without<Player>>,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<SEEvent>,
    mut seed_writer: EventWriter<SpawnSlimeSeed>,
) {
    for parent in sprite_query.iter_mut() {
        let (huge_slime_entity, mut huge_slime, transform) =
            huge_slime_query.get_mut(**parent).unwrap();

        if let HugeSlimeState::Summon = huge_slime.state {
            if let Ok(player) = player_query.get_single() {
                if huge_slime.animation == 60 {
                    let slimes = if huge_slime.promoted { 8 } else { 4 };
                    for i in 0..slimes {
                        let t = std::f32::consts::PI * 2.0 / slimes as f32; // 等間隔に配置した場合の角度
                        let a = rand::random::<f32>() * 3.0; // 起点は適当にばらけさせる
                        let angle = a + t * i as f32 + t * 0.5 * rand::random::<f32>(); // 少しランダムにずらす
                        let offset = Vec2::from_angle(angle) * 100.0; // 100ピクセルの演習場にばらまく
                        let to = player.translation.truncate() + offset;
                        seed_writer.send(SpawnSlimeSeed {
                            from: transform.translation.truncate(),
                            to,
                            actor_group: ActorGroup::Enemy,
                            owner: huge_slime_entity,
                        });
                    }
                    se_writer.send(SEEvent::pos(SE::Puyon, transform.translation.truncate()));
                }
            }

            if 120 <= huge_slime.animation {
                huge_slime.animation = 0;
                huge_slime.state = HugeSlimeState::Approach;
            }
        }
    }
}

fn update_huge_slime_promote(
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform), Without<Player>>,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<SEEvent>,
) {
    for parent in sprite_query.iter_mut() {
        let (mut huge_slime, transform) = huge_slime_query.get_mut(parent.get()).unwrap();
        if let HugeSlimeState::Promote = huge_slime.state {
            if huge_slime.animation == 120 {
                se_writer.send(SEEvent::pos(SE::Growl, transform.translation.truncate()));
            } else if 300 <= huge_slime.animation {
                huge_slime.state = HugeSlimeState::Approach;
                huge_slime.animation = 0;
            }
        }
    }
}

fn promote(mut huge_slime_query: Query<(&mut HugeSlime, &Life), Without<Player>>) {
    for (mut huge_slime, life) in huge_slime_query.iter_mut() {
        if !huge_slime.promoted && life.life < 600 {
            huge_slime.state = HugeSlimeState::Promote;
            huge_slime.animation = 0;
            huge_slime.promoted = true;
        }
    }
}

#[derive(Component)]
pub struct DespawnHugeSlime;

fn despown(
    mut commands: Commands,
    query: Query<(Entity, &Life), With<DespawnHugeSlime>>,
    mut bgm: ResMut<NextBGM>,
    assets: Res<GameAssets>,
) {
    for (entity, life) in query.iter() {
        if life.life <= 0 {
            commands.entity(entity).despawn_recursive();
            bgm.0 = Some(assets.dokutsu.clone());
        }
    }
}

pub struct HugeSlimePlugin;

impl Plugin for HugeSlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_huge_slime,
                update_huge_slime_growl,
                update_huge_slime_approach,
                update_huge_slime_summon,
                update_huge_slime_promote,
                promote,
                despown,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
