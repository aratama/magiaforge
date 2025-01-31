use crate::asset::GameAssets;
use crate::audio::play_se;
use crate::config::GameConfig;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use std::cmp::Ordering;

pub const DAMAGE: &'static str = "audio/dageki.ogg";
pub const NO_DAMAGE: &'static str = "audio/shibafu.ogg";
pub const CRY: &'static str = "audio/hiyoko.ogg";
pub const BREAK: &'static str = "audio/kuzureru.ogg";
pub const CLICK: &'static str = "audio/kettei.ogg";
pub const FIRE: &'static str = "audio/suburi.ogg";
pub const STEPS: &'static str = "audio/アスファルトの上を歩く2.ogg";
pub const TURN_ON: &'static str = "audio/メニューを開く2.ogg";
pub const WARP: &'static str = "audio/ワープ.ogg";
pub const PICK_UP: &'static str = "audio/キャンセル9.ogg";
pub const HEAL: &'static str = "audio/回復魔法1.ogg";
pub const SWITCH: &'static str = "audio/カーソル移動2.ogg";
pub const DROP: &'static str = "audio/爆発3_drop.ogg";
pub const GROWL: &'static str = "audio/イノシシの鳴き声.ogg";
pub const PUYON: &'static str = "audio/ぷよん.ogg";
pub const BICHA: &'static str = "audio/お風呂.ogg";
pub const KAWAII: &'static str = "audio/可愛い動作.ogg";
pub const REGISTER: &'static str = "audio/レジスターで精算.ogg";
pub const SHURIKEN: &'static str = "audio/手裏剣を投げる.ogg";
pub const BUS: &'static str = "audio/バスのドアが開く2.ogg";
pub const GLASS: &'static str = "audio/ガラスが割れる2.ogg";
// pub const KAMINARI: &'static str = "audio/雷魔法4.ogg";
// pub const JISHIN: &'static str = "audio/地震魔法2.ogg";
pub const BAKUHATSU: &'static str = "audio/爆発2.ogg";
pub const STATUS2: &'static str = "audio/ステータス上昇魔法2.ogg";
pub const ZOMBIE: &'static str = "audio/ゾンビの食事.ogg";
pub const KAMAE: &'static str = "audio/構えを取る.ogg";
pub const SEN: &'static str = "audio/コルク栓を抜く1.ogg";
pub const DRAGON: &'static str = "audio/ドラゴンの鳴き声2.ogg";
pub const DRAGON_FLUTTER: &'static str = "audio/ドラゴンの羽ばたき.ogg";
pub const SE_TAORERU: &'static str = "audio/倒れる.ogg";
pub const FREEZE: &'static str = "audio/氷魔法で凍結.ogg";
pub const BASHA2: &'static str = "audio/水をバシャッとかける2.ogg";
pub const SUNA: &'static str = "audio/砂の上を走る.ogg";
pub const CHAKUCHI: &'static str = "audio/ジャンプの着地.ogg";
pub const SCENE2: &'static str = "audio/シーン切り替え2.ogg";
pub const KYUSHU2_SHORT: &'static str = "audio/HP吸収魔法2_short.ogg";
pub const KEN2: &'static str = "audio/剣の素振り2.ogg";
pub const HAKKEN: &'static str = "bgm/発見.ogg";
pub const CURSOR_8: &'static str = "audio/カーソル移動8.ogg";
pub const KETTEI_7: &'static str = "audio/決定ボタンを押す7.ogg";

#[derive(Event, Clone)]
pub struct SEEvent {
    path: String,
    position: Option<Vec2>,
}

impl SEEvent {
    pub fn pos<T: Into<String>>(path: T, position: Vec2) -> Self {
        Self {
            path: path.into(),
            position: Some(position),
        }
    }

    pub fn new<T: Into<String>>(path: T) -> Self {
        Self {
            path: path.into(),
            position: None,
        }
    }
}

/// 効果音イベントを順次再生していきます
/// ただし、同一のフレームに同じ効果音が複数回再生されると極端に音が大きくなり不自然なため、
/// 同じ効果音が同時に複数回リクエストされても、最も距離が近いもののみが再生されます
fn se_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
    mut reader: EventReader<SEEvent>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    let camera_position = camera_query
        .get_single()
        .map_or(Vec2::ZERO, |c| c.translation.truncate());

    let mut vec = Vec::<SEEvent>::new();
    for event in reader.read() {
        vec.push(event.clone());
    }

    vec.sort_by(|a, b| {
        let la = (camera_position - a.position.unwrap_or(camera_position)).length_squared();
        let lb = (camera_position - b.position.unwrap_or(camera_position)).length_squared();
        la.partial_cmp(&lb).unwrap_or(Ordering::Equal)
    });

    let mut played = HashSet::<&String>::new();

    for SEEvent { path, position } in vec.iter() {
        if played.contains(path) {
            continue;
        }

        played.insert(path);

        play_se(
            &mut commands,
            &config,
            asset_server.load(path),
            position,
            camera_position,
        );
    }
}

pub struct SECommandPlugin;

impl Plugin for SECommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SEEvent>();
        app.add_systems(Update, se_events.run_if(resource_exists::<GameAssets>));
    }
}
