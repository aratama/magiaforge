use crate::registry::ActorRegistry;
use crate::registry::GameRegistry;
use crate::registry::SenarioRegistry;
use crate::registry::SpellRegistry;
use crate::registry::TileRegistry;
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

    #[asset(path = "registry.tile.ron")]
    pub tile_registry: Handle<TileRegistry>,

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

    // テクスチャ /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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

    // モンスター //////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(path = "entity/slime.aseprite")]
    pub _slime: Handle<Aseprite>,

    #[asset(path = "entity/friend_slime.aseprite")]
    pub _friend_slime: Handle<Aseprite>,

    #[asset(path = "entity/eyeball.aseprite")]
    pub _eyeball: Handle<Aseprite>,

    #[asset(path = "entity/eyeball_friend.aseprite")]
    pub _eyeball_friend: Handle<Aseprite>,

    #[asset(path = "entity/chicken.aseprite")]
    pub _chicken: Handle<Aseprite>,

    #[asset(path = "entity/sandbag.aseprite")]
    pub _sandbag: Handle<Aseprite>,

    #[asset(path = "entity/shadow.aseprite")]
    pub _shadow: Handle<Aseprite>,

    #[asset(path = "entity/spider.aseprite")]
    pub _spider: Handle<Aseprite>,

    #[asset(path = "entity/salamander.aseprite")]
    pub _salamander: Handle<Aseprite>,

    // ボス ///////////////////////////////////////////////////////////////////////////////////////////////////////

    // #[asset(path = "entity/buer.aseprite")]
    // pub buer: Handle<Aseprite>,
    #[asset(path = "entity/huge_slime.aseprite")]
    pub _huge_slime: Handle<Aseprite>,

    #[asset(path = "entity/huge_slime_shadow.aseprite")]
    pub _huge_slime_shadow: Handle<Aseprite>,

    #[asset(path = "entity/raven.aseprite")]
    pub raven: Handle<Aseprite>,

    // ウサギ //////////////////////////////////////////////////////////////////////////////////////////////////////
    // スプライトは registry ファイルのほうで指定しているのでここでは dead_code になっているが、
    // 先読みのためにここでも含めておく
    // でも通常はタイトル画面を表示しているあいだに読み込みが間に合う？
    #[asset(path = "entity/rabbit_yellow.aseprite")]
    pub _rabbit_yellow: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_red.aseprite")]
    pub _rabbit_red: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_blue.aseprite")]
    pub _rabbit_blue: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_black.aseprite")]
    pub _rabbit_black: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_white.aseprite")]
    pub _rabbit_white: Handle<Aseprite>,

    #[asset(path = "entity/rabbit_green.aseprite")]
    pub _rabbit_green: Handle<Aseprite>,

    // 箱 //////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(path = "entity/chest.aseprite")]
    pub _chest: Handle<Aseprite>,

    #[asset(path = "entity/crate.aseprite")]
    pub _crate: Handle<Aseprite>,

    #[asset(path = "entity/barrel.aseprite")]
    pub _barrel: Handle<Aseprite>,

    #[asset(path = "entity/barrel_bomb.aseprite")]
    pub _barrel_bomb: Handle<Aseprite>,

    #[asset(path = "entity/jar_red.aseprite")]
    pub _jar_red: Handle<Aseprite>,

    #[asset(path = "entity/jar_blue.aseprite")]
    pub _jar_blue: Handle<Aseprite>,

    #[asset(path = "entity/jar_green.aseprite")]
    pub _jar_green: Handle<Aseprite>,

    // 呪文 /////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(path = "entity/freeze.aseprite")]
    pub freeze: Handle<Aseprite>,

    #[asset(path = "entity/slash.aseprite")]
    pub slash: Handle<Aseprite>,

    #[asset(path = "entity/bomb.aseprite")]
    pub _bomb: Handle<Aseprite>,

    #[asset(path = "entity/gold.aseprite")]
    pub gold: Handle<Aseprite>,

    #[asset(path = "entity/fireball.aseprite")]
    pub fireball: Handle<Aseprite>,

    #[asset(path = "entity/fire.aseprite")]
    pub fire: Handle<Aseprite>,

    #[asset(path = "entity/stone_lantern.aseprite")]
    pub _stone_lantern: Handle<Aseprite>,

    #[asset(path = "entity/impact.aseprite")]
    pub impact: Handle<Aseprite>,

    // 効果音 ////////////////////////////////////////////////////////////////////////////
    #[asset(path = "audio/dageki.ogg")]
    pub _dageki: Handle<AudioSource>,

    #[asset(path = "audio/shibafu.ogg")]
    pub _shibafu: Handle<AudioSource>,

    #[asset(path = "audio/hiyoko.ogg")]
    pub _hiyoko: Handle<AudioSource>,

    #[asset(path = "audio/kuzureru.ogg")]
    pub _kuzureru: Handle<AudioSource>,

    #[asset(path = "audio/kettei.ogg")]
    pub _kettei: Handle<AudioSource>,

    #[asset(path = "audio/suburi.ogg")]
    pub _suburi: Handle<AudioSource>,

    #[asset(path = "audio/アスファルトの上を歩く2.ogg")]
    pub _asphalt: Handle<AudioSource>,

    #[asset(path = "audio/メニューを開く2.ogg")]
    pub _menu_open: Handle<AudioSource>,

    #[asset(path = "audio/ワープ.ogg")]
    pub _warp: Handle<AudioSource>,

    #[asset(path = "audio/キャンセル9.ogg")]
    pub _cancel: Handle<AudioSource>,

    #[asset(path = "audio/体育館で走る.ogg")]
    pub taiikukan: Handle<AudioSource>,

    #[asset(path = "audio/回復魔法1.ogg")]
    pub _kaifuku1: Handle<AudioSource>,

    #[asset(path = "audio/カーソル移動2.ogg")]
    pub _cursor2: Handle<AudioSource>,

    #[asset(path = "audio/爆発3_drop.ogg")]
    pub _drop: Handle<AudioSource>,

    #[asset(path = "audio/イノシシの鳴き声.ogg")]
    pub _inoshishi: Handle<AudioSource>,

    #[asset(path = "audio/ぷよん.ogg")]
    pub _puyon: Handle<AudioSource>,

    #[asset(path = "audio/お風呂.ogg")]
    pub _bicha: Handle<AudioSource>,

    #[asset(path = "audio/可愛い動作.ogg")]
    pub _kawaii: Handle<AudioSource>,

    #[asset(path = "audio/レジスターで精算.ogg")]
    pub _register: Handle<AudioSource>,

    #[asset(path = "audio/手裏剣を投げる.ogg")]
    pub _shuriken: Handle<AudioSource>,

    #[asset(path = "audio/バスのドアが開く2.ogg")]
    pub _bus: Handle<AudioSource>,

    #[asset(path = "audio/ガラスが割れる2.ogg")]
    pub _glass: Handle<AudioSource>,

    #[asset(path = "audio/雷魔法4.ogg")]
    pub _kaminari: Handle<AudioSource>,

    #[asset(path = "audio/地震魔法2.ogg")]
    pub _jishin: Handle<AudioSource>,

    #[asset(path = "audio/爆発2.ogg")]
    pub _bakuhatsu: Handle<AudioSource>,

    #[asset(path = "audio/ステータス上昇魔法2.ogg")]
    pub _status2: Handle<AudioSource>,

    #[asset(path = "audio/たき火.ogg")]
    pub takibi: Handle<AudioSource>,

    #[asset(path = "audio/ゾンビの食事.ogg")]
    pub _zombie: Handle<AudioSource>,

    #[asset(path = "audio/構えを取る.ogg")]
    pub _kamae: Handle<AudioSource>,

    #[asset(path = "audio/コルク栓を抜く1.ogg")]
    pub _sen: Handle<AudioSource>,

    #[asset(path = "audio/風が吹く1.ogg")]
    pub _kaze: Handle<AudioSource>,

    #[asset(path = "audio/ドラゴンの鳴き声2.ogg")]
    pub _dragon: Handle<AudioSource>,

    #[asset(path = "audio/ドラゴンの羽ばたき.ogg")]
    pub _dragon_flutter: Handle<AudioSource>,

    #[asset(path = "audio/倒れる.ogg")]
    pub _taoreru: Handle<AudioSource>,

    #[asset(path = "audio/氷魔法で凍結.ogg")]
    pub _freeze_se: Handle<AudioSource>,

    #[asset(path = "audio/水をバシャッとかける2.ogg")]
    pub _basha2: Handle<AudioSource>,

    #[asset(path = "audio/ジャンプの着地.ogg")]
    pub _chakuchi: Handle<AudioSource>,

    #[asset(path = "audio/砂の上を走る.ogg")]
    pub _suna: Handle<AudioSource>,

    #[asset(path = "audio/シーン切り替え2.ogg")]
    pub _scene2: Handle<AudioSource>,

    #[asset(path = "audio/HP吸収魔法2_short.ogg")]
    pub _kyushu2_short: Handle<AudioSource>,

    #[asset(path = "audio/剣の素振り2.ogg")]
    pub _ken2: Handle<AudioSource>,

    #[asset(path = "bgm/発見.ogg")]
    pub _hakken: Handle<AudioSource>,
}
