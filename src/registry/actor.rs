use crate::actor::ActorGroup;
use crate::spell::Spell;
use crate::strategy::Strategy;
use std::collections::HashMap;

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

#[derive(serde::Deserialize, Debug, Default)]
pub enum BodyType {
    #[default]
    Dynamic,
    Fixed,
}

#[derive(serde::Deserialize, Debug)]
pub struct ActorPropsByType {
    // 必須 //////////////////////////////////////////////////////////////////////////////////
    pub life: u32,
    pub actor_group: ActorGroup,
    pub aseprite: String,
    pub name_ja: String,

    // オプション ///////////////////////////////////////////////////////////////////////////
    #[serde(default)]
    pub body_type: BodyType,

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
