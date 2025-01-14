use crate::component::life::Life;
use crate::constant::MAX_WANDS;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::inventory::Inventory;
use crate::inventory_item::InventoryItemType;
use crate::spell::SpellType;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub life: i32,
    pub max_life: i32,
    pub inventory: Inventory,
    pub wands: [Wand; MAX_WANDS],
    pub golds: u32,
    pub current_wand: u32,
    pub discovered_spells: HashSet<SpellType>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let mut inventory = Inventory::new();

        if cfg!(feature = "item") {
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
            inventory.insert_free(InventoryItemType::Spell(SpellType::Lantern));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Lantern));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SpikeBoots));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SpikeBoots));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SpikeBoots));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Telescope));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Magnifier));
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
            inventory.insert_free(InventoryItemType::Spell(SpellType::Impact));
            inventory.insert_free(InventoryItemType::Spell(SpellType::PrecisionUp));
            inventory.insert_free(InventoryItemType::Spell(SpellType::PrecisionUp));
            inventory.insert_free(InventoryItemType::Spell(SpellType::PrecisionUp));
            inventory.insert_free(InventoryItemType::Spell(SpellType::PrecisionUp));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Bomb));
            inventory.insert_free(InventoryItemType::Spell(SpellType::LightSword));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SpawnBookshelf));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SpawnJar));
            inventory.insert_free(InventoryItemType::Spell(SpellType::RockFall));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Fireball));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SummonHugeSlime));
            inventory.insert_free(InventoryItemType::Spell(SpellType::SummonChiken));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Servant));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Web));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Freeze));
            inventory.insert_free(InventoryItemType::Spell(SpellType::Levitation));
            inventory.insert_free(InventoryItemType::Spell(SpellType::ApplyLevitation));
            inventory.sort();

            let wands = [
                Wand::with_slots([
                    Some(WandSpell {
                        spell_type: SpellType::Levitation,
                        price: 0,
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]),
                Wand::with_slots([
                    Some(WandSpell::new(SpellType::QuickCast)),
                    Some(WandSpell::new(SpellType::QuickCast)),
                    Some(WandSpell::new(SpellType::HeavyShot)),
                    Some(WandSpell::new(SpellType::HeavyShot)),
                    Some(WandSpell::new(SpellType::TripleCast)),
                    Some(WandSpell::new(SpellType::MagicBolt)),
                    Some(WandSpell::new(SpellType::MagicBolt)),
                    Some(WandSpell::new(SpellType::MagicBolt)),
                ]),
                Wand::with_slots([
                    Some(WandSpell {
                        spell_type: SpellType::Bomb,
                        price: 0,
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]),
                Wand::with_slots([
                    Some(WandSpell {
                        spell_type: SpellType::Dash,
                        price: 0,
                    }),
                    Some(WandSpell {
                        spell_type: SpellType::Lantern,
                        price: 0,
                    }),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]),
            ];

            let mut instance = PlayerState {
                name: "".to_string(),
                life: 60,
                max_life: 60,
                inventory,
                wands,
                golds: 1000,
                current_wand: 0,
                discovered_spells: HashSet::new(),
            };
            instance.update_discovered_spell();

            instance
        } else {
            let wands = [
                Wand::with_slots([
                    Some(WandSpell::new(SpellType::MagicBolt)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]),
                Wand::empty(),
                Wand::empty(),
                Wand::with_slots([
                    Some(WandSpell::new(SpellType::LightBall)),
                    Some(WandSpell::new(SpellType::Lantern)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ]),
            ];

            let mut instance = PlayerState {
                name: "".to_string(),
                life: 60,
                max_life: 60,
                inventory,
                wands,
                golds: 1000,
                current_wand: 0,
                discovered_spells: HashSet::new(),
            };
            instance.update_discovered_spell();

            instance
        }
    }
}

impl PlayerState {
    pub fn new(player: &Player, actor: &Actor, life: &Life) -> Self {
        PlayerState {
            name: player.name.clone(),
            life: life.life,
            max_life: life.max_life,
            inventory: actor.inventory.clone(),
            wands: actor.wands.clone(),
            golds: actor.golds,
            current_wand: actor.current_wand as u32,
            discovered_spells: player.discovered_spells.clone(),
        }
    }

    pub fn update_discovered_spell(&mut self) {
        let mut discovered_spells = self.discovered_spells.clone();
        for wand in self.wands.iter() {
            for spell in wand.slots.iter() {
                if let Some(spell) = spell {
                    discovered_spells.insert(spell.spell_type);
                }
            }
        }
        self.discovered_spells = discovered_spells;
    }

    pub fn from_player(player: &Player, actor: &Actor, life: &Life) -> Self {
        let mut instance = PlayerState {
            name: player.name.clone(),
            life: life.life,
            max_life: life.max_life,
            inventory: actor.inventory.clone(),
            wands: actor.wands.clone(),
            golds: actor.golds,
            current_wand: actor.current_wand as u32,
            discovered_spells: HashSet::new(),
        };
        instance.update_discovered_spell();
        instance
    }

    pub fn from_query(query: &Query<(&Player, &Actor, &Life)>) -> Self {
        let (player, actor, life) = query.single();
        PlayerState::from_player(player, actor, life)
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, _app: &mut App) {}
}
