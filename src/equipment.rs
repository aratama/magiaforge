use bevy::reflect::Reflect;

use crate::language::Dict;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Reflect)]
pub enum EquipmentType {
    Lantern,
}

impl EquipmentType {
    pub fn to_props(&self) -> EquipmentProps {
        match self {
            EquipmentType::Lantern => EquipmentProps {
                icon: "lantern",
                name: Dict {
                    en: "Lantern",
                    ja: "ランタン",
                },
                description: Dict {
                    ja: "暗闇を照らすランタン",
                    en: "A lantern that illuminates the darkness",
                },
            },
        }
    }
}

pub struct EquipmentProps {
    pub icon: &'static str,
    pub name: Dict,
    pub description: Dict,
}
