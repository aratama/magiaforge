use crate::actor::ActorType;
use crate::actor::Blood;
use crate::asset::GameAssets;
use crate::interpreter::Cmd;
use crate::language::Dict;
use crate::level::map::LevelTile;
use crate::level::tile::Tile;
use crate::page::in_game::GameLevel;
use crate::spell::SpellProps;
use crate::spell::SpellType;
use bevy::asset::Assets;
use bevy::ecs::system::SystemParam;
use bevy::prelude::Res;
use std::collections::HashMap;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct SpellRegistry {
    pub spells: HashMap<String, SpellProps>,
    pub levels: Vec<LevelProps>,
    pub arena: LevelProps,
    pub tiles: HashMap<(u8, u8, u8, u8), LevelTile>,
    pub debug_items: Vec<SpellType>,
    pub debug_wands: Vec<Vec<Option<SpellType>>>,
}

#[derive(serde::Deserialize)]
pub struct LevelProps {
    pub name: Dict<String>,
    pub enemies: u8,
    pub enemy_types: Vec<ActorType>,
    pub items: u8,
    pub item_ranks: Vec<u8>,
    pub biome: Tile,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct ActorRegistry {
    pub dumping_on_ice: f32,
    pub dumping_on_air: f32,
    pub acceleration_on_ice: f32,
    pub acceleration_on_air: f32,
    pub acceleration_on_drowning: f32,
    pub acceleration_on_firing: f32,
    pub actors: HashMap<String, ActorPropsByType>,
}

#[derive(serde::Deserialize)]
pub struct ActorPropsByType {
    pub move_force: f32,
    pub jump: f32,
    pub linear_damping: f32,
    pub blood: Option<Blood>,
    /// 凍結から復帰する速度です
    /// 通常は 1 ですが、ボスなどの特定のモンスターはより大きい値になることがあります
    pub defreeze: u32,
    /// 常時浮遊のモンスターであることを表します
    /// 通常は false ですが、アイボールなどの一部のモンスターは true になります
    pub auto_levitation: bool,
    /// 一部のアクターでコリジョンの半径として使われます
    /// 大型のモンスターは半径が大きいので、中心間の距離では攻撃が当たるかどうか判定できません
    pub radius: f32,
    pub cry: bool,
}

#[derive(Debug)]
pub enum SenarioType {
    HelloRabbit,
    SingleplayRabbit,
    MultiplayerRabbit,
    ReserchRabbit,
    TrainingRabbit,
    SpellListRabbit,
    HugeSlime,
}

impl SenarioType {
    pub fn to_acts<'a>(&self, senarios: &'a SenarioRegistry) -> &'a Vec<Cmd> {
        senarios.senarios.get(&format!("{:?}", self)).unwrap()
    }
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct SenarioRegistry {
    pub senarios: HashMap<String, Vec<Cmd>>,
}

/// アセットの config.*.ron から読み取る設定を取得するシステムパラメータです
/// また、GameAssets も参照できます
/// この構造体がゲーム内から変更されることはありません
#[derive(SystemParam)]
pub struct Registry<'w> {
    pub assets: Res<'w, GameAssets>,
    spell: Res<'w, Assets<SpellRegistry>>,
    actor: Res<'w, Assets<ActorRegistry>>,
    senario: Res<'w, Assets<SenarioRegistry>>,
}

impl<'w> Registry<'w> {
    pub fn spell(&self) -> &SpellRegistry {
        self.spell.get(&self.assets.spell_registry).unwrap()
    }

    #[allow(dead_code)]
    pub fn actor(&self) -> &ActorRegistry {
        self.actor.get(&self.assets.actor_registry).unwrap()
    }

    pub fn senario(&self) -> &SenarioRegistry {
        self.senario.get(&self.assets.senario_registry).unwrap()
    }

    pub fn get_actor_props(&self, actor_type: ActorType) -> &ActorPropsByType {
        let constants = self.actor.get(&self.assets.actor_registry).unwrap();

        let name = format!("{:?}", actor_type);
        &constants
            .actors
            .get(&name)
            .expect(format!("ActorType {:?} not found", name).as_str())
    }

    pub fn get_spell_props(&self, spell_type: SpellType) -> &SpellProps {
        let constants = self.spell.get(&self.assets.spell_registry).unwrap();
        let key = format!("{:?}", spell_type);
        &constants.spells.get(&key).expect(key.as_str())
    }

    pub fn get_level_props(&self, level: GameLevel) -> &LevelProps {
        let constants = self.spell.get(&self.assets.spell_registry).unwrap();
        match level {
            GameLevel::Level(l) => constants.levels.get(l as usize).unwrap(),
            GameLevel::MultiPlayArena => &constants.arena,
        }
    }
}
