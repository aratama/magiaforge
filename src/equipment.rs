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
                    zh_tw: "燈籠",
                    es: "Linterna",
                    fr: "Lanterne",
                    pt: "Lanterna",
                    de: "Laterne",
                    ko: "랜턴",
                    ru: "Фонарь",
                },
                price: 100,
                description: Dict {
                    ja: "暗闇を照らすランタン。装備すると周囲が明るくなります。",
                    en: "A lantern that illuminates the darkness",
                    zh_cn: "照亮黑暗的灯笼",
                    zh_tw: "照亮黑暗的燈籠",
                    es: "Una linterna que ilumina la oscuridad",
                    fr: "Une lanterne qui illumine l'obscurité",
                    pt: "Uma lanterna que ilumina a escuridão",
                    de: "Eine Laterne, die die Dunkelheit erhellt",
                    ko: "어둠을 밝히는 랜턴",
                    ru: "Фонарь, освещающий тьму",
                },
            },
            EquipmentType::Telescope => EquipmentProps {
                rank: 0,
                icon: "zoom_out",
                name: Dict {
                    ja: "望遠鏡",
                    en: "Telescope",
                    zh_cn: "望远镜",
                    zh_tw: "望遠鏡",
                    es: "Telescopio",
                    fr: "Télescope",
                    pt: "Telescópio",
                    de: "Teleskop",
                    ko: "망원경",
                    ru: "Телескоп",
                },
                price: 300,
                description: Dict {
                    ja: "細かい装飾のついか望遠鏡。装備すると遠くまで見えるようになります。",
                    en: "A telescope with fine decorations. You can see far away.",
                    zh_cn: "带有精美装饰的望远镜。您可以看得很远。",
                    zh_tw: "帶有精美裝飾的望遠鏡。您可以看得很遠。",
                    es: "Un telescopio con finas decoraciones. Puedes ver lejos.",
                    fr: "Un télescope avec de fines décorations. Vous pouvez voir loin.",
                    pt: "Um telescópio com decorações finas. Você pode ver longe.",
                    de: "Ein Teleskop mit feinen Verzierungen. Sie können weit sehen.",
                    ko: "정교한 장식이 있는 망원경. 멀리 볼 수 있습니다.",
                    ru: "Телескоп с тонкими украшениями. Вы можете видеть далеко.",
                },
            },
            EquipmentType::Magnifier => EquipmentProps {
                rank: 0,
                icon: "zoom_in",
                name: Dict {
                    ja: "拡大鏡",
                    en: "Magnifier",
                    zh_cn: "放大镜",
                    zh_tw: "放大鏡",
                    es: "Lupa",
                    fr: "Loupe",
                    pt: "Lupa",
                    de: "Lupe",
                    ko: "돋보기",
                    ru: "Лупа",
                },
                price: 300,
                description: Dict {
                    ja: "大きなレンズのついた拡大鏡。装備すると手元が見やすくなります。",
                    en: "A magnifying glass with a large lens. It is easier to see the details.",
                    zh_cn: "带有大型镜片的放大镜。更容易看到细节。",
                    zh_tw: "帶有大型鏡片的放大鏡。更容易看到細節。",
                    es: "Una lupa con una lente grande. Es más fácil ver los detalles.",
                    fr: "Une loupe avec une grande lentille. Il est plus facile de voir les détails.",
                    pt: "Uma lupa com uma lente grande. É mais fácil ver os detalhes.",
                    de: "Eine Lupe mit einer großen Linse. Es ist einfacher, die Details zu sehen.",
                    ko: "큰 렌즈가 있는 돋보기. 세부 사항을 더 쉽게 볼 수 있습니다.",
                    ru: "Лупа с большим объективом. Легче увидеть детали.",
                },
            },
            EquipmentType::SpikeBoots => EquipmentProps {
                rank: 10,
                icon: "boots",
                name: Dict {
                    ja: "スパイクブーツ",
                    en: "Spike Boots",
                    zh_cn: "尖刺靴",
                    zh_tw: "尖刺靴",
                    es: "Botas de clavos",
                    fr: "Bottes à crampons",
                    pt: "Botas com cravos",
                    de: "Stiefel mit Spikes",
                    ko: "스파이크 부츠",
                    ru: "Ботинки с шипами",
                },
                price: 100,
                description: Dict {
                    ja: "滑り止めのついた革靴。装備すると移動速度が上昇します。",
                    en: "Leather shoes with anti-slip. Movement speed increases.",
                    zh_cn: "带有防滑功能的皮鞋。移动速度增加。",
                    zh_tw: "帶有防滑功能的皮鞋。移動速度增加。",
                    es: "Zapatos de cuero con antideslizante. Aumenta la velocidad de movimiento.",
                    fr: "Chaussures en cuir avec antidérapant. La vitesse de déplacement augmente.",
                    pt: "Sapatos de couro com antiderrapante. A velocidade de movimento aumenta.",
                    de: "Lederschuhe mit Anti-Rutsch. Die Bewegungsgeschwindigkeit erhöht sich.",
                    ko: "미끄럼 방지 기능이 있는 가죽 신발. 이동 속도가 증가합니다.",
                    ru: "Кожаные ботинки с противоскользящими вставками. Скорость передвижения увеличивается.",
                },
            },
        }
    }
}
