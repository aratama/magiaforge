use crate::{
    asset::GameAssets,
    command::GameCommand,
    entity::{
        actor::Actor,
        bullet::{spawn_bullets, SpawnBulletProps, BULLET_SPAWNING_MARGIN},
        witch::WITCH_COLLIDER_RADIUS,
    },
    spell::SpellType,
    spell_props::{spell_to_props, SpellCast},
};
use bevy::prelude::*;
use bevy_simple_websocket::ClientMessage;
use rand::random;

/// 現在のインデックスをもとに呪文を唱えます
/// マナが不足している場合は不発になる場合もあります
/// 返り値として詠唱で生じた詠唱遅延を返すので、呼び出し元はその値をアクターの詠唱遅延に加算する必要があります。
pub fn cast_spell(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    writer: &mut EventWriter<ClientMessage>,
    se_writer: &mut EventWriter<GameCommand>,
    actor: &mut Actor,
    actor_transform: &Transform,
    online: bool,
) -> i32 {
    if let Some(ref mut wand) = &mut actor.wands[actor.current_wand] {
        if 0 < actor.spell_delay {
            return 0;
        }

        if let Some(spell) = wand.slots[wand.index] {
            let props = spell_to_props(spell);

            if actor.mana < props.mana_drain {
                // info!(
                //     "not enough mana, current:{}, required:{}",
                //     actor.mana, props.mana_drain
                // );
                wand.shift();
                return 0;
            }

            // info!("cast {:?} ", spell);

            let props = spell_to_props(spell);
            actor.mana -= props.mana_drain;

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

                    spawn_bullets(
                        commands,
                        assets,
                        writer,
                        se_writer,
                        actor.uuid,
                        online,
                        SpawnBulletProps {
                            position: bullet_position,
                            velocity: direction * speed * (1.0 + actor.bullet_speed_buff_factor),
                            lifetime: lifetime,
                            owner: Some(actor.uuid),
                            group: actor.group,
                            filter: actor.filter,
                            damage,
                            impulse,
                            slice: slice.to_string(),
                            collier_radius,
                            light_intensity,
                            light_radius,
                            light_color_hlsa,
                        },
                    );
                    actor.bullet_speed_buff_factor = 0.0;
                    wand.shift();

                    return props.cast_delay as i32;
                }
                SpellCast::BulletSpeedUpDown { delta } => {
                    wand.shift();
                    actor.bullet_speed_buff_factor =
                        (actor.bullet_speed_buff_factor + delta).max(-0.9).min(3.0);

                    return props.cast_delay as i32;
                }
                SpellCast::Heal => {
                    wand.shift();

                    if spell == SpellType::Heal && actor.life == actor.max_life {
                        return 0;
                    }

                    actor.life = (actor.life + 2).min(actor.max_life);
                    se_writer.send(GameCommand::SEKaifuku(Some(
                        actor_transform.translation.truncate(),
                    )));

                    return props.cast_delay as i32;
                }
                SpellCast::MultipleCast { amount } => {
                    wand.shift();
                    let mut delay = 0;
                    for _ in 0..amount {
                        delay += cast_spell(
                            commands,
                            assets,
                            writer,
                            se_writer,
                            actor,
                            actor_transform,
                            online,
                        );
                    }
                    return delay;
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
