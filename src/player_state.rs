use crate::actor::Actor;
use crate::constant::MAX_WANDS;
use crate::controller::player::Player;
use crate::inventory::Inventory;
use crate::spell::Spell;
use crate::wand::Wand;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub life: u32,
    pub max_life: u32,
    pub inventory: Inventory,
    pub wands: [Wand; MAX_WANDS],
    pub golds: u32,
    pub current_wand: u8,
    pub discovered_spells: HashSet<Spell>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let inventory = Inventory::new();

        let wands = [
            Wand::with_slots([None, None, None, None, None, None, None, None]),
            Wand::default(),
            Wand::default(),
            Wand::with_slots([None, None, None, None, None, None, None, None]),
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

impl PlayerState {
    pub fn new(player: &Player, actor: &Actor) -> Self {
        PlayerState {
            name: player.name.clone(),
            life: actor.life,
            max_life: actor.max_life,
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
                    discovered_spells.insert(spell.spell.clone());
                }
            }
        }
        self.discovered_spells = discovered_spells;
    }

    pub fn from_player(player: &Player, actor: &Actor) -> Self {
        let mut instance = PlayerState {
            name: player.name.clone(),
            life: actor.life,
            max_life: actor.max_life,
            inventory: actor.inventory.clone(),
            wands: actor.wands.clone(),
            golds: actor.golds,
            current_wand: actor.current_wand,
            discovered_spells: HashSet::new(),
        };
        instance.update_discovered_spell();
        instance
    }

    pub fn from_query(query: &Query<(&Player, &Actor)>) -> Self {
        let (player, actor) = query.single();
        PlayerState::from_player(player, actor)
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, _app: &mut App) {}
}
