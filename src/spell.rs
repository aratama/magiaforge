use crate::{
    asset::GameAssets,
    bullet_type::BulletType,
    command::GameCommand,
    entity::{actor::Actor, bullet::spawn_bullets},
    spell_props::spell_to_props,
    world::CurrentLevel,
};
use bevy::prelude::*;
use bevy_simple_websocket::{ClientMessage, WebSocketState};

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
    current: &Res<CurrentLevel>,
    mut se_writer: &mut EventWriter<GameCommand>,
    websocket: &Res<WebSocketState>,

    mut actor: &mut Actor,
    actor_transform: &Transform,
    spell: Spell,
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
                &current,
                &mut se_writer,
                &websocket,
                &mut actor,
                &actor_transform,
                BulletType::BlueBullet,
            );
        }
        Spell::PurpleBolt => {
            spawn_bullets(
                &mut commands,
                &assets,
                &mut writer,
                &current,
                &mut se_writer,
                &websocket,
                &mut actor,
                &actor_transform,
                BulletType::PurpleBullet,
            );
        }
        Spell::SlimeCharge => {
            spawn_bullets(
                &mut commands,
                &assets,
                &mut writer,
                &current,
                &mut se_writer,
                &websocket,
                &mut actor,
                &actor_transform,
                BulletType::SlimeAttackBullet,
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
