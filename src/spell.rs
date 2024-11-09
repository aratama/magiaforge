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

    let cost = props.cost;

    if actor.mana < cost {
        return;
    }

    actor.mana = (actor.mana - cost).max(0);

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
            let props = spell_to_props(spell);
            actor.mana = (actor.mana - props.cost).max(0);
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
        }
    }
}
