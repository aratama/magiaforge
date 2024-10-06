// https://bevyengine.org/examples/assets/embedded-asset/
// use bevy::asset::*;
use bevy::prelude::*;

pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, _app: &mut App) {
        // 省略するプリフィックス
        // 埋め込むアセットは src 以下に配置します
        // 読み込むファイルはみんな src/game 以下にあり game を省略すると便利なので、二番目の引数に "game" を指定しています

        // let omit_prefix = "game";

        // バイナリにアセットを埋め込む場合は、src/game/asset 以下にファイルを配置し、
        // 以下のようにして embedded_asset を呼びます
        // ただし、 world.rs にコメントした理由で、world.ldtk を埋め込むことができないため、
        // 現状ではアセットは埋め込まずにファイルを読み込む形になっています
        //
        // embedded_asset!(app, omit_prefix, "asset/asset.aseprite");
        // embedded_asset!(app, omit_prefix, "asset/player.aseprite");
        // embedded_asset!(app, omit_prefix, "asset/world.ldtk");

        // 埋め込まれるアセットのパスは以下のコードで出力できます
        // println!(
        //     "embedded_path {:?}",
        //     embedded_path!(omit_prefix, "asset/asset.aseprite")
        // );
    }
}
