use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::collision::ENEMY_GROUPS;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::life::Life;
use crate::component::vertical::Vertical;
use crate::constant::*;
use crate::controller::player::Player;
use crate::entity::actor::jump_actor;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorSpriteGroup;
use crate::entity::actor::ActorTypes;
use crate::entity::bullet::HomingTarget;
use crate::entity::impact::SpawnImpact;
use crate::entity::servant_seed::ServantType;
use crate::interpreter::Cmd;
use crate::interpreter::InterpreterEvent;
use crate::interpreter::Value;
use crate::language::Dict;
use crate::level::entities::SpawnEntity;
use crate::se::SEEvent;
use crate::se::SE;
use crate::set::FixedUpdateGameActiveSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

const HUGE_SLIME_COLLIDER_RADIUS: f32 = 24.0;

const IMPACT_MARGIN: f32 = 16.0;

#[derive(Component)]
pub struct Boss {
    pub name: Dict<String>,
}

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

pub fn spawn_huge_slime(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
) -> Entity {
    let mut slots = [None; MAX_SPELLS_IN_WAND];
    slots[0] = Some(WandSpell {
        spell_type: SpellType::MagicBolt,
        price: 0,
    });

    let mut actor = Actor {
        actor_type: ActorTypes::HugeSlime,
        radius: HUGE_SLIME_COLLIDER_RADIUS,
        actor_group: ActorGroup::Enemy,

        wands: [
            Wand::with_slots(slots),
            Wand::empty(),
            Wand::empty(),
            Wand::empty(),
        ],
        fire_resistance: true,
        poise: 60,
        ..default()
    };

    // スライムの王は通常のモンスターの4倍の速度で蜘蛛の巣から逃れます
    // 通常1秒しか拘束
    actor.floundering = 8;

    // スライムの王は凍結無効
    actor.defreeze = 10000;

    let entity = commands
        .spawn((
            Name::new("huge slime"),
            StateScoped(GameState::InGame),
            DespawnHugeSlime,
            Life::new(4000),
            HomingTarget,
            HugeSlime {
                state: HugeSlimeState::Growl,
                promoted: false,
            },
            Counter::up(0),
            actor,
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
                Damping::default(),
                ExternalForce::default(),
                ExternalImpulse::default(),
                ActiveEvents::COLLISION_EVENTS,
                *ENEMY_GROUPS,
            ),
        ))
        .with_children(|parent| {
            parent.spawn(ActorSpriteGroup).with_child((
                HugeSlimeSprite,
                CounterAnimated,
                AseSpriteAnimation {
                    aseprite: assets.huge_slime.clone(),
                    animation: Animation::default().with_tag("idle"),
                },
                Transform::from_xyz(0.0, 0.0, ENTITY_LAYER_Z),
            ));
        })
        .id();

    entity
}

fn impact(
    slime_query: Query<(Entity, &Transform, &Vertical), With<HugeSlime>>,
    mut impact_writer: EventWriter<SpawnImpact>,
) {
    for (entity, transform, vertical) in slime_query.iter() {
        if vertical.just_landed {
            impact_writer.send(SpawnImpact {
                owner: Some(entity),
                position: transform.translation.truncate(),
                radius: HUGE_SLIME_COLLIDER_RADIUS + IMPACT_MARGIN,
                impulse: 30000.0,
            });
        }
    }
}

fn update_huge_slime_growl(
    mut huge_slime_query: Query<(&mut HugeSlime, &Transform, &mut Counter), Without<Player>>,
    mut se_writer: EventWriter<SEEvent>,
) {
    for (mut huge_slime, transform, mut counter) in huge_slime_query.iter_mut() {
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
    mut huge_slime_query: Query<
        (
            &mut HugeSlime,
            &mut Counter,
            &mut Vertical,
            &mut Actor,
            &mut ExternalImpulse,
            &mut CollisionGroups,
            &Transform,
        ),
        Without<Player>,
    >,
    assets: Res<GameAssets>,
    constants: Res<Assets<GameActors>>,
    mut se: EventWriter<SEEvent>,
) {
    let constants = constants.get(assets.actors.id()).unwrap();
    let props = ActorTypes::HugeSlime.to_props(&constants);

    for (
        mut huge_slime,
        mut counter,
        mut vertical,
        mut actor,
        mut impulse,
        mut collision_groups,
        transform,
    ) in huge_slime_query.iter_mut()
    {
        let timespan = if huge_slime.promoted { 35 } else { 60 };
        if let HugeSlimeState::Approach = huge_slime.state.clone() {
            // プレイヤーがいる場合はジャンプしながら接近
            // 60フレームに一度ジャンプ
            // スライムを移動するのに、ExternalForceを直接操作しないこと
            // 直接操作すると、実行順序の関係で移動したりしなかったりという不安定なバグになります
            // ExternalForce は Actor の apply_external_force を通じて設定します
            if counter.count % timespan == 0 {
                actor.move_direction =
                    if let Ok((_, player_transform, _)) = player_query.get_single() {
                        (player_transform.translation.truncate() - transform.translation.truncate())
                            .normalize_or_zero()
                    } else {
                        Vec2::ZERO
                    };
                jump_actor(
                    &mut se,
                    &mut actor,
                    &mut vertical,
                    &mut impulse,
                    &mut collision_groups,
                    &transform,
                    props.jump,
                    props.move_force,
                );
            } else {
                actor.move_direction = Vec2::ZERO;
            }

            // 6秒ごとに召喚フェイズに移行
            if counter.count == 360 {
                huge_slime.state = HugeSlimeState::Summon;
                counter.count = 0;
            }
        }
    }
}

fn update_huge_slime_summon(
    player_query: Query<&Transform, With<Player>>,
    mut huge_slime_query: Query<
        (Entity, &mut HugeSlime, &Transform, &mut Counter, &mut Actor),
        Without<Player>,
    >,
    mut se_writer: EventWriter<SEEvent>,
    mut spawn: EventWriter<SpawnEntity>,
) {
    for (huge_slime_entity, mut huge_slime, transform, mut counter, mut actor) in
        huge_slime_query.iter_mut()
    {
        actor.move_direction = Vec2::ZERO;
        if let HugeSlimeState::Summon = huge_slime.state {
            if let Ok(player) = player_query.get_single() {
                if counter.count == 60 {
                    let slimes = if huge_slime.promoted { 8 } else { 4 };
                    let circles = if huge_slime.promoted { 4 } else { 1 };
                    for n in 0..circles {
                        for i in 0..slimes {
                            let t = std::f32::consts::PI * 2.0 / slimes as f32; // 等間隔に配置した場合の角度
                            let a = rand::random::<f32>() * 3.0; // 起点は適当にばらけさせる
                            let angle = a + t * i as f32 + t * 0.5 * rand::random::<f32>(); // 少しランダムにずらす
                            let offset = Vec2::from_angle(angle) * 30.0 * (1.0 + n as f32); // 100ピクセルの演習場にばらまく
                            let to = player.translation.truncate() + offset;
                            spawn.send(SpawnEntity::Seed {
                                from: transform.translation.truncate(),
                                to,
                                actor_group: ActorGroup::Enemy,
                                owner: Some(huge_slime_entity),
                                servant_type: ServantType::Slime,
                                remote: false,
                                servant: false,
                            });
                        }
                    }
                    se_writer.send(SEEvent::pos(SE::Puyon, transform.translation.truncate()));
                }
            }

            if 120 <= counter.count {
                counter.count = 0;
                huge_slime.state = HugeSlimeState::Approach;
            }
        }
    }
}

fn update_huge_slime_promote(
    mut huge_slime_query: Query<
        (&mut HugeSlime, &Transform, &mut Counter, &mut Actor),
        Without<Player>,
    >,
    mut se_writer: EventWriter<SEEvent>,
    assets: Res<GameAssets>,
    mut next_bgm: ResMut<NextBGM>,
) {
    for (mut huge_slime, transform, mut counter, mut actor) in huge_slime_query.iter_mut() {
        actor.move_direction = Vec2::ZERO;
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
        if !huge_slime.promoted && life.life < 2000 {
            huge_slime.state = HugeSlimeState::Promote;
            counter.count = 0;
            huge_slime.promoted = true;
        }
    }
}

#[derive(Component)]
pub struct DespawnHugeSlime;

fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Life, &Transform), With<DespawnHugeSlime>>,
    assets: Res<GameAssets>,
    mut theater_writer: EventWriter<InterpreterEvent>,
    player_query: Query<&Transform, With<Player>>,
    senarios: Res<Assets<GameSenarios>>,
) {
    let senarios = senarios.get(assets.senario.id()).unwrap();

    if let Ok(_player_transform) = player_query.get_single() {
        for (entity, life, boss_transform) in query.iter() {
            if life.life <= 0 {
                // いったんボスを消して、その場所に新しいボスをスプライトだけ出現させる
                commands.entity(entity).despawn_recursive();
                // info!("despawn {} {}", file!(), line!());
                commands.spawn((
                    Name::new("huge slime"),
                    CounterAnimated,
                    AseSpriteAnimation {
                        aseprite: assets.huge_slime.clone(),
                        animation: "idle".into(),
                    },
                    StateScoped(GameState::InGame),
                    Transform::from_translation(boss_transform.translation),
                    EntityDepth::new(),
                ));

                let mut commands = vec![Cmd::Set {
                    name: "position".to_string(),
                    value: Value::Vec2 {
                        x: boss_transform.translation.x,
                        y: boss_transform.translation.y,
                    },
                }];

                commands.extend(SenarioType::HugeSlime.to_acts(&senarios).clone());

                theater_writer.send(InterpreterEvent::Play { commands: commands });
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
                impact,
                update_huge_slime_growl,
                update_huge_slime_approach,
                update_huge_slime_summon,
                update_huge_slime_promote,
                promote,
                despawn,
            )
                .chain(),)
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
