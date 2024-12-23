use crate::{
    config::GameConfig,
    constant::LAST_BOSS_LEVEL,
    controller::player::Player,
    entity::{actor::Actor, life::Life},
    equipment::EquipmentType,
    hud::overlay::OverlayEvent,
    inventory_item::InventoryItemType,
    level::{CurrentLevel, GameLevel},
    physics::GamePhysics,
    player_state::PlayerState,
    spell::SpellType,
    states::GameState,
    wand::{Wand, WandSpell, WandType},
};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_rapier2d::plugin::PhysicsSet;

fn process_debug_command(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
    mut level: ResMut<CurrentLevel>,
    config: Res<GameConfig>,
    mut writer: EventWriter<OverlayEvent>,
    mut physics: ResMut<GamePhysics>,
    mut player_query: Query<(&Player, &mut Actor, &Life)>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match ev.logical_key {
            Key::Character(ref c) => {
                local.push_str(c);
            }
            _ => {}
        }
    }

    if local.ends_with("item") {
        if let Ok((_, mut actor, _)) = player_query.get_single_mut() {
            let ref mut inventory = actor.inventory;

            inventory.insert_free(InventoryItemType::Spell(SpellType::MagicBolt));
            inventory.insert_free(InventoryItemType::Spell(SpellType::MagicBolt));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SlimeCharge));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Heal));
            inventory.insert_free(InventoryItemType::Spell(SpellType::BulletSpeedUp));
            inventory.insert_free(InventoryItemType::Spell(SpellType::BulletSpeedUp));
            inventory.insert_free(InventoryItemType::Spell(SpellType::BulletSpeedUp));
            inventory.insert_free(InventoryItemType::Spell(SpellType::BulletSpeedDoown));
            inventory.insert_free(InventoryItemType::Spell(SpellType::BulletSpeedDoown));
            inventory.insert_free(InventoryItemType::Spell(SpellType::BulletSpeedDoown));
            inventory.insert_free(InventoryItemType::Spell(SpellType::PurpleBolt));
            inventory.insert_free(InventoryItemType::Spell(SpellType::DualCast));
            inventory.insert_free(InventoryItemType::Spell(SpellType::TripleCast));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Lantern));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::SpikeBoots));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::SpikeBoots));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::SpikeBoots));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Telescope));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Telescope));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Magnifier));
            inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Magnifier));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
            inventory.insert_free(InventoryItemType::Wand(WandType::KeyWand));
            inventory.insert_free(InventoryItemType::Spell(SpellType::HeavyShot));
            inventory.insert_free(InventoryItemType::Spell(SpellType::HeavyShot));
            inventory.insert_free(InventoryItemType::Spell(SpellType::HeavyShot));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SummonFriendSlime));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SummonEnemySlime));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SummonFriendEyeball));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SummonEnemyEyeball));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Dash));
            inventory.sort();

            actor.wands[1] = Some(Wand::with_slots(
                WandType::CypressWand,
                [
                    Some(WandSpell {
                        spell_type: SpellType::SummonFriendSlime,
                        price: 0,
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ));
            actor.wands[2] = Some(Wand::with_slots(
                WandType::KeyWand,
                [
                    Some(WandSpell::new(SpellType::HeavyShot)),
                    Some(WandSpell::new(SpellType::HeavyShot)),
                    Some(WandSpell::new(SpellType::TripleCast)),
                    Some(WandSpell::new(SpellType::MagicBolt)),
                    Some(WandSpell::new(SpellType::MagicBolt)),
                    Some(WandSpell::new(SpellType::MagicBolt)),
                    None,
                    None,
                ],
            ));

            actor.wands[3] = Some(Wand::with_slots(
                WandType::CypressWand,
                [
                    Some(WandSpell {
                        spell_type: SpellType::Dash,
                        price: 0,
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
            ));
        }
    } else if local.ends_with("next") {
        local.clear();
        match level.next_level {
            GameLevel::Level(n) => {
                level.next_level = GameLevel::Level((n + 1) % LAST_BOSS_LEVEL);
                level.next_state = PlayerState::from(player_query.get_single(), &config);
            }
            GameLevel::MultiPlayArena => {
                level.next_level = GameLevel::Level(0);
                level.next_state = PlayerState::from(player_query.get_single(), &config);
            }
        };
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("home") {
        local.clear();
        level.next_level = GameLevel::Level(0);
        level.next_state = PlayerState::from(player_query.get_single(), &config);
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("arena") {
        local.clear();
        level.next_level = GameLevel::MultiPlayArena;
        level.next_state = PlayerState::from(player_query.get_single(), &config);
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("boss") {
        local.clear();
        level.next_level = GameLevel::Level(LAST_BOSS_LEVEL);
        level.next_state = PlayerState::from(player_query.get_single(), &config);
        writer.send(OverlayEvent::Close(GameState::Warp));
    } else if local.ends_with("ending") {
        local.clear();
        writer.send(OverlayEvent::Close(GameState::Ending));
    } else if local.ends_with("pause") {
        local.clear();
        physics.active = false;
    } else if local.ends_with("resume") {
        local.clear();
        physics.active = true;
    }
}

pub struct DebugCommandPlugin;

impl Plugin for DebugCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            process_debug_command
                .before(PhysicsSet::SyncBackend)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
