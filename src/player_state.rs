use crate::actor::Actor;
use crate::actor::ActorExtra;
use crate::component::life::Life;
use crate::component::metamorphosis::Metamorphosed;
use crate::constant::MAX_WANDS;
use crate::controller::player::Player;
use crate::inventory::Inventory;
use crate::spell::SpellType;
use crate::wand::Wand;
use crate::wand::WandSpell;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::panic;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub life: u32,
    pub max_life: u32,
    pub inventory: Inventory,
    pub wands: [Wand; MAX_WANDS],
    pub golds: u32,
    pub current_wand: u8,
    pub discovered_spells: HashSet<SpellType>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let mut inventory = Inventory::new();

        if cfg!(feature = "item") {
            inventory.insert_spell(SpellType::MagicBolt);
            inventory.insert_spell(SpellType::MagicBolt);
            inventory.insert_spell(SpellType::LightBall);
            inventory.insert_spell(SpellType::WaterBall);
            inventory.insert_spell(SpellType::SlimeCharge);
            inventory.insert_spell(SpellType::Heal);
            inventory.insert_spell(SpellType::BulletSpeedUp);
            inventory.insert_spell(SpellType::BulletSpeedUp);
            inventory.insert_spell(SpellType::BulletSpeedUp);
            inventory.insert_spell(SpellType::BulletSpeedDoown);
            inventory.insert_spell(SpellType::BulletSpeedDoown);
            inventory.insert_spell(SpellType::BulletSpeedDoown);
            inventory.insert_spell(SpellType::PurpleBolt);
            inventory.insert_spell(SpellType::DualCast);
            inventory.insert_spell(SpellType::TripleCast);
            inventory.insert_spell(SpellType::Lantern);
            inventory.insert_spell(SpellType::Lantern);
            inventory.insert_spell(SpellType::SpikeBoots);
            inventory.insert_spell(SpellType::Telescope);
            inventory.insert_spell(SpellType::Magnifier);
            inventory.insert_spell(SpellType::Homing);
            inventory.insert_spell(SpellType::Homing);
            inventory.insert_spell(SpellType::HeavyShot);
            inventory.insert_spell(SpellType::HeavyShot);
            inventory.insert_spell(SpellType::SummonFriendSlime);
            inventory.insert_spell(SpellType::SummonEnemySlime);
            inventory.insert_spell(SpellType::SummonFriendEyeball);
            inventory.insert_spell(SpellType::SummonEnemyEyeball);
            inventory.insert_spell(SpellType::Dash);
            inventory.insert_spell(SpellType::QuickCast);
            inventory.insert_spell(SpellType::QuickCast);
            inventory.insert_spell(SpellType::Impact);
            inventory.insert_spell(SpellType::PrecisionUp);
            inventory.insert_spell(SpellType::PrecisionUp);
            inventory.insert_spell(SpellType::Bomb);
            inventory.insert_spell(SpellType::LightSword);
            inventory.insert_spell(SpellType::SpawnBookshelf);
            inventory.insert_spell(SpellType::SpawnJar);
            inventory.insert_spell(SpellType::RockFall);
            inventory.insert_spell(SpellType::Fireball);
            inventory.insert_spell(SpellType::SummonHugeSlime);
            inventory.insert_spell(SpellType::SummonChiken);
            inventory.insert_spell(SpellType::Servant);
            inventory.insert_spell(SpellType::Web);
            inventory.insert_spell(SpellType::Freeze);
            inventory.insert_spell(SpellType::Levitation);
            inventory.insert_spell(SpellType::ApplyLevitation);
            inventory.insert_spell(SpellType::Jump);
            inventory.insert_spell(SpellType::Metamorphosis);
            inventory.insert_spell(SpellType::Slash);
            inventory.insert_spell(SpellType::Dispel);
            inventory.sort();

            let wands = [
                Wand::with_slots([
                    Some(WandSpell::new(SpellType::Dispel)),
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
                    Some(WandSpell::new(SpellType::Dash)),
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
            current_wand: actor.current_wand,
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
            current_wand: actor.current_wand,
            discovered_spells: HashSet::new(),
        };
        instance.update_discovered_spell();
        instance
    }

    pub fn from_morph(morph: &Metamorphosed, actor: &Actor, life: &Life) -> Self {
        let ActorExtra::Witch {
            name,
            discovered_spells,
            ..
        } = &morph.original_actor.extra
        else {
            panic!("Not a witch");
        };

        let mut instance = PlayerState {
            name: name.clone(),
            life: life.life,
            max_life: life.max_life,
            inventory: actor.inventory.clone(),
            wands: actor.wands.clone(),
            golds: actor.golds,
            current_wand: actor.current_wand,
            discovered_spells: discovered_spells.clone(),
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
