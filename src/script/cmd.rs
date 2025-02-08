use crate::language::Dict;
use crate::spell::Spell;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Cmd {
    /// フキダシにテキストを表示します
    Speech(Dict<String>),

    /// フキダシを非表示にします
    Close,

    GetSpell {
        spell: Spell,
    },

    Warp {
        destination_iid: String,
    },
    // /// BGMを変更します
    // BGM(Option<String>),

    // SE {
    //     path: String,
    // },

    // /// 次のアクションまで指定したフレーム数待機します
    // #[allow(dead_code)]
    // Wait {
    //     count: u32,
    // },

    // /// 画面を揺らします
    // Shake {
    //     value: f32,
    // },

    // /// 画面を揺らすエフェクトを開始します
    // ShakeStart {
    //     value: Option<f32>,
    // },

    // Flash {
    //     position: Expr,
    //     intensity: f32,
    //     radius: f32,
    //     duration: u32,
    //     reverse: bool,
    // },

    // /// エンディングを再生します
    // #[allow(dead_code)]
    // Ending,
    // Home,
    // Arena,
    // SetTile {
    //     x: i32,
    //     y: i32,
    //     w: u32,
    //     h: u32,
    //     tile: Tile,
    // },

    // SpawnRaven {
    //     name: String,
    //     position: Vec2,
    // },

    // Despawn {
    //     name: String,
    // },

    // Sprite {
    //     name: String,
    //     position: Expr,
    //     aseprite: String,
    // },

    // // todo ravenに合うような仮実装
    // SetCameraTarget {
    //     name: Option<String>,
    // },
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
