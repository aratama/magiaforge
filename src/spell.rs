use crate::cast::SpellCast;
use crate::cast::SpellCastBullet;
use crate::cast::SpellCastEntityType;
use crate::constant::TILE_SIZE;
use crate::entity::servant_seed::ServantType;
use crate::language::Dict;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Reflect,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    strum::EnumIter,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum SpellType {
    MagicBolt,
    LightBall,
    PurpleBolt,
    SlimeCharge,
    WaterBall,
    BulletSpeedDoown,
    DualCast,
    SummonEnemySlime,
    SummonEnemyEyeball,
    HeavyShot,
    Bomb,
    TripleCast,
    Homing,
    SummonFriendSlime,
    PrecisionUp,
    SpawnBookshelf,
    SummonFriendEyeball,
    Dash,
    Impact,
    BulletSpeedUp,
    Heal,
    QuickCast,
    LightSword,
    RockFall,
    Fireball,
    SpawnJar,
    SummonHugeSlime,
    SummonChiken,
    Servant,
}

/// 呪文の基礎情報
pub struct SpellProps {
    pub rank: u32,
    pub name: Dict<&'static str>,
    pub description: Dict<&'static str>,
    pub cast_delay: u32,
    pub icon: &'static str,
    pub price: u32,
    pub cast: SpellCast,
}

impl SpellType {
    pub fn to_props(&self) -> SpellProps {
        match self {
            SpellType::MagicBolt =>  SpellProps {
                rank: 0,
                name: Dict {
                    ja: "マジックボルト",
                    en: "Magic Bolt",
                    zh_cn: "魔法箭",
                    es: "Rayo Mágico",
                    fr: "Éclair Magique",
                    pt: "Raio Mágico",
                    de: "Magischer Blitz",
                    ko: "마법 화살",
                    ru: "Магическая стрела"
                },
                description: Dict {
                    ja: "魔力の塊を発射する、最も基本的な攻撃魔法です。",
                    en: "A basic attack spell that fires a bolt of magic.",
                    zh_cn: "发射魔法箭的基本攻击魔法。",
                    es: "Un hechizo de ataque básico que dispara un rayo de magia.",
                    fr: "Un sort d'attaque de base qui tire un éclair magique.",
                    pt: "Um feitiço de ataque básico que dispara um raio de magia.",
                    de: "Ein grundlegender Angriffsspruch, der einen magischen Blitz abfeuert.",
                    ko: "마법 화살을 발사하는 기본 공격 마법입니다.",
                    ru: "Основное атакующее заклинание, которое выпускает магическую стрелу."
                },
                cast_delay: 20,
                icon: "bullet_magic_bolt",
                price: 10,
                cast: SpellCast::Bullet(SpellCastBullet {
                    slices: vec!["bullet_magic_bolt".to_string()],
                    collier_radius: 5.0,
                    speed: 100.0,
                    lifetime: 240,
                    damage: 8,
                    impulse: 0.0,
                    scattering: 0.4,
                    light_intensity: 1.0,
                    light_radius: 50.0,
                    light_color_hlsa: [245.0, 1.0, 0.6, 1.0],
                    remaining_time: 0
                }),
            },
            SpellType::LightBall =>  SpellProps {
                rank: 0,
                name: Dict {
                    ja: "光球",
                    en: "Light Ball",
                    zh_cn: "光球",
                    es: "Bola de Luz",
                    fr: "Boule de Lumière",
                    pt: "Bola de Luz",
                    de: "Lichtkugel",
                    ko: "빛의 구슬",
                    ru: "Световой шар"
                },
                description: Dict {
                    ja: "周囲をしばらく明るく照らす光の玉を出現させます。威力はありません。",
                    en: "Creates a ball of light that illuminates the area for a while. It has no attack power.",
                    zh_cn: "产生一个照亮区域一段时间的光球。它没有攻击力。",
                    es: "Crea una bola de luz que ilumina el área por un tiempo. No tiene poder de ataque.",
                    fr: "Crée une boule de lumière qui illumine la zone pendant un certain temps. Elle n'a pas de pouvoir d'attaque.",
                    pt: "Cria uma bola de luz que ilumina a área por um tempo. Não tem poder de ataque.",
                    de: "Erzeugt eine Lichtkugel, die den Bereich für eine Weile erhellt. Sie hat keine Angriffskraft.",
                    ko: "주변을 잠시 동안 밝히는 빛의 구슬을 만듭니다. 공격력이 없습니다.",
                    ru: "Создает световой шар, который освещает область на некоторое время. Он не имеет атакующей силы."
                },
                cast_delay: 120,
                icon: "light_ball_icon",
                price: 10,
                cast: SpellCast::Bullet(SpellCastBullet {
                    slices: vec!["light_ball".to_string()],
                    collier_radius: 5.0,
                    speed: 4.0,
                    lifetime: 60 * 60,
                    damage: 0,
                    impulse: 0.0,
                    scattering: 0.4,
                    light_intensity: 4.0,
                    light_radius: TILE_SIZE * 10.0,
                    light_color_hlsa: [0.0, 0.0, 1.0, 1.0],
                    remaining_time: 0
                }),
            },
            SpellType::SpawnJar => SpellProps {
                rank: 0,
                name: Dict {
                    ja: "壺生成",
                    en: "Spawn Jar",
                    zh_cn: "生成罐子",
                    es: "Generar Jarra",
                    fr: "Générer un Pot",
                    pt: "Gerar Jarra",
                    de: "Krüge Erzeugen",
                    ko: "항아리 생성",
                    ru: "Создать Банку"
                },
                description: Dict {
                    ja: "壺を生成します。",
                    en: "Spawns a jar.",
                    zh_cn: "生成一个罐子。",
                    es: "Genera una jarra.",
                    fr: "Génère un pot.",
                    pt: "Gera uma jarra.",
                    de: "Erzeugt Krüge.",
                    ko: "항아리를 생성합니다.",
                    ru: "Создает банку."
                },
                cast_delay: 120,
                icon: "spawn_jar_icon",
                price: 20,
                cast: SpellCast::SpawnEntity(SpellCastEntityType::CrateOrBarrel),
            },
            SpellType::PurpleBolt =>  SpellProps {
                rank: 1,
                name: Dict {
                    ja: "悪意の視線",
                    en: "Evil Eye",
                    zh_cn: "邪恶之眼",
                    es: "Ojo Maligno",
                    fr: "Œil Maléfique",
                    pt: "Olho Maligno",
                    de: "Böser Blick",
                    ko: "악의의 눈",
                    ru: "Злой глаз"
                },
                description: Dict {
                    ja:
                        "邪悪な魔力を帯びた視線です。浴びせられると少し悪寒が走ります。",
                    en: "Fires a slow-moving purple energy bolt. It is weak but consumes little mana.",
                    zh_cn: "发射一个移动缓慢的紫色能量弹。它很弱，但消耗的魔法很少。",
                    es: "Dispara un rayo de energía púrpura de movimiento lento. Es débil pero consume poca mana.",
                    fr: "Tire un éclair d'énergie violette se déplaçant lentement. Il est faible mais consomme peu de mana.",
                    pt: "Dispara um raio de energia roxa de movimento lento. É fraco, mas consome pouca mana.",
                    de: "Feuert einen langsam bewegenden violetten Energieblitz ab. Er ist schwach, verbraucht aber wenig Mana.",
                    ko: "느리게 움직이는 보라색 에너지 볼트를 발사합니다. 약하지만 마나 소모가 적습니다.",
                    ru: "Выпускает медленно движущийся фиолетовый энергетический снаряд. Он слаб, но потребляет мало маны."
                },
                cast_delay: 120,
                icon: "bullet_purple",
                price: 5,
                cast: SpellCast::Bullet(SpellCastBullet {
                    slices: vec!["bullet_purple".to_string()],
                    collier_radius: 5.0,
                    speed: 50.0,
                    lifetime: 500,
                    damage: 3,
                    impulse: 0.0,
                    scattering: 0.6,
                    light_intensity: 0.0,
                    light_radius: 0.0,
                    light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
                    remaining_time: 0
                }),
            },
            SpellType::SlimeCharge => SpellProps {
                rank: 1,
                name: Dict {
                    ja: "スライムの塊",
                    en: "Slime Limp",
                    zh_cn: "史莱姆块",
                    es: "Masa de Slime",
                    fr: "Masse de Slime",
                    pt: "Massa de Slime",
                    de: "Schleimklumpen",
                    ko: "슬라임 덩어리",
                    ru: "Слизистый комок"
                },
                description: Dict {
                    ja: "ぷにぷにとした塊で殴りつけます。痛くはありませんが、相手を大きく吹き飛ばします。",
                    en: "Slap with a soft, squishy lump. It doesn't hurt much, but it knocks the opponent backward.",
                    zh_cn: "用柔软、软乎乎的块状物拍打。虽然不会很疼，但会将对手击退。",
                    es: "Golpea con una masa blanda y esponjosa. No duele mucho, pero empuja al oponente hacia atrás.",
                    fr: "Frappe avec une masse molle et spongieuse. Cela ne fait pas très mal, mais repousse l'adversaire.",
                    pt: "Bata com uma massa macia e fofa. Não dói muito, mas empurra o oponente para trás.",
                    de: "Schlägt mit einem weichen, schwammigen Klumpen. Es tut nicht sehr weh, stößt aber den Gegner zurück.",
                    ko: "부드럽고 말랑말랑한 덩어리로 때립니다. 많이 아프지는 않지만 상대를 뒤로 밀어냅니다.",
                    ru: "Удар мягким, скользким комком. Это не сильно больно, но отталкивает противника назад."
                },
                cast_delay: 30,
                icon: "bullet_slime_charge",
                price: 15,
                cast: SpellCast::Bullet(SpellCastBullet {
                    slices: vec!["bullet_slime_charge".to_string()],
                    collier_radius: 5.0,
                    speed: 2.0,
                    lifetime: 5,
                    damage: 1,
                    impulse: 40000.0,
                    scattering: 0.0,
                    light_intensity: 0.0,
                    light_radius: 0.0,
                    light_color_hlsa: [0.0, 0.0, 0.0, 1.0],
                    remaining_time: 0
                }),
            },
            SpellType::SummonChiken => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "ニワトリ召喚",
                    en: "Summon Chicken",
                    zh_cn: "召唤鸡",
                    es: "Invocar Pollo",
                    fr: "Invoquer Poulet",
                    pt: "Invocar Galinha",
                    de: "Huhn Beschwören",
                    ko: "닭 소환",
                    ru: "Призвать Курицу"
                },
                description: Dict {
                    ja: "おとりのニワトリをどこかから呼び寄せます。",
                    en: "Summons a decoy chicken from nowhere.",
                    zh_cn: "从无处召唤一个诱饵鸡。",
                    es: "Invoca un pollo señuelo de la nada.",
                    fr: "Invoque un poulet leurre de nulle part.",
                    pt: "Invoca uma galinha de isca do nada.",
                    de: "Beschwört ein Lockhuhn aus dem Nichts.",
                    ko: "어디서나 속임수 닭을 소환합니다.",
                    ru: "Призывает прим кающую курицу из ниоткуда."
                },
                cast_delay: 120,
                icon: "spawn_chiken_icon",
                price: 100,
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Chiken, servant: false },
            },
            SpellType::WaterBall =>  SpellProps {
                rank: 2,
                name: Dict {
                    ja: "水の塊",
                    en: "Water Ball",
                    zh_cn: "水球",
                    es: "Bola de Agua",
                    fr: "Boule d'Eau",
                    pt: "Bola de Água",
                    de: "Wasserball",
                    ko: "물구슬",
                    ru: "Водяной шар"
                },
                description: Dict {
                    ja: "水の塊を発射します。威力は低いですが、相手を押し返すことができます。",
                    en: "Fires a ball of water. It is weak but can push the opponent back.",
                    zh_cn: "发射一个水球。它很弱，但可以将对手击退。",
                    es: "Dispara una bola de agua. Es débil pero puede empujar al oponente hacia atrás.",
                    fr: "Tire une boule d'eau. Elle est faible mais peut repousser l'adversaire.",
                    pt: "Dispara uma bola de água. É fraca, mas pode empurrar o oponente para trás.",
                    de: "Feuert einen Wasserball ab. Er ist schwach, kann aber den Gegner zurückstoßen.",
                    ko: "물구슬을 발사합니다. 약하지만 상대를 밀어낼 수 있습니다.",
                    ru: "Выпускает водяной шар. Он слабый, но может оттолкнуть противника."
                },
                cast_delay: 8,
                icon: "spell_water_ball",
                price: 15,
                cast: SpellCast::Bullet(SpellCastBullet {
                    slices: vec!["water_ball".to_string()],
                    collier_radius: 5.0,
                    speed: 80.0,
                    lifetime: 240,
                    damage: 1,
                    impulse: 80000.0,
                    scattering: 0.4,
                    light_intensity: 1.0,
                    light_radius: 50.0,
                    light_color_hlsa: [245.0, 1.0, 0.6, 1.0],
                    remaining_time: 0
                }),
            },
            SpellType::BulletSpeedDoown =>  SpellProps {
                rank: 2,
                name: Dict {
                    ja: "減速",
                    en: "Speed Down",
                    zh_cn: "减速",
                    es: "Reducción de Velocidad",
                    fr: "Ralentissement",
                    pt: "Redução de Velocidade",
                    de: "Geschwindigkeit Verringern",
                    ko: "속도 감소",
                    ru: "Замедление"
                },
                description: Dict {
                    ja: "次に発射する魔法の弾速を50%低下させます。",
                    en: "Reduces the speed of the next magic bullet by 50%.",
                    zh_cn: "将下一个魔法弹的速度降低50%。",
                    es: "Reduce la velocidad de la próxima bala mágica en un 50%.",
                    fr: "Réduit la vitesse de la prochaine balle magique de 50%.",
                    pt: "Reduz a velocidade da próxima bala mágica em 50%.",
                    de: "Reduziert die Geschwindigkeit der nächsten magischen Kugel um 50%.",
                    ko: "다음 마법 탄환의 속도를 50% 감소시킵니다.",
                    ru: "Уменьшает скорость следующей магической пули на 50%."
                },
                cast_delay: 0,
                icon: "bullet_speed_down",
                price: 20,
                cast: SpellCast::BulletSpeedUpDown { delta: -0.5 },
            },
            SpellType::DualCast => SpellProps {
                rank: 2,
                name: Dict {
                    ja: "並列詠唱",
                    en: "Dual Cast",
                    zh_cn: "双重施法",
                    es: "Doble Hechizo",
                    fr: "Double Sort",
                    pt: "Feitiço Duplo",
                    de: "Doppelzauber",
                    ko: "이중 시전",
                    ru: "Двойное заклинание"
                },
                description: Dict {
                    ja: "ふたつの投射物呪文を同時に詠唱します。詠唱遅延は大きいほうに揃えられます。",
                    en: "Casts two projectile spells at the same time.",
                    zh_cn: "同时施放两个投射法术。施法延迟将与较大的值对齐。",
                    es: "Lanza dos hechizos de proyectiles al mismo tiempo.",
                    fr: "Lance deux sorts de projectiles en même temps.",
                    pt: "Lança dois feitiços de projéteis ao mesmo tempo.",
                    de: "Wirkt zwei Projektilzauber gleichzeitig.",
                    ko: "두 개의 투사체 주문을 동시에 시전합니다.",
                    ru: "Одновременно накладывает два заклинания снаряда."
                },
                cast_delay: 0,
                icon: "spell_dual_cast",
                price: 20,
                cast: SpellCast::MultipleCast { amount: 2 },
            },
            SpellType::SummonEnemySlime => SpellProps {
                rank: 2,
                name: Dict {
                    ja: "敵スライム召喚",
                    en: "Summon Enemy Slime",
                    zh_cn: "召唤敌人史莱姆",
                    es: "Invocar Slime Enemigo",
                    fr: "Invoquer Slime Ennemi",
                    pt: "Invocar Slime Inimigo",
                    de: "Feindlichen Schleim Beschwören",
                    ko: "적 슬라임 소환",
                    ru: "Призыв вражеского слизня"
                },
                description: Dict {
                    ja: "敵のスライムを召喚します。",
                    en: "Summons a enemy slime",
                    zh_cn: "召唤一个敌人史莱姆",
                    es: "Invoca un slime enemigo.",
                    fr: "Invoque un slime ennemi.",
                    pt: "Invoca um slime inimigo.",
                    de: "Beschwört einen feindlichen Schleim.",
                    ko: "적 슬라임을 소환합니다.",
                    ru: "Призывает вражеского слизня."
                },
                cast_delay: 30,
                icon: "slime",
                price: 200,
                cast: SpellCast::Summon { friend: false, servant_type: ServantType::Slime, servant: false },
            },
            SpellType::SummonEnemyEyeball => SpellProps {
                rank: 2,
                name: Dict {
                    ja: "敵アイボール召喚",
                    en: "Summon Enemy Eyeball",
                    zh_cn: "召唤敌人眼球",
                    es: "Invocar Ojo Enemigo",
                    fr: "Invoquer Oeil Ennemi",
                    pt: "Invocar Olho Inimigo",
                    de: "Feindlichen Augapfel Beschwören",
                    ko: "적 아이볼 소환",
                    ru: "Призыв вражеского глазного яблока"
                },
                description: Dict {
                    ja: "敵のアイボールを召喚します。",
                    en: "Summons a enemy Eyeball",
                    zh_cn: "召唤一个敌人眼球",
                    es: "Invoca un ojo enemigo.",
                    fr: "Invoque un oeil ennemi.",
                    pt: "Invoca um olho inimigo.",
                    de: "Beschwört einen feindlichen Augapfel.",
                    ko: "적 아이볼을 소환합니다.",
                    ru: "Призывает вражеское глазное яблоко."
                },
                cast_delay: 30,
                icon: "eyeball",
                price: 200,
                cast: SpellCast::Summon { friend: false, servant_type: ServantType::Eyeball, servant: false },
            },
            SpellType::Servant => SpellProps {
                rank: 2,
                name: Dict {
                    ja: "使い魔召喚",
                    en: "Summon Servant",
                    zh_cn: "召唤仆从",
                    es: "Invocar Sirviente",
                    fr: "Invoquer Serviteur",
                    pt: "Invocar Servo",
                    de: "Diener Beschwören",
                    ko: "하수인 소환",
                    ru: "Призвать Слугу"
                },
                description: Dict {
                    ja: "使い魔のニワトリを召喚し、操作することができます。",
                    en: "Summons a servant chicken that you can control.",
                    zh_cn: "召唤一个您可以控制的仆从鸡。",
                    es: "Invoca un pollo sirviente que puedes controlar.",
                    fr: "Invoque un poulet serviteur que vous pouvez contrôler.",
                    pt: "Invoca um servo de galinha que você pode controlar.",
                    de: "Beschwört ein Dienerhuhn, das du kontrollieren kannst.",
                    ko: "조종할 수 있는 하수인 닭을 소환합니다.",
                    ru: "Призывает служебную курицу, которую вы можете контролировать."
                },
                cast_delay: 30,
                icon: "summon_servant",
                price: 200,        
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Chiken, servant: true },
            },
            SpellType::HeavyShot => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "ヘヴィーショット",
                    en: "Heavy Shot",
                    zh_cn: "重型射击",
                    es: "Disparo Pesado",
                    fr: "Tir Lourd",
                    pt: "Tiro Pesado",
                    de: "Schwerer Schuss",
                    ko: "헤비 샷",
                    ru: "Тяжелый выстрел"
                },
                description: Dict {
                    ja: "次に発射する魔法弾の威力が上昇しますが、飛翔速度が低下します。",
                    en: "The next magic bullet you fire will be more powerful and slower.",
                    zh_cn: "下一个魔法弹的威力将增加，但飞行速度将减慢。",
                    es: "La próxima bala mágica que dispares será más poderosa y más lenta.",
                    fr: "La prochaine balle magique que vous tirez sera plus puissante et plus lente.",
                    pt: "A próxima bala mágica que você disparar será mais poderosa e mais lenta.",
                    de: "Die nächste magische Kugel, die du abfeuerst, wird mächtiger und langsamer sein.",
                    ko: "다음에 발사하는 마법 탄환은 더 강력하고 느려질 것입니다.",
                    ru: "Следующая магическая пуля, которую вы выпустите, будет мощнее и медленнее."
                },
                cast_delay: 5,
                icon: "spell_heavy_shot",
                price: 30,
                cast: SpellCast::HeavyShot,
            },
            SpellType::Bomb => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "爆弾",
                    en: "Bomb",
                    zh_cn: "炸弹",
                    es: "Bomba",
                    fr: "Bombe",
                    pt: "Bomba",
                    de: "Bombe",
                    ko: "폭탄",
                    ru: "Бомба"
                },
                description: Dict {
                    ja: "黒色火薬が詰まった爆弾です。時間が経つと爆発します。",
                    en: "A bomb filled with black powder. It explodes after a while.",
                    zh_cn: "装满黑火药的炸弹。过一会儿就会爆炸。",
                    es: "Una bomba llena de pólvora negra. Explota después de un tiempo.",
                    fr: "Une bombe remplie de poudre noire. Elle explose après un certain temps.",
                    pt: "Uma bomba cheia de pólvora negra. Ela explode depois de um tempo.",
                    de: "Eine mit Schwarzpulver gefüllte Bombe. Sie explodiert nach einer Weile.",
                    ko: "흑색 화약이 가득한 폭탄입니다. 시간이 지나면 폭발합니다.",
                    ru: "Бомба, наполненная черным порохом. Она взрывается через некоторое время."
                },
                cast_delay: 120,
                icon: "bomb_icon",
                price: 300,
                cast: SpellCast::Bomb,
            },
            SpellType::TripleCast =>  SpellProps {
                rank: 3,
                name: Dict {
                    ja: "三並列詠唱",
                    en: "Triple Cast",
                    zh_cn: "三重施法",
                    es: "Triple Hechizo",
                    fr: "Triple Sort",
                    pt: "Tríplice Feitiço",
                    de: "Dreifachzauber",
                    ko: "삼중 시전",
                    ru: "Тройное заклинание"
                },
                description: Dict {
                    ja: "みっつの投射物呪文を同時に詠唱します。",
                    en: "Casts three projectile spells at the same time.",
                    zh_cn: "同时施放三个投射法术。",
                    es: "Lanza tres hechizos de proyectiles al mismo tiempo。",
                    fr: "Lance trois sorts de projectiles en même temps.",
                    pt: "Lança três feitiços de projéteis ao mesmo tempo.",
                    de: "Wirkt drei Projektilzauber gleichzeitig.",
                    ko: "세 개의 투사체 주문을 동시에 시전합니다.",
                    ru: "Одновременно накладывает три заклинания снаряда."
                },
                cast_delay: 0,
                icon: "spell_triple_cast",
                price: 30,
                cast: SpellCast::MultipleCast { amount: 3 },
            },
            SpellType::Homing => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "自律型追尾",
                    en: "Self-directed Homing",
                    zh_cn: "自主追踪",
                    es: "Autodirigido",
                    fr: "Autoguidé",
                    pt: "Autodirigido",
                    de: "Selbstgesteuerte Verfolgung",
                    ko: "자율 유도",
                    ru: "Самонаводящийся"
                },
                description: Dict {
                    ja: "次に発射する魔法弾が近くの敵に向かって追尾します。",
                    en: "The next magic bullet you fire will home in on the enemy.",
                    zh_cn: "下一个魔法弹将追踪敌人。",
                    es: "La próxima bala mágica que dispares se dirigirá hacia el enemigo。",
                    fr: "La prochaine balle magique que vous tirez se dirigera vers l'ennemi.",
                    pt: "A próxima bala mágica que você disparar irá se direcionar para o inimigo.",
                    de: "Die nächste magische Kugel, die du abfeuerst, wird den Feind verfolgen.",
                    ko: "다음에 발사하는 마법 탄환이 적을 추적합니다.",
                    ru: "Следующая магическая пуля, которую вы выпустите, будет наводиться на врага."
                },
                cast_delay: 5,
                icon: "spell_homing",
                price: 40,
                cast: SpellCast::Homing,
            },
            SpellType::SummonFriendSlime => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "味方スライム召喚",
                    en: "Summon Friend Slime",
                    zh_cn: "召唤友军史莱姆",
                    es: "Invocar Slime Amigo",
                    fr: "Invoquer Slime Ami",
                    pt: "Invocar Slime Amigo",
                    de: "Freundlichen Schleim Beschwören",
                    ko: "아군 슬라임 소환",
                    ru: "Призыв дружественного слизня"
                },
                description: Dict {
                    ja: "味方のスライムを召喚します。",
                    en: "Summons a friend slime",
                    zh_cn: "召唤一个友军史莱姆",
                    es: "Invoca un slime amigo。",
                    fr: "Invoque un slime ami.",
                    pt: "Invoca um slime amigo.",
                    de: "Beschwört einen freundlichen Schleim.",
                    ko: "아군 슬라임을 소환합니다.",
                    ru: "Призывает дружественного слизня."
                },
                cast_delay: 30,
                icon: "slime",
                price: 200,
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Slime, servant: false },
            },
            SpellType::PrecisionUp => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "精度向上",
                    en: "Precision Up",
                    zh_cn: "精度提高",
                    es: "Precisión Mejorada",
                    fr: "Précision Améliorée",
                    pt: "Precisão Aumentada",
                    de: "Präzision Erhöhen",
                    ko: "정확도 향상",
                    ru: "Повышение точности"
                },
                description: Dict {
                    ja: "弾丸の精度を向上させます。",
                    en: "Increases the accuracy of bullets.",
                    zh_cn: "提高子弹的精度。",
                    es: "Aumenta la precisión de las balas。",
                    fr: "Augmente la précision des balles.",
                    pt: "Aumenta a precisão das balas.",
                    de: "Erhöht die Genauigkeit der Kugeln.",
                    ko: "탄환의 정확도를 높입니다.",
                    ru: "Повышает точность пуль."
                },
                cast_delay: 1,
                icon: "precision_icon",
                price: 500,
                cast: SpellCast::PrecisionUp,
            },
            SpellType::SpawnBookshelf => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "本棚生成",
                    en: "Spawn Bookshelf",
                    zh_cn: "生成书架",
                    es: "Generar Estantería",
                    fr: "Générer une Étagère",
                    pt: "Gerar Estante",
                    de: "Bücherregal Erzeugen",
                    ko: "책장 생성",
                    ru: "Создать Книжную Полку"
                },
                description: Dict {
                    ja: "本棚を生成します。",
                    en: "Spawns a bookshelf.",
                    zh_cn: "生成一个书架。",
                    es: "Genera una estantería.",
                    fr: "Génère une étagère.",
                    pt: "Gera uma estante.",
                    de: "Erzeugt ein Bücherregal.",
                    ko: "책장을 생성합니다.",
                    ru: "Создает книжную полку."
                },
                cast_delay: 120,
                icon: "spawn_bookshelf_icon",
                price: 500,
                cast: SpellCast::SpawnEntity(SpellCastEntityType::BookShelf),
            },
            SpellType::Fireball => SpellProps {
                rank: 3,
                name: Dict {
                    ja: "火球",
                    en: "Fireball",
                    zh_cn: "火球",
                    es: "Bola de Fuego",
                    fr: "Boule de Feu",
                    pt: "Bola de Fogo",
                    de: "Feuerball",
                    ko: "화염구",
                    ru: "Огненный Шар"
                },
                description: Dict {
                    ja: "炎の玉を発射します。落下した場所は燃え上がります。",
                    en: "Fires a ball of flame. The place where it falls will burn.",
                    zh_cn: "发射一团火焰。它落下的地方会燃烧。",
                    es: "Dispara una bola de llamas. El lugar donde cae arderá.",
                    fr: "Tire une boule de flammes. L'endroit où elle tombe brûlera.",
                    pt: "Dispara uma bola de chamas. O lugar onde cai vai queimar.",
                    de: "Feuert eine Feuerkugel ab. Der Ort, an dem sie fällt, wird brennen.",
                    ko: "불꽃 구슬을 발사합니다. 떨어지는 곳은 불타오릅니다.",
                    ru: "Выпускает огненный шар. Место, где он упадет, загорится."
                },
                cast_delay: 30,
                icon: "fireball_icon",
                price: 300,
                cast: SpellCast::Fireball,
            },
            SpellType::SummonFriendEyeball => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "味方アイボール召喚",
                    en: "Summon Friend Eyeball",
                    zh_cn: "召唤友军眼球",
                    es: "Invocar Ojo Amigo",
                    fr: "Invoquer Oeil Ami",
                    pt: "Invocar Olho Amigo",
                    de: "Freundlichen Augapfel Beschwören",
                    ko: "아군 아이볼 소환",
                    ru: "Призыв дружественного глазного яблока"
                },
                description: Dict {
                    ja: "味方のアイボールを召喚します。",
                    en: "Summons a friend Eyeball",
                    zh_cn: "召唤一个友军眼球",
                    es: "Invoca un ojo amigo。",
                    fr: "Invoque un oeil ami.",
                    pt: "Invoca um olho amigo.",
                    de: "Beschwört einen freundlichen Augapfel.",
                    ko: "아군 아이볼을 소환합니다.",
                    ru: "Призывает дружественное глазное яблоко."
                },
                cast_delay: 30,
                icon: "eyeball",
                price: 200,
                cast: SpellCast::Summon { friend: true, servant_type: ServantType::Eyeball, servant: false },
            },
            SpellType::Dash => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "ダッシュ",
                    en: "Dash",
                    zh_cn: "冲刺",
                    es: "Correr",
                    fr: "Course",
                    pt: "Corrida",
                    de: "Sprinten",
                    ko: "대시",
                    ru: "Рывок"
                },
                description: Dict {
                    ja: "短距離を素早く走ります。",
                    en: "Dashes a short distance.",
                    zh_cn: "短距离快速奔跑。",
                    es: "Corre una corta distancia。",
                    fr: "Parcourt une courte distance rapidement.",
                    pt: "Corre uma curta distância rapidamente.",
                    de: "Sprintet eine kurze Distanz.",
                    ko: "짧은 거리를 빠르게 달립니다.",
                    ru: "Совершает рывок на короткое расстояние."
                },
                cast_delay: 50,
                icon: "dash",
                price: 500,
                cast: SpellCast::Dash,
            },
            SpellType::Impact => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "衝撃",
                    en: "Impact",
                    zh_cn: "冲击",
                    es: "Impacto",
                    fr: "Impact",
                    pt: "Impacto",
                    de: "Aufprall",
                    ko: "충격",
                    ru: "Удар"
                },
                description: Dict {
                    ja: "周囲に衝撃波を起こします。敵も味方もまとめて吹き飛ばします。",
                    en: "Creates a shockwave around you that knocks back enemies and allies.",
                    zh_cn: "在你周围产生冲击波，将敌人和盟友一起击退。",
                    es: "Crea una onda de choque a tu alrededor que empuja a enemigos y aliados。",
                    fr: "Crée une onde de choc autour de vous qui repousse les ennemis et les alliés.",
                    pt: "Cria uma onda de choque ao seu redor que empurra inimigos e aliados.",
                    de: "Erzeugt eine Schockwelle um dich herum, die Feinde und Verbündete zurückstößt.",
                    ko: "주변에 충격파를 일으켜 적과 아군을 모두 밀어냅니다.",
                    ru: "Создает ударную волну вокруг вас, отбрасывающую врагов и союзников."
                },
                cast_delay: 60,
                icon: "impact_icon",
                price: 500,
                cast: SpellCast::Impact,
            },
            SpellType::BulletSpeedUp => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "加速",
                    en: "Speed Up",
                    zh_cn: "加速",
                    es: "Aumentar Velocidad",
                    fr: "Accélération",
                    pt: "Aumentar Velocidade",
                    de: "Geschwindigkeit Erhöhen",
                    ko: "속도 증가",
                    ru: "Ускорение"
                },
                description: Dict {
                    ja: "次に発射する魔法の弾速を50%上昇させます。",
                    en: "Increases the speed of the next magic bullet by 50%.",
                    zh_cn: "将下一个魔法弹的速度提高50%。",
                    es: "Aumenta la velocidad de la próxima bala mágica en un 50%。",
                    fr: "Augmente la vitesse de la prochaine balle magique de 50%.",
                    pt: "Aumenta a velocidade da próxima bala mágica em 50%.",
                    de: "Erhöht die Geschwindigkeit der nächsten magischen Kugel um 50%.",
                    ko: "다음 마법 탄환의 속도를 50% 증가시킵니다.",
                    ru: "Увеличивает скорость следующей магической пули на 50%."
                },
                cast_delay: 0,
                icon: "bullet_speed_up",
                price: 20,
                cast: SpellCast::BulletSpeedUpDown { delta: 0.5 },
            },
            SpellType::RockFall => SpellProps {
                rank: 4,
                name: Dict {
                    ja: "岩石落下",
                    en: "Rockfall",
                    zh_cn: "岩石落下",
                    es: "Caída de Rocas",
                    fr: "Chute de Pierres",
                    pt: "Queda de Pedras",
                    de: "Steinschlag",
                    ko: "바위 낙하",
                    ru: "Обвал камней"
                },
                description: Dict {
                    ja: "上空から岩石を落とします。",
                    en: "Drops rocks from the sky.",
                    zh_cn: "从天空掉下岩石。",
                    es: "Deja caer rocas desde el cielo.",
                    fr: "Laisse tomber des rochers du ciel.",
                    pt: "Deixa cair pedras do céu.",
                    de: "Lässt Steine vom Himmel fallen.",
                    ko: "하늘에서 바위를 떨어뜨립니다.",
                    ru: "Бросает камни с неба."
                },
                cast_delay: 120,
                icon: "rockfall_icon",
                price: 500,
                cast: SpellCast::RockFall,
            },
            SpellType::Heal =>  SpellProps {
                rank: 5,
                name: Dict {
                    ja: "回復",
                    en: "Heal",
                    zh_cn: "治疗",
                    es: "Curar",
                    fr: "Soigner",
                    pt: "Cura",
                    de: "Heilen",
                    ko: "치유",
                    ru: "Исцеление"
                },
                description: Dict {
                    ja: "自分自身の体力を少しだけ回復します。",
                    en: "Heals a small amount of your own health.",
                    zh_cn: "治疗自己一点点的生命值。",
                    es: "Cura una pequeña cantidad de tu propia salud。",
                    fr: "Soigne une petite quantité de votre propre santé.",
                    pt: "Cura uma pequena quantidade da sua própria saúde.",
                    de: "Heilt eine kleine Menge deiner eigenen Gesundheit.",
                    ko: "자신의 체력을 조금 회복합니다.",
                    ru: "Исцеляет небольшое количество вашего здоровья."
                },
                cast_delay: 120,
                icon: "spell_heal",
                price: 40,
                cast: SpellCast::Heal,
            },
            SpellType::QuickCast => SpellProps {
                rank: 5,
                name: Dict {
                    ja: "高速詠唱",
                    en: "Quick Cast",
                    zh_cn: "快速施法",
                    es: "Lanzamiento Rápido",
                    fr: "Incantation Rapide",
                    pt: "Lançamento Rápido",
                    de: "Schnelles Zaubern",
                    ko: "빠른 시전",
                    ru: "Быстрое заклинание"
                },
                description: Dict {
                    ja: "次に詠唱する呪文の詠唱時間を減らします。",
                    en: "Reduces the casting time of the next spell.",
                    zh_cn: "减少下一个法术的施法时间。",
                    es: "Reduce el tiempo de lanzamiento del próximo hechizo。",
                    fr: "Réduit le temps d'incantation du prochain sort.",
                    pt: "Reduz o tempo de lançamento do próximo feitiço.",
                    de: "Reduziert die Zauberzeit des nächsten Zaubers.",
                    ko: "다음 주문의 시전 시간을 줄입니다.",
                    ru: "Уменьшает время наложения следующего заклинания."
                },
                cast_delay: 1,
                icon: "quick_cast",
                price: 500,
                cast: SpellCast::QuickCast,
            },
            SpellType::LightSword =>  SpellProps {
                rank: 6,
                name: Dict {
                    ja: "光の剣",
                    en: "Light Sword",
                    zh_cn: "光之剑",
                    es: "Espada de Luz",
                    fr: "Épée de Lumière",
                    pt: "Espada de Luz",
                    de: "Lichtschwert",
                    ko: "빛의 검",
                    ru: "Меч света"
                },
                description: Dict {
                    ja: "鋭い光の剣が敵を容赦なく貫く強力な呪文です。",
                    en: "A powerful spell that pierces enemies with a sharp sword of light.",
                    zh_cn: "用锋利的光剑刺穿敌人的强大法术。",
                    es: "Un poderoso hechizo que atraviesa a los enemigos con una espada de luz afilada.",
                    fr: "Un puissant sort qui transperce les ennemis avec une épée de lumière tranchante.",
                    pt: "Um feitiço poderoso que perfura inimigos com uma espada de luz afiada.",
                    de: "Ein mächtiger Zauber, der Feinde mit einem scharfen Lichtschwert durchbohrt.",
                    ko: "날카로운 빛의 검으로 적을 관통하는 강력한 주문입니다.",
                    ru: "Мощное заклинание, пронзающее врагов острым мечом света."
                },
                cast_delay: 20,
                icon: "light_sowrd_icon",
                price: 1000,
                cast: SpellCast::LightSword,
            },
            SpellType::SummonHugeSlime =>  SpellProps {
                rank: 6,
                name: Dict {
                    ja: "巨大スライム召喚",
                    en: "Summon Giant Slime",
                    zh_cn: "召唤巨型史莱姆",
                    es: "Invocar Slime Gigante",
                    fr: "Invoquer Slime Géant",
                    pt: "Invocar Slime Gigante",
                    de: "Riesigen Schleim Beschwören",
                    ko: "거대 슬라임 소환",
                    ru: "",
                },
                description: Dict {
                    ja: "巨大なスライムの王を召喚します。",
                    en: "Summons a giant slime king",
                    zh_cn: "召唤一个巨型史莱姆王",
                    es: "Invoca un rey de slime gigante.",
                    fr: "Invoque un roi de slime géant.",
                    pt: "Invoca um rei de slime gigante.",
                    de: "Beschwört einen riesigen Schleimkönig.",
                    ko: "거대한 슬라임 왕을 소환합니다.",
                    ru: "Призывает гигантского короля слизней."
                },
                cast_delay: 180,
                icon: "summon_huge_slime_icon",
                price: 10000,
                cast: SpellCast::SpawnEntity(SpellCastEntityType::HugeSlime)
            },
        }
    }
}

const DAMAGE: Dict<&'static str> = Dict {
    ja: " / ダメージ",
    en: " / Damage",
    zh_cn: " / 伤害",
    es: " / Daño",
    fr: " / Dégâts",
    pt: " / Dano",
    de: " / Schaden",
    ko: " / 피해",
    ru: " / Урон",
};

const KNOCKBACK: Dict<&'static str> = Dict {
    ja: " / ノックバック",
    en: " / Knockback",
    zh_cn: " / 击退",
    es: " / Retroceso",
    fr: " / Recul",
    pt: " / Recuo",
    de: " / Rückstoß",
    ko: " / 넉백",
    ru: " / Отбрасывание",
};

const SPEED: Dict<&'static str> = Dict {
    ja: " / 射出速度",
    en: " / Speed",
    zh_cn: " / 速度",
    es: " / Velocidad",
    fr: " / Vitesse",
    pt: " / Velocidade",
    de: " / Geschwindigkeit",
    ko: " / 속도",
    ru: " / Скорость",
};

const LIFETIME: Dict<&'static str> = Dict {
    ja: " / 持続時間",
    en: " / Lifetime",
    zh_cn: " / 持续时间",
    es: " / Duración",
    fr: " / Durée",
    pt: " / Duração",
    de: " / Lebensdauer",
    ko: " / 지속 시간",
    ru: " / Время жизни",
};

const SCATTERING: Dict<&'static str> = Dict {
    ja: " / 拡散",
    en: " / Scattering",
    zh_cn: " / 扩散",
    es: " / Dispersión",
    fr: " / Dispersion",
    pt: " / Dispersão",
    de: " / Streuung",
    ko: " / 산란",
    ru: " / Разброс",
};

const SIZE: Dict<&'static str> = Dict {
    ja: " / 大きさ",
    en: " / Size",
    zh_cn: " / 大小",
    es: " / Tamaño",
    fr: " / Taille",
    pt: " / Tamanho",
    de: " / Größe",
    ko: " / 크기",
    ru: " / Размер",
};

const HEAL_TEXT: Dict<&'static str> = Dict {
    ja: " / 回復",
    en: " / Heal",
    zh_cn: " / 治疗",
    es: " / Curar",
    fr: " / Soigner",
    pt: " / Cura",
    de: " / Heilen",
    ko: " / 치유",
    ru: " / Исцеление",
};

pub fn get_spell_appendix(cast: SpellCast) -> Dict<String> {
    match cast {
        SpellCast::Bullet(SpellCastBullet {
            slices: _,
            collier_radius,
            speed,
            lifetime,
            damage,
            impulse,
            scattering,
            light_intensity: _,
            light_radius: _,
            light_color_hlsa: _,
            remaining_time: _,
        }) => {
            let mut empty = Dict::empty();
            empty += DAMAGE.to_string();
            empty += Dict::literal(damage);
            empty += KNOCKBACK.to_string();
            empty += Dict::literal(impulse * 0.001);
            empty += SPEED.to_string();
            empty += Dict::literal(speed);
            empty += LIFETIME.to_string();
            empty += Dict::literal(lifetime);
            empty += SCATTERING.to_string();
            empty += Dict::literal(scattering);
            empty += SIZE.to_string();
            empty += Dict::literal(collier_radius);
            empty
        }
        SpellCast::Heal => {
            let mut empty = Dict::empty();
            empty += HEAL_TEXT.to_string();
            empty += Dict::literal("10");
            empty
        }
        SpellCast::HeavyShot => {
            // TODO
            // format!("威力: +5"),
            Dict::empty()
        }
        _ => Dict::empty(),
    }
}
