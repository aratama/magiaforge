// use crate::{
//     entity::bullet::SpawnBulletProps,
//     spell::SpellType,
//     spell_props::{spell_to_props, SpellCategory},
//     wand::Wand,
// };

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

// pub struct SpellContext {
//     /// スプライトのスライス名
//     slice: &'static str,

//     /// 威力
//     damage_or_heal_factor: i32,

//     /// 弾速
//     /// スペル本来の弾速と speed_factor の積が実際の弾速になります
//     speed_factor: f32,

//     /// ホーミングの強さ
//     homing: f32,

//     /// 弾丸の生存時間
//     lifetime_factor: usize,

//     /// 拡散(ラジアン)
//     scattering: f32,
// }

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

// fn interplet_spell(context: SpellContext, spell: SpellType) {
//     let props = spell_to_props(spell);

//     match props.category {
//         SpellCategory::Bullet {
//             collier_radius,
//             speed,
//             lifetime,
//             damage,
//             impulse,
//             slice,
//             scattering,
//             light_intensity,
//             light_radius,
//             light_color_hlsa,
//         } => {
//             // spawn_bullets(
//             //     &mut commands,
//             //     &assets,
//             //     &mut writer,
//             //     &mut se_writer,
//             //     &mut actor,
//             //     &actor_transform,
//             //     bullet_type:
//             //     online
//             // );
//             todo!();
//         }
//         SpellCategory::Buff => {
//             todo!();
//         }
//         SpellCategory::Heal => {
//             todo!();
//         }
//     }
// }
