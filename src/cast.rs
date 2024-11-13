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

/// スペルを唱えます
/// マナが不足している場合は不発になる場合もあります
pub fn cast_spell(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    writer: &mut EventWriter<ClientMessage>,
    se_writer: &mut EventWriter<GameCommand>,
    actor: &mut Actor,
    actor_transform: &Transform,
    spell: SpellType,
    online: bool,
) {
    let props = spell_to_props(spell);

    if actor.mana < props.mana_drain {
        return;
    }

    if 0 < actor.spell_delay {
        return;
    }

    if spell == SpellType::Heal && actor.life == actor.max_life {
        return;
    }

    // info!("cast {:?} ", spell);

    let props = spell_to_props(spell);
    actor.mana -= props.mana_drain;
    actor.spell_delay += props.cast_delay as i32;

    match props.category {
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
            let bullet_position = actor_transform.translation.truncate() + range * normalized;

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
        }
        SpellCast::BulletSpeedUpDown { delta } => {
            actor.bullet_speed_buff_factor =
                (actor.bullet_speed_buff_factor + delta).max(-0.9).min(3.0);
        }
        SpellCast::Heal => {
            actor.life = (actor.life + 2).min(actor.max_life);

            se_writer.send(GameCommand::SEKaifuku(Some(
                actor_transform.translation.truncate(),
            )));
        }
    }
}
