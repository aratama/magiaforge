use crate::{
    asset::GameAssets,
    constant::{
        ENEMY_BULLET_GROUP, ENEMY_GROUP, ENTITY_GROUP, MAX_SPELLS_IN_WAND, RABBIT_GROUP,
        WALL_GROUP, WITCH_BULLET_GROUP, WITCH_GROUP,
    },
    controller::remote::{send_remote_message, RemoteMessage},
    entity::{
        actor::{Actor, ActorGroup},
        bullet::{spawn_bullet, SpawnBullet, BULLET_SPAWNING_MARGIN},
        life::Life,
        servant_seed::SpawnServantSeed,
        witch::WITCH_COLLIDER_RADIUS,
    },
    se::{SEEvent, SE},
    spell::{SpellCast, SpellType},
};
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
    wand_index: usize,
) {
    if let Some(ref mut wand) = &mut actor.wands[wand_index] {
        // 1フレームあたりの残りの呪文詠唱回数
        // MultipleCast で増加することがあります
        let mut multicast = 1;

        if 0 < wand.delay {
            return;
        }

        while 0 < multicast && wand.index < MAX_SPELLS_IN_WAND {
            if let Some(spell) = wand.slots[wand.index] {
                let props = spell.spell_type.to_props();

                wand.delay += props.cast_delay.max(1);
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
                            memberships: match actor.actor_group {
                                ActorGroup::Player => WITCH_BULLET_GROUP,
                                ActorGroup::Enemy => ENEMY_BULLET_GROUP,
                            },
                            filters: match actor.actor_group {
                                ActorGroup::Player => ENEMY_GROUP,
                                ActorGroup::Enemy => WITCH_GROUP,
                            } | ENTITY_GROUP
                                | WALL_GROUP
                                | RABBIT_GROUP,
                        };

                        spawn_bullet(commands, assets.atlas.clone(), se_writer, &spawn);
                        actor.effects = default();

                        send_remote_message(writer, online, &RemoteMessage::Fire(spawn));
                    }
                    SpellCast::BulletSpeedUpDown { delta } => {
                        actor.effects.bullet_speed_buff_factor =
                            (actor.effects.bullet_speed_buff_factor + delta)
                                .max(-0.9)
                                .min(3.0);
                    }
                    SpellCast::Heal => {
                        if spell.spell_type == SpellType::Heal
                            && actor_life.life == actor_life.max_life
                        {
                            wand.delay += 1;
                        }

                        actor_life.life = (actor_life.life + 2).min(actor_life.max_life);
                        se_writer.send(SEEvent::pos(
                            SE::Heal,
                            actor_transform.translation.truncate(),
                        ));
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
                            owner: actor_entity,
                            servant_type,
                            actor_group: match (actor.actor_group, friend) {
                                (ActorGroup::Player, true) => ActorGroup::Player,
                                (ActorGroup::Player, false) => ActorGroup::Enemy,
                                (ActorGroup::Enemy, true) => ActorGroup::Enemy,
                                (ActorGroup::Enemy, false) => ActorGroup::Player,
                            },
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
                }
            } else {
                // 空欄の場合は残り詠唱回数は減りません
            }

            wand.index = wand.index + 1;
        }

        actor.effects = default();
        wand.index %= MAX_SPELLS_IN_WAND;
    }
}
