use bevy::reflect::Reflect;

use crate::language::Dict;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Reflect)]
pub enum EquipmentType {
    Lantern,
    SpikeBoots,
}

pub const EQUIPMENTS: [EquipmentType; 1] = [EquipmentType::Lantern];

impl EquipmentType {
    pub fn to_props(&self) -> EquipmentProps {
        match self {
            EquipmentType::Lantern => EquipmentProps {
                icon: "lantern",
                name: Dict {
                    ja: "ランタン",
                    en: "Lantern",
                },
                price: 100,
                description: Dict {
                    ja: "暗闇を照らすランタン",
                    en: "A lantern that illuminates the darkness",
                },
            },
            EquipmentType::SpikeBoots => EquipmentProps {
                icon: "boots",
                name: Dict {
                    ja: "スパイクブーツ",
                    en: "Spike Boots",
                },
                price: 100,
                description: Dict {
                    ja: "滑り止めのついた革靴。移動速度が上昇する。",
                    en: "Leather shoes with anti-slip. Movement speed increases.",
                },
            },
        }
    }
}

pub struct EquipmentProps {
    pub icon: &'static str,
    pub name: Dict,
    pub price: u32,
    pub description: Dict,
}
