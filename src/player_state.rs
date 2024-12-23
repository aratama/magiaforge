use bevy::ecs::query::QuerySingleError;

use crate::{
    config::GameConfig,
    constant::{MAX_ITEMS_IN_EQUIPMENT, MAX_WANDS},
    controller::player::{Equipment, Player},
    entity::{actor::Actor, life::Life},
    equipment::EquipmentType,
    inventory::Inventory,
    inventory_item::InventoryItemType,
    spell::SpellType,
    wand::{Wand, WandSpell, WandType},
};

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub name: String,
    pub life: i32,
    pub max_life: i32,
    pub inventory: Inventory,
    pub equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],
    pub wands: [Option<Wand>; MAX_WANDS],
    pub golds: u32,
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
        }
    }

    pub fn from(
        props: Result<(&Player, &Actor, &Life), QuerySingleError>,
        config: &GameConfig,
    ) -> Self {
        if let Ok((player, actor, life)) = props {
            PlayerState::new(player, actor, life)
        } else {
            PlayerState::from_config(config)
        }
    }

    pub fn from_config(config: &GameConfig) -> Self {
        let inventory = Inventory::new();

        let mut equipments = [None; MAX_ITEMS_IN_EQUIPMENT];
        equipments[0] = Some(Equipment {
            equipment_type: EquipmentType::Lantern,
            price: 0,
        });

        let wands = [
            Some(Wand::with_slots(
                WandType::CypressWand,
                [
                    Some(WandSpell {
                        spell_type: SpellType::MagicBolt,
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
            )),
            None,
            None,
            None,
        ];

        PlayerState {
            name: config.player_name.clone(),
            life: 60,
            max_life: 60,
            inventory,
            equipments,
            wands,
            golds: 1000,
        }
    }
}
