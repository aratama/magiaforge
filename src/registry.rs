use crate::actor::ActorType;
use crate::actor::Blood;
use crate::asset::GameAssets;
use crate::interpreter::Cmd;
use crate::language::Dict;
use crate::level::map::LevelTile;
use crate::level::tile::Tile;
use crate::page::in_game::GameLevel;
use crate::spell::Spell;
use crate::spell::SpellProps;
use bevy::asset::AssetPath;
use bevy::asset::Assets;
use bevy::asset::Handle;
use bevy::audio::AudioSource;
use bevy::ecs::system::SystemParam;
use bevy::log::warn;
use bevy::prelude::Res;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct GameRegistry {
    pub levels: HashMap<String, LevelProps>,
    pub bgms: HashMap<String, BGMProps>,
    pub home_bgm: String,
    pub ending_bgm: String,
    pub tiles: HashMap<(u8, u8, u8, u8), LevelTile>,
    pub debug_items: Vec<Spell>,
    pub debug_wands: Vec<Vec<Option<Spell>>>,
}

#[derive(serde::Deserialize)]
pub struct BGMProps {
    pub author: String,
    pub title: String,
    pub appendix: String,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct SpellRegistry {
    pub spells: HashMap<String, SpellProps>,
}

#[derive(serde::Deserialize)]
pub struct LevelProps {
    pub next: Vec<GameLevel>,
    pub name: Dict<String>,
    pub enemies: u8,
    pub enemy_types: Vec<ActorType>,
    pub items: u8,
    pub item_ranks: Vec<u8>,
    pub biome: Tile,
    pub bgm: String,
    pub brightness: f32,
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
    game: Res<'w, Assets<GameRegistry>>,
    spell: Res<'w, Assets<SpellRegistry>>,
    actor: Res<'w, Assets<ActorRegistry>>,
    senario: Res<'w, Assets<SenarioRegistry>>,
}

impl<'w> Registry<'w> {
    pub fn game(&self) -> &GameRegistry {
        self.game.get(&self.assets.game_registry).unwrap()
    }

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

    pub fn get_senario<T: Into<String>>(&self, name: T) -> &Vec<Cmd> {
        let name = name.into();
        self.senario()
            .senarios
            .get(&name)
            .expect(&format!("Senario '{}' not found", &name))
    }

    pub fn get_actor_props(&self, actor_type: ActorType) -> &ActorPropsByType {
        let constants = self.actor.get(&self.assets.actor_registry).unwrap();

        let name = format!("{:?}", actor_type);
        &constants
            .actors
            .get(&name)
            .expect(format!("ActorType {:?} not found", name).as_str())
    }

    pub fn get_spell_props(&self, Spell(spell_type): &Spell) -> &SpellProps {
        let constants = self.spell.get(&self.assets.spell_registry).unwrap();
        &constants
            .spells
            .get(spell_type)
            .expect(&format!("spell '{}' not found", spell_type))
    }

    pub fn spells(&self) -> Vec<Spell> {
        self.spell().spells.keys().map(|k| Spell::new(k)).collect()
    }

    pub fn get_level(&self, GameLevel(level): &GameLevel) -> &LevelProps {
        let constants = self.game.get(&self.assets.game_registry).unwrap();
        constants
            .levels
            .get(level)
            .expect(&format!("Level '{:?}' not found", level).as_str())
    }

    pub fn get_bgm(&self, handle: &Handle<AudioSource>) -> &BGMProps {
        if let Some(path) = handle.path() {
            let name = path_to_string(path);
            self.game()
                .bgms
                .get(&name)
                .expect(&format!("BGM '{}' not found", name))
        } else {
            warn!("no path in audio handle");
            static DEFAULT_BGM_PROPS: LazyLock<BGMProps> = LazyLock::new(|| BGMProps {
                author: "".to_string(),
                title: "".to_string(),
                appendix: "".to_string(),
            });
            &DEFAULT_BGM_PROPS
        }
    }
}

pub fn path_to_string(path: &AssetPath) -> String {
    path.path().to_str().unwrap_or("").to_string()
}
