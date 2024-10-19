use bevy::asset::*;
// use bevy::asset::{embedded_asset, io::AssetSourceId, AssetPath};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;
// use std::path::Path;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "asset.aseprite")]
    pub asset: Handle<Aseprite>,

    #[asset(path = "level.aseprite")]
    pub level: Handle<Aseprite>,

    #[asset(path = "entity/player.aseprite")]
    pub player: Handle<Aseprite>,

    #[asset(path = "entity/slime.aseprite")]
    pub slime: Handle<Aseprite>,

    #[asset(path = "title.aseprite")]
    pub title: Handle<Aseprite>,

    #[asset(path = "audio/dageki.ogg")]
    pub dageki: Handle<AudioSource>,

    #[asset(path = "audio/shibafu.ogg")]
    pub shibafu: Handle<AudioSource>,

    #[asset(path = "audio/hiyoko.ogg")]
    pub hiyoko: Handle<AudioSource>,

    #[asset(path = "audio/They.ogg")]
    pub they: Handle<AudioSource>,
}

// TODO:
// 現状ではアセットの埋め込みはしていません
// ビルドに時間がかかりすぎたり GitHub Pagesに配置するのに支障が出るかもしれないので、
// 埋め込みをするかは検討中
//
// // https://bevyengine.org/examples/assets/embedded-asset/
// アセットのパスは URL のような形式になっており、
// プロトコルは embedded、それ以下のパスは crate 名とアセットのパスになります
// embedded_asset 側の設定で、パスの game は省略されます
// 実際のパスは Window では例えば、 embedded://my_bevy_game\asset/asset.aseprite になります
// let path = Path::new(CRATE_NAME).join("asset/asset.aseprite");
// let asset_path = AssetPath::from_path(&path).with_source(AssetSourceId::from("embedded"));

// pub struct EmbeddedAssetPlugin;

// impl Plugin for EmbeddedAssetPlugin {
//     fn build(&self, app: &mut App) {
//         // We get to choose some prefix relative to the workspace root which
//         // will be ignored in "embedded://" asset paths.
//         let omit_prefix = "";
//         // Path to asset must be relative to this file, because that's how
//         // include_bytes! works.
//         embedded_asset!(app, omit_prefix, "../../assets/asset.aseprite");

//         let path = embedded_path!("../../", "asset.aseprite");
//         println!("path: {:?}", path);
//     }
// }

// // TODO: ちゃんと動作の検証してない
// #[allow(dead_code)]
// pub fn get_embbeded_asset<A: Asset>(asset_server: Res<AssetServer>, name: &str) -> Handle<A> {
//     const MODULE_PATH: &'static str = module_path!();
//     let crate_name = MODULE_PATH
//         .split_once("::")
//         .map_or(MODULE_PATH, |(first, _)| first);

//     let path = Path::new(crate_name).join(name);
//     let source = AssetSourceId::from("embedded");
//     let asset_path = AssetPath::from_path(&path).with_source(source);

//     asset_server.load(asset_path)
// }
