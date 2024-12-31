use crate::constant::MAX_ITEMS_IN_EQUIPMENT;
use crate::constant::MAX_WANDS;
use crate::controller::player::Equipment;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::entity::life::Life;
use crate::equipment::EquipmentType;
use crate::inventory::Inventory;
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
    pub equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],
    pub wands: [Wand; MAX_WANDS],
    pub golds: u32,
    pub current_wand: u32,
    pub discovered_spells: HashSet<SpellType>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let inventory = Inventory::new();

        let mut equipments = [None; MAX_ITEMS_IN_EQUIPMENT];
        equipments[0] = Some(Equipment {
            equipment_type: EquipmentType::Lantern,
            price: 0,
        });

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
                None,
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
            equipments,
            wands,
            golds: 1000,
            current_wand: 0,
            discovered_spells: HashSet::new(),
        };
        instance.update_discovered_spell();
        instance
    }
}

impl PlayerState {
    pub fn new(player: &Player, actor: &Actor, life: &Life) -> Self {
        PlayerState {
            name: player.name.clone(),
            life: life.life,
            max_life: life.max_life,
            inventory: actor.inventory.clone(),
            equipments: actor.equipments.clone(),
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
            equipments: actor.equipments.clone(),
            wands: actor.wands.clone(),
            golds: actor.golds,
            current_wand: actor.current_wand as u32,
            discovered_spells: HashSet::new(),
        };
        instance.update_discovered_spell();
        instance
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, _app: &mut App) {}
}
