use crate::{spell::SpellType, wand::Wand};

// pub struct SpellCommand {
//     /// スペルの種類
//     pub action: SpellAction,

//     /// 次のコマンドまでの待機時間
//     pub cooltime: usize,

//     /// マナ消費量
//     pub mana_drain: i32,

//     /// スペルの説明
//     pub description: &'static str,
// }

pub struct SpellContext {
    /// スプライトのスライス名
    slice: &'static str,

    /// 威力
    damage: i32,

    /// 弾速
    /// スペル本来の弾速と speed_factor の積が実際の弾速になります
    speed_factor: f32,

    /// ホーミングの強さ
    homing: f32,

    /// 弾丸の生存時間
    lifetime: usize,

    /// 拡散(ラジアン)
    scattering: f32,

    heal: i32,
}

// pub fn interplet_wand(wand: Wand) -> Vec<SpellCommand> {
//     let mut commands = Vec::new();

//     for spell in wand.slots {
//         match spell {
//             None => continue,
//             Some(spell) => commands.push(interplet_spell(spell)),
//         }
//     }

//     return commands;
// }

// pub fn interplet_spell(spell: SpellType) -> SpellCommand {
//     match spell {
//         SpellType::MagicBolt => SpellCommand {
//             action: SpellAction::FireBullet {
//                 slice: "blue_bullet",
//                 damage: 1,
//                 speed: 1.0,
//                 homing: 0.0,
//                 lifetime: 60,
//                 scattering: 0.0,
//             },
//             cooltime: 10,
//             mana_drain: 1,
//             description: "青い弾を発射します",
//         },
//         SpellType::PurpleBolt => {}
//         SpellType::SlimeCharge => {}
//         SpellType::Heal => {}
//     }
// }
