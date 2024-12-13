use crate::{
    config::GameConfig,
    constant::{MAX_ITEMS_IN_EQUIPMENT, MAX_WANDS},
    equipment::Equipment,
    inventory::Inventory,
    inventory_item::InventoryItem,
    spell::SpellType,
    wand::{Wand, WandType},
};

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub name: String,
    pub life: i32,
    pub max_life: i32,
    pub golds: i32,
    pub inventory: Inventory,
    pub equipments: [Option<Equipment>; MAX_ITEMS_IN_EQUIPMENT],
    pub wands: [Option<Wand>; MAX_WANDS],
}

impl PlayerState {
    pub fn from_config(config: &GameConfig) -> Self {
        let mut inventory = Inventory::new();
        inventory.insert(InventoryItem::Spell(SpellType::MagicBolt));
        inventory.insert(InventoryItem::Spell(SpellType::MagicBolt));
        inventory.insert(InventoryItem::Spell(SpellType::SlimeCharge));
        inventory.insert(InventoryItem::Spell(SpellType::Heal));
        inventory.insert(InventoryItem::Spell(SpellType::BulletSpeedUp));
        inventory.insert(InventoryItem::Spell(SpellType::BulletSpeedUp));
        inventory.insert(InventoryItem::Spell(SpellType::BulletSpeedUp));
        inventory.insert(InventoryItem::Spell(SpellType::BulletSpeedDoown));
        inventory.insert(InventoryItem::Spell(SpellType::BulletSpeedDoown));
        inventory.insert(InventoryItem::Spell(SpellType::BulletSpeedDoown));
        inventory.insert(InventoryItem::Spell(SpellType::PurpleBolt));
        inventory.insert(InventoryItem::Spell(SpellType::DualCast));
        inventory.insert(InventoryItem::Spell(SpellType::TripleCast));
        inventory.insert(InventoryItem::Equipment(Equipment::Lantern));
        inventory.insert(InventoryItem::Spell(SpellType::Homing));
        inventory.insert(InventoryItem::Spell(SpellType::Homing));
        inventory.insert(InventoryItem::Spell(SpellType::Homing));
        inventory.insert(InventoryItem::Spell(SpellType::Homing));
        inventory.insert(InventoryItem::Wand(WandType::KeyWand));
        inventory.insert(InventoryItem::Spell(SpellType::HeavyShot));
        inventory.insert(InventoryItem::Spell(SpellType::HeavyShot));
        inventory.insert(InventoryItem::Spell(SpellType::HeavyShot));
        inventory.insert(InventoryItem::Spell(SpellType::SummonSlime));
        inventory.sort();

        let mut equipments = [None; MAX_ITEMS_IN_EQUIPMENT];
        equipments[0] = Some(Equipment::Lantern);

        let wands = [
            Some(Wand {
                wand_type: WandType::CypressWand,
                slots: [
                    Some(SpellType::MagicBolt),
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
            None,
            None,
        ];

        PlayerState {
            name: config.player_name.clone(),
            life: 150,
            max_life: 150,
            golds: 10,
            inventory,
            equipments,
            wands,
        }
    }
}
