use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "DotGothic16-Regular.ttf")]
    pub dotgothic: Handle<Font>,

    #[asset(path = "image/atlas.aseprite")]
    pub atlas: Handle<Aseprite>,

    #[asset(path = "image/level.aseprite")]
    pub level: Handle<Aseprite>,

    #[asset(path = "image/title.aseprite")]
    pub title: Handle<Aseprite>,

    #[asset(path = "image/title_witch.aseprite")]
    pub title_witch: Handle<Aseprite>,

    #[asset(path = "image/title_cloud.aseprite")]
    pub title_cloud: Handle<Aseprite>,

    #[asset(path = "image/title_cloud2.aseprite")]
    pub title_cloud2: Handle<Aseprite>,

    #[asset(path = "entity/player.aseprite")]
    pub player: Handle<Aseprite>,

    #[asset(path = "entity/slime.aseprite")]
    pub slime: Handle<Aseprite>,

    #[asset(path = "entity/eyeball.aseprite")]
    pub eyeball: Handle<Aseprite>,

    #[asset(path = "entity/buer.aseprite")]
    pub buer: Handle<Aseprite>,

    #[asset(path = "entity/stone_lantern.aseprite")]
    pub stone_lantern: Handle<Aseprite>,

    #[asset(path = "audio/dageki.ogg")]
    pub dageki: Handle<AudioSource>,

    #[asset(path = "audio/shibafu.ogg")]
    pub shibafu: Handle<AudioSource>,

    #[asset(path = "audio/hiyoko.ogg")]
    pub hiyoko: Handle<AudioSource>,

    #[asset(path = "audio/kuzureru.ogg")]
    pub kuzureru: Handle<AudioSource>,

    #[asset(path = "audio/kettei.ogg")]
    pub kettei: Handle<AudioSource>,

    #[asset(path = "audio/suburi.ogg")]
    pub suburi: Handle<AudioSource>,

    // #[asset(path = "audio/革靴で走る.ogg")]
    // pub kawagutsu: Handle<AudioSource>,
    #[asset(path = "audio/アスファルトの上を歩く2.ogg")]
    pub asphalt: Handle<AudioSource>,

    #[asset(path = "audio/メニューを開く2.ogg")]
    pub menu_open: Handle<AudioSource>,

    #[asset(path = "audio/ワープ.ogg")]
    pub warp: Handle<AudioSource>,

    #[asset(path = "audio/キャンセル9.ogg")]
    pub cancel: Handle<AudioSource>,

    #[asset(path = "audio/体育館で走る.ogg")]
    pub taiikukan: Handle<AudioSource>,

    #[asset(path = "audio/回復魔法1.ogg")]
    pub kaifuku1: Handle<AudioSource>,

    #[asset(path = "audio/カーソル移動2.ogg")]
    pub cursor2: Handle<AudioSource>,

    // #[asset(path = "bgm/they_bgm.ogg")]
    // pub they: Handle<AudioSource>,

    // #[asset(path = "bgm/realm.ogg")]
    // pub gods_realm: Handle<AudioSource>,
    #[asset(path = "bgm/茫漠たる庭.ogg")]
    pub boubaku: Handle<AudioSource>,

    #[asset(path = "bgm/荒れ地の先へ.ogg")]
    pub arechi: Handle<AudioSource>,

    #[asset(path = "bgm/水のしたたる洞窟.ogg")]
    pub dokutsu: Handle<AudioSource>,
}
