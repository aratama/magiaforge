use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::asset::GameAssets;
use crate::interpreter::Cmd;
use crate::language::Dict;
use crate::level::map::LevelTile;
use crate::level::tile::Tile;
use crate::page::in_game::GameLevel;
use crate::spell::Spell;
use crate::spell::SpellProps;
use crate::strategy::Strategy;
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
    pub bgms: HashMap<String, BGMProps>,
    pub ending_bgm: String,
    pub debug_items: Vec<Spell>,
    pub debug_wands: Vec<Vec<Option<Spell>>>,

    pub tutorial_move: Dict<String>,
    pub tutorial_inventory: Dict<String>,
    pub tutorial_slot: Dict<String>,
    pub tutorial_close_inventory: Dict<String>,
    pub tutorial_cast: Dict<String>,
    pub tutorial_magic_circle: Dict<String>,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct TileRegistry {
    pub levels: HashMap<String, LevelProps>,
    pub tiles: HashMap<(u8, u8, u8, u8), LevelTile>,
    pub tile_types: HashMap<String, TileTypeProps>,
}

#[derive(serde::Deserialize, Debug)]
pub struct TileTypeProps {
    pub tile_type: TileType,
    pub layers: Vec<TileTypeLayer>,
    /// それぞれのタイルに以下の照度をランダムに割り当てます
    #[serde(default)]
    pub light_hue: f32,

    #[serde(default)]
    pub light_saturation: f32,

    #[serde(default)]
    pub light_lightness: f32,

    #[serde(default)]
    pub light_intensity: f32,

    #[serde(default)]
    pub light_radius: f32,

    #[serde(default)]
    pub light_density: f32,

    #[serde(default)]
    pub grasses: bool,
}

#[derive(serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Surface,
    Floor,
}

#[derive(serde::Deserialize, Debug)]
pub struct TileTypeLayer {
    pub depth: f32,
    pub tiling: Tiling,
}

#[derive(serde::Deserialize, Debug)]
pub enum Tiling {
    Simple { patterns: Vec<Vec<String>> },
    Auto { prefixes: Vec<Vec<String>> },
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

    #[serde(default)]
    pub items: HashMap<(i32, i32), Spell>,

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

#[derive(serde::Deserialize, Debug)]
pub struct ActorPropsByType {
    pub move_force: f32,
    pub jump: f32,
    pub linear_damping: f32,
    pub bloods: Vec<String>,
    /// 凍結から復帰する速度です
    /// 通常は 1 ですが、ボスなどの特定のモンスターはより大きい値になることがあります
    pub defreeze: u32,
    /// Staggered からの回復速度
    pub poise: u32,
    /// 蜘蛛の巣から逃れる速度
    /// 毎ターンこの値が trapped から減算され、trappedが0になるとアクターが解放されます
    /// また、解放された瞬間に trap_moratorium が 180 に設定され、
    /// 3秒間は再びトラップにかからないようになります
    pub floundering: u32,
    pub fire_resistance: bool,
    /// 常時浮遊のモンスターであることを表します
    /// 通常は false ですが、アイボールなどの一部のモンスターは true になります
    pub auto_levitation: bool,
    /// 一部のアクターでコリジョンの半径として使われます
    /// 大型のモンスターは半径が大きいので、中心間の距離では攻撃が当たるかどうか判定できません
    pub collider: ActorCollider,
    pub cry: bool,
    pub life: u32,
    pub actor_group: ActorGroup,
    pub aseprite: String,
    pub animations: ActorAnimationMap,
    pub shadow: Option<String>,

    #[serde(default)]
    pub wands: Vec<Vec<Option<Spell>>>,

    #[serde(default)]
    pub strategies: HashMap<String, Strategy>,

    #[serde(default)]
    pub impact_radius: f32,

    #[serde(default)]
    pub point_light_radius: f32,

    #[serde(default)]
    pub point_light_color: (f32, f32, f32, f32),

    #[serde(default)]
    pub point_light_intensity: f32,

    #[serde(default)]
    pub point_light_falloff: f32,
}

#[derive(serde::Deserialize, Debug)]
pub enum ActorCollider {
    Ball(f32),
    Cuboid(f32, f32),
}

impl ActorCollider {
    pub fn size(&self) -> f32 {
        match self {
            ActorCollider::Ball(radius) => *radius,
            ActorCollider::Cuboid(width, height) => width.max(*height),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ActorAnimationMap {
    pub idle_r: String,
    pub idle_d: String,
    pub idle_u: String,
    pub run_r: String,
    pub run_d: String,
    pub run_u: String,
    // すべてのアクターはライフがゼロになると瞬時に消滅するので、
    // パラメータとしての get_down は不要？
    // pub get_down: String,
    pub get_up: String,
    pub frozen: String,
    pub drown: String,
    pub staggered: String,
    // オープニング用
    // pub fly: String,
    // pub turn: String,
    // pub hang: String,
    // pub drown: String,
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
    tile: Res<'w, Assets<TileRegistry>>,
    spell: Res<'w, Assets<SpellRegistry>>,
    actor: Res<'w, Assets<ActorRegistry>>,
    senario: Res<'w, Assets<SenarioRegistry>>,
}

impl<'w> Registry<'w> {
    pub fn game(&self) -> &GameRegistry {
        self.game.get(&self.assets.game_registry).unwrap()
    }

    pub fn tile(&self) -> &TileRegistry {
        self.tile.get(&self.assets.tile_registry).unwrap()
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

    pub fn get_actor_props(&self, actor_type: &ActorType) -> &ActorPropsByType {
        let constants = self.actor.get(&self.assets.actor_registry).unwrap();
        &constants
            .actors
            .get(&actor_type.0)
            .expect(format!("ActorType {:?} not found", actor_type.0).as_str())
    }

    pub fn get_tile(&self, tile: &Tile) -> &TileTypeProps {
        let constants = self.tile.get(&self.assets.tile_registry).unwrap();
        &constants
            .tile_types
            .get(&tile.0)
            .expect(format!("Tile {:?} not found", tile.0).as_str())
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
        let constants: &TileRegistry = self.tile.get(&self.assets.tile_registry).unwrap();
        constants.levels.get(level).expect(
            &format!(
                "Level {:?} not found in {:?}",
                level,
                constants.levels.keys()
            )
            .as_str(),
        )
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
