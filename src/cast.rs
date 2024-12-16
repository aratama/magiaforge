use crate::{
    asset::GameAssets,
    constant::{
        ENEMY_BULLET_GROUP, ENEMY_GROUP, ENTITY_GROUP, WALL_GROUP, WITCH_BULLET_GROUP, WITCH_GROUP,
    },
    controller::remote::{send_remote_message, RemoteMessage},
    entity::{
        actor::{Actor, ActorGroup},
        bullet::{spawn_bullet, SpawnBullet, BULLET_SPAWNING_MARGIN},
        life::Life,
        slime_seed::SpawnSlimeSeed,
        witch::WITCH_COLLIDER_RADIUS,
    },
    se::{SECommand, SE},
    spell::SpellType,
    spell_props::{spell_to_props, SpellCast},
};
use bevy::prelude::*;
use bevy_simple_websocket::ClientMessage;
use rand::random;
use uuid::Uuid;

/// 現在のインデックスをもとに呪文を唱えます
/// マナが不足している場合は不発になる場合もあります
/// 返り値として詠唱で生じた詠唱遅延を返すので、呼び出し元はその値をアクターの詠唱遅延に加算する必要があります。
pub fn cast_spell(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    writer: &mut EventWriter<ClientMessage>,
    se_writer: &mut EventWriter<SECommand>,
    actor_entity: Entity,
    actor: &mut Actor,
    actor_life: &mut Life,
    actor_transform: &Transform,
    online: bool,
    slime_writer: &mut EventWriter<SpawnSlimeSeed>,
) -> i32 {
    if let Some(ref mut wand) = &mut actor.wands[actor.current_wand] {
        if 0 < actor.spell_delay {
            return 0;
        }

        if let Some(spell) = wand.slots[wand.index] {
            let props = spell_to_props(spell.spell_type);

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
                    let angle_with_random = angle + (random::<f32>() - 0.5) * scattering;
                    let direction = Vec2::from_angle(angle_with_random);
                    let range = WITCH_COLLIDER_RADIUS + BULLET_SPAWNING_MARGIN;
                    let bullet_position =
                        actor_transform.translation.truncate() + range * normalized;

                    let spawn = SpawnBullet {
                        uuid: Uuid::new_v4(),
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
                        group: match actor.actor_group {
                            ActorGroup::Player => WITCH_BULLET_GROUP,
                            ActorGroup::Enemy => ENEMY_BULLET_GROUP,
                        },
                        filter: match actor.actor_group {
                            ActorGroup::Player => ENEMY_GROUP,
                            ActorGroup::Enemy => WITCH_GROUP,
                        } | ENTITY_GROUP
                            | WALL_GROUP,
                    };

                    spawn_bullet(commands, assets.atlas.clone(), se_writer, &spawn);
                    actor.effects = default();
                    wand.shift();

                    send_remote_message(writer, online, &RemoteMessage::Fire(spawn));

                    return props.cast_delay as i32;
                }
                SpellCast::BulletSpeedUpDown { delta } => {
                    wand.shift();
                    actor.effects.bullet_speed_buff_factor =
                        (actor.effects.bullet_speed_buff_factor + delta)
                            .max(-0.9)
                            .min(3.0);

                    return props.cast_delay as i32;
                }
                SpellCast::Heal => {
                    wand.shift();

                    if spell.spell_type == SpellType::Heal && actor_life.life == actor_life.max_life
                    {
                        return 0;
                    }

                    actor_life.life = (actor_life.life + 2).min(actor_life.max_life);
                    se_writer.send(SECommand::pos(
                        SE::Heal,
                        actor_transform.translation.truncate(),
                    ));

                    return props.cast_delay as i32;
                }
                SpellCast::MultipleCast { amount } => {
                    wand.shift();
                    let mut delay = 0;
                    for _ in 0..amount {
                        delay = delay.max(cast_spell(
                            commands,
                            assets,
                            writer,
                            se_writer,
                            actor_entity,
                            actor,
                            actor_life,
                            actor_transform,
                            online,
                            slime_writer,
                        ));
                    }
                    return delay;
                }
                SpellCast::Homing => {
                    wand.shift();
                    actor.effects.homing = (actor.effects.homing + 0.01).max(-0.1).min(0.1);
                    return props.cast_delay as i32;
                }
                SpellCast::HeavyShot => {
                    wand.shift();
                    actor.effects.bullet_damage_buff_amount += 5;
                    return props.cast_delay as i32;
                }
                SpellCast::SummonSlime { friend } => {
                    wand.shift();
                    slime_writer.send(SpawnSlimeSeed {
                        from: actor_transform.translation.truncate(),
                        to: actor_transform.translation.truncate() + actor.pointer,
                        owner: actor_entity,
                        actor_group: match (actor.actor_group, friend) {
                            (ActorGroup::Player, true) => ActorGroup::Player,
                            (ActorGroup::Player, false) => ActorGroup::Enemy,
                            (ActorGroup::Enemy, true) => ActorGroup::Enemy,
                            (ActorGroup::Enemy, false) => ActorGroup::Player,
                        },
                    });
                    return props.cast_delay as i32;
                }
            }
        } else {
            wand.shift();
            return 0;
        }
    } else {
        return 0;
    }
}
