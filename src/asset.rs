use crate::registry::ActorRegistry;
use crate::registry::GameRegistry;
use crate::registry::SenarioRegistry;
use crate::registry::SpellRegistry;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // 設定 //////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(path = "registry.game.ron")]
    pub game_registry: Handle<GameRegistry>,

    #[asset(path = "registry.spell.ron")]
    pub spell_registry: Handle<SpellRegistry>,

    #[asset(path = "registry.actor.ron")]
    pub actor_registry: Handle<ActorRegistry>,

    #[asset(path = "registry.senario.ron")]
    pub senario_registry: Handle<SenarioRegistry>,

    // フォント //////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(path = "font/NotoSansJP-Medium.ttf")]
    pub noto_sans_jp: Handle<Font>,

    #[asset(path = "font/NotoSansSC-Medium.ttf")]
    pub noto_sans_sc: Handle<Font>,

    #[asset(path = "font/NotoSansSC-Medium.ttf")]
    pub noto_sans_tc: Handle<Font>,

    #[asset(path = "font/NotoSansKR-Medium.ttf")]
    pub noto_sans_kr: Handle<Font>,

    #[asset(path = "image/atlas.aseprite")]
    pub atlas: Handle<Aseprite>,

    #[asset(path = "image/next_page.aseprite")]
    pub next_page: Handle<Aseprite>,

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

    #[asset(path = "image/opening.aseprite")]
    pub opening: Handle<Aseprite>,

    #[asset(path = "image/ending.aseprite")]
    pub ending: Handle<Aseprite>,

    #[asset(path = "entity/witch.aseprite")]
    pub witch: Handle<Aseprite>,

    #[asset(path = "entity/freeze.aseprite")]
    pub freeze: Handle<Aseprite>,

    #[asset(path = "entity/slash.aseprite")]
    pub slash: Handle<Aseprite>,

    // モンスター //////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(path = "entity/slime.aseprite")]
    pub slime: Handle<Aseprite>,

    #[asset(path = "entity/friend_slime.aseprite")]
    pub friend_slime: Handle<Aseprite>,

    #[asset(path = "entity/eyeball.aseprite")]
    pub eyeball: Handle<Aseprite>,

    #[asset(path = "entity/eyeball_friend.aseprite")]
    pub eyeball_friend: Handle<Aseprite>,

    #[asset(path = "entity/chicken.aseprite")]
    pub chicken: Handle<Aseprite>,

    #[asset(path = "entity/sandbug.aseprite")]
    pub sandbug: Handle<Aseprite>,

    #[asset(path = "entity/shadow.aseprite")]
    pub shadow: Handle<Aseprite>,

    #[asset(path = "entity/spider.aseprite")]
    pub spider: Handle<Aseprite>,

    #[asset(path = "entity/salamander.aseprite")]
    pub salamander: Handle<Aseprite>,

    // ボス ///////////////////////////////////////////////////////////////////////////////////////////////////////

    // #[asset(path = "entity/buer.aseprite")]
    // pub buer: Handle<Aseprite>,
    #[asset(path = "entity/huge_slime.aseprite")]
    pub huge_slime: Handle<Aseprite>,

    #[asset(path = "entity/huge_slime_shadow.aseprite")]
    pub huge_slime_shadow: Handle<Aseprite>,

    #[asset(path = "entity/raven.aseprite")]
    pub raven: Handle<Aseprite>,

    // ウサギ //////////////////////////////////////////////////////////////////////////////////////////////////////

    //
    #[asset(path = "entity/rabbit_yellow.aseprite")]
    pub rabbit_yellow: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_red.aseprite")]
    pub rabbit_red: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_blue.aseprite")]
    pub rabbit_blue: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_black.aseprite")]
    pub rabbit_black: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_white.aseprite")]
    pub rabbit_white: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_green.aseprite")]
    pub rabbit_green: Handle<Aseprite>,

    // 呪文 /////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(path = "entity/bomb.aseprite")]
    pub bomb: Handle<Aseprite>,

    #[asset(path = "entity/gold.aseprite")]
    pub gold: Handle<Aseprite>,

    #[asset(path = "entity/fireball.aseprite")]
    pub fireball: Handle<Aseprite>,

    #[asset(path = "entity/fire.aseprite")]
    pub fire: Handle<Aseprite>,

    #[asset(path = "entity/stone_lantern.aseprite")]
    pub stone_lantern: Handle<Aseprite>,

    #[asset(path = "entity/impact.aseprite")]
    pub impact: Handle<Aseprite>,

    // 効果音 ////////////////////////////////////////////////////////////////////////////
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

    #[asset(path = "audio/手裏剣を投げる.ogg")]
    pub shuriken: Handle<AudioSource>,

    #[asset(path = "audio/バスのドアが開く2.ogg")]
    pub bus: Handle<AudioSource>,

    #[asset(path = "audio/ガラスが割れる2.ogg")]
    pub glass: Handle<AudioSource>,

    #[asset(path = "audio/雷魔法4.ogg")]
    pub kaminari: Handle<AudioSource>,

    #[asset(path = "audio/地震魔法2.ogg")]
    pub jishin: Handle<AudioSource>,

    #[asset(path = "audio/爆発2.ogg")]
    pub bakuhatsu: Handle<AudioSource>,

    #[asset(path = "audio/ステータス上昇魔法2.ogg")]
    pub status2: Handle<AudioSource>,

    #[asset(path = "audio/たき火.ogg")]
    pub takibi: Handle<AudioSource>,

    #[asset(path = "audio/ゾンビの食事.ogg")]
    pub zombie: Handle<AudioSource>,

    #[asset(path = "audio/構えを取る.ogg")]
    pub kamae: Handle<AudioSource>,

    #[asset(path = "audio/コルク栓を抜く1.ogg")]
    pub sen: Handle<AudioSource>,

    #[asset(path = "audio/風が吹く1.ogg")]
    pub kaze: Handle<AudioSource>,

    #[asset(path = "audio/ドラゴンの鳴き声2.ogg")]
    pub dragon: Handle<AudioSource>,

    #[asset(path = "audio/ドラゴンの羽ばたき.ogg")]
    pub dragon_flutter: Handle<AudioSource>,

    #[asset(path = "audio/倒れる.ogg")]
    pub taoreru: Handle<AudioSource>,

    #[asset(path = "audio/氷魔法で凍結.ogg")]
    pub freeze_se: Handle<AudioSource>,

    #[asset(path = "audio/水をバシャッとかける2.ogg")]
    pub basha2: Handle<AudioSource>,

    #[asset(path = "audio/ジャンプの着地.ogg")]
    pub chakuchi: Handle<AudioSource>,

    #[asset(path = "audio/砂の上を走る.ogg")]
    pub suna: Handle<AudioSource>,

    #[asset(path = "audio/シーン切り替え2.ogg")]
    pub scene2: Handle<AudioSource>,

    #[asset(path = "audio/HP吸収魔法2_short.ogg")]
    pub kyushu2_short: Handle<AudioSource>,

    #[asset(path = "audio/剣の素振り2.ogg")]
    pub ken2: Handle<AudioSource>,

    #[asset(path = "bgm/発見.ogg")]
    pub hakken: Handle<AudioSource>,
}
