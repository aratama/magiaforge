use crate::asset::GameAssets;
use crate::constant::ENEMY_BULLET_GROUP;
use crate::constant::ENEMY_GROUP;
use crate::constant::ENTITY_GROUP;
use crate::constant::MAX_SPELLS_IN_WAND;
use crate::constant::WALL_GROUP;
use crate::constant::WITCH_BULLET_GROUP;
use crate::constant::WITCH_GROUP;
use crate::controller::remote::send_remote_message;
use crate::controller::remote::RemoteMessage;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::entity::bomb::spawn_bomb;
use crate::entity::bomb::SpawnBomb;
use crate::entity::bullet::spawn_bullet;
use crate::entity::bullet::SpawnBullet;
use crate::entity::bullet::BULLET_SPAWNING_MARGIN;
use crate::entity::impact::SpawnImpact;
use crate::entity::life::Life;
use crate::entity::servant_seed::SpawnServantSeed;
use crate::entity::witch::WITCH_COLLIDER_RADIUS;
use crate::se::SEEvent;
use crate::se::SE;
use crate::spell::SpellCast;
use crate::spell::SpellType;
use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_simple_websocket::ClientMessage;
use rand::random;
use uuid::Uuid;

/// 現在のインデックスをもとに呪文を唱えます
/// マナが不足している場合は不発になる場合もあります
/// 返り値として詠唱で生じた詠唱遅延を返すので、呼び出し元はその値をアクターの詠唱遅延に加算する必要があります。
pub fn cast_spell(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    actor_entity: Entity,
    actor: &mut Actor,
    actor_life: &mut Life,
    actor_transform: &Transform,
    actor_impulse: &mut ExternalImpulse,
    online: bool,
    writer: &mut EventWriter<ClientMessage>,
    se_writer: &mut EventWriter<SEEvent>,
    slime_writer: &mut EventWriter<SpawnServantSeed>,
    impact_writer: &mut EventWriter<SpawnImpact>,
    bomb_writer: &mut EventWriter<SpawnBomb>,
    wand_index: usize,
    is_player: bool,
) {
    let ref mut wand = &mut actor.wands[wand_index];
    // 1フレームあたりの残りの呪文詠唱回数
    // MultipleCast で増加することがあります
    let mut multicast = 1;

    if 0 < wand.delay {
        return;
    }

    let mut clear_effect = false;

    while 0 < multicast && wand.index < MAX_SPELLS_IN_WAND {
        if let Some(spell) = wand.slots[wand.index] {
            let props = spell.spell_type.to_props();
            let original_delay = props.cast_delay.max(1) as i32;
            let delay = (original_delay as i32 - actor.effects.quick_cast as i32).max(1);
            actor.effects.quick_cast -= (original_delay - delay) as u32;
            wand.delay += delay as u32;
            multicast -= 1;

            match props.cast {
                SpellCast::Bullet {
                    slice,
                    collier_radius,
                    speed,
                    lifetime,
                    damage,
                    impulse,
                    scattering,
                    light_intensity,
                    light_radius,
                    light_color_hlsa,
                } => {
                    let normalized = actor.pointer.normalize();
                    let angle = actor.pointer.to_angle();
                    let updated_scattering = (scattering - actor.effects.precision).max(0.0);
                    let angle_with_random = angle + (random::<f32>() - 0.5) * updated_scattering;
                    let direction = Vec2::from_angle(angle_with_random);
                    let range = WITCH_COLLIDER_RADIUS + BULLET_SPAWNING_MARGIN;
                    let bullet_position =
                        actor_transform.translation.truncate() + range * normalized;

                    // 誰が発射したかに関わらず、弾丸は以下のグループに衝突します
                    let filter_base = ENTITY_GROUP | WALL_GROUP;

                    let spawn = SpawnBullet {
                        uuid: Uuid::new_v4(),
                        actor_group: actor.actor_group,
                        position: bullet_position,
                        velocity: direction
                            * speed
                            * (1.0 + actor.effects.bullet_speed_buff_factor),
                        bullet_lifetime: lifetime,
                        sender: Some(actor.uuid),
                        damage: damage + actor.effects.bullet_damage_buff_amount,
                        impulse,
                        slice: slice.to_string(),
                        collier_radius,
                        light_intensity,
                        light_radius,
                        light_color_hlsa,
                        homing: actor.effects.homing,
                        memberships: match actor.actor_group {
                            ActorGroup::Player => WITCH_BULLET_GROUP,
                            ActorGroup::Enemy => ENEMY_BULLET_GROUP,
                        },
                        filters: match actor.actor_group {
                            ActorGroup::Player => ENEMY_GROUP,
                            ActorGroup::Enemy => WITCH_GROUP,
                        } | filter_base,
                    };

                    spawn_bullet(commands, assets.atlas.clone(), se_writer, &spawn);
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
                    if is_player {
                        // リモートの詠唱イベントを送信します
                        // ここでは、受信で生成される弾丸のプロパティは送信側で設定しています
                        let mut remove_bullet_props = spawn.clone();

                        // 送信側の魔女が発射した弾丸は、受信側では敵が発射した弾丸として扱われ、
                        // membershipsやfilterが逆になります
                        remove_bullet_props.memberships = match actor.actor_group {
                            ActorGroup::Player => ENEMY_BULLET_GROUP,
                            ActorGroup::Enemy => WITCH_BULLET_GROUP,
                        };
                        // ややこしいが、受信側にとってはプレイヤーキャラクター自信は WITCH_GROUP
                        remove_bullet_props.filters = match actor.actor_group {
                            ActorGroup::Player => WITCH_GROUP,
                            ActorGroup::Enemy => ENEMY_GROUP,
                        } | filter_base;

                        send_remote_message(
                            writer,
                            online,
                            &RemoteMessage::Fire(remove_bullet_props.clone()),
                        );
                    }
                }
                SpellCast::BulletSpeedUpDown { delta } => {
                    actor.effects.bullet_speed_buff_factor =
                        (actor.effects.bullet_speed_buff_factor + delta)
                            .max(-0.9)
                            .min(3.0);
                }
                SpellCast::Heal => {
                    if spell.spell_type == SpellType::Heal && actor_life.life == actor_life.max_life
                    {
                        wand.delay += 1;
                    } else {
                        actor_life.life = (actor_life.life + 2).min(actor_life.max_life);
                        se_writer.send(SEEvent::pos(
                            SE::Heal,
                            actor_transform.translation.truncate(),
                        ));
                    }
                }
                SpellCast::MultipleCast { amount } => {
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
                } => {
                    slime_writer.send(SpawnServantSeed {
                        from: actor_transform.translation.truncate(),
                        to: actor_transform.translation.truncate() + actor.pointer,
                        owner: Some(actor_entity),
                        servant_type,
                        actor_group: match (actor.actor_group, friend) {
                            (ActorGroup::Player, true) => ActorGroup::Player,
                            (ActorGroup::Player, false) => ActorGroup::Enemy,
                            (ActorGroup::Enemy, true) => ActorGroup::Enemy,
                            (ActorGroup::Enemy, false) => ActorGroup::Player,
                        },
                        remote: true,
                    });
                }
                SpellCast::Dash => {
                    actor_impulse.impulse += if 0.0 < actor.move_direction.length() {
                        actor.move_direction
                    } else {
                        actor.pointer.normalize()
                    } * 50000.0;
                    se_writer.send(SEEvent::pos(
                        SE::Shuriken,
                        actor_transform.translation.truncate(),
                    ));
                }
                SpellCast::QuickCast => {
                    actor.effects.quick_cast += 6;
                }
                SpellCast::Impact => {
                    impact_writer.send(SpawnImpact {
                        owner: actor_entity,
                        position: actor_transform.translation.truncate(),
                        radius: 24.0,
                        impulse: 30000.0,
                    });
                }
                SpellCast::PrecisionUp => {
                    actor.effects.precision += 0.1;
                }
                SpellCast::Bomb => {
                    bomb_writer.send(SpawnBomb {
                        position: actor_transform.translation.truncate(), // TODO
                    });
                }
            }
        } else {
            // 空欄の場合は残り詠唱回数は減りません
        }

        wand.index = wand.index + 1;
    }

    if clear_effect {
        actor.effects = default();
    }

    // actor.effects = default();
    wand.index %= MAX_SPELLS_IN_WAND;
}
