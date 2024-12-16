use crate::language::Dict;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EquipmentType {
    Lantern,
}

pub struct EquipmentProps {
    pub icon: &'static str,
    pub name: Dict,
    pub description: Dict,
}

pub fn equipment_to_props(equipment: EquipmentType) -> EquipmentProps {
    match equipment {
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
