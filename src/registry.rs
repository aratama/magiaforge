use crate::actor::ActorGroup;
use crate::actor::ActorType;
use crate::asset::GameAssets;
use crate::hud::life_bar::LifeBarResource;
use crate::interpreter::Cmd;
use crate::language::Dict;
use crate::ldtk::generated::Level;
use crate::ldtk::loader::LevelCustomFields;
use crate::ldtk::loader::LDTK;
use crate::level::entities::Spawn;
use crate::level::tile::Tile;
use crate::level::world::GameLevel;
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
use serde_json::from_value;
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
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct TileRegistry {
    pub tile_types: HashMap<String, TileTypeProps>,
}

#[derive(serde::Deserialize, Debug)]
pub struct TileTypeProps {
    pub tile_type: TileType,

    #[serde(default)]
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

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SpawnEntityProps {
    pub entity: Spawn,

    #[serde(default)]
    pub spawn_offset_x: f32,
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

fn default_defreeze() -> u32 {
    1
}

fn default_poise() -> u32 {
    1
}

fn default_floundering() -> u32 {
    1
}

fn default_linear_damping() -> f32 {
    0.9
}

fn default_collider() -> ActorCollider {
    ActorCollider::Ball(5.0)
}

fn default_animation_map() -> ActorAnimationMap {
    ActorAnimationMap {
        idle_r: "idle".to_string(),
        idle_d: "idle".to_string(),
        idle_u: "idle".to_string(),
        run_r: "idle".to_string(),
        run_d: "idle".to_string(),
        run_u: "idle".to_string(),
        get_up: "idle".to_string(),
        frozen: "idle".to_string(),
        drown: "idle".to_string(),
        staggered: "idle".to_string(),
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ActorPropsByType {
    // 必須 //////////////////////////////////////////////////////////////////////////////////
    pub life: u32,
    pub actor_group: ActorGroup,
    pub aseprite: String,

    // オプション ///////////////////////////////////////////////////////////////////////////
    #[serde(default = "default_animation_map")]
    pub animations: ActorAnimationMap,

    #[serde(default)]
    pub cry: bool,

    #[serde(default = "default_collider")]
    pub collider: ActorCollider,

    #[serde(default)]
    pub move_force: f32,

    #[serde(default)]
    pub jump: f32,

    #[serde(default = "default_linear_damping")]
    pub linear_damping: f32,

    #[serde(default)]
    pub bloods: Vec<String>,

    /// 凍結から復帰する速度です
    /// 通常は 1 ですが、ボスなどの特定のモンスターはより大きい値になることがあります
    #[serde(default = "default_defreeze")]
    pub defreeze: u32,

    /// Staggered からの回復速度
    #[serde(default = "default_poise")]
    pub poise: u32,

    /// 蜘蛛の巣から逃れる速度
    /// 毎ターンこの値が trapped から減算され、trappedが0になるとアクターが解放されます
    /// また、解放された瞬間に trap_moratorium が 180 に設定され、
    /// 3秒間は再びトラップにかからないようになります
    #[serde(default = "default_floundering")]
    pub floundering: u32,

    #[serde(default)]
    pub fire_resistance: bool,

    /// 常時浮遊のモンスターであることを表します
    /// 通常は false ですが、アイボールなどの一部のモンスターは true になります
    #[serde(default)]
    pub auto_levitation: bool,

    #[serde(default)]
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

fn default_animation_map_item() -> String {
    "idle".to_string()
}

#[derive(serde::Deserialize, Debug)]
pub struct ActorAnimationMap {
    #[serde(default = "default_animation_map_item")]
    pub idle_r: String,
    #[serde(default = "default_animation_map_item")]
    pub idle_d: String,
    #[serde(default = "default_animation_map_item")]
    pub idle_u: String,
    #[serde(default = "default_animation_map_item")]
    pub run_r: String,
    #[serde(default = "default_animation_map_item")]
    pub run_d: String,
    #[serde(default = "default_animation_map_item")]
    pub run_u: String,
    // すべてのアクターはライフがゼロになると瞬時に消滅するので、
    // パラメータとしての get_down は不要？
    // pub get_down: String,
    pub get_up: String,
    #[serde(default = "default_animation_map_item")]
    pub frozen: String,
    #[serde(default = "default_animation_map_item")]
    pub drown: String,
    #[serde(default = "default_animation_map_item")]
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
    ldtk_assets: Res<'w, Assets<LDTK>>,

    game: Res<'w, Assets<GameRegistry>>,
    tile: Res<'w, Assets<TileRegistry>>,
    spell: Res<'w, Assets<SpellRegistry>>,
    actor: Res<'w, Assets<ActorRegistry>>,
    senario: Res<'w, Assets<SenarioRegistry>>,
    pub life_bar_resource: Res<'w, LifeBarResource>,
}

impl<'w> Registry<'w> {
    pub fn ldtk(&self) -> &LDTK {
        self.ldtk_assets.get(self.assets.ldtk_level.id()).unwrap()
    }

    pub fn game(&self) -> &GameRegistry {
        self.game.get(&self.assets.game_registry).unwrap()
    }

    #[allow(dead_code)]
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

    pub fn get_level(&self, level: &GameLevel) -> LevelCustomFields {
        let ldtk = self.ldtk();
        let Some(level) = ldtk.get_level(level) else {
            warn!("Level {:?} not found", level);
            return LevelCustomFields::default();
        };
        let map = serde_json::Map::from_iter(level.field_instances.iter().map(|v| {
            (
                v.identifier.clone(),
                v.value.as_ref().unwrap_or(&serde_json::Value::Null).clone(),
            )
        }));
        from_value(serde_json::Value::Object(map)).expect("Error occurred in get_level")
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

    pub fn get_level_by_iid(&self, iid: &str) -> Level {
        let ldtk = self.ldtk();
        ldtk.coordinate
            .levels
            .iter()
            .find(|l| l.iid == iid)
            .expect(&format!("Level {:?} not found", iid))
            .clone()
    }
}

pub fn path_to_string(path: &AssetPath) -> String {
    path.path().to_str().unwrap_or("").to_string()
}
