use crate::{
    config::GameConfig,
    constant::{MAX_ITEMS_IN_EQUIPMENT, MAX_WANDS},
    controller::player::Equipment,
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
}

impl PlayerState {
    pub fn from_config(config: &GameConfig) -> Self {
        let mut inventory = Inventory::new();
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
        inventory.sort();

        let mut equipments = [None; MAX_ITEMS_IN_EQUIPMENT];
        equipments[0] = Some(Equipment {
            equipment_type: EquipmentType::Lantern,
            price: 0,
        });

        let wands = [
            Some(Wand {
                wand_type: WandType::CypressWand,
                price: 0,
                slots: [
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
                index: 0,
            }),
            Some(Wand {
                wand_type: WandType::CypressWand,
                price: 0,
                slots: [
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
                index: 0,
            }),
            Some(Wand {
                wand_type: WandType::CypressWand,
                price: 0,
                slots: [
                    Some(WandSpell {
                        spell_type: SpellType::SummonEnemySlime,
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
                index: 0,
            }),
            None,
        ];

        PlayerState {
            name: config.player_name.clone(),
            life: 60,
            max_life: 60,
            inventory,
            equipments,
            wands,
        }
    }
}
