use crate::ldtk::loader::LDTK;
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
    #[asset(key = "registry.game.ron")]
    pub game_registry: Handle<GameRegistry>,

    #[asset(key = "registry.spell.ron")]
    pub spell_registry: Handle<SpellRegistry>,

    #[asset(key = "registry.actor.ron")]
    pub actor_registry: Handle<ActorRegistry>,

    #[asset(key = "registry.tile.ron")]
    pub tile_registry: Handle<TileRegistry>,

    #[asset(key = "registry.senario.ron")]
    pub senario_registry: Handle<SenarioRegistry>,

    // フォント //////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(key = "font/NotoSansJP-Medium.ttf")]
    pub noto_sans_jp: Handle<Font>,

    #[asset(key = "font/NotoSansSC-Medium.ttf")]
    pub noto_sans_sc: Handle<Font>,

    #[asset(key = "font/NotoSansSC-Medium.ttf")]
    pub noto_sans_tc: Handle<Font>,

    #[asset(key = "font/NotoSansKR-Medium.ttf")]
    pub noto_sans_kr: Handle<Font>,

    // テクスチャ /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(key = "image/atlas.aseprite")]
    pub atlas: Handle<Aseprite>,

    #[asset(key = "image/next_page.aseprite")]
    pub next_page: Handle<Aseprite>,

    #[asset(key = "level.ldtk")]
    pub ldtk_level: Handle<LDTK>,

    #[asset(key = "image/title.aseprite")]
    pub title: Handle<Aseprite>,

    #[asset(key = "image/title_witch.aseprite")]
    pub title_witch: Handle<Aseprite>,

    #[asset(key = "image/title_cloud.aseprite")]
    pub title_cloud: Handle<Aseprite>,

    #[asset(key = "image/title_cloud2.aseprite")]
    pub title_cloud2: Handle<Aseprite>,

    #[asset(key = "image/opening.aseprite")]
    pub opening: Handle<Aseprite>,

    #[asset(key = "image/ending.aseprite")]
    pub ending: Handle<Aseprite>,

    // モンスター //////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(key = "enemy/slime.aseprite")]
    pub _slime: Handle<Aseprite>,

    #[asset(key = "enemy/friend_slime.aseprite")]
    pub _friend_slime: Handle<Aseprite>,

    #[asset(key = "enemy/eyeball.aseprite")]
    pub _eyeball: Handle<Aseprite>,

    #[asset(key = "enemy/eyeball_friend.aseprite")]
    pub _eyeball_friend: Handle<Aseprite>,

    #[asset(key = "enemy/shadow.aseprite")]
    pub _shadow: Handle<Aseprite>,

    #[asset(key = "enemy/spider.aseprite")]
    pub _spider: Handle<Aseprite>,

    #[asset(key = "enemy/salamander.aseprite")]
    pub _salamander: Handle<Aseprite>,

    // #[asset(key = "entity/buer.aseprite")]
    // pub buer: Handle<Aseprite>,
    #[asset(key = "enemy/huge_slime.aseprite")]
    pub _huge_slime: Handle<Aseprite>,

    #[asset(key = "enemy/huge_slime_shadow.aseprite")]
    pub _huge_slime_shadow: Handle<Aseprite>,

    // そのほかのアクター ///////////////////////////////////////////////////////////////////////////////////////////
    #[asset(key = "actor/witch.aseprite")]
    pub witch: Handle<Aseprite>,

    #[asset(key = "actor/chicken.aseprite")]
    pub _chicken: Handle<Aseprite>,

    #[asset(key = "actor/sandbag.aseprite")]
    pub _sandbag: Handle<Aseprite>,

    #[asset(key = "actor/bomb.aseprite")]
    pub _bomb: Handle<Aseprite>,

    #[asset(key = "actor/stone_lantern.aseprite")]
    pub _stone_lantern: Handle<Aseprite>,

    // ウサギ //////////////////////////////////////////////////////////////////////////////////////////////////////
    // スプライトは registry ファイルのほうで指定しているのでここでは dead_code になっているが、
    // 先読みのためにここでも含めておく
    // でも通常はタイトル画面を表示しているあいだに読み込みが間に合う？
    #[asset(key = "rabbit/rabbit_yellow.aseprite")]
    pub _rabbit_yellow: Handle<Aseprite>,

    #[asset(key = "rabbit/rabbit_red.aseprite")]
    pub _rabbit_red: Handle<Aseprite>,

    #[asset(key = "rabbit/rabbit_blue.aseprite")]
    pub _rabbit_blue: Handle<Aseprite>,

    #[asset(key = "rabbit/rabbit_black.aseprite")]
    pub _rabbit_black: Handle<Aseprite>,

    #[asset(key = "rabbit/rabbit_white.aseprite")]
    pub _rabbit_white: Handle<Aseprite>,

    #[asset(key = "rabbit/rabbit_green.aseprite")]
    pub _rabbit_green: Handle<Aseprite>,

    // 箱 //////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(key = "chest/chest.aseprite")]
    pub _chest: Handle<Aseprite>,

    #[asset(key = "chest/crate.aseprite")]
    pub _crate: Handle<Aseprite>,

    #[asset(key = "chest/barrel.aseprite")]
    pub _barrel: Handle<Aseprite>,

    #[asset(key = "chest/barrel_bomb.aseprite")]
    pub _barrel_bomb: Handle<Aseprite>,

    #[asset(key = "chest/jar_red.aseprite")]
    pub _jar_red: Handle<Aseprite>,

    #[asset(key = "chest/jar_blue.aseprite")]
    pub _jar_blue: Handle<Aseprite>,

    #[asset(key = "chest/jar_green.aseprite")]
    pub _jar_green: Handle<Aseprite>,

    // そのほかのエンティティ ///////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(key = "entity/raven.aseprite")]
    pub raven: Handle<Aseprite>,

    // 呪文 /////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[asset(key = "entity/freeze.aseprite")]
    pub freeze: Handle<Aseprite>,

    #[asset(key = "entity/slash.aseprite")]
    pub slash: Handle<Aseprite>,

    #[asset(key = "entity/gold.aseprite")]
    pub gold: Handle<Aseprite>,

    #[asset(key = "entity/fireball.aseprite")]
    pub _fireball: Handle<Aseprite>,

    #[asset(key = "entity/fire.aseprite")]
    pub fire: Handle<Aseprite>,

    #[asset(key = "entity/impact.aseprite")]
    pub impact: Handle<Aseprite>,

    // 効果音 ////////////////////////////////////////////////////////////////////////////
    #[asset(key = "audio/dageki.ogg")]
    pub _dageki: Handle<AudioSource>,

    #[asset(key = "audio/shibafu.ogg")]
    pub _shibafu: Handle<AudioSource>,

    #[asset(key = "audio/hiyoko.ogg")]
    pub _hiyoko: Handle<AudioSource>,

    #[asset(key = "audio/kuzureru.ogg")]
    pub _kuzureru: Handle<AudioSource>,

    #[asset(key = "audio/kettei.ogg")]
    pub _kettei: Handle<AudioSource>,

    #[asset(key = "audio/suburi.ogg")]
    pub _suburi: Handle<AudioSource>,

    #[asset(key = "audio/アスファルトの上を歩く2.ogg")]
    pub _asphalt: Handle<AudioSource>,

    #[asset(key = "audio/メニューを開く2.ogg")]
    pub _menu_open: Handle<AudioSource>,

    #[asset(key = "audio/ワープ.ogg")]
    pub _warp: Handle<AudioSource>,

    #[asset(key = "audio/キャンセル9.ogg")]
    pub _cancel: Handle<AudioSource>,

    #[asset(key = "audio/体育館で走る.ogg")]
    pub taiikukan: Handle<AudioSource>,

    #[asset(key = "audio/回復魔法1.ogg")]
    pub _kaifuku1: Handle<AudioSource>,

    #[asset(key = "audio/カーソル移動2.ogg")]
    pub _cursor2: Handle<AudioSource>,

    #[asset(key = "audio/爆発3_drop.ogg")]
    pub _drop: Handle<AudioSource>,

    #[asset(key = "audio/イノシシの鳴き声.ogg")]
    pub _inoshishi: Handle<AudioSource>,

    #[asset(key = "audio/ぷよん.ogg")]
    pub _puyon: Handle<AudioSource>,

    #[asset(key = "audio/お風呂.ogg")]
    pub _bicha: Handle<AudioSource>,

    #[asset(key = "audio/可愛い動作.ogg")]
    pub _kawaii: Handle<AudioSource>,

    #[asset(key = "audio/レジスターで精算.ogg")]
    pub _register: Handle<AudioSource>,

    #[asset(key = "audio/手裏剣を投げる.ogg")]
    pub _shuriken: Handle<AudioSource>,

    #[asset(key = "audio/バスのドアが開く2.ogg")]
    pub _bus: Handle<AudioSource>,

    #[asset(key = "audio/ガラスが割れる2.ogg")]
    pub _glass: Handle<AudioSource>,

    #[asset(key = "audio/雷魔法4.ogg")]
    pub _kaminari: Handle<AudioSource>,

    #[asset(key = "audio/地震魔法2.ogg")]
    pub _jishin: Handle<AudioSource>,

    #[asset(key = "audio/爆発2.ogg")]
    pub _bakuhatsu: Handle<AudioSource>,

    #[asset(key = "audio/ステータス上昇魔法2.ogg")]
    pub _status2: Handle<AudioSource>,

    #[asset(key = "audio/たき火.ogg")]
    pub takibi: Handle<AudioSource>,

    #[asset(key = "audio/ゾンビの食事.ogg")]
    pub _zombie: Handle<AudioSource>,

    #[asset(key = "audio/構えを取る.ogg")]
    pub _kamae: Handle<AudioSource>,

    #[asset(key = "audio/コルク栓を抜く1.ogg")]
    pub _sen: Handle<AudioSource>,

    #[asset(key = "audio/風が吹く1.ogg")]
    pub _kaze: Handle<AudioSource>,

    #[asset(key = "audio/ドラゴンの鳴き声2.ogg")]
    pub _dragon: Handle<AudioSource>,

    #[asset(key = "audio/ドラゴンの羽ばたき.ogg")]
    pub _dragon_flutter: Handle<AudioSource>,

    #[asset(key = "audio/倒れる.ogg")]
    pub _taoreru: Handle<AudioSource>,

    #[asset(key = "audio/氷魔法で凍結.ogg")]
    pub _freeze_se: Handle<AudioSource>,

    #[asset(key = "audio/水をバシャッとかける2.ogg")]
    pub _basha2: Handle<AudioSource>,

    #[asset(key = "audio/ジャンプの着地.ogg")]
    pub _chakuchi: Handle<AudioSource>,

    #[asset(key = "audio/砂の上を走る.ogg")]
    pub _suna: Handle<AudioSource>,

    #[asset(key = "audio/シーン切り替え2.ogg")]
    pub _scene2: Handle<AudioSource>,

    #[asset(key = "audio/HP吸収魔法2_short.ogg")]
    pub _kyushu2_short: Handle<AudioSource>,

    #[asset(key = "audio/剣の素振り2.ogg")]
    pub _ken2: Handle<AudioSource>,

    #[asset(key = "bgm/発見.ogg")]
    pub _hakken: Handle<AudioSource>,

    #[asset(key = "audio/カーソル移動8.ogg")]
    pub _cursor_8: Handle<AudioSource>,

    #[asset(key = "audio/決定ボタンを押す7.ogg")]
    pub _kettei_7: Handle<AudioSource>,

    #[asset(key = "audio/決定ボタンを押す31.ogg")]
    pub _kettei_31: Handle<AudioSource>,
}
