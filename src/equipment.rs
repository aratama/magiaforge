use crate::language::Dict;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Reflect,
    strum::EnumIter,
    Serialize,
    Deserialize,
)]
pub enum EquipmentType {
    Lantern,
    SpikeBoots,
    Telescope,
    Magnifier,
}

pub struct EquipmentProps {
    /// 現状ショップで購入できる装備品はランク0のもののみ
    pub rank: u32,
    pub icon: &'static str,
    pub name: Dict<&'static str>,
    pub price: u32,
    pub description: Dict<&'static str>,
}

impl EquipmentType {
    pub fn to_props(&self) -> EquipmentProps {
        match self {
            EquipmentType::Lantern => EquipmentProps {
                rank: 0,
                icon: "lantern",
                name: Dict {
                    ja: "ランタン",
                    en: "Lantern",
                    zh_cn: "灯笼",
                    es: "Linterna",
                    fr: "Lanterne",
                },
                price: 100,
                description: Dict {
                    ja: "暗闇を照らすランタン。装備すると周囲が明るくなります。",
                    en: "A lantern that illuminates the darkness",
                    zh_cn: "照亮黑暗的灯笼",
                    es: "Una linterna que ilumina la oscuridad",
                    fr: "Une lanterne qui illumine l'obscurité",
                },
            },
            EquipmentType::Telescope => EquipmentProps {
                rank: 0,
                icon: "zoom_out",
                name: Dict {
                    ja: "望遠鏡",
                    en: "Telecope",
                    zh_cn: "望远镜",
                    es: "Telescopio",
                    fr: "Télescope",
                },
                price: 300,
                description: Dict {
                    ja: "細かい装飾のついか望遠鏡。装備すると遠くまで見えるようになります。",
                    en: "A telescope with fine decorations. You can see far away.",
                    zh_cn: "带有精美装饰的望远镜。您可以看得很远。",
                    es: "Un telescopio con finas decoraciones. Puedes ver lejos.",
                    fr: "Un télescope avec de fines décorations. Vous pouvez voir loin.",
                },
            },
            EquipmentType::Magnifier => EquipmentProps {
                rank: 0,
                icon: "zoom_in",
                name: Dict {
                    ja: "拡大鏡",
                    en: "Magnifier",
                    zh_cn: "放大镜",
                    es: "Lupa",
                    fr: "Loupe",
                },
                price: 300,
                description: Dict {
                    ja: "大きなレンズのついた拡大鏡。装備すると手元が見やすくなります。",
                    en: "A magnifying glass with a large lens. It is easier to see the details.",
                    zh_cn: "带有大型镜片的放大镜。更容易看到细节。",
                    es: "Una lupa con una lente grande. Es más fácil ver los detalles.",
                    fr: "Une loupe avec une grande lentille. Il est plus facile de voir les détails.",
                },
            },
            EquipmentType::SpikeBoots => EquipmentProps {
                rank: 10,
                icon: "boots",
                name: Dict {
                    ja: "スパイクブーツ",
                    en: "Spike Boots",
                    zh_cn: "尖刺靴",
                    es: "Botas de clavos",
                    fr: "Bottes à crampons",
                },
                price: 100,
                description: Dict {
                    ja: "滑り止めのついた革靴。装備すると移動速度が上昇します。",
                    en: "Leather shoes with anti-slip. Movement speed increases.",
                    zh_cn: "带有防滑功能的皮鞋。移动速度增加。",
                    es: "Zapatos de cuero con antideslizante. Aumenta la velocidad de movimiento.",
                    fr: "Chaussures en cuir avec antidérapant. La vitesse de déplacement augmente.",
                },
            },
        }
    }
}
