use crate::actor::jump_actor;
use crate::actor::rock::spawn_falling_rock;
use crate::actor::Actor;
use crate::actor::ActorFireState;
use crate::actor::ActorGroup;
use crate::actor::ActorState;
use crate::actor::ActorType;
use crate::collision::ENEMY_BULLET_GROUP;
use crate::collision::PLAYER_BULLET_GROUP;
use crate::component::metamorphosis::cast_metamorphosis;
use crate::component::metamorphosis::metamorphosis_effect;
use crate::component::metamorphosis::random_actor_type;
use crate::component::metamorphosis::Metamorphosed;
use crate::component::vertical::Vertical;
use crate::constant::MAX_SPELLS_IN_WAND;
use crate::constant::TILE_SIZE;
use crate::controller::player::Player;
use crate::controller::remote::send_remote_message;
use crate::controller::remote::RemoteMessage;
use crate::entity::bullet::spawn_bullet;
use crate::entity::bullet::BulletImage;
use crate::entity::bullet::SpawnBullet;
use crate::entity::bullet::Trigger;
use crate::entity::bullet::BULLET_SPAWNING_MARGIN;
use crate::entity::impact::SpawnImpact;
use crate::entity::servant_seed::ServantType;
use crate::entity::web::spawn_web;
use crate::hud::life_bar::LifeBarResource;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::page::in_game::LevelSetup;
use crate::random::randomize_velocity;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::SE;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionGroups;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_rapier2d::prelude::Group;
use bevy_rapier2d::prelude::Velocity;
use bevy_simple_websocket::ClientMessage;
use core::f32;
use rand::random;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct SpellCastBullet {
    pub slices: BulletImage,

    pub collier_radius: f32,

    /// 魔法弾の速度
    /// pixels_per_meter が 100.0 に設定されているので、
    /// 200は1フレームに2ピクセル移動する速度です
    pub speed: f32,

    pub lifetime: u32,
    pub damage: i32,
    pub impulse: f32,

    pub scattering: f32,

    pub light_intensity: f32,
    pub light_radius: f32,
    pub light_color_hlsa: [f32; 4],

    ///// 当たった対象への特殊効果の付与 /////////
    pub freeze: u32,
    pub levitation: u32,
    pub stagger: u32,
}

/// 呪文を詠唱したときの動作を表します
/// 弾丸系魔法は Bullet にまとめられており、
/// そのほかの魔法も動作の種別によって分類されています
#[derive(Debug, serde::Deserialize, Clone)]
pub enum SpellCast {
    NoCast,
    Bullet(SpellCastBullet),
    Heal,
    BulletSpeedUpDown {
        delta: f32,
    },
    MultipleCast {
        amount: u32,
    },
    Homing,
    HeavyShot,
    Summon {
        friend: bool,
        servant_type: ServantType,
        servant: bool,
    },
    Dash,
    QuickCast,
    Impact,
    PrecisionUp,
    Bomb,
    RockFall,
    Fireball,
    Spawn(Spawn),
    LightSword,
    Web,
    Levitation,
    Jump {
        velocity: f32,
        impulse: f32,
    },
    Metamorphosis,
    Slash {
        damage: u32,
    },
    Dispel,
    Clone,
    InfinityClone,
}

/// 現在のインデックスをもとに呪文を唱えます
/// マナが不足している場合は不発になる場合もあります
/// 返り値として詠唱で生じた詠唱遅延を返すので、呼び出し元はその値をアクターの詠唱遅延に加算する必要があります。
pub fn cast_spell(
    mut commands: &mut Commands,
    // resources
    registry: &Registry,
    life_bar_resource: &Res<LifeBarResource>,
    level: &Res<LevelSetup>,
    // events
    writer: &mut EventWriter<ClientMessage>,
    mut se: &mut EventWriter<SEEvent>,
    impact: &mut EventWriter<SpawnImpact>,
    mut spawn: &mut EventWriter<SpawnEvent>,
    // components
    actor_entity: Entity,
    mut actor: &mut Actor,
    actor_transform: &Transform,
    mut actor_impulse: &mut ExternalImpulse,
    actor_velocity: &Velocity,
    mut actor_vertical: &mut Vertical,
    mut collision_groups: &mut CollisionGroups,
    actor_metamorphosis: &Option<&Metamorphosed>,
    player: Option<&Player>,
    player_controlled: bool,
    // misc
    online: bool,
    wand_index: u8,
    trigger: Trigger,
) {
    let mut rng = rand::thread_rng();

    // 1フレームあたりの残りの呪文詠唱回数
    // MultipleCast で増加することがあります
    let mut multicast = 1;

    if 0 < actor.wands[wand_index as usize].delay {
        return;
    }

    let mut wand_delay = 0;

    let mut spell_index: usize = actor.wands[wand_index as usize].index;

    let mut clear_effect = false;

    let actor_position = actor_transform.translation.truncate();

    while 0 < multicast && spell_index < MAX_SPELLS_IN_WAND {
        if let Some(spell) = actor.wands[wand_index as usize].slots[spell_index].as_ref() {
            let props = registry.get_spell_props(&spell.spell_type);
            let original_delay = props.cast_delay.max(1) as i32;
            let delay = (original_delay as i32 - actor.effects.quick_cast as i32).max(1);
            actor.effects.quick_cast -= (original_delay - delay) as u32;
            wand_delay = wand_delay.max(delay as u32);
            multicast -= 1;

            let actor_props = registry.get_actor_props(actor.to_type());

            match props.cast {
                SpellCast::NoCast => {}
                SpellCast::Bullet(ref cast) => {
                    let normalized = actor.pointer.normalize();
                    let angle = actor.pointer.to_angle();
                    let updated_scattering = (cast.scattering - actor.effects.precision).max(0.0);
                    let angle_with_random = angle + (random::<f32>() - 0.5) * updated_scattering;
                    let direction = Vec2::from_angle(angle_with_random);
                    let range = actor_props.radius + BULLET_SPAWNING_MARGIN;
                    let bullet_position = actor_position + range * normalized;

                    let slash = actor.effects.slash.pop();

                    let spawn = SpawnBullet {
                        uuid: Uuid::new_v4(),
                        holder: None,
                        actor_group: actor.actor_group,
                        position: bullet_position,
                        velocity: direction
                            * cast.speed
                            * (1.0 + actor.effects.bullet_speed_buff_factor),
                        bullet_lifetime: cast.lifetime,
                        sender: Some(actor.uuid),
                        damage: cast.damage + actor.effects.bullet_damage_buff_amount,
                        impulse: cast.impulse,
                        slices: cast.slices.clone(),
                        collier_radius: cast.collier_radius,
                        light_intensity: cast.light_intensity,
                        light_radius: cast.light_radius,
                        light_color_hlsa: cast.light_color_hlsa,
                        homing: actor.effects.homing,
                        freeze: cast.freeze,
                        stagger: cast.stagger,
                        levitation: cast.levitation,
                        metamorphose: actor.effects.metamorphse,
                        dispel: 0 < actor.effects.dispel,
                        web: 0 < actor.effects.web,
                        slash,
                        groups: actor.actor_group.to_bullet_group(),
                    };

                    if 0 < actor.effects.dispel {
                        actor.effects.dispel -= 1;
                    }

                    if 0 < actor.effects.web {
                        actor.effects.web -= 1;
                    }

                    spawn_bullet(commands, registry, se, &spawn);
                    clear_effect = true;

                    // リモートへ呪文詠唱を伝えます
                    // リモートへ呪文詠唱を伝えるのはプレイヤーキャラクターのみです
                    // 味方モンスター召喚を行うと味方モンスターが敵プレイヤーへ攻撃を行っているように見えますが、
                    // 実際にはその攻撃は当たりません。マルチプレイ対戦における召喚は、
                    // あくまで相手ワールドに相手の敵となるモンスターを指定した位置に生成することで、
                    // それ以上の同期は行われません
                    //
                    // 味方モンスターと敵プレイヤーが戦っているように見えても、
                    // リモートのプレイヤーからはまた違う状況に見えている可能性があります
                    // たとえば、味方モンスターが敵プレイヤーを攻撃しているはずなのに、敵プレイヤーにダメージが入らないなどです
                    // これは、ホーム世界では味方モンスターが生き残っているのに、リモート世界ではその味方モンスターは既に倒されているということです
                    // 逆に、味方モンスターはいないはずなのに、敵プレイヤーが攻撃を受けているように見える、という状況もありえます
                    if player.is_some() {
                        // リモートの詠唱イベントを送信します
                        // ここでは、受信で生成される弾丸のプロパティは送信側で設定しています
                        let mut remove_bullet_props = spawn.clone();

                        // 送信側の魔女が発射した弾丸は、受信側では敵が発射した弾丸として扱われ、
                        // membershipsやfilterが逆になります
                        // ややこしいが、受信側にとってはプレイヤーキャラクター自信は WITCH_GROUP
                        remove_bullet_props.groups = match actor.actor_group {
                            ActorGroup::Friend => *ENEMY_BULLET_GROUP,
                            ActorGroup::Enemy => *PLAYER_BULLET_GROUP,
                            ActorGroup::Neutral => CollisionGroups::new(Group::NONE, Group::NONE), // 中立グループは弾丸を発射しません
                            ActorGroup::Entity => CollisionGroups::new(Group::NONE, Group::NONE), // エンティティは弾丸を発射しません
                        };
                        send_remote_message(
                            writer,
                            online,
                            &RemoteMessage::Fire(remove_bullet_props.clone()),
                        );
                    }
                }
                SpellCast::BulletSpeedUpDown { ref delta } => {
                    actor.effects.bullet_speed_buff_factor =
                        (actor.effects.bullet_speed_buff_factor + delta)
                            .max(-0.9)
                            .min(3.0);
                }
                SpellCast::Heal => {
                    actor.life = (actor.life + 2).min(actor.max_life);
                    se.send(SEEvent::pos(SE::Heal, actor_position));
                }
                SpellCast::MultipleCast { ref amount } => {
                    multicast += amount;
                }
                SpellCast::Homing => {
                    actor.effects.homing = (actor.effects.homing + 0.01).max(-0.1).min(0.1);
                }
                SpellCast::HeavyShot => {
                    actor.effects.bullet_damage_buff_amount += 5;
                }
                SpellCast::Summon {
                    friend,
                    servant_type,
                    servant,
                } => {
                    spawn.send(SpawnEvent {
                        position: actor_position,
                        entity: Spawn::Seed {
                            to: actor_position + actor.pointer,
                            owner: Some(actor_entity),
                            servant_type,
                            actor_group: match (actor.actor_group, friend) {
                                (ActorGroup::Friend, true) => ActorGroup::Friend,
                                (ActorGroup::Friend, false) => ActorGroup::Enemy,
                                (ActorGroup::Enemy, true) => ActorGroup::Enemy,
                                (ActorGroup::Enemy, false) => ActorGroup::Friend,
                                (ActorGroup::Neutral, _) => ActorGroup::Neutral,
                                (ActorGroup::Entity, _) => ActorGroup::Neutral,
                            },
                            remote: true,
                            servant,
                        },
                    });
                }
                SpellCast::Dash => {
                    actor_impulse.impulse += if 0.0 < actor.move_direction.length() {
                        actor.move_direction
                    } else {
                        actor.pointer.normalize()
                    } * 50000.0;
                    se.send(SEEvent::pos(SE::Shuriken, actor_position));
                }
                SpellCast::QuickCast => {
                    actor.effects.quick_cast += 6;
                }
                SpellCast::Impact => {
                    impact.send(SpawnImpact {
                        owner: Some(actor_entity),
                        position: actor_position,
                        radius: 32.0,
                        impulse: 60000.0,
                    });
                }
                SpellCast::PrecisionUp => {
                    actor.effects.precision += 0.1;
                }
                SpellCast::Bomb => {
                    let angle = actor.pointer.normalize_or_zero().to_angle();
                    let direction = Vec2::from_angle(angle) * 16.0;
                    let position = actor_position + direction;
                    spawn.send(SpawnEvent {
                        position,
                        entity: Spawn::Actor {
                            actor_type: ActorType::Bomb,
                            actor_group: ActorGroup::Neutral,
                        },
                    });
                }
                SpellCast::Spawn(ref entity) => {
                    let angle = actor.pointer.normalize_or_zero().to_angle();
                    let direction = Vec2::from_angle(angle) * 32.0;
                    let position = actor_position + direction;
                    spawn.send(SpawnEvent {
                        position,
                        entity: entity.clone(),
                    });
                    se.send(SEEvent::pos(SE::Status2, position));
                }
                SpellCast::RockFall => {
                    let position = actor_position + actor.pointer;
                    spawn_falling_rock(&mut commands, registry, position);
                    se.send(SEEvent::pos(SE::Status2, position));
                }
                SpellCast::Fireball => {
                    let position = actor_position + actor.pointer.normalize_or_zero() * 8.0;
                    let velocity = randomize_velocity(actor.pointer * 1.2, 0.5, 0.5);
                    spawn.send(SpawnEvent {
                        position,
                        entity: Spawn::Fireball {
                            velocity,
                            actor_group: actor.actor_group,
                        },
                    });
                }
                SpellCast::LightSword => {
                    let normalized = actor.pointer.normalize_or_zero();

                    // ポインターのベクトルに垂直な単位ベクトル
                    let vertical: Vec2 = Vec2::from_angle(f32::consts::PI * 0.5).rotate(normalized);

                    let bullet_offset = (rand::random::<f32>() - 0.5) * 128.0;

                    let bullet_position = actor_position
                        + normalized * -64.0 // ポインタと反対方向に戻る
                        + vertical* bullet_offset; // ポインタに垂直な方向にランダムにずらす

                    let slash = actor.effects.slash.pop();

                    let spawn = SpawnBullet {
                        uuid: Uuid::new_v4(),
                        holder: Some((actor_entity, trigger)),
                        actor_group: actor.actor_group,
                        position: bullet_position,
                        velocity: normalized * 0.01,
                        bullet_lifetime: 240,
                        sender: Some(actor.uuid),
                        damage: 330 + actor.effects.bullet_damage_buff_amount,
                        impulse: 0.0,
                        slices: BulletImage::Slice {
                            names: vec![
                                "light_sword".to_string(),
                                "light_catlass".to_string(),
                                "light_knife".to_string(),
                                "light_spear".to_string(),
                                "light_axe".to_string(),
                                "light_trident".to_string(),
                                "light_rapier".to_string(),
                                "light_flamberge".to_string(),
                            ],
                        },
                        collier_radius: 5.0,
                        light_intensity: 1.0,
                        light_radius: 50.0,
                        light_color_hlsa: [0.0, 1.0, 0.5, 1.0],
                        homing: actor.effects.homing,
                        freeze: 0,
                        stagger: 10,
                        levitation: 0,
                        metamorphose: None,
                        dispel: 0 < actor.effects.dispel,
                        web: 0 < actor.effects.web,
                        slash,
                        groups: actor.actor_group.to_bullet_group(),
                    };

                    if 0 < actor.effects.dispel {
                        actor.effects.dispel -= 1;
                    }

                    if 0 < actor.effects.web {
                        actor.effects.web -= 1;
                    }

                    spawn_bullet(commands, registry, se, &spawn);
                    clear_effect = true;
                }
                SpellCast::Web => {
                    actor.effects.web += 1;
                }
                SpellCast::Levitation => {
                    actor.levitation += 300;
                    se.send(SEEvent::pos(SE::Status2, actor_position));
                }
                SpellCast::Jump { velocity, impulse } => {
                    jump_actor(
                        &mut se,
                        &mut actor,
                        &mut actor_vertical,
                        &mut actor_impulse,
                        &mut collision_groups,
                        &actor_transform,
                        velocity,
                        impulse,
                    );
                }
                SpellCast::Metamorphosis => {
                    let morphing_to = random_actor_type(&mut rng, actor.to_type());
                    actor.effects.metamorphse = Some(morphing_to);
                }
                SpellCast::Slash { damage } => {
                    actor.effects.slash.push(damage);
                }
                SpellCast::Dispel => {
                    actor.effects.dispel = (actor.effects.dispel + 1).min(4);
                    se.send(SEEvent::pos(SE::Heal, actor_position));
                }
                SpellCast::Clone => {
                    // 分身も分身を詠唱できると指数関数的に増えてしまって強力すぎるので、
                    // プレイヤー本人が操作しているキャラクターのみ分身を詠唱できるものとします
                    if player.is_some() {
                        cast_clone(
                            &mut se,
                            &mut spawn,
                            actor,
                            actor_position,
                            &level,
                            player_controlled,
                            60 * 10,
                        );
                    }
                }
                SpellCast::InfinityClone => {
                    cast_clone(
                        &mut se,
                        &mut spawn,
                        actor,
                        actor_position,
                        &level,
                        player_controlled,
                        std::u32::MAX,
                    );
                }
            }
        } else {
            // 空欄の場合は残り詠唱回数は減りません
        }

        spell_index += 1;
    }

    // 変身効果が消費されていない場合は自分に効果を及ぼします

    if clear_effect {
        actor.effects = default();
    } else {
        if let Some(metamorphse) = actor.effects.metamorphse {
            // このフレームで変身効果が弾丸として発射されていなければ、自身に影響を及ぼします
            let entity = cast_metamorphosis(
                &mut commands,
                registry,
                &life_bar_resource,
                &mut se,
                &mut spawn,
                &actor_entity,
                actor.clone(),
                &actor_metamorphosis,
                actor_position,
                metamorphse,
            );
            commands.entity(entity).insert(Player::new(
                player.map(|p| p.name.clone()).unwrap_or_default(),
                &player
                    .map(|p| p.discovered_spells.clone())
                    .unwrap_or_default(),
            ));

            actor.effects.metamorphse = None;
        }

        if 0 < actor.effects.web {
            spawn_web(
                &mut commands,
                registry,
                &mut se,
                actor_position,
                actor.actor_group,
            );
            actor.effects.web = 0;
        }

        for damage in actor.effects.slash.iter() {
            spawn.send(SpawnEvent {
                position: actor_position,
                entity: Spawn::Slash {
                    velocity: actor_velocity.linvel,
                    actor_group: actor.actor_group,
                    angle: actor.pointer.to_angle(),
                    damage: *damage,
                },
            });
        }

        actor.effects.slash.clear();
    }

    actor.wands[wand_index as usize].delay = wand_delay;
    actor.wands[wand_index as usize].index = spell_index % MAX_SPELLS_IN_WAND;
}

fn cast_clone(
    se: &mut EventWriter<SEEvent>,
    spawn: &mut EventWriter<SpawnEvent>,
    actor: &Actor,
    actor_position: Vec2,
    level: &LevelSetup,
    player_controlled: bool,
    lifetime: u32,
) {
    if let Some(ref chunk) = level.chunk {
        for _ in 0..16 {
            let angle = f32::consts::PI * 2.0 * rand::random::<f32>();
            let position = actor_position + TILE_SIZE * 2.0 * Vec2::from_angle(angle);
            if chunk.get_tile_by_coords(position).is_floor() {
                let mut cloned = actor.clone();
                cloned.uuid = Uuid::new_v4();
                cloned.wait = 30; // これがないと一瞬で無限クローンしてしまうので注意
                cloned.fire_state = ActorFireState::Idle;
                cloned.fire_state_secondary = ActorFireState::Idle;
                cloned.move_direction = Vec2::ZERO;
                cloned.state = ActorState::Idle;
                cloned.cloned = Some(lifetime);
                cloned.life = 1;
                cloned.max_life = 1;
                cloned.golds = 0;
                spawn.send(SpawnEvent {
                    position,
                    entity: Spawn::Respawn {
                        actor: cloned,
                        player_controlled,
                    },
                });
                spawn.send(SpawnEvent {
                    position,
                    entity: Spawn::Particle {
                        particle: metamorphosis_effect(),
                    },
                });
                se.send(SEEvent::pos(SE::Heal, actor_position));

                break;
            }
        }
    }
}
