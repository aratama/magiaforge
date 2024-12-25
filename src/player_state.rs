use crate::config::GameConfig;
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
use bevy::ecs::query::QuerySingleError;

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub name: String,
    pub life: i32,
    pub max_life: i32,
    pub inventory: Inventory,
    pub equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],
    pub wands: [Wand; MAX_WANDS],
    pub golds: u32,
    pub current_wand: usize,
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
            current_wand: 0,
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
            Wand::with_slots([
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
            ]),
            Wand::empty(),
            Wand::empty(),
            Wand::empty(),
        ];

        PlayerState {
            name: config.player_name.clone(),
            life: 60,
            max_life: 60,
            inventory,
            equipments,
            wands,
            golds: 1000,
            current_wand: 0,
        }
    }
}
