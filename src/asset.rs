use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

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

    #[asset(path = "image/ending.aseprite")]
    pub ending: Handle<Aseprite>,

    #[asset(path = "entity/player.aseprite")]
    pub player: Handle<Aseprite>,

    #[asset(path = "entity/slime.aseprite")]
    pub slime: Handle<Aseprite>,

    #[asset(path = "entity/friend_slime.aseprite")]
    pub friend_slime: Handle<Aseprite>,

    #[asset(path = "entity/eyeball.aseprite")]
    pub eyeball: Handle<Aseprite>,

    // #[asset(path = "entity/buer.aseprite")]
    // pub buer: Handle<Aseprite>,
    #[asset(path = "entity/huge_slime.aseprite")]
    pub huge_slime: Handle<Aseprite>,

    #[asset(path = "entity/huge_slime_shadow.aseprite")]
    pub huge_slime_shadow: Handle<Aseprite>,

    #[asset(path = "entity/stone_lantern.aseprite")]
    pub stone_lantern: Handle<Aseprite>,

    #[asset(path = "entity/impact.aseprite")]
    pub impact: Handle<Aseprite>,

    #[asset(path = "entity/rabbit.aseprite")]
    pub rabbit: Handle<Aseprite>,

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

    // #[asset(path = "audio/カーソル移動8.ogg")]
    // pub cursor8: Handle<AudioSource>,
    //
    #[asset(path = "audio/爆発3_drop.ogg")]
    pub drop: Handle<AudioSource>,

    #[asset(path = "audio/イノシシの鳴き声.ogg")]
    pub inoshishi: Handle<AudioSource>,

    #[asset(path = "audio/ぷよん.ogg")]
    pub puyon: Handle<AudioSource>,

    #[asset(path = "audio/お風呂.ogg")]
    pub bicha: Handle<AudioSource>,

    #[asset(path = "audio/可愛い動作.ogg")]
    pub kawaii: Handle<AudioSource>,

    #[asset(path = "audio/レジスターで精算.ogg")]
    pub register: Handle<AudioSource>,

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

    // 通常フロア ////////////////////////////////////////////////////////////////
    #[asset(path = "bgm/Tides_of_Adventure.ogg")]
    pub ending_bgm: Handle<AudioSource>,

    #[asset(path = "bgm/ダンジョンを踏破せし者.ogg")]
    pub touha: Handle<AudioSource>,

    #[asset(path = "bgm/森のいざない.ogg")]
    pub mori: Handle<AudioSource>,

    #[asset(path = "bgm/迷宮.ogg")]
    pub meikyu: Handle<AudioSource>,

    #[asset(path = "bgm/忘れられた神殿.ogg")]
    pub shiden: Handle<AudioSource>,

    #[asset(path = "bgm/midnight-forest-184304.ogg")]
    pub midnight_forest: Handle<AudioSource>,

    // ボス //////////////////////////////////////////////////////////////////////
    #[asset(path = "bgm/悪魔との戦闘.ogg")]
    pub deamon: Handle<AudioSource>,

    #[asset(path = "bgm/アクション・バトル.ogg")]
    pub action: Handle<AudioSource>,

    #[asset(path = "bgm/Decisive_Battle.ogg")]
    pub decisive: Handle<AudioSource>,

    #[asset(path = "bgm/炎神の吐息.ogg")]
    pub enjin: Handle<AudioSource>,

    #[asset(path = "bgm/Sacred_Sacrifice.ogg")]
    pub sacred: Handle<AudioSource>,

    #[asset(path = "bgm/battle-cinematic-trailer-royalty-free-music-210434.ogg")]
    pub battle_cinematic: Handle<AudioSource>,

    #[asset(path = "bgm/battle-fight-music-dynamic-warrior-background-intro-theme-272176.ogg")]
    pub battle_fight: Handle<AudioSource>,

    #[asset(path = "bgm/final-battle-trailer-music-217488.ogg")]
    pub final_battle: Handle<AudioSource>,

    #[asset(path = "bgm/human-vs-machine-dark-orchestral-cinematic-epic-action-271968.ogg")]
    pub human_vs_machine: Handle<AudioSource>,
}
