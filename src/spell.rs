use crate::{
    asset::GameAssets,
    bullet_type::BulletType,
    command::GameCommand,
    entity::{actor::Actor, bullet::spawn_bullets},
    spell_props::spell_to_props,
};
use bevy::prelude::*;
use bevy_simple_websocket::ClientMessage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spell {
    MagicBolt,
    PurpleBolt,
    SlimeCharge,
    Heal,
}

/// スペルを唱えます
/// マナが不足している場合は不発になる場合もあります
pub fn cast_spell(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    mut writer: &mut EventWriter<ClientMessage>,
    mut se_writer: &mut EventWriter<GameCommand>,
    mut actor: &mut Actor,
    actor_transform: &Transform,
    spell: Spell,
    online: bool,
) {
    let props = spell_to_props(spell);

    if actor.mana < props.mana_drain {
        return;
    }

    if 0 < actor.spell_delay {
        return;
    }

    if spell == Spell::Heal && actor.life == actor.max_life {
        return;
    }

    let props = spell_to_props(spell);
    actor.mana -= props.mana_drain;
    actor.spell_delay += props.cast_delay as i32;

    match spell {
        Spell::MagicBolt => {
            spawn_bullets(
                &mut commands,
                &assets,
                &mut writer,
                &mut se_writer,
                &mut actor,
                &actor_transform,
                BulletType::BlueBullet,
                online,
            );
        }
        Spell::PurpleBolt => {
            spawn_bullets(
                &mut commands,
                &assets,
                &mut writer,
                &mut se_writer,
                &mut actor,
                &actor_transform,
                BulletType::PurpleBullet,
                online,
            );
        }
        Spell::SlimeCharge => {
            spawn_bullets(
                &mut commands,
                &assets,
                &mut writer,
                &mut se_writer,
                &mut actor,
                &actor_transform,
                BulletType::SlimeAttackBullet,
                online,
            );
        }
        Spell::Heal => {
            actor.life = (actor.life + 2).min(actor.max_life);

            se_writer.send(GameCommand::SEKaifuku(Some(
                actor_transform.translation.truncate(),
            )));
        }
    }
}
