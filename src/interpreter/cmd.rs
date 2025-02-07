use std::collections::HashMap;

use crate::{
    language::Dict,
    level::{tile::Tile, world::GameLevel},
    spell::Spell,
};
use bevy::{math::Vec2, prelude::Entity};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Cmd {
    Set {
        name: String,
        value: Value,
    },

    /// フキダシを表示するキャラクターを指定します
    Focus(Entity),

    /// フキダシにテキストを表示します
    Speech(Dict<String>),

    /// BGMを変更します
    BGM(Option<String>),

    SE {
        path: String,
    },

    /// フキダシを非表示にします
    Close,

    /// 次のアクションまで指定したフレーム数待機します
    #[allow(dead_code)]
    Wait {
        count: u32,
    },

    /// 画面を揺らします
    Shake {
        value: f32,
    },

    /// 画面を揺らすエフェクトを開始します
    ShakeStart {
        value: Option<f32>,
    },

    Flash {
        position: Expr,
        intensity: f32,
        radius: f32,
        duration: u32,
        reverse: bool,
    },

    /// エンディングを再生します
    #[allow(dead_code)]
    Ending,

    Home,

    Arena,

    Warp {
        destination_level: GameLevel,
        destination_iid: String,
    },

    SetTile {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        tile: Tile,
    },

    SpawnRaven {
        name: String,
        position: Vec2,
    },

    Despawn {
        name: String,
    },

    Sprite {
        name: String,
        position: Expr,
        aseprite: String,
    },

    // todo ravenに合うような仮実装
    SetCameraTarget {
        name: Option<String>,
    },

    GetSpell {
        spell: Spell,
    },

    OnNewSpell {
        spell: Spell,
        commands_then: Vec<Cmd>,
        commands_else: Vec<Cmd>,
    },
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub enum Expr {
    Vec2(f32, f32),
    String(String),
    Var(String),
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub enum Value {
    Vec2 { x: f32, y: f32 },
    String { value: String },
}

impl Expr {
    pub fn to_vec2(&self, environment: &HashMap<String, Value>) -> Vec2 {
        match self {
            Expr::Vec2(x, y) => Vec2::new(*x, *y),
            Expr::Var(key) => match environment.get(key) {
                None => panic!("Value not found: {:?}, environment: {:?}", key, environment),
                Some(Value::Vec2 { x, y }) => Vec2::new(*x, *y),
                _ => panic!(
                    "Value is not Vec2: {:?}, environment: {:?}",
                    self, environment
                ),
            },
            _ => panic!("Value is not Vec2: {:?}", self),
        }
    }
}
