use crate::config::GameConfig;
use crate::constant::LAST_BOSS_LEVEL;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::life::Life;
use crate::equipment::EquipmentType;
use crate::hud::overlay::OverlayEvent;
use crate::inventory_item::InventoryItemType;
use crate::page::in_game::CurrentLevel;
use crate::page::in_game::GameLevel;
use crate::physics::GamePhysics;
use crate::player_state::PlayerState;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;

fn process_debug_command(
    mut commands: Commands,
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
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

    if local.ends_with("@item") {
        local.clear();
        commands.run_system_cached(debug_item);
    } else if local.ends_with("@next") {
        local.clear();
        commands.run_system_cached(debug_next);
    } else if local.ends_with("@home") {
        local.clear();
        commands.run_system_cached(debug_home);
    } else if local.ends_with("@arena") {
        local.clear();
        commands.run_system_cached(debug_arena);
    } else if local.ends_with("@boss") {
        local.clear();
        commands.run_system_cached(debug_boss);
    } else if local.ends_with("@ending") {
        local.clear();
        commands.run_system_cached(debug_ending);
    } else if local.ends_with("@pause") {
        local.clear();
        commands.run_system_cached(debug_pause);
    } else if local.ends_with("@resume") {
        local.clear();
        commands.run_system_cached(debug_resume);
    }
}

fn debug_item(mut player_query: Query<(&Player, &mut Actor, &Life)>) {
    if let Ok((_, mut actor, _)) = player_query.get_single_mut() {
        let ref mut inventory = actor.inventory;

        inventory.insert_free(InventoryItemType::Spell(SpellType::MagicBolt));
        inventory.insert_free(InventoryItemType::Spell(SpellType::MagicBolt));
        inventory.insert_free(InventoryItemType::Spell(SpellType::WaterBall));
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
        inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Lantern));
        inventory.insert_free(InventoryItemType::Equipment(EquipmentType::SpikeBoots));
        inventory.insert_free(InventoryItemType::Equipment(EquipmentType::SpikeBoots));
        inventory.insert_free(InventoryItemType::Equipment(EquipmentType::SpikeBoots));
        inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Telescope));
        inventory.insert_free(InventoryItemType::Equipment(EquipmentType::Magnifier));
        inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
        inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
        inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
        inventory.insert_free(InventoryItemType::Spell(SpellType::Homing));
        inventory.insert_free(InventoryItemType::Spell(SpellType::HeavyShot));
        inventory.insert_free(InventoryItemType::Spell(SpellType::HeavyShot));
        inventory.insert_free(InventoryItemType::Spell(SpellType::HeavyShot));
        inventory.insert_free(InventoryItemType::Spell(SpellType::SummonFriendSlime));
        inventory.insert_free(InventoryItemType::Spell(SpellType::SummonEnemySlime));
        inventory.insert_free(InventoryItemType::Spell(SpellType::SummonFriendEyeball));
        inventory.insert_free(InventoryItemType::Spell(SpellType::SummonEnemyEyeball));
        inventory.insert_free(InventoryItemType::Spell(SpellType::Dash));
        inventory.insert_free(InventoryItemType::Spell(SpellType::QuickCast));
        inventory.insert_free(InventoryItemType::Spell(SpellType::QuickCast));
        inventory.insert_free(InventoryItemType::Spell(SpellType::QuickCast));
        inventory.insert_free(InventoryItemType::Spell(SpellType::QuickCast));
        inventory.insert_free(InventoryItemType::Spell(SpellType::QuickCast));
        inventory.insert_free(InventoryItemType::Spell(SpellType::QuickCast));
        inventory.sort();

        actor.wands[1] = Some(Wand::with_slots([
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
        ]));
        actor.wands[2] = Some(Wand::with_slots([
            Some(WandSpell::new(SpellType::HeavyShot)),
            Some(WandSpell::new(SpellType::HeavyShot)),
            Some(WandSpell::new(SpellType::TripleCast)),
            Some(WandSpell::new(SpellType::MagicBolt)),
            Some(WandSpell::new(SpellType::MagicBolt)),
            Some(WandSpell::new(SpellType::MagicBolt)),
            None,
            None,
        ]));

        actor.wands[3] = Some(Wand::with_slots([
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
        ]));
    }
}

fn debug_next(
    mut level: ResMut<CurrentLevel>,
    config: Res<GameConfig>,
    player_query: Query<(&Player, &mut Actor, &Life)>,
    mut writer: EventWriter<OverlayEvent>,
) {
    match level.next_level {
        GameLevel::Level(n) => {
            level.next_level = GameLevel::Level((n + 1) % LAST_BOSS_LEVEL);
            level.next_state = PlayerState::from(player_query.get_single(), &config);
            writer.send(OverlayEvent::Close(GameState::Warp));
        }
        GameLevel::MultiPlayArena => {
            level.next_level = GameLevel::Level(0);
            level.next_state = PlayerState::from(player_query.get_single(), &config);
            writer.send(OverlayEvent::Close(GameState::Warp));
        }
    };
}

fn debug_home(
    mut level: ResMut<CurrentLevel>,
    config: Res<GameConfig>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &mut Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(0);
    level.next_state = PlayerState::from(player_query.get_single(), &config);
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_arena(
    mut level: ResMut<CurrentLevel>,
    config: Res<GameConfig>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &mut Actor, &Life)>,
) {
    level.next_level = GameLevel::MultiPlayArena;
    level.next_state = PlayerState::from(player_query.get_single(), &config);
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_boss(
    mut level: ResMut<CurrentLevel>,
    config: Res<GameConfig>,
    mut writer: EventWriter<OverlayEvent>,
    player_query: Query<(&Player, &mut Actor, &Life)>,
) {
    level.next_level = GameLevel::Level(LAST_BOSS_LEVEL);
    level.next_state = PlayerState::from(player_query.get_single(), &config);
    writer.send(OverlayEvent::Close(GameState::Warp));
}

fn debug_ending(mut writer: EventWriter<OverlayEvent>) {
    writer.send(OverlayEvent::Close(GameState::Ending));
}

fn debug_pause(mut physics: ResMut<GamePhysics>) {
    physics.active = false;
}

fn debug_resume(mut physics: ResMut<GamePhysics>) {
    physics.active = true;
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
