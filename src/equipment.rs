use crate::language::Dict;
use bevy::reflect::Reflect;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Reflect, strum::EnumIter)]
pub enum EquipmentType {
    Lantern,
    SpikeBoots,
    Telescope,
    Magnifier,
}

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
                    ja: "暗闇を照らすランタン。装備すると周囲が明るくなります。",
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
                    ja: "滑り止めのついた革靴。装備すると移動速度が上昇します。",
                    en: "Leather shoes with anti-slip. Movement speed increases.",
                },
            },
            EquipmentType::Telescope => EquipmentProps {
                icon: "zoom_out",
                name: Dict {
                    ja: "望遠鏡",
                    en: "Telecope",
                },
                price: 300,
                description: Dict {
                    ja: "細かい装飾のついか望遠鏡。装備すると遠くまで見えるようになります。",
                    en: "You can see far away.",
                },
            },
            EquipmentType::Magnifier => EquipmentProps {
                icon: "zoom_in",
                name: Dict {
                    ja: "拡大鏡",
                    en: "Magnifier",
                },
                price: 300,
                description: Dict {
                    ja: "大きなレンズのついた拡大鏡。装備すると手元が見やすくなります。",
                    en: "It is easier to see the details.",
                },
            },
        }
    }
}

pub struct EquipmentProps {
    pub icon: &'static str,
    pub name: Dict<&'static str>,
    pub price: u32,
    pub description: Dict<&'static str>,
}
