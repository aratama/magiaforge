use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::component::falling::Falling;
use crate::component::life::Life;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorState;
use crate::entity::bullet::HomingTarget;
use crate::entity::impact::SpawnImpact;
use crate::entity::servant_seed::ServantType;
use crate::entity::servant_seed::SpawnServantSeed;
use crate::entity::EntityDepth;
use crate::inventory::Inventory;
use crate::se::SEEvent;
use crate::se::SE;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::states::TimeState;
use crate::theater::Act;
use crate::theater::TheaterEvent;
use crate::wand::Wand;
use crate::wand::WandSpell;
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
    state: HugeSlimeState,
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
                fire_damage_wait: 0,
            },
            HomingTarget,
            HugeSlime {
                state: HugeSlimeState::Growl,
                promoted: false,
            },
            Counter::up(0),
            Actor {
                uuid: Uuid::new_v4(),
                pointer: Vec2::ZERO,
                point_light_radius: 0.0,
                radius: HUGE_SLIME_COLLIDER_RADIUS,
                move_direction: Vec2::ZERO,
                move_force: 0.0,
                fire_state: ActorFireState::Idle,
                fire_state_secondary: ActorFireState::Idle,
                current_wand: 0,
                effects: default(),
                actor_group: ActorGroup::Enemy,
                golds: 0,
                inventory: Inventory::new(),
                equipments: [None; MAX_ITEMS_IN_EQUIPMENT],
                wands: [
                    Wand::with_slots(slots),
                    Wand::empty(),
                    Wand::empty(),
                    Wand::empty(),
                ],
                state: ActorState::default(),
                wait: 0,
            },
            EntityDepth::new(),
            CounterAnimated,
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
                    ENTITY_GROUP
                        | WALL_GROUP
                        | NEUTRAL_GROUP
                        | WITCH_GROUP
                        | WITCH_BULLET_GROUP
                        | ENEMY_GROUP
                        | RABBIT_GROUP,
                ),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                Falling::new(0.0, -0.2),
                HugeSlimeSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: assets.huge_slime.clone(),
                    animation: Animation::default().with_tag("idle"),
                },
                Transform::from_xyz(0.0, 0.0, ENTITY_LAYER_Z),
            ));
        });
}

// プレイヤーがいる場合はジャンプしながら接近
// 空中にいる場合は移動の外力が働く
fn control(
    player_query: Query<&Transform, With<Player>>,
    mut slime_query: Query<(&Transform, &mut Actor), (With<HugeSlime>, Without<Player>)>,
    mut sprite_query: Query<
        (&Parent, &Transform),
        (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>),
    >,
) {
    for (parent, offset) in sprite_query.iter_mut() {
        let (transform, mut actor) = slime_query.get_mut(parent.get()).unwrap();

        actor.move_force = 0.0;

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
    }
}

fn impact(
    slime_query: Query<&Transform, With<HugeSlime>>,
    sprite_query: Query<(&Parent, &Falling), With<HugeSlimeSprite>>,
    mut impact_writer: EventWriter<SpawnImpact>,
) {
    for (parent, falling) in sprite_query.iter() {
        if falling.just_landed {
            let transform = slime_query.get(parent.get()).unwrap();
            impact_writer.send(SpawnImpact {
                owner: Some(parent.get()),
                position: transform.translation.truncate(),
                radius: HUGE_SLIME_COLLIDER_RADIUS + IMPACT_MARGIN,
                impulse: 30000.0,
            });
        }
    }
}

fn update_huge_slime_growl(
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform, &mut Counter), Without<Player>>,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<SEEvent>,
) {
    for parent in sprite_query.iter_mut() {
        let (mut huge_slime, transform, mut counter) =
            huge_slime_query.get_mut(parent.get()).unwrap();
        if let HugeSlimeState::Growl = huge_slime.state {
            if counter.count == 120 {
                se_writer.send(SEEvent::pos(SE::Growl, transform.translation.truncate()));
            } else if 300 <= counter.count {
                huge_slime.state = HugeSlimeState::Approach;
                counter.count = 0;
            }
        }
    }
}

fn update_huge_slime_approach(
    player_query: Query<(&mut Life, &Transform, &mut ExternalImpulse), With<Player>>,
    mut huge_slime_query: Query<(&mut HugeSlime, &mut Counter), Without<Player>>,
    mut huge_slime_sprite_query: Query<
        (&Parent, &mut Falling),
        (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>),
    >,
) {
    const JUMP_POWER: f32 = 3.0;

    for (parent, mut falling) in huge_slime_sprite_query.iter_mut() {
        let (mut huge_slime, mut counter) = huge_slime_query.get_mut(parent.get()).unwrap();
        let timespan = if huge_slime.promoted { 35 } else { 60 };
        if let HugeSlimeState::Approach = huge_slime.state.clone() {
            // プレイヤーがいる場合はジャンプしながら接近
            if let Ok(_) = player_query.get_single() {
                // 60フレームに一度ジャンプ
                if counter.count % timespan == 0 {
                    falling.velocity = JUMP_POWER;
                }
            }

            // 6秒ごとに召喚フェイズに移行
            if counter.count == 360 {
                huge_slime.state = HugeSlimeState::Summon;
                info!("huge_slime.state = HugeSlimeState::Summon;");
                counter.count = 0;
            }
        }
    }
}

fn update_huge_slime_summon(
    player_query: Query<&Transform, With<Player>>,
    mut huge_slime_query: Query<
        (Entity, &mut HugeSlime, &Transform, &mut Counter),
        Without<Player>,
    >,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<SEEvent>,
    mut seed_writer: EventWriter<SpawnServantSeed>,
) {
    for parent in sprite_query.iter_mut() {
        let (huge_slime_entity, mut huge_slime, transform, mut counter) =
            huge_slime_query.get_mut(parent.get()).unwrap();

        if let HugeSlimeState::Summon = huge_slime.state {
            if let Ok(player) = player_query.get_single() {
                if counter.count == 60 {
                    info!("huge_slime.animation == 60");
                    let slimes = if huge_slime.promoted { 8 } else { 4 };
                    let circles = if huge_slime.promoted { 4 } else { 1 };
                    for n in 0..circles {
                        for i in 0..slimes {
                            let t = std::f32::consts::PI * 2.0 / slimes as f32; // 等間隔に配置した場合の角度
                            let a = rand::random::<f32>() * 3.0; // 起点は適当にばらけさせる
                            let angle = a + t * i as f32 + t * 0.5 * rand::random::<f32>(); // 少しランダムにずらす
                            let offset = Vec2::from_angle(angle) * 100.0 * (1.0 + n as f32); // 100ピクセルの演習場にばらまく
                            let to = player.translation.truncate() + offset;
                            info!("seed_writer.send(SpawnServantSeed");
                            seed_writer.send(SpawnServantSeed {
                                from: transform.translation.truncate(),
                                to,
                                actor_group: ActorGroup::Enemy,
                                owner: Some(huge_slime_entity),
                                servant_type: ServantType::Slime,
                                remote: false,
                            });
                        }
                    }
                    se_writer.send(SEEvent::pos(SE::Puyon, transform.translation.truncate()));
                }
            }

            if 120 <= counter.count {
                counter.count = 0;
                huge_slime.state = HugeSlimeState::Approach;
                info!("huge_slime.state = HugeSlimeState::Approach;");
            }
        }
    }
}

fn update_huge_slime_promote(
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform, &mut Counter), Without<Player>>,
    mut sprite_query: Query<&Parent, (With<HugeSlimeSprite>, Without<HugeSlime>, Without<Player>)>,
    mut se_writer: EventWriter<SEEvent>,
    assets: Res<GameAssets>,
    mut next_bgm: ResMut<NextBGM>,
) {
    for parent in sprite_query.iter_mut() {
        let (mut huge_slime, transform, mut counter) =
            huge_slime_query.get_mut(parent.get()).unwrap();
        if let HugeSlimeState::Promote = huge_slime.state {
            if counter.count == 120 {
                se_writer.send(SEEvent::pos(SE::Growl, transform.translation.truncate()));
                *next_bgm = NextBGM(Some(assets.sacred.clone()));
            } else if 300 <= counter.count {
                huge_slime.state = HugeSlimeState::Approach;
                counter.count = 0;
            }
        }
    }
}

fn promote(mut huge_slime_query: Query<(&mut HugeSlime, &Life, &mut Counter)>) {
    for (mut huge_slime, life, mut counter) in huge_slime_query.iter_mut() {
        if !huge_slime.promoted && life.life < 600 {
            huge_slime.state = HugeSlimeState::Promote;
            counter.count = 0;
            huge_slime.promoted = true;
        }
    }
}

#[derive(Component)]
pub struct DespawnHugeSlime;

fn despown(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform), With<DespawnHugeSlime>>,
    assets: Res<GameAssets>,
    mut theater_writer: EventWriter<TheaterEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(_player_transform) = player_query.get_single() {
        for (entity, life, boss_transform) in query.iter() {
            if life.life <= 0 {
                // いったんボスを消して、その場所に新しいボスをスプライトだけ出現させる
                commands.entity(entity).despawn_recursive();
                let e = commands.spawn((
                    CounterAnimated,
                    AseSpriteAnimation {
                        aseprite: assets.huge_slime.clone(),
                        animation: "idle".into(),
                    },
                    StateScoped(GameState::InGame),
                    Transform::from_translation(boss_transform.translation),
                    EntityDepth::new(),
                ));

                let entity = e.id();

                theater_writer.send(TheaterEvent::Play {
                    acts: vec![
                        Act::BGM(None),
                        Act::Shake(6.0),
                        Act::SE(SE::Kaminari),
                        Act::Flash {
                            position: boss_transform.translation.truncate(),
                            intensity: 10.0,
                            radius: TILE_SIZE * 30.0,
                            duration: 10,
                            reverse: false,
                        },
                        Act::Wait(60),
                        Act::Shake(6.0),
                        Act::SE(SE::Kaminari),
                        Act::Flash {
                            position: boss_transform.translation.truncate(),
                            intensity: 10.0,
                            radius: TILE_SIZE * 30.0,
                            duration: 10,
                            reverse: false,
                        },
                        Act::Wait(180),
                        Act::ShakeStart(Some(6.0)),
                        Act::SE(SE::Jishin),
                        Act::Flash {
                            position: boss_transform.translation.truncate(),
                            intensity: 10.0,
                            radius: TILE_SIZE * 15.0,
                            duration: 240,
                            reverse: true,
                        },
                        Act::Wait(240),
                        Act::SE(SE::Kaminari),
                        Act::Flash {
                            position: boss_transform.translation.truncate(),
                            intensity: 10.0,
                            radius: TILE_SIZE * 15.0,
                            duration: 240,
                            reverse: false,
                        },
                        Act::Despown(entity),
                        Act::ShakeStart(None),
                        Act::Wait(240),
                        Act::Ending,
                    ],
                });
            }
        }
    }
}

pub struct HugeSlimePlugin;

impl Plugin for HugeSlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            ((
                control,
                impact,
                update_huge_slime_growl,
                update_huge_slime_approach,
                update_huge_slime_summon,
                update_huge_slime_promote,
                promote,
                despown,
            )
                .chain(),)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
